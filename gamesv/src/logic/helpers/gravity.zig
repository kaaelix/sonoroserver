const std = @import("std");
const Assets = @import("../../data/Assets.zig");
const LevelEntityConfig = @import("../../data/tables/LevelEntityConfig.zig");
const BaseInfo = @import("../../data/tables/entity_components/BaseInfo.zig");

pub const GravityResult = struct {
    dir: [3]f32,
    configured: bool,
};

pub fn resolveDirection(direction: BaseInfo.GravityDirection, base_rot: [3]f32) GravityResult {
    if (direction.Direction) |v| {
        return .{ .dir = .{ v.X, v.Y, v.Z }, .configured = true };
    }
    if (direction.WorldAxis) |axis| {
        if (std.mem.eql(u8, axis, "PositiveX")) return .{ .dir = .{ 1, 0, 0 }, .configured = true };
        if (std.mem.eql(u8, axis, "NegativeX")) return .{ .dir = .{ -1, 0, 0 }, .configured = true };
        if (std.mem.eql(u8, axis, "PositiveY")) return .{ .dir = .{ 0, 1, 0 }, .configured = true };
        if (std.mem.eql(u8, axis, "NegativeY")) return .{ .dir = .{ 0, -1, 0 }, .configured = true };
        if (std.mem.eql(u8, axis, "PositiveZ")) return .{ .dir = .{ 0, 0, 1 }, .configured = true };
        if (std.mem.eql(u8, axis, "NegativeZ")) return .{ .dir = .{ 0, 0, -1 }, .configured = true };
    }
    if (direction.Type) |kind| {
        if (std.mem.eql(u8, kind, "SelfRotation")) {
            const to_rad = std.math.pi / 180.0;
            const sr = @sin(base_rot[0] * to_rad);
            const cr = @cos(base_rot[0] * to_rad);
            const sp = @sin(base_rot[1] * to_rad);
            const cp = @cos(base_rot[1] * to_rad);
            const sy = @sin(base_rot[2] * to_rad);
            const cy = @cos(base_rot[2] * to_rad);
            return .{
                .dir = .{
                    cr * sp * cy + sr * sy,
                    cr * sp * sy - cy * sr,
                    -(cr * cp),
                },
                .configured = true,
            };
        }
    }
    return .{ .dir = .{ 0, 0, -1 }, .configured = false };
}

pub fn resolveEntityGravity(entity_cfg: *const LevelEntityConfig, base_rot: [3]f32) GravityResult {
    const base_info = entity_cfg.ComponentsData.BaseInfoComponent orelse return .{ .dir = .{ 0, 0, -1 }, .configured = false };
    const gravity_config = base_info.GravityConfig orelse return .{ .dir = .{ 0, 0, -1 }, .configured = false };
    const direction = gravity_config.GravityDirection orelse return .{ .dir = .{ 0, 0, -1 }, .configured = false };
    return resolveDirection(direction, base_rot);
}

fn findHookLockPoint(components: anytype) ?@import("../../data/tables/entity_components/Extra.zig").HookLockPoint {
    for (components.extra) |entry| {
        if (entry == .HookLockPoint) return entry.HookLockPoint.*;
    }
    return null;
}

/// Resolves the gravity direction for a `HookLockPoint` / `GravityHook` entity.
/// Mirrors `GrapplingHookPointComponent.GetGravityDirect` and falls back to
/// `BaseInfoComponent.GravityConfig` on the level entity or its template.
pub fn resolveHookGravity(
    assets: *const Assets,
    map_id: i32,
    cfg_id: i64,
    base_rot: [3]f32,
) GravityResult {
    const level_cfg = assets.findLevelEntity(map_id, cfg_id) orelse return .{ .dir = .{ 0, 0, -1 }, .configured = false };
    const effective_rot = if (base_rot[0] == 0 and base_rot[1] == 0 and base_rot[2] == 0)
        Assets.AssetIndexes.entityWorldRot(level_cfg)
    else
        base_rot;

    if (findHookLockPoint(level_cfg.ComponentsData)) |hlp| {
        if (hlp.HookInteractConfig) |hic| {
            if (hic.Type) |kind| {
                if (std.mem.eql(u8, kind, "GravityHook")) {
                    if (hic.GravityDirection) |dir| {
                        const res = resolveDirection(dir, effective_rot);
                        if (res.configured) return res;
                    }
                }
            }
        }
        if (hlp.GravityConfig) |gc| {
            if (gc.GravityDirection) |dir| {
                const res = resolveDirection(dir, effective_rot);
                if (res.configured) return res;
            }
        }
    }

    const level_base = resolveEntityGravity(level_cfg, effective_rot);
    if (level_base.configured) return level_base;

    if (assets.findTemplateConfig(level_cfg.BlueprintType)) |tpl_cfg| {
        if (findHookLockPoint(tpl_cfg.ComponentsData)) |hlp| {
            if (hlp.HookInteractConfig) |hic| {
                if (hic.Type) |kind| {
                    if (std.mem.eql(u8, kind, "GravityHook")) {
                        if (hic.GravityDirection) |dir| {
                            const res = resolveDirection(dir, effective_rot);
                            if (res.configured) return res;
                        }
                    }
                }
            }
            if (hlp.GravityConfig) |gc| {
                if (gc.GravityDirection) |dir| {
                    const res = resolveDirection(dir, effective_rot);
                    if (res.configured) return res;
                }
            }
        }
        if (tpl_cfg.ComponentsData.BaseInfoComponent) |bi| {
            if (bi.GravityConfig) |gc| {
                if (gc.GravityDirection) |dir| {
                    const res = resolveDirection(dir, effective_rot);
                    if (res.configured) return res;
                }
            }
        }
    }

    return .{ .dir = .{ 0, 0, -1 }, .configured = false };
}
