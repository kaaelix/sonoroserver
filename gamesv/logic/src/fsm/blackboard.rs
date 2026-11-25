use sonoro_gamesv_data::fsm_graph_registry::{Graph, TaskKind};
use sonoro_proto::enums::EFsmStateFlag;
use sonoro_proto::pb::{
    CombatCommon, CombatNotifyData, CombatNotifyDataMessageOneof, CombatReceiveData,
    CombatReceiveDataMessageOneof, DFsm, DFsmBlackBoard, DFsmBlackboardCustom,
    EntityFsmComponentPb, FsmBlackboardNotify, FsmCustomBlackboardDatas, FsmResetNotify,
};

use crate::component::entity::fsm::FsmComponent;
use crate::fsm::state_hierarchy;

pub const BLACKBOARD_MONTAGE_KEY: usize = 1;
pub const BLACKBOARD_TARGET_KEY: usize = 2;

pub fn select_montage_index(hash_code: i32, state_id: i32, activated_at: i64, count: usize) -> i32 {
    if count <= 1 {
        return 0;
    }
    let mut seed = activated_at as u64;
    let state_bits = state_id as u32 as u64;
    let hash_bits = hash_code as u32 as u64;
    seed ^= state_bits.wrapping_mul(0x9E37_79B1_85EB_CA87);
    seed ^= hash_bits.wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    seed ^= seed >> 12;
    seed ^= seed << 25;
    seed ^= seed >> 27;
    (seed % (count as u64)) as i32
}

pub fn set_value(comp: &mut FsmComponent, key: usize, value: Option<i32>, notify: bool) -> bool {
    if key >= comp.blackboard.len() {
        return false;
    }
    if comp.blackboard[key] == value {
        return false;
    }
    comp.blackboard[key] = value;
    if notify {
        comp.dirty_blackboard_mask |= 1u16 << key;
    }
    true
}

pub fn prepare_path(comp: &mut FsmComponent, path: &[i32], since_ms: &[i64], notify: bool) {
    let mut montage_value: Option<i32> = None;
    for (idx, &state_id) in path.iter().enumerate() {
        let Some(node) = state_hierarchy::find_node(comp, state_id) else {
            continue;
        };
        let Some(task) = &node.Task else {
            continue;
        };
        if let Some(rm) = &task.TaskRandomMontage
            && !rm.RandomByClient
            && !rm.MontageNames.is_empty()
        {
            let seed = since_ms.get(idx).copied().unwrap_or(0);
            montage_value = Some(select_montage_index(
                comp.graph.hash_code,
                state_id,
                seed,
                rm.MontageNames.len(),
            ));
        }
    }
    if let Some(idx_val) = montage_value {
        set_value(comp, BLACKBOARD_MONTAGE_KEY, Some(idx_val), notify);
    }
}

pub fn collect_blackboard_entries(comp: &FsmComponent) -> Vec<DFsmBlackBoard> {
    let mut out = Vec::new();
    for (key, val) in comp.blackboard.iter().enumerate().skip(1) {
        if let Some(v) = val {
            out.push(DFsmBlackBoard {
                Key: key as i32,
                Value: *v,
            });
        }
    }
    out
}

pub fn append_blackboard_notify(
    comp: &mut FsmComponent,
    entity_id: i64,
    output: &mut Vec<CombatReceiveData>,
) {
    if comp.dirty_blackboard_mask == 0 {
        return;
    }
    let mut entries = Vec::new();
    for key in 1..comp.blackboard.len() {
        if (comp.dirty_blackboard_mask & (1u16 << key)) == 0 {
            continue;
        }
        entries.push(DFsmBlackBoard {
            Key: key as i32,
            Value: comp.blackboard[key].unwrap_or(0),
        });
    }
    comp.dirty_blackboard_mask = 0;
    if entries.is_empty() {
        return;
    }
    output.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::FsmBlackboardNotify(
                    FsmBlackboardNotify {
                        FsmBlackBoards: entries,
                    },
                )),
            },
        )),
    });
}

pub fn to_proto(comp: &FsmComponent, now_ms: i64) -> EntityFsmComponentPb {
    let fsms = get_initial_fsm(comp, now_ms);
    let custom = if comp.custom_blackboards.is_empty() {
        None
    } else {
        Some(FsmCustomBlackboardDatas {
            BlackboardIntValues: comp
                .custom_blackboards
                .iter()
                .map(|(k, v)| DFsmBlackboardCustom {
                    Key: k.as_bytes().to_vec(),
                    Value: *v,
                })
                .collect(),
        })
    };

    EntityFsmComponentPb {
        Fsms: fsms,
        HashCode: comp.graph.hash_code,
        CommonHashCode: comp.graph.common_hash_code,
        BlackBoard: collect_blackboard_entries(comp),
        FsmCustomBlackboardDatas: custom,
    }
}

pub fn collect_blackboard_snapshot_entries(comp: &FsmComponent) -> Vec<DFsmBlackBoard> {
    comp.blackboard
        .iter()
        .enumerate()
        .skip(1)
        .map(|(key, val)| DFsmBlackBoard {
            Key: key as i32,
            Value: val.unwrap_or(0),
        })
        .collect()
}

pub fn append_reset_notify(
    comp: &mut FsmComponent,
    entity_id: i64,
    now_ms: i64,
    output: &mut Vec<CombatReceiveData>,
) {
    let mut proto = to_proto(comp, now_ms);
    let snapshot = collect_blackboard_snapshot_entries(comp);
    proto.BlackBoard = snapshot.clone();
    output.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::FsmBlackboardNotify(
                    FsmBlackboardNotify {
                        FsmBlackBoards: snapshot,
                    },
                )),
            },
        )),
    });
    output.push(CombatReceiveData {
        Message: Some(CombatReceiveDataMessageOneof::CombatNotifyData(
            CombatNotifyData {
                CombatCommon: Some(CombatCommon {
                    EntityId: entity_id,
                    ..Default::default()
                }),
                Message: Some(CombatNotifyDataMessageOneof::FsmResetNotify(
                    FsmResetNotify {
                        EntityFsmComponentPb: Some(proto),
                    },
                )),
            },
        )),
    });
    comp.dirty_blackboard_mask = 0;
}

pub fn get_initial_fsm(comp: &FsmComponent, now_ms: i64) -> Vec<DFsm> {
    let _ = (TaskKind::Invalid, Graph::canonical_state);
    if !comp.runtime_nodes.is_empty() {
        return comp
            .runtime_nodes
            .iter()
            .filter_map(|runtime| {
                let leaf = runtime.leaf()?;
                let since = runtime
                    .active_len
                    .checked_sub(1)
                    .and_then(|i| runtime.active_since_ms.get(i).copied())
                    .unwrap_or(now_ms);
                let elapsed = now_ms.saturating_sub(since).clamp(0, i64::from(i32::MAX)) as i32;
                let is_anim = state_hierarchy::find_node(comp, runtime.fsm_id)
                    .and_then(|n| n.IsAnimStateMachine)
                    .unwrap_or(false);
                Some(DFsm {
                    FsmId: state_hierarchy::client_state(comp, runtime.fsm_id),
                    CurrentState: state_hierarchy::client_state(comp, leaf),
                    Flag: Some(if is_anim {
                        EFsmStateFlag::Confirmed
                    } else {
                        EFsmStateFlag::Changed
                    }),
                    StateElapseTime: elapsed,
                })
            })
            .collect();
    }

    let mut seen_roots = Vec::new();
    let mut out = Vec::new();
    for &raw_id in &comp.graph.state_list {
        let id = state_hierarchy::canonical_state(comp, raw_id);
        if seen_roots.contains(&id) {
            continue;
        }
        seen_roots.push(id);
        let Some(node) = state_hierarchy::find_node(comp, id) else {
            continue;
        };
        let mut active_path = [0i32; sonoro_gamesv_data::fsm_graph_registry::MAX_STATE_DEPTH];
        let Some(active_len) = state_hierarchy::build_initial_path(comp, id, &mut active_path) else {
            continue;
        };
        let is_anim = node.IsAnimStateMachine.unwrap_or(false);
        out.push(DFsm {
            FsmId: state_hierarchy::client_state(comp, id),
            CurrentState: state_hierarchy::client_state(comp, active_path[active_len - 1]),
            Flag: Some(if is_anim {
                EFsmStateFlag::Confirmed
            } else {
                EFsmStateFlag::Changed
            }),
            StateElapseTime: 0,
        });
    }
    out
}
