const PlayerRoleComponent = @This();
const std = @import("std");
const common = @import("common");
const comp_util = @import("../comp_util.zig");
const file_util = @import("../../../fs/file_util.zig");

const Assets = @import("../../../data/Assets.zig");
const RoleInfo = @import("../../../fs/RoleInfo.zig");
const RoleEquipment = @import("../../helpers/role_equipment.zig");
const PlayerWeaponComponent = @import("PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("PlayerEchoComponent.zig");
const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;

player_id: i32,
role_map: std.array_hash_map.Auto(i32, RoleInfo),
equipment_map: std.array_hash_map.Auto(i32, RoleEquipment),
trial_roles: std.AutoHashMapUnmanaged(i32, void) = .empty,

pub fn init(gpa: Allocator, fs: *FileSystem, assets: *const Assets, player_id: i32) !PlayerRoleComponent {
    var arena = std.heap.ArenaAllocator.init(gpa);
    defer arena.deinit();

    return .{
        .player_id = player_id,
        .role_map = try comp_util.loadItems(RoleInfo, gpa, fs, assets, player_id, .by_data_id),
        .equipment_map = .empty,
    };
}

pub fn deinit(comp: *PlayerRoleComponent, gpa: Allocator) void {
    comp.trial_roles.deinit(gpa);
    for (comp.equipment_map.values()) |*equipment| equipment.deinit(gpa);
    comp.equipment_map.deinit(gpa);
    comp_util.freeMap(gpa, &comp.role_map);
}

pub fn isTrialRole(comp: *const PlayerRoleComponent, role_id: i32) bool {
    return comp.trial_roles.contains(role_id);
}

pub fn removeEquipmentCache(comp: *PlayerRoleComponent, gpa: Allocator, role_id: i32) void {
    if (comp.equipment_map.fetchSwapRemove(role_id)) |removed| {
        var equipment = removed.value;
        equipment.deinit(gpa);
    }
}

pub fn rebuildEquipment(
    comp: *PlayerRoleComponent,
    gpa: Allocator,
    assets: *const Assets,
    role_id: i32,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
) !*RoleEquipment {
    const role_info = comp.role_map.getPtr(role_id) orelse return error.RoleNotFound;
    var equipment = try RoleEquipment.build(gpa, assets, role_id, role_info, weapon_comp, echo_comp);
    errdefer equipment.deinit(gpa);

    const gop = try comp.equipment_map.getOrPut(gpa, role_id);
    if (gop.found_existing) gop.value_ptr.deinit(gpa);
    gop.value_ptr.* = equipment;
    return gop.value_ptr;
}
