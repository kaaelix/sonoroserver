const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../../handlers.zig").Transaction;
const mem = @import("../../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../../data/Assets.zig");
const Scene = @import("../../../logic/Scene.zig");
const PlayerRoleComponent = @import("../../../logic/component/player/PlayerRoleComponent.zig");
const PlayerBasicComponent = @import("../../../logic/component/player/PlayerBasicComponent.zig");
const PlayerEchoComponent = @import("../../../logic/component/player/PlayerEchoComponent.zig");
const PlayerWeaponComponent = @import("../../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerProgressComponent = @import("../../../logic/component/player/PlayerProgressComponent.zig");
const EchoInfo = @import("../../../fs/EchoInfo.zig");
const special_item_incr = @import("../../../fs/special_item_incr.zig");
const Entity = Scene.Entity;
const shared = @import("shared.zig");
const BuffTimerScheduler = @import("../../../logic/schedulers/BuffTimerScheduler.zig");

const visionEquipGroupList = shared.visionEquipGroupList;
const changedEquipInfoList = shared.changedEquipInfoList;
const pushRolePropUpdate = shared.pushRolePropUpdate;
const refreshRoleEntities = shared.refreshRoleEntities;

pub fn onVisionEquipGroupInfoRequest(
    txn: *Transaction(pb.VisionEquipGroupInfoRequest),
    alloc: mem.Alloc,
    echo_comp: *PlayerEchoComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .VisionEquipList = try visionEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onAddVisionEquipGroupRequest(
    txn: *Transaction(pb.AddVisionEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (role_comp.role_map.get(txn.message.RoleId) == null) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    const equip = echo_comp.roleEquip(txn.message.RoleId);
    if (echo_comp.preset_info.groups.len >= 20) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }

    const incr_id = try special_item_incr.next(alloc.gpa, fs, echo_comp.player_id);
    try appendVisionGroup(alloc.gpa, echo_comp, incr_id, txn.message.Name, equip.slots);
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);

    txn.respond(.{
        .ErrorCode = .Success,
        .VisionEquipList = try visionEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onApplyVisionGroupRequest(
    txn: *Transaction(pb.ApplyVisionGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    query: Scene.Query(&.{
        Entity,
        *Entity.ConfigComponent,
        *Entity.VisionSkillComponent,
        *Entity.ConcomitantComponent,
        *Entity.AttributeComponent,
        *Entity.FightBuffComponent,
    }),
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    if (!shared.roleExistsAndEditable(role_comp, txn.message.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrRoleNotActive });
        return;
    }
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const group = echo_comp.preset_info.groups[@intCast(txn.message.Index)];

    var seen: std.hash_map.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.deinit(alloc.gpa);
    for (group.slots) |inc_id| {
        if (inc_id == 0) continue;
        if (echo_comp.echo_map.get(inc_id) == null or seen.contains(inc_id)) {
            txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
            return;
        }
        try seen.put(alloc.gpa, inc_id, {});
    }

    if (shared.validateEquipSlots(assets, basic_comp, echo_comp, group.slots, .ErrPhantomEquipSourceCost)) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try changed_roles.put(alloc.gpa, txn.message.RoleId, {});

    var iterator = echo_comp.echo_map.iterator();
    while (iterator.next()) |kv| {
        const role_id = kv.value_ptr.role_id orelse continue;
        const equipped_pos = kv.value_ptr.equipped_pos;
        if (equipped_pos < 0 or equipped_pos >= 5) continue;
        if (role_id == txn.message.RoleId or seen.contains(kv.key_ptr.*)) {
            try changed_roles.put(alloc.gpa, role_id, {});
            kv.value_ptr.role_id = null;
            kv.value_ptr.equipped_pos = -1;
        }
    }
    var split_infos: std.ArrayList(pb.PhantomStackSplitInfo) = .empty;
    for (group.slots, 0..) |inc_id, index| {
        if (inc_id == 0) continue;
        if (try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, inc_id)) |split| {
            try split_infos.append(alloc.arena, split);
        }
        const item = echo_comp.echo_map.getPtr(inc_id) orelse continue;
        item.role_id = txn.message.RoleId;
        item.equipped_pos = @intCast(index);
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
    try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);

    txn.respond(.{
        .ErrorCode = .Success,
        .EquipInfoList = try changedEquipInfoList(echo_comp, changed_roles, alloc.arena),
        .StackSplitInfos = split_infos,
    });
}

pub fn onPutVisionGroupToTopRequest(
    txn: *Transaction(pb.PutVisionGroupToTopRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const index: usize = @intCast(txn.message.Index);
    if (index != 0) {
        const group = echo_comp.preset_info.groups[index];
        std.mem.copyBackwards(EchoInfo.VisionGroup, echo_comp.preset_info.groups[1 .. index + 1], echo_comp.preset_info.groups[0..index]);
        echo_comp.preset_info.groups[0] = group;
        try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .VisionEquipList = try visionEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onChangeVisionGroupNameRequest(
    txn: *Transaction(pb.ChangeVisionGroupNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const index: usize = @intCast(txn.message.Index);
    if (echo_comp.preset_info.groups[index].name.len != 0) {
        alloc.gpa.free(echo_comp.preset_info.groups[index].name);
    }
    echo_comp.preset_info.groups[index].name = try alloc.gpa.dupe(u8, txn.message.Name);
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);

    txn.respond(.{
        .ErrorCode = .Success,
        .VisionEquipList = try visionEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onDeleteVisionEquipGroupRequest(
    txn: *Transaction(pb.DeleteVisionEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    try removeVisionGroup(alloc.gpa, echo_comp, @intCast(txn.message.Index));
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);

    txn.respond(.{
        .ErrorCode = .Success,
        .VisionEquipList = try visionEquipGroupList(echo_comp, alloc.arena),
    });
}

fn appendVisionGroup(
    gpa: std.mem.Allocator,
    echo_comp: *PlayerEchoComponent,
    incr_id: i32,
    name: []const u8,
    slots: [5]i32,
) !void {
    const new_groups = try gpa.alloc(EchoInfo.VisionGroup, echo_comp.preset_info.groups.len + 1);
    @memcpy(new_groups[0..echo_comp.preset_info.groups.len], echo_comp.preset_info.groups);
    new_groups[echo_comp.preset_info.groups.len] = .{
        .incr_id = incr_id,
        .name = try gpa.dupe(u8, name),
        .slots = slots,
    };

    if (echo_comp.preset_info.groups.len != 0) gpa.free(echo_comp.preset_info.groups);
    echo_comp.preset_info.groups = new_groups;
}

fn removeVisionGroup(
    gpa: std.mem.Allocator,
    echo_comp: *PlayerEchoComponent,
    index: usize,
) !void {
    if (echo_comp.preset_info.groups[index].name.len != 0) {
        gpa.free(echo_comp.preset_info.groups[index].name);
    }

    const new_groups = try gpa.alloc(EchoInfo.VisionGroup, echo_comp.preset_info.groups.len - 1);
    @memcpy(new_groups[0..index], echo_comp.preset_info.groups[0..index]);
    @memcpy(new_groups[index..], echo_comp.preset_info.groups[index + 1 ..]);

    if (echo_comp.preset_info.groups.len != 0) gpa.free(echo_comp.preset_info.groups);
    echo_comp.preset_info.groups = new_groups;
}

fn phantomEquipGroupList(echo_comp: *PlayerEchoComponent, arena: std.mem.Allocator) !std.ArrayList(pb.PhantomEquipGroupInfo) {
    var list: std.ArrayList(pb.PhantomEquipGroupInfo) = .empty;
    try list.ensureTotalCapacity(arena, echo_comp.preset_info.groups.len);
    for (echo_comp.preset_info.groups) |group| {
        var inc_ids: std.ArrayList(i32) = .empty;
        for (group.slots) |slot| try inc_ids.append(arena, slot);
        list.appendAssumeCapacity(.{
            .IncId = inc_ids,
            .Name = group.name,
        });
    }
    return list;
}

pub fn onPhantomEquipGroupInfosRequest(
    txn: *Transaction(pb.PhantomEquipGroupInfosRequest),
    alloc: mem.Alloc,
    echo_comp: *PlayerEchoComponent,
) !void {
    txn.respond(.{
        .ErrorCode = .Success,
        .IsOldData = false,
        .PhantomEquipGroupInfos = try phantomEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onAddPhantomEquipGroupRequest(
    txn: *Transaction(pb.AddPhantomEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (role_comp.role_map.get(txn.message.RoleId) == null) {
        txn.respond(.{ .ErrorCode = .RequestParamError });
        return;
    }
    const equip = echo_comp.roleEquip(txn.message.RoleId);
    if (echo_comp.preset_info.groups.len >= 20) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const incr_id = try special_item_incr.next(alloc.gpa, fs, echo_comp.player_id);
    try appendVisionGroup(alloc.gpa, echo_comp, incr_id, txn.message.Name, equip.slots);
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);
    txn.respond(.{
        .ErrorCode = .Success,
        .IsOldData = false,
        .PhantomEquipGroupInfos = try phantomEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onTopPhantomEquipGroupRequest(
    txn: *Transaction(pb.TopPhantomEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const index: usize = @intCast(txn.message.Index);
    if (index != 0) {
        const group = echo_comp.preset_info.groups[index];
        std.mem.copyBackwards(EchoInfo.VisionGroup, echo_comp.preset_info.groups[1 .. index + 1], echo_comp.preset_info.groups[0..index]);
        echo_comp.preset_info.groups[0] = group;
        try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomEquipGroupInfos = try phantomEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onChangePhantomEquipGroupNameRequest(
    txn: *Transaction(pb.ChangePhantomEquipGroupNameRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const index: usize = @intCast(txn.message.Index);
    if (echo_comp.preset_info.groups[index].name.len != 0) {
        alloc.gpa.free(echo_comp.preset_info.groups[index].name);
    }
    echo_comp.preset_info.groups[index].name = try alloc.gpa.dupe(u8, txn.message.Name);
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomEquipGroupInfos = try phantomEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onDeletePhantomEquipGroupRequest(
    txn: *Transaction(pb.DeletePhantomEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
) !void {
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    try removeVisionGroup(alloc.gpa, echo_comp, @intCast(txn.message.Index));
    try PlayerEchoComponent.savePresets(alloc.gpa, fs, echo_comp.player_id, echo_comp.preset_info);
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomEquipGroupInfos = try phantomEquipGroupList(echo_comp, alloc.arena),
    });
}

pub fn onUsePhantomEquipGroupRequest(
    txn: *Transaction(pb.UsePhantomEquipGroupRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    scene: *Scene,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
    echo_comp: *PlayerEchoComponent,
    weapon_comp: *PlayerWeaponComponent,
    query: Scene.Query(&.{
        Entity,
        *Entity.ConfigComponent,
        *Entity.VisionSkillComponent,
        *Entity.ConcomitantComponent,
        *Entity.AttributeComponent,
        *Entity.FightBuffComponent,
    }),
    buff_timers: *BuffTimerScheduler,
    io: std.Io,
) !void {
    const now_ms = (std.Io.Clock.awake).now(io).toMilliseconds();
    if (!shared.roleExistsAndEditable(role_comp, txn.message.RoleId)) {
        txn.respond(.{ .ErrorCode = .ErrRoleNotActive });
        return;
    }
    if (txn.message.Index < 0 or txn.message.Index >= echo_comp.preset_info.groups.len) {
        txn.respond(.{ .ErrorCode = .ErrVisionSkillSlotNotFound });
        return;
    }
    const group = echo_comp.preset_info.groups[@intCast(txn.message.Index)];

    var seen: std.hash_map.AutoHashMapUnmanaged(i32, void) = .empty;
    defer seen.deinit(alloc.gpa);
    for (group.slots) |inc_id| {
        if (inc_id == 0) continue;
        if (echo_comp.echo_map.get(inc_id) == null or seen.contains(inc_id)) {
            txn.respond(.{ .ErrorCode = .ErrPhantomItemNotExist });
            return;
        }
        try seen.put(alloc.gpa, inc_id, {});
    }

    if (shared.validateEquipSlots(assets, basic_comp, echo_comp, group.slots, .ErrPhantomEquipSourceCost)) |err| {
        txn.respond(.{ .ErrorCode = err });
        return;
    }

    var changed_roles: std.array_hash_map.Auto(i32, void) = .empty;
    defer changed_roles.deinit(alloc.gpa);
    try changed_roles.put(alloc.gpa, txn.message.RoleId, {});

    var iterator = echo_comp.echo_map.iterator();
    while (iterator.next()) |kv| {
        const role_id = kv.value_ptr.role_id orelse continue;
        const equipped_pos = kv.value_ptr.equipped_pos;
        if (equipped_pos < 0 or equipped_pos >= 5) continue;
        if (role_id == txn.message.RoleId or seen.contains(kv.key_ptr.*)) {
            try changed_roles.put(alloc.gpa, role_id, {});
            kv.value_ptr.role_id = null;
            kv.value_ptr.equipped_pos = -1;
        }
    }
    var split_infos: std.ArrayList(pb.PhantomStackSplitInfo) = .empty;
    for (group.slots, 0..) |inc_id, index| {
        if (inc_id == 0) continue;
        if (try shared.splitSingleFromStack(alloc.gpa, alloc.arena, fs, echo_comp, inc_id)) |split| {
            try split_infos.append(alloc.arena, split);
        }
        const item = echo_comp.echo_map.getPtr(inc_id) orelse continue;
        item.role_id = txn.message.RoleId;
        item.equipped_pos = @intCast(index);
    }

    try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    try pushRolePropUpdate(txn, alloc, assets, role_comp, echo_comp, weapon_comp, changed_roles, io);
    try refreshRoleEntities(txn, alloc, fs, assets, scene, role_comp, echo_comp, weapon_comp, query, changed_roles, buff_timers, io, now_ms);

    txn.respond(.{
        .ErrorCode = .Success,
        .EquipInfoList = try changedEquipInfoList(echo_comp, changed_roles, alloc.arena),
        .StackSplitInfos = split_infos,
    });
}

pub fn onPhantomBatchOperRequest(
    txn: *Transaction(pb.PhantomBatchOperRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const new_func: i32 = switch (txn.message.Oper orelse .Reset) {
        .Lock => 1,
        .Disuse => 2,
        .Reset => 0,
    };
    var changed = false;
    for (txn.message.IncrId.items) |incr_id| {
        const item = echo_comp.echo_map.getPtr(incr_id) orelse continue;
        item.func_value = new_func;
        changed = true;
        try txn.conn.push(alloc.gpa, io, pb.ItemFuncValueUpdateNotify{
            .IncrId = incr_id,
            .FuncValue = new_func,
        });
    }
    if (changed) {
        try PlayerEchoComponent.saveAll(alloc.gpa, fs, echo_comp.player_id, echo_comp.echo_map);
    }
    txn.respond(.{ .ErrCode = .Success });
}

pub fn onPhantomSettingRequest(
    txn: *Transaction(pb.PhantomSettingRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    var auto_lock: std.ArrayList(pb.OnePhantomSetting) = .empty;
    var auto_disuse: std.ArrayList(pb.OnePhantomSetting) = .empty;
    var i: i32 = 0;
    while (i < 5) : (i += 1) {
        const lkey = try std.fmt.allocPrint(alloc.arena, "phantom_set:0:{d}", .{i});
        const lval = progress_comp.info.getKeyInt(lkey);
        if (lval != 0) {
            const name_key = try std.fmt.allocPrint(alloc.arena, "phantom_set:0:name:{d}", .{i});
            try auto_lock.append(alloc.arena, .{
                .Index = i,
                .On = lval > 0,
                .Name = progress_comp.info.getKeyStr(name_key),
            });
        }
        const dkey = try std.fmt.allocPrint(alloc.arena, "phantom_set:1:{d}", .{i});
        const dval = progress_comp.info.getKeyInt(dkey);
        if (dval != 0) {
            const name_key = try std.fmt.allocPrint(alloc.arena, "phantom_set:1:name:{d}", .{i});
            try auto_disuse.append(alloc.arena, .{
                .Index = i,
                .On = dval > 0,
                .Name = progress_comp.info.getKeyStr(name_key),
            });
        }
    }
    txn.respond(.{
        .AutoLockSettings = auto_lock,
        .AutoDisuseSettings = auto_disuse,
    });
}

pub fn onPhantomSettingUpdateRequest(
    txn: *Transaction(pb.PhantomSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    if (txn.message.Setting) |setting| {
        const stype = @intFromEnum(txn.message.SettingType orelse .AutoLock);
        const key = try std.fmt.allocPrint(alloc.arena, "phantom_set:{d}:{d}", .{ stype, setting.Index });
        try progress_comp.info.setKeyInt(alloc.gpa, key, if (setting.On) 1 else -1);
        const nkey = try std.fmt.allocPrint(alloc.arena, "phantom_set:{d}:name:{d}", .{ stype, setting.Index });
        try progress_comp.info.setKeyStr(alloc.gpa, nkey, setting.Name);
        try progress_comp.save(fs, alloc.arena);
    }
    txn.respond(.{
        .ErrorCode = .Success,
        .Setting = txn.message.Setting,
    });
}

pub fn buildCostAttrRecommends(arena: std.mem.Allocator) !std.ArrayList(pb.CostPhantomAttrRecommendInfo) {
    var out: std.ArrayList(pb.CostPhantomAttrRecommendInfo) = .empty;
    for ([_]struct { cost: i32, main: i32 }{
        .{ .cost = 4, .main = 20001 },
        .{ .cost = 3, .main = 7 },
        .{ .cost = 1, .main = 1 },
    }) |spec| {
        var mains: std.ArrayList(pb.PhantomAttrRecommendInfo) = .empty;
        try mains.append(arena, .{ .AttrType = spec.main, .AddType = 2, .Usage = 1 });
        var vices: std.ArrayList(pb.PhantomAttrRecommendInfo) = .empty;
        for ([_]i32{ 20001, 20002, 7 }) |va| {
            try vices.append(arena, .{ .AttrType = va, .AddType = 2, .Usage = 1 });
        }
        try out.append(arena, .{
            .Cost = spec.cost,
            .MainAttrRecommendInfos = mains,
            .ViceAttrRecommendInfos = vices,
        });
    }
    return out;
}

pub fn buildFetterRecommends(arena: std.mem.Allocator, assets: *const Assets) !std.ArrayList(pb.PhantomFetterRecommendInfo) {
    var out: std.ArrayList(pb.PhantomFetterRecommendInfo) = .empty;
    var groups: std.ArrayList(pb.PhantomFetterGroupInfo) = .empty;
    var count: usize = 0;
    for (assets.tables.phantom_fetter_group.items) |fg| {
        try groups.append(arena, .{ .PhantomFetterGroupId = fg.Id, .CountNeed = 5 });
        count += 1;
        if (count >= 3) break;
    }
    var combos: std.ArrayList(pb.CostCombinationInfo) = .empty;
    try combos.append(arena, .{ .CostId = 43311, .Usage = 1 });
    try out.append(arena, .{
        .Usage = 1,
        .PhantomFetterGroupInfos = groups,
        .CostCombinationInfos = combos,
    });
    return out;
}

pub fn onPhantomAttrRecommendRequest(
    txn: *Transaction(pb.PhantomAttrRecommendRequest),
    alloc: mem.Alloc,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .CostPhantomAttrRecommendInfos = try buildCostAttrRecommends(alloc.arena),
    });
}

pub fn onPhantomFetterRecommendRequest(
    txn: *Transaction(pb.PhantomFetterRecommendRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    _ = txn.message;
    txn.respond(.{
        .ErrorCode = .Success,
        .PhantomFetterRecommendInfos = try buildFetterRecommends(alloc.arena, assets),
    });
}
