use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::assets_db::AssetsDb;
use crate::data_tables::{DataTables, TableRef};
use crate::tables::blueprint_config::{BlueprintConfig, EntityLogic, EntityType};
use crate::tables::entity_components::extra::{EntityGroupComponent, ExtraEntry, ExtraKind};
use crate::tables::instance_dungeon::InstanceDungeon;
use crate::tables::level_entity_config::LevelEntityConfig;
use crate::tables::level_play_data::LevelPlayData;
use crate::tables::teleporter::Teleporter;
use crate::tables::template_config::TemplateConfig;

pub const SPATIAL_CELL_SIZE: f32 = 5000.0;
pub const MAX_SPATIAL_QUERY_CELLS: usize = 169;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapEntityKey {
    pub map_id: i32,
    pub entity_id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpatialCellKey {
    pub map_id: i32,
    pub cell_x: i32,
    pub cell_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellRange {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

pub struct SpanTable<K: Eq + Hash> {
    spans: HashMap<K, Span>,
    values: Vec<u32>,
}

impl<K: Eq + Hash> Default for SpanTable<K> {
    fn default() -> Self {
        Self {
            spans: HashMap::new(),
            values: Vec::new(),
        }
    }
}

impl<K: Eq + Hash + Copy> SpanTable<K> {
    pub fn from_pairs(pairs: impl IntoIterator<Item = (K, u32)>) -> Self {
        let mut counts: HashMap<K, u32> = HashMap::new();
        let collected: Vec<(K, u32)> = pairs.into_iter().collect();
        for &(k, _) in &collected {
            *counts.entry(k).or_insert(0) += 1;
        }
        let mut total = 0u32;
        let mut spans: HashMap<K, Span> = HashMap::with_capacity(counts.len());
        let mut cursors: HashMap<K, u32> = HashMap::with_capacity(counts.len());
        for (k, count) in counts {
            spans.insert(k, Span { start: total, len: count });
            cursors.insert(k, total);
            total += count;
        }
        let mut values = vec![0u32; total as usize];
        for (k, v) in collected {
            if let Some(pos) = cursors.get_mut(&k) {
                values[*pos as usize] = v;
                *pos += 1;
            }
        }
        Self { spans, values }
    }

    pub fn get(&self, key: K) -> &[u32] {
        let Some(span) = self.spans.get(&key) else {
            return &[];
        };
        let start = span.start as usize;
        &self.values[start..start + span.len as usize]
    }
}

pub struct Inputs<'a> {
    pub level_entity_configs: &'a [LevelEntityConfig],
    pub blueprint_configs: &'a [BlueprintConfig],
    pub template_configs: &'a [TemplateConfig],
    pub teleporters: &'a [Teleporter],
    pub level_play_data: &'a [LevelPlayData],
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EntityQuickMeta {
    pub entity_id: i64,
    pub area_id: i32,
    pub in_sleep: bool,
    pub is_hidden: bool,
}

#[derive(Debug, Clone)]
pub struct EntityGroupSpec {
    pub group_entity_id: i64,
    pub component: EntityGroupComponent,
}

#[derive(Default)]
pub struct AssetIndexes {
    pub entity_by_map_pair: HashMap<MapEntityKey, u32>,
    pub entity_by_id: SpanTable<i64>,
    pub entity_meta_by_idx: Vec<EntityQuickMeta>,
    pub spawnables_by_map_cell: SpanTable<SpatialCellKey>,
    pub spawnables_by_map: SpanTable<i32>,
    pub passerby_spawners_by_map: SpanTable<i32>,
    pub teleporters_by_map: SpanTable<i32>,
    pub teleporters_by_area: SpanTable<i32>,
    pub level_plays_by_entity: SpanTable<i32>,
    pub level_plays_by_ref_entity: SpanTable<i32>,
    pub level_plays_by_reward_entity: SpanTable<i32>,
    pub non_spawnable_enter_lps: Vec<u32>,
    pub blueprint_by_type: HashMap<String, u32>,
    pub template_by_type: HashMap<String, u32>,
    pub entity_groups_by_member: HashMap<MapEntityKey, Vec<EntityGroupSpec>>,
    pub action_opened_functions: HashSet<i32>,
}

impl AssetIndexes {
    pub fn from_tables(tables: &DataTables) -> Self {
        let mut out = Self::from_slices(Inputs {
            level_entity_configs: tables.level_entity_config.as_slice(),
            blueprint_configs: tables.blueprint_config.as_slice(),
            template_configs: tables.template_config.as_slice(),
            teleporters: tables.teleporter.as_slice(),
            level_play_data: tables.level_play_data.as_slice(),
        });
        build_teleporter_area_index(
            tables.teleporter.as_slice(),
            tables.instance_dungeon.as_slice(),
            &mut out,
        );
        out
    }

    pub fn from_db(db: &AssetsDb, tables: &DataTables) -> Result<Self, String> {
        let mut out = Self::default();
        build_string_index_bp(tables.blueprint_config.as_slice(), &mut out.blueprint_by_type);
        build_string_index_tpl(tables.template_config.as_slice(), &mut out.template_by_type);
        build_level_entity_indexes_from_db(db, tables, &mut out).map_err(|e| e.to_string())?;
        build_teleporter_index(tables.teleporter.as_slice(), &mut out);
        build_teleporter_area_index(
            tables.teleporter.as_slice(),
            tables.instance_dungeon.as_slice(),
            &mut out,
        );
        build_level_play_indexes(tables.level_play_data.as_slice(), &mut out);
        if let Ok(fids) = db.query_action_opened_function_ids() {
            out.action_opened_functions.extend(fids);
        }
        Ok(out)
    }

    pub fn from_slices(inputs: Inputs<'_>) -> Self {
        let mut out = Self::default();
        build_string_index_bp(inputs.blueprint_configs, &mut out.blueprint_by_type);
        build_string_index_tpl(inputs.template_configs, &mut out.template_by_type);
        build_level_entity_indexes(
            inputs.level_entity_configs,
            inputs.blueprint_configs,
            inputs.template_configs,
            &mut out,
        );
        build_teleporter_index(inputs.teleporters, &mut out);
        build_teleporter_area_index(inputs.teleporters, &[], &mut out);
        build_level_play_indexes(inputs.level_play_data, &mut out);
        out
    }

    pub fn quick_entity_meta(&self, idx: u32) -> Option<EntityQuickMeta> {
        let meta = *self.entity_meta_by_idx.get(idx as usize)?;
        (meta.entity_id > 0).then_some(meta)
    }

    pub fn find_level_entity<'a>(
        &self,
        tables: &'a DataTables,
        map_id: i32,
        entity_id: i64,
    ) -> Option<TableRef<'a, LevelEntityConfig>> {
        let &idx = self
            .entity_by_map_pair
            .get(&MapEntityKey { map_id, entity_id })?;
        if let Some(db) = tables.assets_db() {
            return db.get_level_entity_by_idx(idx).map(TableRef::Owned);
        }
        tables
            .level_entity_config
            .as_slice()
            .get(idx as usize)
            .map(TableRef::Borrowed)
    }

    pub fn level_entity_indices_by_id(&self, entity_id: i64) -> &[u32] {
        self.entity_by_id.get(entity_id)
    }

    pub fn teleporter_indices_for_map(&self, map_id: i32) -> &[u32] {
        self.teleporters_by_map.get(map_id)
    }

    pub fn teleporter_indices_for_area(&self, area_id: i32) -> &[u32] {
        self.teleporters_by_area.get(area_id)
    }

    pub fn spawnable_indices_for_map_cell(
        &self,
        map_id: i32,
        cell_x: i32,
        cell_y: i32,
    ) -> &[u32] {
        self.spawnables_by_map_cell.get(SpatialCellKey {
            map_id,
            cell_x,
            cell_y,
        })
    }

    pub fn spawnable_indices_for_map(&self, map_id: i32) -> &[u32] {
        self.spawnables_by_map.get(map_id)
    }

    pub fn passerby_spawner_indices_for_map(&self, map_id: i32) -> &[u32] {
        self.passerby_spawners_by_map.get(map_id)
    }

    pub fn find_blueprint_config<'a>(
        &self,
        tables: &'a DataTables,
        blueprint_type: &str,
    ) -> Option<&'a BlueprintConfig> {
        let &idx = self.blueprint_by_type.get(blueprint_type)?;
        tables.blueprint_config.as_slice().get(idx as usize)
    }

    pub fn find_template_config<'a>(
        &self,
        tables: &'a DataTables,
        blueprint_type: &str,
    ) -> Option<TableRef<'a, TemplateConfig>> {
        if let Some(db) = tables.assets_db() {
            return db.find_template_by_blueprint(blueprint_type).map(TableRef::Owned);
        }
        let &idx = self.template_by_type.get(blueprint_type)?;
        tables
            .template_config
            .as_slice()
            .get(idx as usize)
            .map(TableRef::Borrowed)
    }

    pub fn level_play_indices_for_entity(&self, entity_id: i32) -> &[u32] {
        self.level_plays_by_entity.get(entity_id)
    }

    pub fn level_play_indices_for_ref_entity(&self, entity_id: i32) -> &[u32] {
        self.level_plays_by_ref_entity.get(entity_id)
    }

    pub fn level_play_indices_for_reward_entity(&self, entity_id: i32) -> &[u32] {
        self.level_plays_by_reward_entity.get(entity_id)
    }

    pub fn is_action_opened_function(&self, function_id: i32) -> bool {
        self.action_opened_functions.contains(&function_id)
    }
}

pub fn entity_world_pos(cfg: &LevelEntityConfig) -> Option<[f32; 3]> {
    let raw = cfg.Transform.first()?;
    let x = raw.X / 100.0;
    let y = raw.Y / 100.0;
    let z = raw.Z / 100.0;
    if x.is_finite() && y.is_finite() && z.is_finite() {
        Some([x, y, z])
    } else {
        None
    }
}

pub fn entity_world_rot(cfg: &LevelEntityConfig) -> [f32; 3] {
    let Some(raw) = cfg.Transform.get(1) else {
        return [0.0, 0.0, 0.0];
    };
    let r = raw.X / 100.0;
    let p = raw.Y / 100.0;
    let y = raw.Z / 100.0;
    if r.is_finite() && p.is_finite() && y.is_finite() {
        [r, p, y]
    } else {
        [0.0, 0.0, 0.0]
    }
}

pub fn entity_world_scale(cfg: &LevelEntityConfig) -> [f32; 3] {
    if !cfg.IsScaleEnabled {
        return [1.0, 1.0, 1.0];
    }
    let Some(raw) = cfg.Transform.get(2) else {
        return [1.0, 1.0, 1.0];
    };
    [
        if raw.X.is_finite() && raw.X > 0.0 { raw.X / 100.0 } else { 1.0 },
        if raw.Y.is_finite() && raw.Y > 0.0 { raw.Y / 100.0 } else { 1.0 },
        if raw.Z.is_finite() && raw.Z > 0.0 { raw.Z / 100.0 } else { 1.0 },
    ]
}

pub fn is_spawnable_blueprint(bp: &str) -> bool {
    bp.contains("Monster")
        || bp.contains("Animal")
        || bp.to_ascii_lowercase().contains("npc")
        || bp.contains("Treasure")
        || bp.contains("Teleport")
        || bp.contains("AirWall")
}

pub fn is_spawnable_item_config(cfg: &LevelEntityConfig, entity_type: EntityType) -> bool {
    if matches!(entity_type, EntityType::Chair | EntityType::Chair2) {
        if cfg
            .ComponentsData
            .BaseInfoComponent
            .as_ref()
            .and_then(|b| b.IsClientEntityFlag)
            == Some(true)
        {
            return false;
        }
        // Keep synthetic/inert MonsterChair configs (without interaction/state components) non-spawnable.
        if cfg.BlueprintType == "MonsterChair"
            && cfg.ComponentsData.InteractComponent.is_none()
            && cfg.ComponentsData.EntityStateComponent.is_none()
        {
            return false;
        }
        return true;
    }
    if cfg.BlueprintType == "Gameplay365" {
        return true;
    }
    !matches!(
        entity_type,
        EntityType::AdviseItem
            | EntityType::Audio
            | EntityType::AudioBox
            | EntityType::CustomAoiEditor
            | EntityType::DynamicSpawnMonster
            | EntityType::DynamicSpawnMonsterPoint
            | EntityType::EntityBatchRefresh
            | EntityType::EntityList
            | EntityType::EntityPackage
            | EntityType::FollowDecorationChair
            | EntityType::FurnitureSlot
            | EntityType::GroupAi
            | EntityType::LevelPlay
            | EntityType::LevelPlay2
            | EntityType::LevelPlayReward
            | EntityType::PathDrivenActorSpawner
            | EntityType::PerformanceOptimizer
            | EntityType::Position
            | EntityType::RefreshGroup
            | EntityType::RenderSpecifiedRange
            | EntityType::RenderSpecifiedRange2
            | EntityType::SceneItemStateHint
            | EntityType::SceneText
            | EntityType::SpawnMonster
            | EntityType::Spline
            | EntityType::SplineFX
            | EntityType::SplineRange
            | EntityType::StreamingSourcePoint
            | EntityType::TemplateEntitySpawner
            | EntityType::TimeScheduleManagerEntity
            | EntityType::VarManager
            | EntityType::WuWaGoEntity
    )
}

pub fn cell_coord(coord: f32) -> Option<i32> {
    if !coord.is_finite() {
        return None;
    }
    let scaled = (coord / SPATIAL_CELL_SIZE).floor();
    if (-1_000_000.0..=1_000_000.0).contains(&scaled) {
        Some(scaled as i32)
    } else {
        None
    }
}

pub fn spatial_cell_range(px: f32, py: f32, radius: f32) -> Option<CellRange> {
    if !radius.is_finite() || radius < 0.0 {
        return None;
    }
    let pad = radius + 1.0;
    let min_x = cell_coord(px - pad)?;
    let max_x = cell_coord(px + pad)?;
    let min_y = cell_coord(py - pad)?;
    let max_y = cell_coord(py + pad)?;
    if max_x < min_x || max_y < min_y {
        return None;
    }
    let nx = i64::from(max_x) - i64::from(min_x) + 1;
    let ny = i64::from(max_y) - i64::from(min_y) + 1;
    let max = MAX_SPATIAL_QUERY_CELLS as i64;
    if nx > max || ny > max || nx * ny > max {
        return None;
    }
    Some(CellRange {
        min_x,
        max_x,
        min_y,
        max_y,
    })
}

pub fn is_spawnable_blueprint_and_template(
    cfg: &LevelEntityConfig,
    bp: &BlueprintConfig,
    tpl_opt: Option<&TemplateConfig>,
) -> bool {
    match bp.EntityLogic {
        EntityLogic::Monster
        | EntityLogic::Animal
        | EntityLogic::Npc
        | EntityLogic::Vision
        | EntityLogic::Vehicle
        | EntityLogic::SimpleCombat => true,
        EntityLogic::Custom => matches!(
            bp.EntityType,
            EntityType::ClientTrigger | EntityType::EnrichmentArea | EntityType::LocationSafety
        ),
        EntityLogic::Item => {
            if is_spawnable_item_config(cfg, bp.EntityType) {
                return true;
            }
            if let Some(tpl) = tpl_opt {
                if matches!(
                    bp.EntityType,
                    EntityType::TreasureBox
                        | EntityType::HookTreasureBox
                        | EntityType::VisionTreasureBox
                        | EntityType::Destructible
                        | EntityType::DestructibleWithHook
                        | EntityType::DestructibleExploreInteractor
                ) {
                    return tpl.ComponentsData.RewardComponent.is_some()
                        || tpl.ComponentsData.TreasureBoxComponent.is_some();
                }
                if tpl.ComponentsData.RewardComponent.is_some()
                    && cfg.BlueprintType.contains("Treasure")
                {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn is_spawnable_config(
    cfg: &LevelEntityConfig,
    blueprints: &[BlueprintConfig],
    bp_by_type: &HashMap<String, u32>,
    templates: &[TemplateConfig],
    tpl_by_type: &HashMap<String, u32>,
) -> bool {
    if cfg.BlueprintType.starts_with("Editor") {
        return false;
    }
    if let Some(&bp_idx) = bp_by_type.get(&cfg.BlueprintType) {
        let bp = &blueprints[bp_idx as usize];
        let tpl_opt = tpl_by_type
            .get(&cfg.BlueprintType)
            .and_then(|&idx| templates.get(idx as usize));
        return is_spawnable_blueprint_and_template(cfg, bp, tpl_opt);
    }
    is_spawnable_blueprint(&cfg.BlueprintType)
}

fn spawnable_cell_for_entity(
    cfg: &LevelEntityConfig,
    blueprints: &[BlueprintConfig],
    bp_by_type: &HashMap<String, u32>,
    templates: &[TemplateConfig],
    tpl_by_type: &HashMap<String, u32>,
) -> Option<SpatialCellKey> {
    if !is_spawnable_config(cfg, blueprints, bp_by_type, templates, tpl_by_type) {
        return None;
    }
    let pos = entity_world_pos(cfg)?;
    let cell_x = cell_coord(pos[0])?;
    let cell_y = cell_coord(pos[1])?;
    Some(SpatialCellKey {
        map_id: cfg.MapId,
        cell_x,
        cell_y,
    })
}

fn build_string_index_bp(items: &[BlueprintConfig], map: &mut HashMap<String, u32>) {
    map.reserve(items.len());
    for (idx, entry) in items.iter().enumerate() {
        if !entry.BlueprintType.is_empty() {
            map.entry(entry.BlueprintType.clone()).or_insert(idx as u32);
        }
    }
}

fn build_string_index_tpl(items: &[TemplateConfig], map: &mut HashMap<String, u32>) {
    map.reserve(items.len());
    for (idx, entry) in items.iter().enumerate() {
        if !entry.BlueprintType.is_empty() {
            map.entry(entry.BlueprintType.clone()).or_insert(idx as u32);
        }
    }
}

fn build_level_entity_indexes_from_db(
    db: &AssetsDb,
    tables: &DataTables,
    out: &mut AssetIndexes,
) -> Result<(), sonoro_common::sqlite::SqliteError> {
    let rows = db.query_level_entity_index_rows()?;
    out.entity_by_map_pair.reserve(rows.len());
    let mut id_pairs = Vec::with_capacity(rows.len());
    let mut cell_pairs = Vec::new();
    let mut map_pairs = Vec::new();
    let max_idx = rows.iter().map(|r| r.idx as usize).max().unwrap_or(0);
    if !rows.is_empty() && max_idx <= 2_000_000 {
        out.entity_meta_by_idx
            .resize(max_idx + 1, EntityQuickMeta::default());
    }
    for r in rows {
        if let Some(slot) = out.entity_meta_by_idx.get_mut(r.idx as usize) {
            *slot = EntityQuickMeta {
                entity_id: r.entity_id,
                area_id: r.area_id,
                in_sleep: r.in_sleep,
                is_hidden: r.is_hidden,
            };
        }
        out.entity_by_map_pair
            .entry(MapEntityKey {
                map_id: r.map_id,
                entity_id: r.entity_id,
            })
            .or_insert(r.idx);
        id_pairs.push((r.entity_id, r.idx));
        let is_newly_spawnable = out
            .blueprint_by_type
            .get(r.blueprint_type.as_str())
            .and_then(|bp_idx| tables.blueprint_config.as_slice().get(*bp_idx as usize))
            .is_some_and(|bp| match bp.EntityType {
                EntityType::Monitor
                | EntityType::DaemonHack
                | EntityType::DollGrabShowcase
                | EntityType::ExhibitBase
                | EntityType::FlowerPollutionManager
                | EntityType::Furniture
                | EntityType::Slots
                | EntityType::SpawnPasserbyNpc => true,
                EntityType::RefreshGroup => r.blueprint_type == "Gameplay365",
                EntityType::Chair | EntityType::Chair2 => {
                    !r.is_client_flag && r.blueprint_type != "MonsterChair"
                }
                _ => false,
            });
        if r.spawnable || is_newly_spawnable {
            cell_pairs.push((
                SpatialCellKey {
                    map_id: r.map_id,
                    cell_x: r.cell_x,
                    cell_y: r.cell_y,
                },
                r.idx,
            ));
            map_pairs.push((r.map_id, r.idx));
        }
    }
    out.entity_by_id = SpanTable::from_pairs(id_pairs);
    out.spawnables_by_map_cell = SpanTable::from_pairs(cell_pairs);
    out.spawnables_by_map = SpanTable::from_pairs(map_pairs);
    if let Ok(sp_rows) = db.query_passerby_spawner_rows() {
        out.passerby_spawners_by_map = SpanTable::from_pairs(sp_rows);
    }
    if let Ok(group_rows) = db.query_entity_group_rows() {
        for (map_id, group_entity_id, raw) in group_rows {
            if let Ok(cfg) = serde_json::from_str::<LevelEntityConfig>(&raw)
                && let Some(ExtraEntry::EntityGroupComponent(egc)) =
                    cfg.ComponentsData.find_extra_entry(ExtraKind::EntityGroupComponent)
            {
                register_entity_group_spec(
                    map_id,
                    group_entity_id,
                    (**egc).clone(),
                    &mut out.entity_groups_by_member,
                );
            }
        }
    }
    Ok(())
}

fn register_entity_group_spec(
    map_id: i32,
    group_entity_id: i64,
    component: EntityGroupComponent,
    map: &mut HashMap<MapEntityKey, Vec<EntityGroupSpec>>,
) {
    if component.Disabled == Some(true) {
        return;
    }
    let Some(ids) = component.EntityIds.as_ref() else {
        return;
    };
    let spec = EntityGroupSpec {
        group_entity_id,
        component: component.clone(),
    };
    for &member_id in ids {
        if member_id > 0 {
            map.entry(MapEntityKey {
                map_id,
                entity_id: member_id,
            })
            .or_default()
            .push(spec.clone());
        }
    }
}

fn build_level_entity_indexes(
    items: &[LevelEntityConfig],
    blueprints: &[BlueprintConfig],
    templates: &[TemplateConfig],
    out: &mut AssetIndexes,
) {
    out.entity_by_map_pair.reserve(items.len());
    out.entity_meta_by_idx
        .resize(items.len(), EntityQuickMeta::default());
    let mut id_pairs = Vec::with_capacity(items.len());
    let mut cell_pairs = Vec::new();
    let mut map_pairs = Vec::new();
    let mut passerby_pairs = Vec::new();
    for (idx, cfg) in items.iter().enumerate() {
        let idx_u32 = idx as u32;
        out.entity_meta_by_idx[idx] = EntityQuickMeta {
            entity_id: cfg.EntityId,
            area_id: cfg.AreaId,
            in_sleep: cfg.InSleep,
            is_hidden: cfg.IsHidden,
        };
        out.entity_by_map_pair
            .entry(MapEntityKey {
                map_id: cfg.MapId,
                entity_id: cfg.EntityId,
            })
            .or_insert(idx_u32);
        id_pairs.push((cfg.EntityId, idx_u32));
        if cfg
            .ComponentsData
            .has_extra(ExtraKind::PasserbyNpcSpawnComponent)
        {
            passerby_pairs.push((cfg.MapId, idx_u32));
        }
        if let Some(ExtraEntry::EntityGroupComponent(egc)) =
            cfg.ComponentsData.find_extra_entry(ExtraKind::EntityGroupComponent)
        {
            register_entity_group_spec(
                cfg.MapId,
                cfg.EntityId,
                (**egc).clone(),
                &mut out.entity_groups_by_member,
            );
        }
        if let Some(cell) = spawnable_cell_for_entity(
            cfg,
            blueprints,
            &out.blueprint_by_type,
            templates,
            &out.template_by_type,
        ) {
            cell_pairs.push((cell, idx_u32));
            map_pairs.push((cfg.MapId, idx_u32));
        }
    }
    out.entity_by_id = SpanTable::from_pairs(id_pairs);
    out.spawnables_by_map_cell = SpanTable::from_pairs(cell_pairs);
    out.spawnables_by_map = SpanTable::from_pairs(map_pairs);
    out.passerby_spawners_by_map = SpanTable::from_pairs(passerby_pairs);
}

fn build_teleporter_index(items: &[Teleporter], out: &mut AssetIndexes) {
    let pairs = items
        .iter()
        .enumerate()
        .map(|(idx, tp)| (tp.MapId, idx as u32));
    out.teleporters_by_map = SpanTable::from_pairs(pairs);
}

fn build_teleporter_area_index(
    items: &[Teleporter],
    instance_dungeons: &[InstanceDungeon],
    out: &mut AssetIndexes,
) {
    let inst_map: HashMap<i32, i32> = instance_dungeons
        .iter()
        .map(|d| (d.Id, d.MapConfigId))
        .collect();
    let mut area_pairs = Vec::with_capacity(items.len());
    for (idx, tp) in items.iter().enumerate() {
        let mut cfg_idx = out
            .entity_by_map_pair
            .get(&MapEntityKey {
                map_id: tp.MapId,
                entity_id: tp.TeleportEntityConfigId,
            })
            .copied();
        if cfg_idx.is_none()
            && let Some(&resolved) = inst_map.get(&tp.MapId)
            && resolved != tp.MapId
        {
            cfg_idx = out
                .entity_by_map_pair
                .get(&MapEntityKey {
                    map_id: resolved,
                    entity_id: tp.TeleportEntityConfigId,
                })
                .copied();
        }
        if let Some(cidx) = cfg_idx
            && let Some(meta) = out.quick_entity_meta(cidx)
            && meta.area_id > 0
        {
            area_pairs.push((meta.area_id, idx as u32));
        }
    }
    out.teleporters_by_area = SpanTable::from_pairs(area_pairs);
}

pub fn parse_ref_suffix_id(r: &str) -> Option<i32> {
    let idx = r.rfind('_')?;
    let digits = &r[idx + 1..];
    if digits.is_empty() || digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let parsed = digits.parse::<i32>().ok()?;
    (parsed > 0).then_some(parsed)
}

fn collect_level_play_ref_ids(lp: &LevelPlayData, refs: &mut Vec<i32>) {
    refs.clear();
    let mut push_unique = |id: i32| {
        if id > 0 && !refs.contains(&id) {
            refs.push(id);
        }
    };
    if lp.Data.LevelPlayEntityId > 0 {
        push_unique(lp.Data.LevelPlayEntityId);
    }
    for r in &lp.Data.Children {
        if let Some(id) = parse_ref_suffix_id(r) {
            push_unique(id);
        }
    }
    for r in &lp.Data.Reference {
        if let Some(id) = parse_ref_suffix_id(r) {
            push_unique(id);
        }
    }
}

fn build_level_play_indexes(items: &[LevelPlayData], out: &mut AssetIndexes) {
    let mut entity_pairs = Vec::new();
    let mut ref_pairs = Vec::new();
    let mut reward_pairs = Vec::new();
    let mut non_spawnable_enter = Vec::new();
    let mut refs = Vec::new();
    for (idx, lp) in items.iter().enumerate() {
        let idx_u32 = idx as u32;
        if lp.Data.LevelPlayEntityId > 0 {
            entity_pairs.push((lp.Data.LevelPlayEntityId, idx_u32));
        }
        collect_level_play_ref_ids(lp, &mut refs);
        for &ref_id in &refs {
            ref_pairs.push((ref_id, idx_u32));
        }
        if let Some(rc) = &lp.Data.LevelPlayRewardConfig
            && rc.RewardEntityId > 0
        {
            reward_pairs.push((rc.RewardEntityId, idx_u32));
        }
        if lp.Data.EnterRadius > 0
            && lp.Data.LevelPlayEntityId > 0
            && lp.Data.Type != "Quest"
            && lp.Data.Type != "Dungeon"
            && lp.Data.Type != "LordGym"
        {
            non_spawnable_enter.push(idx_u32);
        }
    }
    out.level_plays_by_entity = SpanTable::from_pairs(entity_pairs);
    out.level_plays_by_ref_entity = SpanTable::from_pairs(ref_pairs);
    out.level_plays_by_reward_entity = SpanTable::from_pairs(reward_pairs);
    out.non_spawnable_enter_lps = non_spawnable_enter;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tables::level_entity_config::Vector;
    use crate::tables::level_play_data::{DataEntry, RewardConfig};
    use crate::tables::entity_components::components::Components;
    use crate::tables::entity_components::reward::RewardComponent;

    #[test]
    fn indexes_level_entities_by_map_pair_id_spans_and_spatial_grid() {
        let payload = r#"[
          {"Id": 1, "EntityId": 1001, "MapId": 8, "BlueprintType": "Monster_01", "Name": "m1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 200.0, "Y": 400.0, "Z": 50.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 2.0, "Y": 2.0, "Z": 1.0}], "ComponentsData": {}},
          {"Id": 2, "EntityId": 1001, "MapId": 9, "BlueprintType": "Npc_01", "Name": "n1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 9000.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
          {"Id": 3, "EntityId": 1002, "MapId": 8, "BlueprintType": "Monster_ZeroScale", "Name": "m2", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 0.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
          {"Id": 4, "EntityId": 1003, "MapId": 8, "BlueprintType": "Item_Chest", "Name": "c1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": 100.0, "Y": 200.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}},
          {"Id": 5, "EntityId": 1004, "MapId": 8, "BlueprintType": "Animal_Boundary", "Name": "a1", "InSleep": false, "IsHidden": false, "AreaId": 1, "IsScaleEnabled": false, "Transform": [{"X": -100.0, "Y": 4999.0, "Z": 0.0}, {"X": 0.0, "Y": 0.0, "Z": 0.0}, {"X": 1.0, "Y": 1.0, "Z": 1.0}], "ComponentsData": {}}
        ]"#;
        let entities: Vec<LevelEntityConfig> = serde_json::from_str(payload).unwrap();
        let teleporters = [
            Teleporter { Id: 1, MapId: 8, TeleportEntityConfigId: 1003, ..Default::default() },
            Teleporter { Id: 2, MapId: 9, TeleportEntityConfigId: 1001, ..Default::default() },
            Teleporter { Id: 3, MapId: 8, TeleportEntityConfigId: 1001, ..Default::default() },
        ];
        let level_plays = [
            LevelPlayData {
                LevelPlayId: 500,
                Data: DataEntry {
                    LevelPlayEntityId: 1001,
                    Children: vec!["e_8_1001".into(), "e_8_1003".into(), "e_8_1001".into()],
                    Reference: vec!["e_8_1003".into(), "prefix_0100".into(), "bad_ref".into()],
                    LevelPlayRewardConfig: Some(RewardConfig {
                        RewardEntityId: 777,
                        RewardId: 88,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            },
            LevelPlayData {
                LevelPlayId: 501,
                Data: DataEntry {
                    LevelPlayEntityId: 2002,
                    ..Default::default()
                },
            },
        ];
        let blueprints = [BlueprintConfig {
            Id: 1,
            BlueprintType: "Monster_01".into(),
            EntityType: EntityType::Monster,
            EntityLogic: EntityLogic::Monster,
            ModelId: 1,
            HalfHeight: 1,
            TrackHeight: 1,
        }];
        let templates = [TemplateConfig {
            Id: 10,
            BlueprintType: "Monster_01".into(),
            ComponentsData: Components::default(),
        }];

        let indexes = AssetIndexes::from_slices(Inputs {
            level_entity_configs: &entities,
            blueprint_configs: &blueprints,
            template_configs: &templates,
            teleporters: &teleporters,
            level_play_data: &level_plays,
        });

        assert_eq!(indexes.entity_by_map_pair.get(&MapEntityKey { map_id: 8, entity_id: 1001 }), Some(&0));
        assert_eq!(indexes.entity_by_map_pair.get(&MapEntityKey { map_id: 9, entity_id: 1001 }), Some(&1));
        assert_eq!(indexes.entity_by_map_pair.get(&MapEntityKey { map_id: 10, entity_id: 1001 }), None);
        assert_eq!(indexes.level_entity_indices_by_id(1001), &[0, 1]);
        assert_eq!(indexes.level_entity_indices_by_id(1002), &[2]);
        assert!(indexes.level_entity_indices_by_id(9999).is_empty());

        let pos0 = entity_world_pos(&entities[0]).unwrap();
        assert!((pos0[0] - 2.0).abs() < 0.001);
        assert!((pos0[1] - 4.0).abs() < 0.001);
        assert!((pos0[2] - 0.5).abs() < 0.001);

        assert_eq!(indexes.spawnable_indices_for_map(8), &[0, 2, 4]);
        assert_eq!(indexes.spawnable_indices_for_map(9), &[1]);
        assert_eq!(indexes.spawnable_indices_for_map_cell(8, 0, 0), &[0, 2]);
        assert_eq!(indexes.spawnable_indices_for_map_cell(8, -1, 0), &[4]);

        let range = spatial_cell_range(0.0, 5000.0, 200.0).unwrap();
        let mut found_boundary = false;
        for cx in range.min_x..=range.max_x {
            for cy in range.min_y..=range.max_y {
                if indexes.spawnable_indices_for_map_cell(8, cx, cy).contains(&4) {
                    found_boundary = true;
                }
            }
        }
        assert!(found_boundary);
        assert!(spatial_cell_range(f32::NAN, 0.0, 1000.0).is_none());
        assert!(spatial_cell_range(0.0, 0.0, 10_000_000.0).is_none());

        assert_eq!(indexes.teleporter_indices_for_map(8), &[0, 2]);
        assert_eq!(indexes.teleporter_indices_for_map(9), &[1]);
        assert_eq!(indexes.level_play_indices_for_entity(1001), &[0]);
        assert_eq!(indexes.level_play_indices_for_ref_entity(1001), &[0]);
        assert_eq!(indexes.level_play_indices_for_ref_entity(1003), &[0]);
        assert!(indexes.level_play_indices_for_ref_entity(100).is_empty());
        assert_eq!(indexes.level_play_indices_for_reward_entity(777), &[0]);
        assert_eq!(indexes.blueprint_by_type.get("Monster_01"), Some(&0));
        assert_eq!(indexes.template_by_type.get("Monster_01"), Some(&0));
    }

    #[test]
    fn includes_interactive_items_and_excludes_non_interactive_props() {
        let payload = r#"[
          {"Id": 1, "EntityId": 136000479, "MapId": 8, "BlueprintType": "Animal016", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}},
          {"Id": 2, "EntityId": 136000472, "MapId": 8, "BlueprintType": "Animal006", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}},
          {"Id": 3, "EntityId": 139000395, "MapId": 8, "BlueprintType": "MonsterChair", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}},
          {"Id": 4, "EntityId": 139000396, "MapId": 8, "BlueprintType": "Treasure004", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}}
        ]"#;
        let entities: Vec<LevelEntityConfig> = serde_json::from_str(payload).unwrap();
        let blueprints = [
            BlueprintConfig { Id: 35, BlueprintType: "Animal016".into(), EntityType: EntityType::Collect, EntityLogic: EntityLogic::Item, ModelId: 630057, HalfHeight: 0, TrackHeight: 0 },
            BlueprintConfig { Id: 36, BlueprintType: "Animal006".into(), EntityType: EntityType::Animal, EntityLogic: EntityLogic::Animal, ModelId: 630006, HalfHeight: 50, TrackHeight: 0 },
            BlueprintConfig { Id: 2659, BlueprintType: "MonsterChair".into(), EntityType: EntityType::Chair, EntityLogic: EntityLogic::Item, ModelId: 709994, HalfHeight: 0, TrackHeight: 0 },
            BlueprintConfig { Id: 4001, BlueprintType: "Treasure004".into(), EntityType: EntityType::TreasureBox, EntityLogic: EntityLogic::Item, ModelId: 700001, HalfHeight: 0, TrackHeight: 0 },
        ];
        let indexes = AssetIndexes::from_slices(Inputs {
            level_entity_configs: &entities,
            blueprint_configs: &blueprints,
            template_configs: &[],
            teleporters: &[],
            level_play_data: &[],
        });
        assert_eq!(indexes.spawnable_indices_for_map(8), &[0, 1, 3]);
        assert_eq!(indexes.spawnable_indices_for_map_cell(8, 0, 0), &[0, 1, 3]);
    }

    #[test]
    fn indexes_reward_and_interactive_non_reward_destructibles() {
        let payload = r#"[
          {"Id": 1, "EntityId": 500001, "MapId": 8, "BlueprintType": "Destructible_Reward", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}},
          {"Id": 2, "EntityId": 500002, "MapId": 8, "BlueprintType": "Destructible_Plain", "InSleep": false, "IsHidden": false, "AreaId": 1, "Transform": [{"X": 100.0, "Y": 100.0, "Z": 0.0}], "ComponentsData": {}}
        ]"#;
        let entities: Vec<LevelEntityConfig> = serde_json::from_str(payload).unwrap();
        let blueprints = [
            BlueprintConfig { Id: 5001, BlueprintType: "Destructible_Reward".into(), EntityType: EntityType::Destructible, EntityLogic: EntityLogic::Item, ModelId: 700010, HalfHeight: 0, TrackHeight: 0 },
            BlueprintConfig { Id: 5002, BlueprintType: "Destructible_Plain".into(), EntityType: EntityType::DestructibleExploreInteractor, EntityLogic: EntityLogic::Item, ModelId: 700011, HalfHeight: 0, TrackHeight: 0 },
        ];
        let templates = [
            TemplateConfig {
                Id: 5001,
                BlueprintType: "Destructible_Reward".into(),
                ComponentsData: Components {
                    RewardComponent: Some(RewardComponent::default()),
                    ..Default::default()
                },
            },
            TemplateConfig {
                Id: 5002,
                BlueprintType: "Destructible_Plain".into(),
                ComponentsData: Components::default(),
            },
        ];
        let indexes = AssetIndexes::from_slices(Inputs {
            level_entity_configs: &entities,
            blueprint_configs: &blueprints,
            template_configs: &templates,
            teleporters: &[],
            level_play_data: &[],
        });
        assert_eq!(indexes.spawnable_indices_for_map(8), &[0, 1]);
    }

    #[test]
    fn entity_world_pos_and_rot_divide_by_fixed_100() {
        let cfg = LevelEntityConfig {
            Id: 1,
            EntityId: 105000233,
            BlueprintType: "Gameplay292".into(),
            InSleep: false,
            IsHidden: false,
            IsScaleEnabled: false,
            MapId: 8,
            AreaId: 1,
            Transform: vec![
                Vector { X: -13502878.0, Y: 8241805.0, Z: 383788.0 },
                Vector { X: 0.0, Y: 0.0, Z: 9000.0 },
                Vector { X: 100.0, Y: 47.0, Z: 150.0 },
            ],
            ComponentsData: Components::default(),
        };
        let pos = entity_world_pos(&cfg).unwrap();
        let rot = entity_world_rot(&cfg);
        assert!((pos[0] - (-135028.78)).abs() < 0.01);
        assert!((pos[1] - 82418.05).abs() < 0.01);
        assert!((pos[2] - 3837.88).abs() < 0.01);
        assert!((rot[2] - 90.0).abs() < 0.01);
    }
}
