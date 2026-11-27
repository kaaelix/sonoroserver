use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::assets_db::AssetsDb;

const MAX_CACHED_OWNERS: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct ByteSpan {
    pub offset: u64,
    pub len: u32,
}

#[derive(Debug, Clone)]
pub struct NodeEntry {
    pub id: i32,
    pub parent_id: i32,
    pub node_type: String,
    pub count: i32,
    pub reward_id: i32,
    pub data: Value,
}

#[derive(Default)]
struct CacheState {
    slots: HashMap<i32, Arc<Vec<NodeEntry>>>,
    order: VecDeque<i32>,
}

#[derive(Default)]
pub struct LazyNodeStore {
    db: Option<Arc<AssetsDb>>,
    db_owners: HashSet<i32>,
    has_db_owners: bool,
    kind: i32,
    file_path: PathBuf,
    owner_spans: HashMap<i32, Vec<ByteSpan>>,
    cache: Mutex<CacheState>,
}

impl LazyNodeStore {
    pub fn with_db(db: Arc<AssetsDb>, kind: i32, file_path: impl Into<PathBuf>) -> Self {
        let (db_owners, has_db_owners) = match db.query_node_tree_owners(kind) {
            Ok(ids) => (ids.into_iter().collect(), true),
            Err(_) => (HashSet::new(), false),
        };
        Self {
            db: Some(db),
            db_owners,
            has_db_owners,
            kind,
            file_path: file_path.into(),
            owner_spans: HashMap::new(),
            cache: Mutex::new(CacheState::default()),
        }
    }

    pub fn from_file(kind: i32, file_path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = file_path.into();
        let mut file = File::open(&path)?;
        let mut read_buf = [0u8; 64 * 1024];
        let mut owner_spans: HashMap<i32, Vec<ByteSpan>> = HashMap::new();

        let mut in_string = false;
        let mut escaped = false;
        let mut depth: i32 = 0;
        let mut obj_start: u64 = 0;
        let mut in_object = false;
        let mut header_buf: Vec<u8> = Vec::with_capacity(512);
        let mut file_offset: u64 = 0;

        loop {
            let n = file.read(&mut read_buf)?;
            if n == 0 {
                break;
            }
            for (i, &ch) in read_buf[..n].iter().enumerate() {
                let abs_pos = file_offset + i as u64;
                if in_object && header_buf.len() < 512 {
                    header_buf.push(ch);
                }
                if in_string {
                    if escaped {
                        escaped = false;
                    } else if ch == b'\\' {
                        escaped = true;
                    } else if ch == b'"' {
                        in_string = false;
                    }
                    continue;
                }
                match ch {
                    b'"' => in_string = true,
                    b'{' => {
                        if depth == 0 {
                            in_object = true;
                            obj_start = abs_pos;
                            header_buf.clear();
                            header_buf.push(b'{');
                        }
                        depth += 1;
                    }
                    b'}' => {
                        if depth > 0 {
                            depth -= 1;
                        }
                        if depth == 0 && in_object {
                            in_object = false;
                            let obj_end = abs_pos + 1;
                            let obj_len_u64 = obj_end - obj_start;
                            let owner_opt = parse_owner_id(&header_buf).or_else(|| {
                                if obj_len_u64 <= 512 || obj_len_u64 > 1024 * 1024 {
                                    return None;
                                }
                                let saved = file.stream_position().ok()?;
                                file.seek(SeekFrom::Start(obj_start)).ok()?;
                                let mut full_buf = vec![0u8; obj_len_u64 as usize];
                                file.read_exact(&mut full_buf).ok()?;
                                let _ = file.seek(SeekFrom::Start(saved));
                                parse_owner_id(&full_buf)
                            });
                            if let Some(owner_id) = owner_opt
                                && let Ok(obj_len) = u32::try_from(obj_len_u64)
                            {
                                owner_spans.entry(owner_id).or_default().push(ByteSpan {
                                    offset: obj_start,
                                    len: obj_len,
                                });
                            }
                            header_buf.clear();
                        }
                    }
                    _ => {}
                }
            }
            file_offset += n as u64;
        }

        Ok(Self {
            db: None,
            db_owners: HashSet::new(),
            has_db_owners: false,
            kind,
            file_path: path,
            owner_spans,
            cache: Mutex::new(CacheState::default()),
        })
    }

    pub fn has_owner(&self, owner_id: i32) -> bool {
        if self.db.is_some() {
            if self.has_db_owners {
                return self.db_owners.contains(&owner_id);
            }
            if let Some(db) = &self.db {
                return db.has_node_tree_owner(self.kind, owner_id);
            }
        }
        self.owner_spans.contains_key(&owner_id)
    }

    pub fn get_owner_nodes(&self, owner_id: i32) -> std::io::Result<Arc<Vec<NodeEntry>>> {
        let mut cache = self
            .cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some(hit) = cache.slots.get(&owner_id) {
            return Ok(Arc::clone(hit));
        }

        let mut nodes = Vec::new();
        if let Some(db) = &self.db {
            let raw_slices = db
                .query_node_tree_data_slices(self.kind, owner_id)
                .map_err(std::io::Error::other)?;
            if raw_slices.is_empty() {
                let arc = Arc::new(Vec::new());
                if cache.order.len() >= MAX_CACHED_OWNERS
                    && let Some(oldest) = cache.order.pop_front()
                {
                    cache.slots.remove(&oldest);
                }
                cache.order.push_back(owner_id);
                cache.slots.insert(owner_id, Arc::clone(&arc));
                return Ok(arc);
            }
            for buf in raw_slices {
                let data_val: Value = serde_json::from_str(&buf)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                append_node_flattened(&mut nodes, data_val, None);
            }
        } else {
            let Some(spans) = self.owner_spans.get(&owner_id) else {
                return Ok(Arc::new(Vec::new()));
            };
            let mut file = File::open(&self.file_path)?;
            for span in spans {
                file.seek(SeekFrom::Start(span.offset))?;
                let mut buf = vec![0u8; span.len as usize];
                file.read_exact(&mut buf)?;
                let parsed: Value = serde_json::from_slice(&buf)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
                if let Some(data_val) = parsed.as_object().and_then(|o| o.get("Data").cloned()) {
                    append_node_flattened(&mut nodes, data_val, None);
                }
            }
        }

        normalize_sequence_child_order(owner_id, &mut nodes);
        let arc = Arc::new(nodes);

        if cache.order.len() >= MAX_CACHED_OWNERS
            && let Some(oldest) = cache.order.pop_front()
        {
            cache.slots.remove(&oldest);
        }
        cache.order.push_back(owner_id);
        cache.slots.insert(owner_id, Arc::clone(&arc));
        Ok(arc)
    }
}

pub fn append_node_flattened(
    nodes: &mut Vec<NodeEntry>,
    node_val: Value,
    override_parent: Option<i32>,
) {
    let Some(obj) = node_val.as_object() else {
        return;
    };
    let Some(id) = obj.get("Id").and_then(Value::as_i64).and_then(|v| i32::try_from(v).ok()) else {
        return;
    };
    let parent_id = override_parent.unwrap_or_else(|| {
        obj.get("ParentNodeId")
            .and_then(Value::as_i64)
            .and_then(|v| i32::try_from(v).ok())
            .unwrap_or(0)
    });
    let node_type = obj
        .get("Type")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let count = obj
        .get("Count")
        .and_then(Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
        .unwrap_or(1);
    let reward_id = obj
        .get("RewardId")
        .and_then(Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
        .unwrap_or(0);

    let children_vals = obj
        .get("Children")
        .and_then(Value::as_array)
        .cloned();
    let slots_nodes: Option<Vec<Value>> = obj.get("Slots").and_then(Value::as_array).map(|arr| {
        arr.iter()
            .filter_map(|s| s.as_object()?.get("Node").cloned())
            .collect()
    });

    nodes.push(NodeEntry {
        id,
        parent_id,
        node_type,
        count,
        reward_id,
        data: node_val,
    });

    if let Some(children) = children_vals {
        for child_val in children {
            append_node_flattened(nodes, child_val, Some(id));
        }
    }
    if let Some(slot_nodes) = slots_nodes {
        for slot_node in slot_nodes {
            append_node_flattened(nodes, slot_node, Some(id));
        }
    }
}

fn parse_tid_tip_key(owner_id: i32, data: &Value) -> Option<u64> {
    let tid = data.as_object()?.get("TidTip")?.as_str()?;
    let last_us = tid.rfind('_')?;
    let lo = tid[last_us + 1..].parse::<u32>().ok()?;
    let hi = tid[..last_us]
        .rfind('_')
        .and_then(|prev| tid[prev + 1..last_us].parse::<u32>().ok())
        .unwrap_or(0);
    if owner_id >= 1_000_000 {
        let owner_prefix = (owner_id / 1_000_000) as u32;
        if hi != 0 && hi != owner_prefix {
            return None;
        }
    }
    Some((u64::from(hi) << 32) | u64::from(lo))
}

fn find_teleport_dungeon_target(val: &Value) -> Option<i32> {
    match val {
        Value::Object(obj) => {
            if obj.get("Name").and_then(Value::as_str) == Some("TeleportDungeon")
                && let Some(did) = obj
                    .get("Params")
                    .and_then(Value::as_object)
                    .and_then(|p| p.get("DungeonId"))
                    .and_then(Value::as_i64)
                    .and_then(|v| i32::try_from(v).ok())
                && did > 0
                && did != 8
            {
                return Some(did);
            }
            for (k, v) in obj {
                if k == "Children" || k == "Slots" {
                    continue;
                }
                if let Some(did) = find_teleport_dungeon_target(v) {
                    return Some(did);
                }
            }
            None
        }
        Value::Array(arr) => arr.iter().find_map(find_teleport_dungeon_target),
        _ => None,
    }
}

fn parse_dungeon_id(data: &Value) -> i32 {
    let Some(did) = data
        .as_object()
        .and_then(|o| o.get("DungeonId"))
        .and_then(Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
    else {
        return 0;
    };
    if did > 0 && did != 8 { did } else { 0 }
}

fn has_teleport_dungeon_finish_action(data: &Value) -> bool {
    let Some(arr) = data
        .as_object()
        .and_then(|o| o.get("FinishActions"))
        .and_then(Value::as_array)
    else {
        return false;
    };
    arr.iter().any(|act| {
        act.as_object()
            .and_then(|o| o.get("Name"))
            .and_then(Value::as_str)
            == Some("TeleportDungeon")
    })
}

fn collect_awake_entities_from_actions(out: &mut Vec<i32>, acts_opt: Option<&Value>) {
    let Some(arr) = acts_opt.and_then(Value::as_array) else {
        return;
    };
    for act in arr {
        let Some(aobj) = act.as_object() else {
            continue;
        };
        if aobj.get("Name").and_then(Value::as_str) != Some("AwakeEntity") {
            continue;
        }
        let Some(p) = aobj.get("Params").and_then(Value::as_object) else {
            continue;
        };
        if let Some(eid) = p
            .get("EntityId")
            .and_then(Value::as_i64)
            .and_then(|v| i32::try_from(v).ok())
            && eid > 0
        {
            out.push(eid);
        }
        if let Some(eids) = p.get("EntityIds").and_then(Value::as_array) {
            for ev in eids {
                if let Some(eid) = ev.as_i64().and_then(|v| i32::try_from(v).ok())
                    && eid > 0
                {
                    out.push(eid);
                }
            }
        }
    }
}

fn json_contains_entity_id(val: &Value, target: i32) -> bool {
    match val {
        Value::Number(n) => n.as_i64() == Some(i64::from(target)),
        Value::Object(obj) => obj.iter().any(|(k, v)| {
            if matches!(
                k.as_str(),
                "EntityId" | "EntityIds" | "TargetsToAwake" | "ExistTargets" | "RangeEntities"
            ) || v.is_object()
                || v.is_array()
            {
                json_contains_entity_id(v, target)
            } else {
                false
            }
        }),
        Value::Array(arr) => arr.iter().any(|item| json_contains_entity_id(item, target)),
        _ => false,
    }
}

fn collect_subtree_awakened_entities(nodes: &[NodeEntry], root_id: i32, out: &mut Vec<i32>) {
    for n in nodes {
        if n.id != root_id {
            continue;
        }
        if n.node_type == "AlwaysFalse" {
            return;
        }
        if let Some(obj) = n.data.as_object() {
            collect_awake_entities_from_actions(out, obj.get("EnterActions"));
            collect_awake_entities_from_actions(out, obj.get("Actions"));
        }
        break;
    }
    for child in nodes {
        if child.parent_id == root_id && child.node_type != "AlwaysFalse" {
            collect_subtree_awakened_entities(nodes, child.id, out);
        }
    }
}

fn subtree_condition_references_any(nodes: &[NodeEntry], root_id: i32, targets: &[i32]) -> bool {
    if targets.is_empty() {
        return false;
    }
    for n in nodes {
        if n.id != root_id {
            continue;
        }
        if n.node_type == "AlwaysFalse" {
            return false;
        }
        if let Some(cond_val) = n.data.as_object().and_then(|o| o.get("Condition")) {
            for &tid in targets {
                if json_contains_entity_id(cond_val, tid) {
                    return true;
                }
            }
        }
        break;
    }
    for child in nodes {
        if child.parent_id == root_id
            && child.node_type != "AlwaysFalse"
            && subtree_condition_references_any(nodes, child.id, targets)
        {
            return true;
        }
    }
    false
}

fn is_terminal_quest_node(node_type: &str) -> bool {
    node_type == "QuestSucceed" || node_type == "QuestFailed"
}

fn manual_sequence_order(owner_id: i32, parent_id: i32) -> Option<&'static [i32]> {
    if owner_id == 145000005 {
        return match parent_id {
            24 => Some(&[36, 585, 37, 304, 414, 501, 349, 306, 361]),
            304 => Some(&[303, 413]),
            306 => Some(&[422, 309]),
            361 => Some(&[310, 210, 41, 42]),
            _ => None,
        };
    }
    None
}

#[derive(Clone)]
struct ChildSlot {
    entry: NodeEntry,
    tid_key: Option<u64>,
    tp_dungeon: i32,
    sub_dungeon: i32,
    orig_pos: usize,
    paired: bool,
}

#[derive(Clone)]
struct TidSlot {
    tid_key: u64,
    tp_dungeon: i32,
    sub_dungeon: i32,
    orig_pos: usize,
    entry: NodeEntry,
}

pub fn normalize_sequence_child_order(owner_id: i32, nodes: &mut [NodeEntry]) {
    if nodes.len() <= 1 {
        return;
    }

    let mut min_tids: Vec<Option<u64>> = vec![None; nodes.len()];
    let mut teleport_dungeons: Vec<i32> = vec![0; nodes.len()];
    let mut subtree_dungeons: Vec<i32> = vec![0; nodes.len()];

    for (i, node) in nodes.iter().enumerate() {
        if node.node_type == "AlwaysFalse" || has_teleport_dungeon_finish_action(&node.data) {
            min_tids[i] = None;
        } else {
            min_tids[i] = parse_tid_tip_key(owner_id, &node.data);
        }
        if node.node_type == "AlwaysFalse" {
            teleport_dungeons[i] = 0;
            subtree_dungeons[i] = 0;
        } else {
            teleport_dungeons[i] = find_teleport_dungeon_target(&node.data).unwrap_or(0);
            subtree_dungeons[i] = parse_dungeon_id(&node.data);
        }
    }

    for _ in 0..nodes.len() {
        let mut changed = false;
        for (i, node) in nodes.iter().enumerate() {
            if node.node_type == "AlwaysFalse" || node.parent_id <= 0 {
                continue;
            }
            let child_key = min_tids[i];
            let child_tp = teleport_dungeons[i];
            let child_dung = subtree_dungeons[i];
            for (pi, p) in nodes.iter().enumerate() {
                if p.id != node.parent_id || p.node_type == "AlwaysFalse" {
                    continue;
                }
                if let Some(ck) = child_key
                    && (min_tids[pi].is_none() || Some(ck) < min_tids[pi])
                {
                    min_tids[pi] = Some(ck);
                    changed = true;
                }
                if child_tp > 0 && teleport_dungeons[pi] == 0 {
                    teleport_dungeons[pi] = child_tp;
                    changed = true;
                }
                if child_dung > 0 && subtree_dungeons[pi] == 0 {
                    subtree_dungeons[pi] = child_dung;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    let mut seq_parents: Vec<(i32, bool)> = Vec::new();
    for p in nodes.iter() {
        let is_seq = p.node_type == "Sequence";
        let is_sel = p.node_type == "Select";
        if !is_seq && !is_sel {
            continue;
        }
        if !seq_parents.iter().any(|&(sid, _)| sid == p.id) {
            seq_parents.push((p.id, is_seq));
        }
    }

    for (parent_id, is_sequence) in seq_parents {
        let mut positions = Vec::new();
        let mut non_term = Vec::new();
        let mut term = Vec::new();

        for (i, node) in nodes.iter().enumerate() {
            if node.parent_id != parent_id {
                continue;
            }
            positions.push(i);
            let slot = ChildSlot {
                entry: node.clone(),
                tid_key: min_tids[i],
                tp_dungeon: teleport_dungeons[i],
                sub_dungeon: subtree_dungeons[i],
                orig_pos: i,
                paired: false,
            };
            if is_sequence && is_terminal_quest_node(&node.node_type) {
                term.push(slot);
            } else {
                non_term.push(slot);
            }
        }
        if positions.len() <= 1 {
            continue;
        }

        if let Some(order) = manual_sequence_order(owner_id, parent_id) {
            let rank = |id: i32| order.iter().position(|&x| x == id).unwrap_or(order.len());
            non_term.sort_by(|a, b| {
                rank(a.entry.id)
                    .cmp(&rank(b.entry.id))
                    .then_with(|| a.orig_pos.cmp(&b.orig_pos))
            });
        } else {
            let mut tagged_indices = Vec::new();
            let mut tagged_items = Vec::new();
            for (idx, c) in non_term.iter().enumerate() {
                if let Some(tk) = c.tid_key {
                    tagged_indices.push(idx);
                    tagged_items.push(TidSlot {
                        tid_key: tk,
                        tp_dungeon: c.tp_dungeon,
                        sub_dungeon: c.sub_dungeon,
                        orig_pos: c.orig_pos,
                        entry: c.entry.clone(),
                    });
                }
            }
            if tagged_items.len() > 1 {
                tagged_items.sort_by(|a, b| {
                    a.tid_key
                        .cmp(&b.tid_key)
                        .then_with(|| a.orig_pos.cmp(&b.orig_pos))
                });
                for (slot_idx, sorted_item) in tagged_indices.into_iter().zip(tagged_items) {
                    non_term[slot_idx] = ChildSlot {
                        entry: sorted_item.entry,
                        tid_key: Some(sorted_item.tid_key),
                        tp_dungeon: sorted_item.tp_dungeon,
                        sub_dungeon: sorted_item.sub_dungeon,
                        orig_pos: sorted_item.orig_pos,
                        paired: false,
                    };
                }
            }

            if is_sequence && non_term.len() > 1 {
                let mut k = 0usize;
                while k < non_term.len() {
                    let tp_did = non_term[k].tp_dungeon;
                    if tp_did > 0
                        && let Some(m) = non_term
                            .iter()
                            .enumerate()
                            .find(|&(m, cand)| m != k && !cand.paired && cand.sub_dungeon == tp_did)
                            .map(|(m, _)| m)
                    {
                        let mut target = non_term.remove(m);
                        target.paired = true;
                        let insert_idx = if m < k { k } else { k + 1 };
                        non_term.insert(insert_idx, target);
                        k = insert_idx;
                    }
                    k += 1;
                }

                for _ in 0..non_term.len() {
                    let mut moved_any = false;
                    let mut j = 1usize;
                    while j < non_term.len() {
                        let mut producer_eids = Vec::new();
                        collect_subtree_awakened_entities(
                            nodes,
                            non_term[j].entry.id,
                            &mut producer_eids,
                        );
                        if !producer_eids.is_empty() {
                            let mut earliest_consumer = None;
                            for (i, slot_i) in non_term.iter().enumerate().take(j) {
                                if subtree_condition_references_any(
                                    nodes,
                                    slot_i.entry.id,
                                    &producer_eids,
                                ) {
                                    let mut consumer_awakens = Vec::new();
                                    collect_subtree_awakened_entities(
                                        nodes,
                                        slot_i.entry.id,
                                        &mut consumer_awakens,
                                    );
                                    if !producer_eids
                                        .iter()
                                        .any(|pe| consumer_awakens.contains(pe))
                                    {
                                        earliest_consumer = Some(i);
                                        break;
                                    }
                                }
                            }
                            if let Some(dest_i) = earliest_consumer {
                                let item = non_term.remove(j);
                                non_term.insert(dest_i, item);
                                moved_any = true;
                            }
                        }
                        j += 1;
                    }
                    if !moved_any {
                        break;
                    }
                }
            }
        }

        for (write_i, c) in non_term.into_iter().chain(term).enumerate() {
            let dst = positions[write_i];
            nodes[dst] = c.entry;
            min_tids[dst] = c.tid_key;
            teleport_dungeons[dst] = c.tp_dungeon;
            subtree_dungeons[dst] = c.sub_dungeon;
        }
    }
}

fn parse_owner_id(header: &[u8]) -> Option<i32> {
    let s = std::str::from_utf8(header).ok()?;
    let pos = s.find("\"Key\"")?;
    let after = &s[pos + "\"Key\"".len()..];
    let colon = after.find(':')?;
    let open_q = after[colon + 1..].find('"')?;
    let rest = &after[colon + 1 + open_q + 1..];
    let underscore = rest.find('_')?;
    rest[..underscore].parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_sequence_child_order_pairs_teleport_dungeon_with_sibling_sequence() {
        let raw_nodes = [
            r#"{"Type":"Sequence","Id":2,"ParentNodeId":1}"#,
            r#"{"Type":"Sequence","Id":13,"ParentNodeId":2}"#,
            r#"{"Type":"ChildQuest","Id":20,"ParentNodeId":13,"TidTip":"Quest_145000005_ChildQuestTip_145_10"}"#,
            r#"{"Type":"Sequence","Id":24,"ParentNodeId":2,"DungeonId":904}"#,
            r#"{"Type":"ChildQuest","Id":36,"ParentNodeId":24,"TidTip":"Quest_145000005_ChildQuestTip_145_32"}"#,
            r#"{"Type":"Sequence","Id":26,"ParentNodeId":2}"#,
            r#"{"Type":"ChildQuest","Id":27,"ParentNodeId":26,"TidTip":"Quest_145000005_ChildQuestTip_145_23"}"#,
            r#"{"Type":"ParallelSelect","Id":190,"ParentNodeId":2}"#,
            r#"{"Type":"ChildQuest","Id":405,"ParentNodeId":190,"TidTip":"Quest_145000005_ChildQuestTip_145_20","Condition":{"Type":"DoInteract","AddOptions":[{"EntityId":145750144,"Option":{"Type":{"Type":"Actions","Actions":[{"Name":"TeleportDungeon","Params":{"DungeonId":904}}]}}}]}}"#,
        ];
        let mut nodes = Vec::new();
        for raw in raw_nodes {
            let val: Value = serde_json::from_str(raw).unwrap();
            append_node_flattened(&mut nodes, val, None);
        }
        normalize_sequence_child_order(145000005, &mut nodes);
        let child_order: Vec<i32> = nodes
            .iter()
            .filter(|n| n.parent_id == 2)
            .map(|n| n.id)
            .collect();
        assert_eq!(child_order, vec![13, 190, 24, 26]);
    }

    #[test]
    fn normalize_sequence_child_order_keeps_145000005_n24_doors_before_climax() {
        let raw_nodes = [
            r#"{"Type":"Sequence","Id":24,"ParentNodeId":2,"DungeonId":904}"#,
            r#"{"Type":"ChildQuest","Id":36,"ParentNodeId":24,"TidTip":"Quest_145000005_ChildQuestTip_145_32"}"#,
            r#"{"Type":"ChildQuest","Id":37,"ParentNodeId":24,"TidTip":"Quest_145000005_ChildQuestTip_145_33"}"#,
            r#"{"Type":"Sequence","Id":304,"ParentNodeId":24}"#,
            r#"{"Type":"ChildQuest","Id":303,"ParentNodeId":304,"TidTip":"Quest_145000005_ChildQuestTip_145_211"}"#,
            r#"{"Type":"ChildQuest","Id":413,"ParentNodeId":304,"TidTip":"Quest_145000005_ChildQuestTip_114_11"}"#,
            r#"{"Type":"Sequence","Id":306,"ParentNodeId":24}"#,
            r#"{"Type":"ChildQuest","Id":309,"ParentNodeId":306,"TidTip":"Quest_145000005_ChildQuestTip_145_215"}"#,
            r#"{"Type":"ChildQuest","Id":422,"ParentNodeId":306,"TidTip":"Quest_145000005_ChildQuestTip_114_22"}"#,
            r#"{"Type":"Sequence","Id":349,"ParentNodeId":24}"#,
            r#"{"Type":"Sequence","Id":361,"ParentNodeId":24}"#,
            r#"{"Type":"ChildQuest","Id":41,"ParentNodeId":361,"TidTip":"Quest_145000005_ChildQuestTip_145_37"}"#,
            r#"{"Type":"ChildQuest","Id":42,"ParentNodeId":361,"TidTip":"Quest_145000005_ChildQuestTip_145_38"}"#,
            r#"{"Type":"ChildQuest","Id":210,"ParentNodeId":361,"TidTip":"Quest_145000005_ChildQuestTip_145_156"}"#,
            r#"{"Type":"ChildQuest","Id":310,"ParentNodeId":361,"TidTip":"Quest_145000005_ChildQuestTip_145_216"}"#,
            r#"{"Type":"ChildQuest","Id":414,"ParentNodeId":24,"TidTip":"Quest_145000005_ChildQuestTip_114_12"}"#,
            r#"{"Type":"ParallelSelect","Id":501,"ParentNodeId":24}"#,
            r#"{"Type":"ParallelSelect","Id":585,"ParentNodeId":24}"#,
        ];
        let mut nodes = Vec::new();
        for raw in raw_nodes {
            let val: Value = serde_json::from_str(raw).unwrap();
            append_node_flattened(&mut nodes, val, None);
        }
        normalize_sequence_child_order(145000005, &mut nodes);
        let n24: Vec<i32> = nodes
            .iter()
            .filter(|n| n.parent_id == 24)
            .map(|n| n.id)
            .collect();
        let n361: Vec<i32> = nodes
            .iter()
            .filter(|n| n.parent_id == 361)
            .map(|n| n.id)
            .collect();
        assert_eq!(n24, vec![36, 585, 37, 304, 414, 501, 349, 306, 361]);
        assert_eq!(n361, vec![310, 210, 41, 42]);
    }
}
