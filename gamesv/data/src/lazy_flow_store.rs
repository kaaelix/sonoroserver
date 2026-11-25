use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::assets_db::AssetsDb;

const MAX_CACHED_FLOWS: usize = 1024;

#[derive(Debug, Clone, Copy)]
pub struct ByteSpan {
    pub offset: u64,
    pub len: u32,
}

#[derive(Default)]
struct CacheState {
    slots: HashMap<String, Arc<Value>>,
    order: VecDeque<String>,
}

#[derive(Default)]
pub struct LazyFlowStore {
    db: Option<Arc<AssetsDb>>,
    file_path: PathBuf,
    state_spans: HashMap<String, ByteSpan>,
    cache: Mutex<CacheState>,
}

#[derive(Default)]
struct JsonObjectScanner {
    in_string: bool,
    escaped: bool,
    depth: i32,
    in_object: bool,
    obj_start: u64,
    header_buf: Vec<u8>,
}

impl JsonObjectScanner {
    fn feed(
        &mut self,
        chunk: &[u8],
        chunk_offset: u64,
        full_src: Option<&[u8]>,
        mut file: Option<&mut File>,
        spans: &mut HashMap<String, ByteSpan>,
    ) -> std::io::Result<()> {
        for (i, &ch) in chunk.iter().enumerate() {
            let abs_pos = chunk_offset + i as u64;
            if self.in_object && self.header_buf.len() < 4096 {
                self.header_buf.push(ch);
            }
            if self.in_string {
                if self.escaped {
                    self.escaped = false;
                } else if ch == b'\\' {
                    self.escaped = true;
                } else if ch == b'"' {
                    self.in_string = false;
                }
                continue;
            }
            match ch {
                b'"' => self.in_string = true,
                b'{' => {
                    if self.depth == 0 {
                        self.in_object = true;
                        self.obj_start = abs_pos;
                        self.header_buf.clear();
                        self.header_buf.push(b'{');
                    }
                    self.depth += 1;
                }
                b'}' => {
                    if self.depth > 0 {
                        self.depth -= 1;
                    }
                    if self.depth == 0 && self.in_object {
                        self.in_object = false;
                        let obj_end = abs_pos + 1;
                        let obj_len = obj_end - self.obj_start;
                        let key_opt = extract_state_key(&self.header_buf).or_else(|| {
                            if obj_len <= 4096 || obj_len > 2 * 1024 * 1024 {
                                return None;
                            }
                            if let Some(src) = full_src {
                                let slice = src.get(self.obj_start as usize..obj_end as usize)?;
                                return extract_state_key(slice);
                            }
                            let f = file.as_deref_mut()?;
                            f.seek(SeekFrom::Start(self.obj_start)).ok()?;
                            let mut buf = vec![0u8; obj_len as usize];
                            f.read_exact(&mut buf).ok()?;
                            extract_state_key(&buf)
                        });
                        if let Some(key) = key_opt
                            && let Ok(len_u32) = u32::try_from(obj_len)
                        {
                            spans.insert(
                                key,
                                ByteSpan {
                                    offset: self.obj_start,
                                    len: len_u32,
                                },
                            );
                        }
                        self.header_buf.clear();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl LazyFlowStore {
    pub fn with_db(db: Arc<AssetsDb>, file_path: impl Into<PathBuf>) -> Self {
        Self {
            db: Some(db),
            file_path: file_path.into(),
            state_spans: HashMap::new(),
            cache: Mutex::new(CacheState::default()),
        }
    }

    pub fn from_file(file_path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = file_path.into();
        let mut f = File::open(&path)?;
        let mut buf = [0u8; 64 * 1024];
        let mut scanner = JsonObjectScanner::default();
        let mut spans = HashMap::new();
        let mut offset = 0u64;
        loop {
            let n = f.read(&mut buf)?;
            if n == 0 {
                break;
            }
            scanner.feed(&buf[..n], offset, None, Some(&mut f), &mut spans)?;
            offset += n as u64;
        }
        Ok(Self {
            db: None,
            file_path: path,
            state_spans: spans,
            cache: Mutex::new(CacheState::default()),
        })
    }

    pub fn has_state_key(&self, state_key: &str) -> bool {
        if let Some(db) = &self.db {
            return db
                .query_flow_state_json(state_key)
                .ok()
                .flatten()
                .is_some();
        }
        self.state_spans.contains_key(state_key)
    }

    pub fn get_flow_state(
        &self,
        flow_list_name: &str,
        flow_id: i32,
        state_id: i32,
    ) -> std::io::Result<Option<Arc<Value>>> {
        if flow_list_name.is_empty() || flow_id <= 0 || state_id <= 0 {
            return Ok(None);
        }
        let key = format!("{flow_list_name}_{flow_id}_{state_id}");
        self.get_state_by_key(&key)
    }

    pub fn get_state_by_key(&self, state_key: &str) -> std::io::Result<Option<Arc<Value>>> {
        let mut cache = self
            .cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(hit) = cache.slots.get(state_key) {
            return Ok(Some(Arc::clone(hit)));
        }

        let raw_json: String = if let Some(db) = &self.db {
            match db.query_flow_state_json(state_key) {
                Ok(Some(s)) => s,
                Ok(None) => return Ok(None),
                Err(e) => return Err(std::io::Error::other(e)),
            }
        } else if let Some(span) = self.state_spans.get(state_key) {
            let mut f = File::open(&self.file_path)?;
            f.seek(SeekFrom::Start(span.offset))?;
            let mut bytes = vec![0u8; span.len as usize];
            f.read_exact(&mut bytes)?;
            String::from_utf8(bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        } else {
            return Ok(None);
        };

        let parsed: Value = serde_json::from_str(&raw_json)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let arc = Arc::new(parsed);

        if cache.order.len() >= MAX_CACHED_FLOWS
            && let Some(oldest) = cache.order.pop_front()
        {
            cache.slots.remove(&oldest);
        }
        let key_owned = state_key.to_string();
        cache.order.push_back(key_owned.clone());
        cache.slots.insert(key_owned, Arc::clone(&arc));
        Ok(Some(arc))
    }
}

pub fn find_action_by_id(val: &Value, target_action_id: i32) -> Option<&Value> {
    match val {
        Value::Object(obj) => {
            if obj.contains_key("Name")
                && let Some(id_val) = obj.get("ActionId")
                && id_val.as_i64() == Some(i64::from(target_action_id))
            {
                return Some(val);
            }
            for child in obj.values() {
                if let Some(found) = find_action_by_id(child, target_action_id) {
                    return Some(found);
                }
            }
            None
        }
        Value::Array(arr) => arr
            .iter()
            .find_map(|item| find_action_by_id(item, target_action_id)),
        _ => None,
    }
}

pub fn collect_set_entity_visible_ids(val: &Value, out: &mut Vec<i32>) {
    match val {
        Value::Object(obj) => {
            if let Some(name) = obj.get("Name").and_then(Value::as_str)
                && let Some(Value::Object(params)) = obj.get("Params")
            {
                match name {
                    "SetEntityVisible" | "AwakeEntity" | "ChangeEntityState" => {
                        append_entity_id_list(params.get("EntityIds"), out);
                        append_single_entity_id(params.get("EntityId"), out);
                    }
                    "SetEntityPos" => {
                        append_single_entity_id(params.get("EntityId"), out);
                    }
                    "EntityLookAt" => {
                        append_single_entity_id(params.get("EntityId"), out);
                        append_single_entity_id(params.get("TargetEntityId"), out);
                    }
                    "BeginFlowTemplate" => {
                        if let Some(Value::Array(actors)) = params.get("Actors") {
                            for actor in actors {
                                if let Some(aobj) = actor.as_object() {
                                    append_single_entity_id(aobj.get("EntityId"), out);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            for child in obj.values() {
                collect_set_entity_visible_ids(child, out);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_set_entity_visible_ids(item, out);
            }
        }
        _ => {}
    }
}

pub fn does_flow_switch_main_role(val: &Value) -> bool {
    match val {
        Value::Object(obj) => {
            if obj.get("IsSwitchMainRole").and_then(Value::as_bool) == Some(true) {
                return true;
            }
            if obj.get("Name").and_then(Value::as_str) == Some("BeginFlowTemplate")
                && let Some(Value::Object(pv)) = obj.get("Params")
                && let Some(Value::Array(actors)) = pv.get("Actors")
            {
                for actor in actors {
                    let Some(aobj) = actor.as_object() else {
                        continue;
                    };
                    let is_player = aobj
                        .get("IsPlayer")
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                    let eid = parse_json_int(aobj.get("EntityId")).unwrap_or(0);
                    if is_player || eid == -1 {
                        return true;
                    }
                }
            }
            obj.values().any(does_flow_switch_main_role)
        }
        Value::Array(arr) => arr.iter().any(does_flow_switch_main_role),
        _ => false,
    }
}

pub fn collect_awake_entity_ids(val: &Value, out: &mut Vec<i32>) {
    match val {
        Value::Object(obj) => {
            if let Some(name) = obj.get("Name").and_then(Value::as_str)
                && let Some(Value::Object(p)) = obj.get("Params")
            {
                match name {
                    "AwakeEntity" => {
                        append_single_entity_id(p.get("EntityId"), out);
                        append_entity_id_list(p.get("EntityIds"), out);
                    }
                    "SetEntityVisible" => {
                        let visible = p
                            .get("Visible")
                            .and_then(Value::as_bool)
                            .unwrap_or(true);
                        if visible {
                            append_single_entity_id(p.get("EntityId"), out);
                            append_entity_id_list(p.get("EntityIds"), out);
                        }
                    }
                    "BeginFlowTemplate" => {
                        if let Some(Value::Array(actors)) = p.get("Actors") {
                            for actor in actors {
                                if let Some(aobj) = actor.as_object() {
                                    append_single_entity_id(aobj.get("EntityId"), out);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            for child in obj.values() {
                collect_awake_entity_ids(child, out);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                collect_awake_entity_ids(item, out);
            }
        }
        _ => {}
    }
}

fn parse_json_int(v_opt: Option<&Value>) -> Option<i32> {
    let v = v_opt?;
    match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i32::try_from(i).ok()
            } else if let Some(f) = n.as_f64() {
                if f.is_finite() { Some(f as i32) } else { None }
            } else {
                None
            }
        }
        Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    }
}

fn append_single_entity_id(v_opt: Option<&Value>, out: &mut Vec<i32>) {
    let Some(eid) = parse_json_int(v_opt) else {
        return;
    };
    if eid > 0 && !out.contains(&eid) {
        out.push(eid);
    }
}

fn append_entity_id_list(ids_val_opt: Option<&Value>, out: &mut Vec<i32>) {
    let Some(Value::Array(arr)) = ids_val_opt else {
        return;
    };
    for id_item in arr {
        append_single_entity_id(Some(id_item), out);
    }
}

fn extract_state_key(header: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(header).ok()?;
    let pos = s.find("\"StateKey\"")?;
    let after = &s[pos + "\"StateKey\"".len()..];
    let colon = after.find(':')?;
    let after_colon = &after[colon + 1..];
    let open_q = after_colon.find('"')?;
    let rest = &after_colon[open_q + 1..];
    let bytes = rest.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            return Some(rest[..i].to_string());
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_state_key_spans_across_chunk_boundaries_and_extracts_nested_actions() {
        let sample = br#"[
  {
    "StateKey": "plot_ce_1_12",
    "Id": 12,
    "Actions": [
      {
        "Name": "ShowTalk",
        "ActionId": 5000013,
        "Params": {
          "TalkItems": [
            {
              "Id": 7,
              "Actions": [
                {
                  "Name": "SetEntityVisible",
                  "ActionId": 5000157,
                  "Params": { "Visible": true, "EntityIds": [139000395] }
                }
              ]
            }
          ]
        }
      },
      {
        "Name": "SetEntityVisible",
        "ActionId": 5000007,
        "Params": { "Visible": false, "EntityIds": [122002966] }
      }
    ]
  }
]"#;

        let mut spans = HashMap::new();
        let mut scanner = JsonObjectScanner::default();
        let mut offset = 0usize;
        while offset < sample.len() {
            let end = (offset + 17).min(sample.len());
            scanner
                .feed(
                    &sample[offset..end],
                    offset as u64,
                    Some(sample),
                    None,
                    &mut spans,
                )
                .unwrap();
            offset = end;
        }

        let span = spans.get("plot_ce_1_12").unwrap();
        let slice = &sample[span.offset as usize..span.offset as usize + span.len as usize];
        let parsed: Value = serde_json::from_slice(slice).unwrap();

        let nested = find_action_by_id(&parsed, 5000157).unwrap();
        assert_eq!(nested["Name"].as_str(), Some("SetEntityVisible"));
        let top = find_action_by_id(&parsed, 5000007).unwrap();
        assert_eq!(top["Name"].as_str(), Some("SetEntityVisible"));
        assert!(find_action_by_id(&parsed, 999999).is_none());

        let mut entity_ids = Vec::new();
        collect_set_entity_visible_ids(&parsed, &mut entity_ids);
        assert_eq!(entity_ids, vec![139000395, 122002966]);
    }

    #[test]
    fn collects_awake_entity_change_state_and_begin_flow_template_actors() {
        let parsed: Value = serde_json::from_str(
            r#"{
              "StateKey": "plot_test_1_1",
              "Actions": [
                {
                  "Name": "BeginFlowTemplate",
                  "Params": {
                    "IsSwitchMainRole": true,
                    "Actors": [
                      { "EntityId": -1, "IsPlayer": true, "TalkerId": 750088 },
                      { "EntityId": 145750135 }
                    ]
                  }
                },
                { "Name": "AwakeEntity", "Params": { "EntityIds": [145000246, 145750147] } },
                { "Name": "ChangeEntityState", "Params": { "EntityId": 147900228 } }
              ]
            }"#,
        )
        .unwrap();

        let mut entity_ids = Vec::new();
        collect_set_entity_visible_ids(&parsed, &mut entity_ids);
        assert_eq!(entity_ids, vec![145750135, 145000246, 145750147, 147900228]);
        assert!(does_flow_switch_main_role(&parsed));

        let mut awake_ids = Vec::new();
        collect_awake_entity_ids(&parsed, &mut awake_ids);
        assert_eq!(awake_ids, vec![145750135, 145000246, 145750147]);
    }

    #[test]
    fn collect_awake_entity_ids_skips_hide_and_destroy() {
        let parsed: Value = serde_json::from_str(
            r#"{
              "StateKey": "plot_test_1_2",
              "Actions": [
                { "Name": "SetEntityVisible", "Params": { "Visible": false, "EntityIds": [101, 102] } },
                { "Name": "DestroyEntity", "Params": { "EntityIds": [103] } },
                { "Name": "SetEntityVisible", "Params": { "Visible": true, "EntityIds": [104] } },
                { "Name": "AwakeEntity", "Params": { "EntityId": 105 } }
              ]
            }"#,
        )
        .unwrap();

        let mut awake_ids = Vec::new();
        collect_awake_entity_ids(&parsed, &mut awake_ids);
        assert_eq!(awake_ids, vec![104, 105]);
    }
}
