use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::asset_indexes;
use sonoro_gamesv_data::tables::entity_components::base_info::GravityDirection;
use sonoro_gamesv_data::tables::entity_components::components::Components;
use sonoro_gamesv_data::tables::entity_components::extra::{ExtraEntry, ExtraKind};
use sonoro_gamesv_data::tables::level_entity_config::LevelEntityConfig;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravityResult {
    pub dir: [f32; 3],
    pub configured: bool,
}

pub fn rotate_local_offset_ue(local: [f32; 3], rot_deg: [f32; 3]) -> [f32; 3] {
    let to_rad = std::f32::consts::PI / 180.0;
    let (sr, cr) = (rot_deg[0] * to_rad).sin_cos();
    let (sp, cp) = (rot_deg[1] * to_rad).sin_cos();
    let (sy, cy) = (rot_deg[2] * to_rad).sin_cos();

    [
        local[0] * (cp * cy) + local[1] * (sr * sp * cy - cr * sy) + local[2] * (-(cr * sp * cy + sr * sy)),
        local[0] * (cp * sy) + local[1] * (sr * sp * sy + cr * cy) + local[2] * (cy * sr - cr * sp * sy),
        local[0] * sp + local[1] * (-sr * cp) + local[2] * (cr * cp),
    ]
}

pub fn resolve_direction(direction: &GravityDirection, base_rot: [f32; 3]) -> GravityResult {
    if let Some(v) = &direction.Direction {
        return GravityResult {
            dir: [v.X, v.Y, v.Z],
            configured: true,
        };
    }
    if let Some(axis) = direction.WorldAxis.as_deref() {
        let dir = match axis {
            "PositiveX" => Some([1.0, 0.0, 0.0]),
            "NegativeX" => Some([-1.0, 0.0, 0.0]),
            "PositiveY" => Some([0.0, 1.0, 0.0]),
            "NegativeY" => Some([0.0, -1.0, 0.0]),
            "PositiveZ" => Some([0.0, 0.0, 1.0]),
            "NegativeZ" => Some([0.0, 0.0, -1.0]),
            _ => None,
        };
        if let Some(dir) = dir {
            return GravityResult {
                dir,
                configured: true,
            };
        }
    }
    if direction.Type.as_deref() == Some("SelfRotation") {
        let to_rad = std::f32::consts::PI / 180.0;
        let (sr, cr) = (base_rot[0] * to_rad).sin_cos();
        let (sp, cp) = (base_rot[1] * to_rad).sin_cos();
        let (sy, cy) = (base_rot[2] * to_rad).sin_cos();
        return GravityResult {
            dir: [cr * sp * cy + sr * sy, cr * sp * sy - cy * sr, -(cr * cp)],
            configured: true,
        };
    }
    GravityResult {
        dir: [0.0, 0.0, -1.0],
        configured: false,
    }
}

pub fn resolve_entity_gravity(entity_cfg: &LevelEntityConfig, base_rot: [f32; 3]) -> GravityResult {
    resolve_components_gravity(&entity_cfg.ComponentsData, base_rot)
}

pub fn resolve_components_gravity(components: &Components, base_rot: [f32; 3]) -> GravityResult {
    let Some(base_info) = &components.BaseInfoComponent else {
        return GravityResult {
            dir: [0.0, 0.0, -1.0],
            configured: false,
        };
    };
    let Some(gravity_config) = &base_info.GravityConfig else {
        return GravityResult {
            dir: [0.0, 0.0, -1.0],
            configured: false,
        };
    };
    let Some(direction) = &gravity_config.GravityDirection else {
        return GravityResult {
            dir: [0.0, 0.0, -1.0],
            configured: false,
        };
    };
    resolve_direction(direction, base_rot)
}

pub fn resolve_hook_gravity(
    assets: &Assets,
    map_id: i32,
    cfg_id: i64,
    base_rot: [f32; 3],
) -> GravityResult {
    let Some(level_cfg) = assets.find_level_entity(map_id, cfg_id) else {
        return GravityResult {
            dir: [0.0, 0.0, -1.0],
            configured: false,
        };
    };
    let effective_rot = if base_rot == [0.0, 0.0, 0.0] {
        asset_indexes::entity_world_rot(&level_cfg)
    } else {
        base_rot
    };

    if let Some(ExtraEntry::HookLockPoint(hlp)) =
        level_cfg.ComponentsData.find_extra_entry(ExtraKind::HookLockPoint)
    {
        if let Some(hic) = &hlp.HookInteractConfig
            && hic.Type.as_deref() == Some("GravityHook")
            && let Some(dir) = &hic.GravityDirection
        {
            let res = resolve_direction(dir, effective_rot);
            if res.configured {
                return res;
            }
        }
        if let Some(gc) = &hlp.GravityConfig
            && let Some(dir) = &gc.GravityDirection
        {
            let res = resolve_direction(dir, effective_rot);
            if res.configured {
                return res;
            }
        }
    }

    let level_base = resolve_entity_gravity(&level_cfg, effective_rot);
    if level_base.configured {
        return level_base;
    }

    if let Some(tpl_cfg) = assets.find_template_config(&level_cfg.BlueprintType) {
        if let Some(ExtraEntry::HookLockPoint(hlp)) =
            tpl_cfg.ComponentsData.find_extra_entry(ExtraKind::HookLockPoint)
        {
            if let Some(hic) = &hlp.HookInteractConfig
                && hic.Type.as_deref() == Some("GravityHook")
                && let Some(dir) = &hic.GravityDirection
            {
                let res = resolve_direction(dir, effective_rot);
                if res.configured {
                    return res;
                }
            }
            if let Some(gc) = &hlp.GravityConfig
                && let Some(dir) = &gc.GravityDirection
            {
                let res = resolve_direction(dir, effective_rot);
                if res.configured {
                    return res;
                }
            }
        }
        if let Some(bi) = &tpl_cfg.ComponentsData.BaseInfoComponent
            && let Some(gc) = &bi.GravityConfig
            && let Some(dir) = &gc.GravityDirection
        {
            let res = resolve_direction(dir, effective_rot);
            if res.configured {
                return res;
            }
        }
    }

    GravityResult {
        dir: [0.0, 0.0, -1.0],
        configured: false,
    }
}
