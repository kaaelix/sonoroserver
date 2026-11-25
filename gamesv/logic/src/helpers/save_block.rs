//! Save blocks of a behavior tree.
//!
//! A node carrying `SaveConfig` defines a persistence boundary: progress made inside it is not
//! restored across login. Instead, login clears progress inside active save blocks so the tree
//! re-enters the block once the scene is ready, applying `PosRollbackCfg` if applicable.

use serde_json::Value;
use sonoro_gamesv_data::lazy_node_store::NodeEntry;

const MAX_DEPTH: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosRollback {
    pub range_entity_id: i32,
    pub position_entity_id: i32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Discard {
    pub blocks: Vec<i32>,
    pub reenter: Vec<i32>,
    pub unfinish: Vec<i32>,
}

impl Discard {
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}

fn find_node(nodes: &[NodeEntry], node_id: i32) -> Option<&NodeEntry> {
    nodes.iter().find(|n| n.id == node_id)
}

fn save_config_of(node: &NodeEntry) -> Option<&serde_json::Map<String, Value>> {
    node.data.as_object()?.get("SaveConfig")?.as_object()
}

/// Innermost node carrying `SaveConfig` among `node_id` and its ancestors.
pub fn find_enclosing_save_node(nodes: &[NodeEntry], node_id: i32) -> Option<i32> {
    let mut cur = node_id;
    for _ in 0..MAX_DEPTH {
        if cur <= 0 {
            break;
        }
        let node = find_node(nodes, cur)?;
        if save_config_of(node).is_some() {
            return Some(cur);
        }
        cur = node.parent_id;
    }
    None
}

/// Returns true if `node_id` is `block_id` itself or one of its descendants.
pub fn is_in_block(nodes: &[NodeEntry], node_id: i32, block_id: i32) -> bool {
    let mut cur = node_id;
    for _ in 0..MAX_DEPTH {
        if cur <= 0 {
            break;
        }
        if cur == block_id {
            return true;
        }
        let Some(node) = find_node(nodes, cur) else {
            return false;
        };
        cur = node.parent_id;
    }
    false
}

/// Reads `SaveConfig.PosRollbackCfg` from a save node when both range and position IDs exist.
pub fn pos_rollback_of(nodes: &[NodeEntry], block_id: i32) -> Option<PosRollback> {
    let node = find_node(nodes, block_id)?;
    let cfg = save_config_of(node)?;
    let pos_cfg = cfg.get("PosRollbackCfg")?.as_object()?;
    let range_entity_id = pos_cfg
        .get("RangeEntityId")
        .and_then(|v| v.as_i64())
        .and_then(|v| i32::try_from(v).ok())?;
    let position_entity_id = pos_cfg
        .get("PositionEntityId")
        .and_then(|v| v.as_i64())
        .and_then(|v| i32::try_from(v).ok())?;
    if position_entity_id <= 0 {
        return None;
    }
    Some(PosRollback {
        range_entity_id,
        position_entity_id,
    })
}

/// Computes which active/finished nodes inside save blocks must be reset on login.
pub fn plan_login_discard(
    nodes: &[NodeEntry],
    active_nodes: &[i32],
    finished_nodes: &[i32],
) -> Discard {
    let mut blocks = Vec::new();
    let mut reenter = Vec::new();

    for &node_id in active_nodes {
        let Some(block_id) = find_enclosing_save_node(nodes, node_id) else {
            continue;
        };
        reenter.push(node_id);
        if !blocks.contains(&block_id) {
            blocks.push(block_id);
        }
    }
    if blocks.is_empty() {
        return Discard::default();
    }

    let mut unfinish = Vec::new();
    for &node_id in finished_nodes {
        if blocks
            .iter()
            .any(|&block_id| node_id != block_id && is_in_block(nodes, node_id, block_id))
        {
            unfinish.push(node_id);
        }
    }

    Discard {
        blocks,
        reenter,
        unfinish,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn node(id: i32, parent_id: i32, data: Value) -> NodeEntry {
        NodeEntry {
            id,
            parent_id,
            node_type: String::new(),
            count: 0,
            reward_id: 0,
            data,
        }
    }

    fn sample_tree() -> Vec<NodeEntry> {
        let block = json!({
            "SaveConfig": {
                "DisableRollbackWhileReconnect": true,
                "PosRollbackCfg": {
                    "RangeEntityId": 12000000,
                    "PositionEntityId": 12000001
                }
            }
        });
        vec![
            node(1, 0, json!({})),
            node(111, 1, block),
            node(99, 111, json!({})),
            node(147, 111, json!({})),
            node(148, 147, json!({})),
            node(136, 1, json!({})),
            node(8, 136, json!({})),
            node(188, 136, json!({})),
            node(189, 188, json!({})),
            node(190, 189, json!({})),
        ]
    }

    #[test]
    fn resolves_innermost_save_node_and_ignores_outside_nodes() {
        let nodes = sample_tree();
        assert_eq!(find_enclosing_save_node(&nodes, 148), Some(111));
        assert_eq!(find_enclosing_save_node(&nodes, 111), Some(111));
        assert_eq!(find_enclosing_save_node(&nodes, 190), None);
        assert_eq!(find_enclosing_save_node(&nodes, 1), None);
    }

    #[test]
    fn discards_progress_inside_save_block_and_keeps_outside() {
        let nodes = sample_tree();
        let discard = plan_login_discard(&nodes, &[99], &[136, 8, 148]);
        assert_eq!(discard.blocks, vec![111]);
        assert_eq!(discard.reenter, vec![99]);
        assert_eq!(discard.unfinish, vec![148]);

        let outside = plan_login_discard(&nodes, &[8, 190], &[111, 99, 148]);
        assert!(outside.is_empty());
    }

    #[test]
    fn reads_pos_rollback_cfg() {
        let nodes = sample_tree();
        assert_eq!(
            pos_rollback_of(&nodes, 111),
            Some(PosRollback {
                range_entity_id: 12000000,
                position_entity_id: 12000001,
            })
        );
        assert_eq!(pos_rollback_of(&nodes, 136), None);
    }
}
