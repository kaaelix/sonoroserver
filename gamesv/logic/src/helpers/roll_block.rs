use std::collections::{HashMap, HashSet};
use std::io;

use serde_json::{Map, Value};
use sonoro_gamesv_data::asset_indexes::{entity_world_pos, entity_world_rot};
use sonoro_gamesv_data::assets::{
    Assets, RollBlockCellConfig, RollBlockConfig, RollBlockLevelConfig, RollBlockTemplateConfig,
};
use sonoro_proto::enums::{
    BtType, ErrorCode, GameCtxType, RbGridDirection, RollBlockGameActionType,
};
use sonoro_proto::pb::{
    ActionNotify, BehaviorTreeCtxPb, BtVarUpdateNotify, CombatReceiveData, CombatReceivePackNotify,
    EntityAddNotify, EntityRemoveInfo, EntityRemoveNotify, GameCtxPb, GameCtxPbCtxInfoOneof,
    RbBlockComponentPb, RbBlockComponentPbTypeOneof, RbBlockIdlePbState, RbBlockMovementPbAction,
    RbBlockMovementPbActionTypeOneof, RbBlockMovingPbState, RbBlockPbState,
    RbBlockPbStateStateOneof, RbBlockStateChangeNotify, RbBreakableObstaclePbType,
    RbDefaultBlockPbType, RbFloorComponentPb, RbGridPosition, RbInput, RbItemComponentPb,
    RbItemComponentPbTypeOneof, RbJumpMovement, RbLaserEmitterPbType, RbRollInput,
    RbRollMovement, RollBlockCloseGamePlayNotify, RollBlockGamePlayActionCtxPb,
    RollBlockGamePlayPbInfo, RollBlockItemUpdateNotify, RollBlockOpenGamePlayNotify,
    RollBlockResetGamePlayNotify, RollBlockUpdateAvailableInputsNotify,
    RollBlockUpdateGamePlayNotify, VarDefinePb, VarDefinePbValueOneof, Vector,
};

use crate::component::entity::{
    ConfigComponent, ConfigType, EntityComponentStorage, EntityState, EntityType, PositionComponent,
};
use crate::helpers::level_play;
use crate::helpers::quest_actions;
use crate::helpers::quest_engine::{self, QuestEventKind};
use crate::helpers::quest_types::{QuestContext, json_i64, json_int, json_str, resolve_var_name};
use crate::helpers::reward;
use crate::scene::{Scene, StateTagUpdate};
use crate::sink::PushSink;

const DEFAULT_CELL_LEN: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct RollBlockEntityState {
    pub block_id: i32,
    pub block_type: String,
    pub net_id: i64,
    pub config_id: i32,
    pub occupied: Vec<(i32, i32)>,
    pub is_vision: bool,
}

#[derive(Debug, Clone)]
pub struct RollBlockObstacleState {
    pub net_id: i64,
    pub config_id: i32,
    pub key: i32,
    pub is_quantum: bool,
    pub quantum_active: bool,
    pub pos: (i32, i32),
    pub broken: bool,
    pub link_points: Vec<Vector>,
}

#[derive(Debug, Clone)]
pub struct RollBlockLaserState {
    pub net_id: i64,
    pub pos: (i32, i32),
    pub dir: RbGridDirection,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct PendingRollMove {
    pub block_net_id: i64,
    pub block_id: i32,
    pub target_occupied: Vec<(i32, i32)>,
    pub pending_jump: Option<RbGridDirection>,
    pub fell_off: bool,
}

#[derive(Debug, Clone)]
pub struct RollBlockSession {
    pub inc_id: i32,
    pub bt_type: BtType,
    pub owner_id: i32,
    pub node_id: i32,
    pub center_entity_id: i64,
    pub group_id: i32,
    pub template_id: i32,
    pub cur_diff_idx: usize,
    pub total_difficulties: usize,
    pub center_pos: [f32; 3],
    pub center_rot: [f32; 3],
    pub forward: [f32; 3],
    pub right: [f32; 3],
    pub width: i32,
    pub height: i32,
    pub cell_len: f32,
    pub controlled_block_id: i32,
    pub blocks: Vec<RollBlockEntityState>,
    pub floor_entities: HashMap<(i32, i32), i64>,
    pub spawned_net_ids: Vec<i64>,
    pub fallen_cells: HashSet<(i32, i32)>,
    pub broken_keys: HashSet<i32>,
    pub obstacles: Vec<RollBlockObstacleState>,
    pub lasers: Vec<RollBlockLaserState>,
    pub move_count: i32,
    pub hint_open: bool,
    pub ready: bool,
    pub pending_move: Option<PendingRollMove>,
    pub cond_json: Value,
}

fn dir_from_str(s: Option<&str>) -> RbGridDirection {
    match s.unwrap_or("Forward") {
        "Backward" => RbGridDirection::RbBackward,
        "Right" => RbGridDirection::RbRight,
        "Left" => RbGridDirection::RbLeft,
        _ => RbGridDirection::RbForward,
    }
}

fn dir_delta(dir: RbGridDirection) -> (i32, i32) {
    match dir {
        RbGridDirection::RbForward => (0, -1),
        RbGridDirection::RbBackward => (0, 1),
        RbGridDirection::RbRight => (1, 0),
        RbGridDirection::RbLeft => (-1, 0),
    }
}

fn pb_vec(v: [f32; 3]) -> Vector {
    Vector {
        X: v[0],
        Y: v[1],
        Z: v[2],
    }
}

fn axes_from_rot(rot: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let yaw_rad = rot[2].to_radians();
    let (sy, cy) = yaw_rad.sin_cos();
    let forward = [sy, -cy, 0.0];
    let right = [cy, sy, 0.0];
    (forward, right)
}

fn cell_world_pos(
    center_pos: [f32; 3],
    forward: [f32; 3],
    right: [f32; 3],
    w: i32,
    h: i32,
    cell_len: f32,
    x: i32,
    y: i32,
    z_add: f32,
) -> [f32; 3] {
    let rx = (x as f32 - (w - 1) as f32 * 0.5) * cell_len;
    let fy = ((h - 1) as f32 * 0.5 - y as f32) * cell_len;
    [
        center_pos[0] + right[0] * rx + forward[0] * fy,
        center_pos[1] + right[1] * rx + forward[1] * fy,
        center_pos[2] + z_add,
    ]
}

fn block_world_pose(
    session: &RollBlockSession,
    block_type: &str,
    occ: &[(i32, i32)],
) -> ([f32; 3], [f32; 3]) {
    if occ.is_empty() {
        return (session.center_pos, session.center_rot);
    }
    if occ.len() == 1 {
        let (x, y) = occ[0];
        let z_add = if block_type == "Block2" {
            session.cell_len
        } else {
            session.cell_len * 0.5
        };
        let pos = cell_world_pos(
            session.center_pos,
            session.forward,
            session.right,
            session.width,
            session.height,
            session.cell_len,
            x,
            y,
            z_add,
        );
        return (pos, session.center_rot);
    }
    let (x1, y1) = occ[0];
    let (x2, y2) = occ[1];
    let p1 = cell_world_pos(
        session.center_pos,
        session.forward,
        session.right,
        session.width,
        session.height,
        session.cell_len,
        x1,
        y1,
        session.cell_len * 0.5,
    );
    let p2 = cell_world_pos(
        session.center_pos,
        session.forward,
        session.right,
        session.width,
        session.height,
        session.cell_len,
        x2,
        y2,
        session.cell_len * 0.5,
    );
    let mid = [
        (p1[0] + p2[0]) * 0.5,
        (p1[1] + p2[1]) * 0.5,
        session.cell_len * 0.5 + session.center_pos[2],
    ];
    let rot = if y1 == y2 {
        [session.center_rot[0], session.center_rot[1] + 90.0, session.center_rot[2]]
    } else {
        [session.center_rot[0] + 90.0, session.center_rot[1], session.center_rot[2]]
    };
    (mid, rot)
}

fn floor_type_code(ty: &str) -> i32 {
    match ty {
        "Goal" => 1,
        "LinkGoal" => 2,
        "FallFloor" => 3,
        "JumpFloor" => 4,
        "BreakTrigger" => 5,
        "Quantum" => 6,
        _ => 0,
    }
}

fn sort_occ(mut occ: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    occ.sort_unstable();
    occ
}

fn compute_roll(block_type: &str, occ: &[(i32, i32)], dir: RbGridDirection) -> Vec<(i32, i32)> {
    let (dx, dy) = dir_delta(dir);
    if occ.len() == 1 {
        let (ox, oy) = occ[0];
        if block_type == "Block1" {
            vec![(ox + dx, oy + dy)]
        } else {
            sort_occ(vec![(ox + dx, oy + dy), (ox + 2 * dx, oy + 2 * dy)])
        }
    } else if occ.len() >= 2 {
        let (x1, y1) = occ[0];
        let (x2, y2) = occ[1];
        if x1 == x2 {
            if dy != 0 {
                let ny = if dy > 0 { y1.max(y2) + 1 } else { y1.min(y2) - 1 };
                vec![(x1, ny)]
            } else {
                sort_occ(vec![(x1 + dx, y1), (x2 + dx, y2)])
            }
        } else if dx != 0 {
            let nx = if dx > 0 { x1.max(x2) + 1 } else { x1.min(x2) - 1 };
            vec![(nx, y1)]
        } else {
            sort_occ(vec![(x1, y1 + dy), (x2, y2 + dy)])
        }
    } else {
        Vec::new()
    }
}

fn grid_cell_at<'a>(level: &'a RollBlockLevelConfig, x: i32, y: i32) -> Option<&'a RollBlockCellConfig> {
    let w = level.GridConfig.BoardSize.X;
    let h = level.GridConfig.BoardSize.Y;
    if x < 0 || y < 0 || x >= w || y >= h {
        return None;
    }
    let idx = (y * w + x) as usize;
    level.GridConfig.Grids.get(idx)
}

fn resolve_level<'a>(
    assets: &'a Assets,
    group_id: i32,
    diff_idx: usize,
) -> Option<(&'a RollBlockConfig, &'a RollBlockLevelConfig)> {
    let grp = assets.roll_blocks.get(&group_id)?;
    let lv = grp.Levels.get(diff_idx)?;
    Some((grp, lv))
}

fn resolve_template<'a>(assets: &'a Assets, template_id: i32) -> Option<&'a RollBlockTemplateConfig> {
    assets
        .roll_block_templates
        .get(&template_id)
        .or_else(|| assets.roll_block_templates.values().next())
}

fn laser_blocked_cells(session: &RollBlockSession, level: &RollBlockLevelConfig) -> HashSet<(i32, i32)> {
    let mut blocked = HashSet::new();
    for obs in &session.obstacles {
        if !obs.broken && (!obs.is_quantum || obs.quantum_active) {
            blocked.insert(obs.pos);
        }
    }
    for l in &session.lasers {
        blocked.insert(l.pos);
        let (dx, dy) = dir_delta(l.dir);
        let mut cx = l.pos.0 + dx;
        let mut cy = l.pos.1 + dy;
        while let Some(c) = grid_cell_at(level, cx, cy) {
            if c.Type == "Empty" {
                break;
            }
            if session
                .obstacles
                .iter()
                .any(|o| o.pos == (cx, cy) && !o.broken && (!o.is_quantum || o.quantum_active))
            {
                break;
            }
            if session
                .blocks
                .iter()
                .any(|b| b.occupied.contains(&(cx, cy)))
            {
                break;
            }
            blocked.insert((cx, cy));
            cx += dx;
            cy += dy;
        }
    }
    blocked
}

fn is_cell_solid_blocked(
    session: &RollBlockSession,
    level: &RollBlockLevelConfig,
    block_id: i32,
    x: i32,
    y: i32,
) -> bool {
    if session
        .obstacles
        .iter()
        .any(|o| o.pos == (x, y) && !o.broken && (!o.is_quantum || o.quantum_active))
    {
        return true;
    }
    if session.lasers.iter().any(|l| l.pos == (x, y)) {
        return true;
    }
    if session
        .blocks
        .iter()
        .any(|b| b.block_id != block_id && b.occupied.contains(&(x, y)))
    {
        return true;
    }
    let _ = level;
    false
}

fn compute_available_inputs(session: &RollBlockSession, assets: &Assets) -> Vec<RbInput> {
    let Some((_, level)) = resolve_level(assets, session.group_id, session.cur_diff_idx) else {
        return Vec::new();
    };
    let Some(blk) = session
        .blocks
        .iter()
        .find(|b| b.block_id == session.controlled_block_id)
    else {
        return Vec::new();
    };
    let mut out = Vec::with_capacity(4);
    for dir in [
        RbGridDirection::RbForward,
        RbGridDirection::RbBackward,
        RbGridDirection::RbRight,
        RbGridDirection::RbLeft,
    ] {
        let next = compute_roll(&blk.block_type, &blk.occupied, dir);
        let solid_blocked = next
            .iter()
            .any(|&(cx, cy)| is_cell_solid_blocked(session, level, blk.block_id, cx, cy));
        if !solid_blocked {
            out.push(RbInput {
                Roll: Some(RbRollInput {
                    Direction: Some(dir),
                }),
            });
        }
    }
    out
}

fn build_rb_gameplay_info(
    session: &RollBlockSession,
    assets: &Assets,
    is_main_control: bool,
) -> RollBlockGamePlayPbInfo {
    let cur_diff = resolve_level(assets, session.group_id, session.cur_diff_idx)
        .map(|(_, l)| l.Difficulty)
        .unwrap_or(session.cur_diff_idx as i32 + 1);
    let delay = resolve_template(assets, session.template_id)
        .map(|t| t.DelayTime)
        .unwrap_or(1.5);
    RollBlockGamePlayPbInfo {
        IncId: session.inc_id,
        GroupId: session.group_id,
        Difficulty: cur_diff,
        IsMainControl: is_main_control,
        InitVisibleEntityIds: Vec::new(),
        Position: Some(pb_vec(session.center_pos)),
        Rotation: Some(pb_vec(session.center_rot)),
        AvailableInputs: compute_available_inputs(session, assets),
        GridSizeX: session.width,
        GridSizeY: session.height,
        ShowHintErrorInputCount: resolve_level(assets, session.group_id, session.cur_diff_idx)
            .and_then(|(_, l)| l.HintStepThreshold)
            .unwrap_or(5),
        SubCameraTag: 0,
        TotalDifficultyCount: session.total_difficulties as i32,
        ReadyDelaySecond: delay,
        InitInvisibleEntityIds: session.spawned_net_ids.clone(),
    }
}

fn spawn_template_entity(
    scene: &mut Scene,
    assets: &Assets,
    tpl_id: &str,
    pos: [f32; 3],
    rot: [f32; 3],
) -> Option<(i64, EntityComponentStorage)> {
    let tpl = assets.find_template_config(tpl_id)?;
    let _ = assets.find_blueprint_config(&tpl.BlueprintType)?;
    let base_info = tpl.ComponentsData.BaseInfoComponent.as_ref();
    let camp = base_info.and_then(|i| i.Camp).unwrap_or(2);
    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp,
            config_id: tpl.Id,
            config_type: ConfigType::Template,
            entity_type: EntityType::SceneItem,
            state: crate::component::entity::EntityState::Default,
        },
    );
    storage.position = Some(PositionComponent {
        location: pos,
        rotation: rot,
    });
    storage.visible = true;
    storage.actor_visible = true;
    let handle = scene.spawn(storage.clone());
    Some((handle.net_id, storage))
}

fn build_laser_points(
    session: &RollBlockSession,
    level: &RollBlockLevelConfig,
    origin: (i32, i32),
    dir: RbGridDirection,
    height: f32,
) -> Vec<Vector> {
    let p0 = cell_world_pos(
        session.center_pos,
        session.forward,
        session.right,
        session.width,
        session.height,
        session.cell_len,
        origin.0,
        origin.1,
        height,
    );
    let (dx, dy) = dir_delta(dir);
    let mut last = origin;
    let mut cx = origin.0 + dx;
    let mut cy = origin.1 + dy;
    while let Some(c) = grid_cell_at(level, cx, cy) {
        if c.Type == "Empty" {
            break;
        }
        last = (cx, cy);
        if session
            .obstacles
            .iter()
            .any(|o| o.pos == (cx, cy) && !o.broken && (!o.is_quantum || o.quantum_active))
            || session.blocks.iter().any(|b| b.occupied.contains(&(cx, cy)))
        {
            break;
        }
        cx += dx;
        cy += dy;
    }
    let p1 = cell_world_pos(
        session.center_pos,
        session.forward,
        session.right,
        session.width,
        session.height,
        session.cell_len,
        last.0,
        last.1,
        height,
    );
    vec![pb_vec(p0), pb_vec(p1)]
}

fn populate_board_entities<S: PushSink + ?Sized>(
    scene: &mut Scene,
    assets: &Assets,
    sink: &mut S,
    session: &mut RollBlockSession,
    now_ms: i64,
) -> bool {
    despawn_board_entities(scene, sink, session);

    let Some((_, level_owned)) = resolve_level(assets, session.group_id, session.cur_diff_idx).map(|(g, l)| (g, l.clone())) else {
        return false;
    };
    let Some(tpl_cfg) = resolve_template(assets, session.template_id).cloned() else {
        return false;
    };

    session.cell_len = if tpl_cfg.Length > 0.0 {
        tpl_cfg.Length
    } else {
        DEFAULT_CELL_LEN
    };
    session.width = level_owned.GridConfig.BoardSize.X.max(1);
    session.height = level_owned.GridConfig.BoardSize.Y.max(1);
    session.controlled_block_id = level_owned.DefaultControlledBlockId;
    session.fallen_cells.clear();
    session.broken_keys.clear();
    session.obstacles.clear();
    session.lasers.clear();
    session.blocks.clear();
    session.floor_entities.clear();
    session.move_count = 0;
    session.ready = false;
    session.pending_move = None;

    let mut add_pbs = Vec::new();
    let mut trigger_world_pos: HashMap<i32, [f32; 3]> = HashMap::new();
    for (idx, cell) in level_owned.GridConfig.Grids.iter().enumerate() {
        let x = idx as i32 % session.width;
        let y = idx as i32 / session.width;
        if cell.Type == "BreakTrigger"
            && let Some(key) = cell.Key
        {
            trigger_world_pos.insert(
                key,
                cell_world_pos(
                    session.center_pos,
                    session.forward,
                    session.right,
                    session.width,
                    session.height,
                    session.cell_len,
                    x,
                    y,
                    0.0,
                ),
            );
        }
    }

    for (idx, cell) in level_owned.GridConfig.Grids.iter().enumerate() {
        let x = idx as i32 % session.width;
        let y = idx as i32 / session.width;
        if cell.Type == "Empty" {
            continue;
        }
        let fpos = cell_world_pos(
            session.center_pos,
            session.forward,
            session.right,
            session.width,
            session.height,
            session.cell_len,
            x,
            y,
            0.0,
        );
        let grid_tpl = tpl_cfg
            .GridTemplates
            .iter()
            .find(|g| g.GridType == cell.Type)
            .or_else(|| tpl_cfg.GridTemplates.iter().find(|g| g.GridType == "Floor"));
        if let Some(gt) = grid_tpl
            && let Some((net_id, _)) = spawn_template_entity(scene, assets, &gt.TemplateId, fpos, session.center_rot)
        {
            if let Some(ent) = scene.entity_mut(net_id) {
                ent.world_item.rb_floor = Some(RbFloorComponentPb {
                    GamePlayIncId: session.inc_id,
                    Type: floor_type_code(&cell.Type),
                    OccupiedCellPositions: vec![RbGridPosition { X: x, Y: y }],
                });
                add_pbs.push(ent.to_proto(assets, now_ms));
            }
            session.floor_entities.insert((x, y), net_id);
            session.spawned_net_ids.push(net_id);
        }

        if let Some(blk) = &cell.Block {
            let mut occ = if let Some(lay) = blk.LayType.as_deref() {
                let (dx, dy) = dir_delta(dir_from_str(Some(lay)));
                sort_occ(vec![(x, y), (x + dx, y + dy)])
            } else {
                vec![(x, y)]
            };
            occ = sort_occ(occ);
            let is_two = blk.Type == "Block2";
            let (bpos, brot) = block_world_pose(session, &blk.Type, &occ);
            if let Some(bt) = tpl_cfg
                .BlockTemplate
                .iter()
                .find(|b| b.BlockType == blk.Type)
                && let Some((net_id, storage)) = spawn_template_entity(scene, assets, &bt.TemplateId, bpos, brot)
            {
                let cfg_id = storage.config.config_id;
                let state_pb = RbBlockPbState {
                    State: Some(RbBlockPbStateStateOneof::IdleState(RbBlockIdlePbState {
                        Position: Some(pb_vec(bpos)),
                        Rotation: Some(pb_vec(brot)),
                    })),
                };
                let occ_pb: Vec<RbGridPosition> = occ
                    .iter()
                    .map(|&(ox, oy)| RbGridPosition { X: ox, Y: oy })
                    .collect();
                if let Some(ent) = scene.entity_mut(net_id) {
                    ent.world_item.rb_block = Some(RbBlockComponentPb {
                        Type: Some(RbBlockComponentPbTypeOneof::DefaultBlockType(
                            RbDefaultBlockPbType {
                                IsMainControl: blk.BlockId == session.controlled_block_id,
                            },
                        )),
                        CenterPosition: Some(pb_vec(bpos)),
                        SizeX: 1,
                        SizeY: 1,
                        SizeZ: if is_two { 2 } else { 1 },
                        Forward: Some(pb_vec(session.forward)),
                        Right: Some(pb_vec(session.right)),
                        State: Some(state_pb),
                        GamePlayIncId: session.inc_id,
                        OccupiedCellPositions: occ_pb,
                    });
                    add_pbs.push(ent.to_proto(assets, now_ms));
                }
                session.blocks.push(RollBlockEntityState {
                    block_id: blk.BlockId,
                    block_type: blk.Type.clone(),
                    net_id,
                    config_id: cfg_id,
                    occupied: occ,
                    is_vision: false,
                });
                session.spawned_net_ids.push(net_id);
            }
        }

        for item in &cell.Items {
            let item_tpl = tpl_cfg
                .ItemTemplates
                .iter()
                .find(|it| it.ItemType == item.Type);
            let Some(it_entry) = item_tpl else {
                continue;
            };
            let height = item.Height.unwrap_or(50.0) as f32;
            let ipos = cell_world_pos(
                session.center_pos,
                session.forward,
                session.right,
                session.width,
                session.height,
                session.cell_len,
                x,
                y,
                height,
            );
            if let Some((net_id, storage)) =
                spawn_template_entity(scene, assets, &it_entry.TemplateId, ipos, session.center_rot)
            {
                let cfg_id = storage.config.config_id;
                let occ_pb = vec![RbGridPosition { X: x, Y: y }];
                let is_quantum = item.Type == "QuantumObstacle";
                let q_active = item.InitActive.unwrap_or(true);
                let (oneof, is_obs) = if item.Type == "LaserEmitter" {
                    let dir = dir_from_str(item.Direction.as_deref());
                    let pts = build_laser_points(session, &level_owned, (x, y), dir, height);
                    session.lasers.push(RollBlockLaserState {
                        net_id,
                        pos: (x, y),
                        dir,
                        height,
                    });
                    (
                        RbItemComponentPbTypeOneof::RbLaserEmitterType(RbLaserEmitterPbType {
                            LaserPoints: pts,
                        }),
                        false,
                    )
                } else {
                    let key = item.Key.unwrap_or(0);
                    let link_points = trigger_world_pos
                        .get(&key)
                        .map(|&tp| vec![pb_vec(tp)])
                        .unwrap_or_default();
                    session.obstacles.push(RollBlockObstacleState {
                        net_id,
                        config_id: cfg_id,
                        key,
                        is_quantum,
                        quantum_active: q_active,
                        pos: (x, y),
                        broken: false,
                        link_points: link_points.clone(),
                    });
                    (
                        RbItemComponentPbTypeOneof::BreakableObstacleType(
                            RbBreakableObstaclePbType {
                                LinkPoints: link_points,
                            },
                        ),
                        true,
                    )
                };
                if let Some(ent) = scene.entity_mut(net_id) {
                    ent.world_item.rb_item = Some(RbItemComponentPb {
                        Type: Some(oneof),
                        GamePlayIncId: session.inc_id,
                        OccupiedCellPositions: occ_pb,
                    });
                    if is_obs && is_quantum {
                        ent.config.state = if q_active {
                            EntityState::Default
                        } else {
                            EntityState::Sleep
                        };
                        ent.visible = q_active;
                        ent.actor_visible = q_active;
                    }
                }
                if is_obs && is_quantum {
                    let state_name = if q_active {
                        "关卡.Common.状态.激活"
                    } else {
                        "关卡.Common.状态.休眠"
                    };
                    let _ = scene.update_entity_state_tag(
                        &assets.tables.gameplay_tag_remap,
                        net_id,
                        state_name,
                    );
                }
                if let Some(ent) = scene.entity(net_id) {
                    add_pbs.push(ent.to_proto(assets, now_ms));
                }
                let _ = is_obs;
                session.spawned_net_ids.push(net_id);
            }
        }
    }

    if !add_pbs.is_empty() {
        sink.push(
            EntityAddNotify::MSG_ID,
            &EntityAddNotify {
                EntityPbs: add_pbs,
                RemoveTagIds: false,
            },
        );
    }
    true
}

fn despawn_board_entities<S: PushSink + ?Sized>(
    scene: &mut Scene,
    sink: &mut S,
    session: &mut RollBlockSession,
) {
    let ids = std::mem::take(&mut session.spawned_net_ids);
    let mut removed = Vec::new();
    for nid in ids {
        if scene.entity(nid).is_some() {
            scene.rollback_spawn(nid);
            removed.push(EntityRemoveInfo {
                EntityId: nid,
                Type: None,
            });
        }
    }
    if !removed.is_empty() {
        sink.push(
            EntityRemoveNotify::MSG_ID,
            &EntityRemoveNotify {
                RemoveInfos: removed,
                IsRemove: true,
            },
        );
    }
}

fn update_bt_int_var<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    bt_type: BtType,
    owner_id: i32,
    var_ref_key: &str,
    cond_json: &Value,
    val: i32,
) {
    let Some(vref) = cond_json.get(var_ref_key).and_then(Value::as_object) else {
        return;
    };
    let Some(vname) = resolve_var_name(vref) else {
        return;
    };
    let vsrc = json_str(vref.get("Source")).unwrap_or("Self");
    let prefix = if bt_type == BtType::BtTypeLevelPlay {
        "actor_var"
    } else {
        "quest_var"
    };
    ctx.quest_comp
        .info
        .set_var_int(&format!("{prefix}:{owner_id}:{vname}"), val);
    if vsrc == "Global" {
        ctx.quest_comp.info.set_var_int(vname, val);
        if let Some(pc) = ctx.progress_comp.as_deref_mut() {
            pc.info.set_key_int(&format!("var:{vname}"), i64::from(val));
        }
    } else if vsrc == "Other" {
        if let Some(ref_type) = json_str(vref.get("RefType"))
            && let Some(ref_id) = json_int(vref.get("RefId"))
            && !ref_type.is_empty()
            && ref_id != 0
        {
            ctx.quest_comp
                .info
                .set_var_int(&format!("{ref_type}:{ref_id}:{vname}"), val);
        }
        ctx.quest_comp.info.set_var_int(vname, val);
    } else {
        if owner_id > 0 {
            if bt_type == BtType::BtTypeLevelPlay {
                ctx.quest_comp
                    .info
                    .set_var_int(&format!("LpSelf:{owner_id}:{vname}"), val);
            }
            ctx.quest_comp
                .info
                .set_var_int(&format!("Self:{owner_id}:{vname}"), val);
        }
        ctx.quest_comp.info.set_var_int(vname, val);
    }
    if let Some(pc) = ctx.progress_comp.as_deref_mut() {
        pc.info
            .set_key_int(&format!("actor_var:{owner_id}:{vname}"), i64::from(val));
    }
    ctx.sink.push(
        BtVarUpdateNotify::MSG_ID,
        &BtVarUpdateNotify {
            TreeIncId: crate::helpers::quest_types::bt_tree_inc_id(bt_type, owner_id),
            VarName: vname.as_bytes().to_vec(),
            VarDefine: Some(VarDefinePb {
                VarType: 1,
                Value: Some(VarDefinePbValueOneof::Int(i64::from(val))),
            }),
            TreeOwnerId: owner_id,
        },
    );
}

fn push_roll_block_actions<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    bt_type: BtType,
    owner_id: i32,
    node_id: i32,
    action_type: RollBlockGameActionType,
    field: &str,
    cond_json: &Value,
) -> io::Result<()> {
    let Some(Value::Array(acts)) = cond_json.get(field) else {
        return Ok(());
    };
    if acts.is_empty() {
        return Ok(());
    }
    let inc_id = ctx
        .scene
        .as_deref_mut()
        .map(Scene::alloc_action_inc_id)
        .unwrap_or(1);
    ctx.sink.push(
        ActionNotify::MSG_ID,
        &ActionNotify {
            GameCtx: Some(GameCtxPb {
                CtxType: Some(GameCtxType::RollBlockGamePlayActionCtx),
                CtxInfo: Some(GameCtxPbCtxInfoOneof::RollBlockGamePlayAction(
                    RollBlockGamePlayActionCtxPb {
                        BehaviorTreeCtx: Some(BehaviorTreeCtxPb {
                            IncId: crate::helpers::quest_types::bt_tree_inc_id(bt_type, owner_id),
                            BtType: Some(bt_type),
                            BtId: owner_id,
                            NodeId: node_id,
                        }),
                        ParamType: Some(action_type),
                    },
                )),
            }),
            TotalCount: acts.len() as i32,
            PlayerId: ctx.basic_comp.player_id,
            StartIndex: 0,
            EndIndex: (acts.len() - 1) as i32,
            NeedFinishReq: false,
            IncId: inc_id,
        },
    );
    let mut cb = quest_engine::action_callbacks();
    cb.bt_type = bt_type;
    quest_actions::execute_action_list(ctx, &mut cb, owner_id, node_id, acts)?;
    Ok(())
}

fn find_active_finish_roll_block(
    ctx: &QuestContext<'_, impl PushSink + ?Sized>,
    trigger_type: &str,
    target_cfg_id: i32,
) -> Option<(BtType, i32, i32, Value)> {
    let target_i64 = i64::from(target_cfg_id);
    for aq in &ctx.quest_comp.info.active_quests {
        if let Ok(nodes) = ctx.assets.quest_nodes.get_owner_nodes(aq.quest_id) {
            for &nid in &aq.active_nodes {
                let Some(node) = nodes.iter().find(|n| n.id == nid) else {
                    continue;
                };
                if let Some(cond) = quest_engine::get_child_condition_pub(&node.data)
                    && json_str(cond.get("Type")) == Some("FinishRollBlock")
                    && matches_launch_method(cond, trigger_type, target_i64)
                {
                    return Some((
                        BtType::BtTypeQuest,
                        aq.quest_id,
                        nid,
                        Value::Object(cond.clone()),
                    ));
                }
            }
        }
    }
    for &lp_id in &ctx.quest_comp.info.progress.entered_level_plays {
        if let Ok(nodes) = ctx.assets.level_play_nodes.get_owner_nodes(lp_id) {
            for node in nodes.iter() {
                if node.node_type != "ChildQuest" {
                    continue;
                }
                if !level_play::is_lp_node_active_pub(ctx.progress_comp.as_deref(), lp_id, node.id) {
                    continue;
                }
                if let Some(cond) = quest_engine::get_child_condition_pub(&node.data)
                    && json_str(cond.get("Type")) == Some("FinishRollBlock")
                    && matches_launch_method(cond, trigger_type, target_i64)
                {
                    return Some((
                        BtType::BtTypeLevelPlay,
                        lp_id,
                        node.id,
                        Value::Object(cond.clone()),
                    ));
                }
            }
        }
    }
    None
}

fn matches_launch_method(cond: &Map<String, Value>, trigger_type: &str, target_id: i64) -> bool {
    if trigger_type == "Interact"
        && json_i64(cond.get("InteractEntityId")) == Some(target_id)
    {
        return true;
    }
    if let Some(Value::Array(methods)) = cond.get("LaunchMethod") {
        for m in methods {
            let Some(obj) = m.as_object() else {
                continue;
            };
            if json_str(obj.get("Type")) == Some(trigger_type)
                && json_i64(obj.get("TargetId")) == Some(target_id)
            {
                return true;
            }
        }
    }
    false
}

pub fn try_launch_for_trigger<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    trigger_type: &str,
    target_cfg_id: i32,
) -> io::Result<bool> {
    let Some((bt_type, owner_id, node_id, cond_json)) =
        find_active_finish_roll_block(ctx, trigger_type, target_cfg_id)
    else {
        return Ok(false);
    };
    let group_id = json_int(cond_json.get("RollBlockGroupId")).unwrap_or(0);
    let template_id = json_int(cond_json.get("RollBlockTemplateId")).unwrap_or(100001);
    let center_entity_id = json_i64(cond_json.get("CenterEntityId")).unwrap_or(0);
    if group_id <= 0 || center_entity_id <= 0 {
        return Ok(false);
    }

    let already_open = ctx.scene.as_deref().is_some_and(|s| {
        s.roll_block_sessions
            .values()
            .any(|sess| sess.center_entity_id == center_entity_id && sess.group_id == group_id)
    });
    if already_open {
        return Ok(false);
    }

    let Some(grp) = ctx.assets.roll_blocks.get(&group_id) else {
        return Ok(false);
    };
    if grp.Levels.is_empty() {
        return Ok(false);
    }

    let inst_id = ctx.scene.as_deref().map_or(8, |s| s.instance_id);
    let center_cfg = ctx
        .assets
        .find_level_entity(inst_id, center_entity_id)
        .or_else(|| ctx.assets.find_level_entity_any_map(center_entity_id));
    let (center_pos, center_rot) = if let Some(cfg) = center_cfg {
        (entity_world_pos(&cfg).unwrap_or([0.0; 3]), entity_world_rot(&cfg))
    } else {
        ([0.0; 3], [0.0; 3])
    };
    let (forward, right) = axes_from_rot(center_rot);

    let inc_id = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return Ok(false);
        };
        let id = scene.next_roll_block_inc_id.max(1);
        scene.next_roll_block_inc_id = id + 1;
        id
    };

    let mut session = RollBlockSession {
        inc_id,
        bt_type,
        owner_id,
        node_id,
        center_entity_id,
        group_id,
        template_id,
        cur_diff_idx: 0,
        total_difficulties: grp.Levels.len(),
        center_pos,
        center_rot,
        forward,
        right,
        width: 1,
        height: 1,
        cell_len: DEFAULT_CELL_LEN,
        controlled_block_id: 0,
        blocks: Vec::new(),
        floor_entities: HashMap::new(),
        spawned_net_ids: Vec::new(),
        fallen_cells: HashSet::new(),
        broken_keys: HashSet::new(),
        obstacles: Vec::new(),
        lasers: Vec::new(),
        move_count: 0,
        hint_open: false,
        ready: false,
        pending_move: None,
        cond_json: cond_json.clone(),
    };

    let now_ms = quest_engine::now_ms_pub();
    let info = {
        let scene = ctx.scene.as_deref_mut().expect("scene present");
        if !populate_board_entities(scene, ctx.assets, ctx.sink, &mut session, now_ms) {
            return Ok(false);
        }
        let info = build_rb_gameplay_info(&session, ctx.assets, true);
        scene.roll_block_sessions.insert(inc_id, session);
        info
    };

    update_bt_int_var(
        ctx,
        bt_type,
        owner_id,
        "AllDifficultyCountVarRef",
        &cond_json,
        grp.Levels.len() as i32,
    );
    update_bt_int_var(
        ctx,
        bt_type,
        owner_id,
        "CurrentDifficultyVarRef",
        &cond_json,
        1,
    );
    push_roll_block_actions(
        ctx,
        bt_type,
        owner_id,
        node_id,
        RollBlockGameActionType::RbEnter,
        "EnterActions",
        &cond_json,
    )?;

    ctx.sink.push(
        RollBlockOpenGamePlayNotify::MSG_ID,
        &RollBlockOpenGamePlayNotify { Info: Some(info) },
    );
    Ok(true)
}

pub fn handle_ready<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
) -> ErrorCode {
    let inputs = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return ErrorCode::Success;
        };
        let Some(session) = scene.roll_block_sessions.get_mut(&inc_id) else {
            return ErrorCode::Success;
        };
        session.ready = true;
        compute_available_inputs(session, ctx.assets)
    };
    ctx.sink.push(
        RollBlockUpdateAvailableInputsNotify::MSG_ID,
        &RollBlockUpdateAvailableInputsNotify {
            IncId: inc_id,
            AvailableInputs: inputs,
        },
    );
    ErrorCode::Success
}

pub fn handle_input<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
    input: Option<RbInput>,
) -> ErrorCode {
    let Some(dir) = input.and_then(|i| i.Roll).and_then(|r| r.Direction) else {
        return ErrorCode::Success;
    };
    let state_change = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return ErrorCode::Success;
        };
        let Some(session) = scene.roll_block_sessions.get_mut(&inc_id) else {
            return ErrorCode::Success;
        };
        let Some((_, level_owned)) =
            resolve_level(ctx.assets, session.group_id, session.cur_diff_idx)
                .map(|(g, l)| (g, l.clone()))
        else {
            return ErrorCode::Success;
        };
        let Some(blk_idx) = session
            .blocks
            .iter()
            .position(|b| b.block_id == session.controlled_block_id)
        else {
            return ErrorCode::Success;
        };
        let blk_id = session.blocks[blk_idx].block_id;
        let blk_net_id = session.blocks[blk_idx].net_id;
        let next_occ = compute_roll(
            &session.blocks[blk_idx].block_type,
            &session.blocks[blk_idx].occupied,
            dir,
        );
        if next_occ
            .iter()
            .any(|&(cx, cy)| is_cell_solid_blocked(session, &level_owned, blk_id, cx, cy))
        {
            let inputs = compute_available_inputs(session, ctx.assets);
            ctx.sink.push(
                RollBlockUpdateAvailableInputsNotify::MSG_ID,
                &RollBlockUpdateAvailableInputsNotify {
                    IncId: inc_id,
                    AvailableInputs: inputs,
                },
            );
            return ErrorCode::Success;
        }
        let fell_off = next_occ.iter().any(|&(cx, cy)| {
            session.fallen_cells.contains(&(cx, cy))
                || grid_cell_at(&level_owned, cx, cy).is_none_or(|c| c.Type == "Empty")
        });
        let pending_jump = if !fell_off && next_occ.len() == 1 {
            let (jx, jy) = next_occ[0];
            if let Some(cell) = grid_cell_at(&level_owned, jx, jy)
                && cell.Type == "JumpFloor"
            {
                let jdir = dir_from_str(cell.JumpDirection.as_deref());
                let (jdx, jdy) = dir_delta(jdir);
                let tx = jx + 2 * jdx;
                let ty = jy + 2 * jdy;
                if !is_cell_solid_blocked(session, &level_owned, blk_id, tx, ty) {
                    Some(jdir)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        session.pending_move = Some(PendingRollMove {
            block_net_id: blk_net_id,
            block_id: blk_id,
            target_occupied: next_occ,
            pending_jump,
            fell_off,
        });
        RbBlockStateChangeNotify {
            EntityId: blk_net_id,
            IncId: inc_id,
            State: Some(RbBlockPbState {
                State: Some(RbBlockPbStateStateOneof::MovingState(
                    RbBlockMovingPbState {
                        Action: Some(RbBlockMovementPbAction {
                            Type: Some(RbBlockMovementPbActionTypeOneof::Roll(RbRollMovement {
                                Direction: Some(dir),
                            })),
                        }),
                    },
                )),
            }),
        }
    };
    ctx.sink
        .push(RbBlockStateChangeNotify::MSG_ID, &state_change);
    ErrorCode::Success
}

fn is_goal_reached(session: &RollBlockSession, level: &RollBlockLevelConfig) -> bool {
    let main_blocks: Vec<&RollBlockEntityState> =
        session.blocks.iter().filter(|b| !b.is_vision).collect();
    if main_blocks.is_empty() {
        return false;
    }
    main_blocks.iter().all(|b| {
        if b.occupied.len() == 1 {
            let (x, y) = b.occupied[0];
            grid_cell_at(level, x, y).is_some_and(|c| c.Type == "Goal")
        } else if b.occupied.len() == 2 {
            let (x1, y1) = b.occupied[0];
            let (x2, y2) = b.occupied[1];
            let c1 = grid_cell_at(level, x1, y1);
            let c2 = grid_cell_at(level, x2, y2);
            let matches_pair = |a: Option<&RollBlockCellConfig>,
                                ax: i32,
                                ay: i32,
                                b_cell: Option<&RollBlockCellConfig>,
                                bx: i32,
                                by: i32|
             -> bool {
                let Some(ac) = a else { return false };
                let Some(bc) = b_cell else { return false };
                if ac.Type == "LinkGoal" && bc.Type == "Goal" {
                    let (dx, dy) = dir_delta(dir_from_str(ac.LinkDirection.as_deref()));
                    return ax + dx == bx && ay + dy == by;
                }
                false
            };
            matches_pair(c1, x1, y1, c2, x2, y2) || matches_pair(c2, x2, y2, c1, x1, y1)
        } else {
            false
        }
    })
}

pub fn handle_movement_finish<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
    entity_id: i64,
) -> io::Result<ErrorCode> {
    let now_ms = quest_engine::now_ms_pub();

    enum FinishOutcome {
        Jump {
            blk_net_id: i64,
            jdir: RbGridDirection,
        },
        Reset(RollBlockGamePlayPbInfo),
        NextLevel {
            reward_id: i32,
            new_diff: i32,
            bt_type: BtType,
            owner_id: i32,
            cond_json: Value,
            info: RollBlockGamePlayPbInfo,
        },
        CompletedAll {
            reward_id: i32,
            bt_type: BtType,
            owner_id: i32,
            node_id: i32,
            group_id: i32,
            center_entity_id: i64,
            cond_json: Value,
        },
        Continue {
            blk_net_id: i64,
            pos: [f32; 3],
            rot: [f32; 3],
            inputs: Vec<RbInput>,
            item_updates: Vec<(i64, RbItemComponentPb)>,
            removed_entities: Vec<i64>,
            state_tag_updates: Vec<(i64, StateTagUpdate)>,
            combat_pack: Vec<CombatReceiveData>,
        },
    }

    let outcome = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return Ok(ErrorCode::Success);
        };
        let Some(mut session) = scene.roll_block_sessions.remove(&inc_id) else {
            return Ok(ErrorCode::Success);
        };
        let Some(pending) = session.pending_move.take() else {
            scene.roll_block_sessions.insert(inc_id, session);
            return Ok(ErrorCode::Success);
        };
        let Some((_, level_owned)) =
            resolve_level(ctx.assets, session.group_id, session.cur_diff_idx)
                .map(|(g, l)| (g, l.clone()))
        else {
            scene.roll_block_sessions.insert(inc_id, session);
            return Ok(ErrorCode::Success);
        };

        let mut removed_entities = Vec::new();
        let prev_occ = session
            .blocks
            .iter()
            .find(|b| b.block_id == pending.block_id)
            .map(|b| b.occupied.clone())
            .unwrap_or_default();

        // JumpFloor chain step: trigger jump animation if JumpFloor triggered
        if let Some(jdir) = pending.pending_jump
            && pending.target_occupied.len() == 1
        {
            let (jx, jy) = pending.target_occupied[0];
            let (jdx, jdy) = dir_delta(jdir);
            let tx = jx + 2 * jdx;
            let ty = jy + 2 * jdy;
            let fell_after_jump = session.fallen_cells.contains(&(tx, ty))
                || grid_cell_at(&level_owned, tx, ty).is_none_or(|c| c.Type == "Empty");
            session.pending_move = Some(PendingRollMove {
                block_net_id: pending.block_net_id,
                block_id: pending.block_id,
                target_occupied: vec![(tx, ty)],
                pending_jump: None,
                fell_off: fell_after_jump,
            });
            scene.roll_block_sessions.insert(inc_id, session);
            FinishOutcome::Jump {
                blk_net_id: pending.block_net_id,
                jdir,
            }
        } else if pending.fell_off {
            populate_board_entities(scene, ctx.assets, ctx.sink, &mut session, now_ms);
            let info = build_rb_gameplay_info(&session, ctx.assets, true);
            scene.roll_block_sessions.insert(inc_id, session);
            FinishOutcome::Reset(info)
        } else {
            session.move_count += 1;
            // Crack any FallFloor the block stood on cleanly and just stepped off
            if prev_occ.len() == 1 {
                let (px, py) = prev_occ[0];
                if grid_cell_at(&level_owned, px, py).is_some_and(|c| c.Type == "FallFloor")
                    && !pending.target_occupied.contains(&(px, py))
                {
                    session.fallen_cells.insert((px, py));
                    if let Some(fnet) = session.floor_entities.remove(&(px, py)) {
                        removed_entities.push(fnet);
                    }
                }
            }
            // Update block occupied coordinates
            let updated_pose = if let Some(idx) = session
                .blocks
                .iter()
                .position(|b| b.block_id == pending.block_id)
            {
                session.blocks[idx].occupied = pending.target_occupied.clone();
                let blk_type = session.blocks[idx].block_type.clone();
                let blk_occ = session.blocks[idx].occupied.clone();
                block_world_pose(&session, &blk_type, &blk_occ)
            } else {
                (session.center_pos, session.center_rot)
            };
            if let Some(ent) = scene.entity_mut(pending.block_net_id) {
                ent.position = Some(PositionComponent {
                    location: updated_pose.0,
                    rotation: updated_pose.1,
                });
            }
            // BreakTrigger handling
            for &(cx, cy) in &pending.target_occupied {
                if let Some(cell) = grid_cell_at(&level_owned, cx, cy)
                    && cell.Type == "BreakTrigger"
                    && let Some(key) = cell.Key
                    && session.broken_keys.insert(key)
                {
                    for obs in &mut session.obstacles {
                        if !obs.is_quantum && obs.key == key && !obs.broken {
                            obs.broken = true;
                            removed_entities.push(obs.net_id);
                        }
                    }
                }
            }
            // QuantumObstacle toggle
            let mut quantum_toggled = Vec::new();
            for obs in &mut session.obstacles {
                if obs.is_quantum && !obs.broken {
                    obs.quantum_active = !obs.quantum_active;
                    quantum_toggled.push((
                        obs.net_id,
                        obs.pos,
                        obs.quantum_active,
                        obs.link_points.clone(),
                    ));
                }
            }
            for nid in &removed_entities {
                scene.rollback_spawn(*nid);
            }
            let laser_hits = laser_blocked_cells(&session, &level_owned);
            let hit_by_laser = pending
                .target_occupied
                .iter()
                .any(|pos| laser_hits.contains(pos));
            if hit_by_laser {
                populate_board_entities(scene, ctx.assets, ctx.sink, &mut session, now_ms);
                let info = build_rb_gameplay_info(&session, ctx.assets, true);
                scene.roll_block_sessions.insert(inc_id, session);
                FinishOutcome::Reset(info)
            } else if is_goal_reached(&session, &level_owned) {
                let reward_id = level_owned.RewardId;
                if session.cur_diff_idx + 1 < session.total_difficulties {
                    session.cur_diff_idx += 1;
                    populate_board_entities(scene, ctx.assets, ctx.sink, &mut session, now_ms);
                    let info = build_rb_gameplay_info(&session, ctx.assets, true);
                    let new_diff = info.Difficulty;
                    let bt_type = session.bt_type;
                    let owner_id = session.owner_id;
                    let cond_json = session.cond_json.clone();
                    scene.roll_block_sessions.insert(inc_id, session);
                    FinishOutcome::NextLevel {
                        reward_id,
                        new_diff,
                        bt_type,
                        owner_id,
                        cond_json,
                        info,
                    }
                } else {
                    despawn_board_entities(scene, ctx.sink, &mut session);
                    FinishOutcome::CompletedAll {
                        reward_id,
                        bt_type: session.bt_type,
                        owner_id: session.owner_id,
                        node_id: session.node_id,
                        group_id: session.group_id,
                        center_entity_id: session.center_entity_id,
                        cond_json: session.cond_json,
                    }
                }
            } else {
                let mut item_updates = Vec::new();
                let mut state_tag_updates = Vec::new();
                let mut combat_pack = Vec::new();
                for (nid, pos, active, link_points) in quantum_toggled {
                    let comp = RbItemComponentPb {
                        Type: Some(RbItemComponentPbTypeOneof::BreakableObstacleType(
                            RbBreakableObstaclePbType {
                                LinkPoints: link_points,
                            },
                        )),
                        GamePlayIncId: session.inc_id,
                        OccupiedCellPositions: vec![RbGridPosition { X: pos.0, Y: pos.1 }],
                    };
                    if let Some(ent) = scene.entity_mut(nid) {
                        ent.world_item.rb_item = Some(comp.clone());
                        ent.config.state = if active {
                            EntityState::Default
                        } else {
                            EntityState::Sleep
                        };
                    }
                    let state_name = if active {
                        "关卡.Common.状态.激活"
                    } else {
                        "关卡.Common.状态.休眠"
                    };
                    if let Some(upd) = scene.update_entity_state_tag(
                        &ctx.assets.tables.gameplay_tag_remap,
                        nid,
                        state_name,
                    ) {
                        state_tag_updates.push((nid, upd));
                    }
                    let _ = scene.set_entity_visibility(nid, active, &mut combat_pack);
                    item_updates.push((nid, comp));
                }
                for l in &session.lasers {
                    let pts = build_laser_points(&session, &level_owned, l.pos, l.dir, l.height);
                    let comp = RbItemComponentPb {
                        Type: Some(RbItemComponentPbTypeOneof::RbLaserEmitterType(
                            RbLaserEmitterPbType { LaserPoints: pts },
                        )),
                        GamePlayIncId: session.inc_id,
                        OccupiedCellPositions: vec![RbGridPosition {
                            X: l.pos.0,
                            Y: l.pos.1,
                        }],
                    };
                    if let Some(ent) = scene.entity_mut(l.net_id) {
                        ent.world_item.rb_item = Some(comp.clone());
                    }
                    item_updates.push((l.net_id, comp));
                }
                let inputs = compute_available_inputs(&session, ctx.assets);
                scene.roll_block_sessions.insert(inc_id, session);
                FinishOutcome::Continue {
                    blk_net_id: pending.block_net_id,
                    pos: updated_pose.0,
                    rot: updated_pose.1,
                    inputs,
                    item_updates,
                    removed_entities,
                    state_tag_updates,
                    combat_pack,
                }
            }
        }
    };

    match outcome {
        FinishOutcome::Jump { blk_net_id, jdir } => {
            ctx.sink.push(
                RbBlockStateChangeNotify::MSG_ID,
                &RbBlockStateChangeNotify {
                    EntityId: blk_net_id,
                    IncId: inc_id,
                    State: Some(RbBlockPbState {
                        State: Some(RbBlockPbStateStateOneof::MovingState(
                            RbBlockMovingPbState {
                                Action: Some(RbBlockMovementPbAction {
                                    Type: Some(RbBlockMovementPbActionTypeOneof::Jump(
                                        RbJumpMovement {
                                            Direction: Some(jdir),
                                        },
                                    )),
                                }),
                            },
                        )),
                    }),
                },
            );
        }
        FinishOutcome::Reset(info) => {
            ctx.sink.push(
                RollBlockResetGamePlayNotify::MSG_ID,
                &RollBlockResetGamePlayNotify { Info: Some(info) },
            );
        }
        FinishOutcome::NextLevel {
            reward_id,
            new_diff,
            bt_type,
            owner_id,
            cond_json,
            info,
        } => {
            grant_level_reward(ctx, reward_id);
            update_bt_int_var(
                ctx,
                bt_type,
                owner_id,
                "CurrentDifficultyVarRef",
                &cond_json,
                new_diff,
            );
            ctx.sink.push(
                RollBlockUpdateGamePlayNotify::MSG_ID,
                &RollBlockUpdateGamePlayNotify { Info: Some(info) },
            );
        }
        FinishOutcome::CompletedAll {
            reward_id,
            bt_type,
            owner_id,
            node_id,
            group_id,
            center_entity_id,
            cond_json,
        } => {
            grant_level_reward(ctx, reward_id);
            if let Some(pc) = ctx.progress_comp.as_deref_mut() {
                pc.info.set_key_int("rb_blk:fin", 1);
                pc.info.mark_claim(&format!("rb_blk:fin:{group_id}"));
            }
            push_roll_block_actions(
                ctx,
                bt_type,
                owner_id,
                node_id,
                RollBlockGameActionType::RbPass,
                "CompleteActions",
                &cond_json,
            )?;
            ctx.sink.push(
                RollBlockCloseGamePlayNotify::MSG_ID,
                &RollBlockCloseGamePlayNotify { IncId: inc_id },
            );
            quest_engine::on_condition_event(
                ctx,
                QuestEventKind::FinishRollBlock,
                center_entity_id as i32,
                group_id,
            )?;
        }
        FinishOutcome::Continue {
            blk_net_id,
            pos,
            rot,
            inputs,
            item_updates,
            removed_entities,
            state_tag_updates,
            combat_pack,
        } => {
            if !removed_entities.is_empty() {
                ctx.sink.push(
                    EntityRemoveNotify::MSG_ID,
                    &EntityRemoveNotify {
                        RemoveInfos: removed_entities
                            .into_iter()
                            .map(|id| EntityRemoveInfo {
                                EntityId: id,
                                Type: None,
                            })
                            .collect(),
                        IsRemove: true,
                    },
                );
            }
            if !state_tag_updates.is_empty() {
                if let Some(sc) = ctx.scene.as_deref() {
                    for (nid, upd) in state_tag_updates {
                        quest_actions::emit_entity_state_tag_update_checked(
                            sc,
                            Some(ctx.assets),
                            ctx.sink,
                            nid,
                            upd,
                        );
                    }
                } else {
                    for (nid, upd) in state_tag_updates {
                        quest_actions::emit_entity_state_tag_update(ctx.sink, nid, upd);
                    }
                }
            }
            if !combat_pack.is_empty() {
                ctx.sink.push(
                    CombatReceivePackNotify::MSG_ID,
                    &CombatReceivePackNotify { Data: combat_pack },
                );
            }
            for (eid, comp) in item_updates {
                ctx.sink.push(
                    RollBlockItemUpdateNotify::MSG_ID,
                    &RollBlockItemUpdateNotify {
                        Component: Some(comp),
                        EntityId: eid,
                    },
                );
            }
            ctx.sink.push(
                RbBlockStateChangeNotify::MSG_ID,
                &RbBlockStateChangeNotify {
                    EntityId: blk_net_id,
                    IncId: inc_id,
                    State: Some(RbBlockPbState {
                        State: Some(RbBlockPbStateStateOneof::IdleState(RbBlockIdlePbState {
                            Position: Some(pb_vec(pos)),
                            Rotation: Some(pb_vec(rot)),
                        })),
                    }),
                },
            );
            ctx.sink.push(
                RollBlockUpdateAvailableInputsNotify::MSG_ID,
                &RollBlockUpdateAvailableInputsNotify {
                    IncId: inc_id,
                    AvailableInputs: inputs,
                },
            );
        }
    }
    let _ = entity_id;
    Ok(ErrorCode::Success)
}

fn grant_level_reward<S: PushSink + ?Sized>(ctx: &mut QuestContext<'_, S>, reward_id: i32) {
    if reward_id <= 0 {
        return;
    }
    let mut rctx = ctx.make_reward_context();
    let _ = reward::grant_drop_package(&mut rctx, reward_id, 0);
}

pub fn handle_reset<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
) -> ErrorCode {
    let now_ms = quest_engine::now_ms_pub();
    let info_opt = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return ErrorCode::Success;
        };
        let Some(mut session) = scene.roll_block_sessions.remove(&inc_id) else {
            return ErrorCode::Success;
        };
        populate_board_entities(scene, ctx.assets, ctx.sink, &mut session, now_ms);
        let info = build_rb_gameplay_info(&session, ctx.assets, true);
        scene.roll_block_sessions.insert(inc_id, session);
        Some(info)
    };
    if let Some(info) = info_opt {
        ctx.sink.push(
            RollBlockResetGamePlayNotify::MSG_ID,
            &RollBlockResetGamePlayNotify { Info: Some(info) },
        );
    }
    ErrorCode::Success
}

pub fn handle_switch_control_block<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
) -> (i64, i64) {
    let Some(scene) = ctx.scene.as_deref_mut() else {
        return (0, 0);
    };
    let Some(session) = scene.roll_block_sessions.get_mut(&inc_id) else {
        return (0, 0);
    };
    let main_blocks: Vec<(i32, i64)> = session
        .blocks
        .iter()
        .filter(|b| !b.is_vision)
        .map(|b| (b.block_id, b.net_id))
        .collect();
    if main_blocks.len() < 2 {
        let cur = main_blocks.first().map_or(0, |x| x.1);
        return (cur, cur);
    }
    let cur_pos = main_blocks
        .iter()
        .position(|(bid, _)| *bid == session.controlled_block_id)
        .unwrap_or(0);
    let next_pos = (cur_pos + 1) % main_blocks.len();
    let prev_net = main_blocks[cur_pos].1;
    let (next_bid, next_net) = main_blocks[next_pos];
    session.controlled_block_id = next_bid;
    let inputs = compute_available_inputs(session, ctx.assets);
    ctx.sink.push(
        RollBlockUpdateAvailableInputsNotify::MSG_ID,
        &RollBlockUpdateAvailableInputsNotify {
            IncId: inc_id,
            AvailableInputs: inputs,
        },
    );
    (prev_net, next_net)
}

pub fn handle_exit<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    inc_id: i32,
) -> io::Result<ErrorCode> {
    let removed_session = {
        let Some(scene) = ctx.scene.as_deref_mut() else {
            return Ok(ErrorCode::Success);
        };
        let Some(mut session) = scene.roll_block_sessions.remove(&inc_id) else {
            return Ok(ErrorCode::Success);
        };
        despawn_board_entities(scene, ctx.sink, &mut session);
        session
    };
    push_roll_block_actions(
        ctx,
        removed_session.bt_type,
        removed_session.owner_id,
        removed_session.node_id,
        RollBlockGameActionType::RbMidWayExit,
        "ExitActions",
        &removed_session.cond_json,
    )?;
    ctx.sink.push(
        RollBlockCloseGamePlayNotify::MSG_ID,
        &RollBlockCloseGamePlayNotify { IncId: inc_id },
    );
    Ok(ErrorCode::Success)
}

pub fn handle_toggle_hint(scene: Option<&mut Scene>, inc_id: i32, is_open: bool) -> ErrorCode {
    if let Some(sc) = scene
        && let Some(sess) = sc.roll_block_sessions.get_mut(&inc_id)
    {
        sess.hint_open = is_open;
    }
    ErrorCode::Success
}

pub fn cleanup_all_sessions<S: PushSink + ?Sized>(scene: &mut Scene, sink: &mut S) {
    let keys: Vec<i32> = scene.roll_block_sessions.keys().copied().collect();
    for k in keys {
        if let Some(mut sess) = scene.roll_block_sessions.remove(&k) {
            despawn_board_entities(scene, sink, &mut sess);
            sink.push(
                RollBlockCloseGamePlayNotify::MSG_ID,
                &RollBlockCloseGamePlayNotify { IncId: k },
            );
        }
    }
}

pub fn try_grant_action_reward<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    params: &Map<String, Value>,
) {
    let explicit_drop = json_int(params.get("RewardId"))
        .or_else(|| json_int(params.get("DropId")))
        .unwrap_or(0);
    if explicit_drop > 0 {
        grant_level_reward(ctx, explicit_drop);
        return;
    }
    let gid = json_int(params.get("GroupId"))
        .or_else(|| json_int(params.get("RollBlockGroupId")))
        .unwrap_or(0);
    let diff = json_int(params.get("Difficulty")).unwrap_or(1);
    if gid > 0
        && let Some(grp) = ctx.assets.roll_blocks.get(&gid)
        && let Some(lv) = grp.Levels.iter().find(|l| l.Difficulty == diff)
    {
        grant_level_reward(ctx, lv.RewardId);
    }
}
