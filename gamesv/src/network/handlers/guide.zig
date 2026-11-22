const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const FileSystem = @import("common").FileSystem;
const Connection = @import("../Connection.zig");
const mem = @import("../../mem.zig");
const Assets = @import("../../data/Assets.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const GuideInfo = @import("../../fs/GuideInfo.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerCosmeticComponent = @import("../../logic/component/player/PlayerCosmeticComponent.zig");
const PlayerGuideComponent = @import("../../logic/component/player/PlayerGuideComponent.zig");
const QuestEngine = @import("../../logic/helpers/quest_engine.zig");
const CalabashHelper = @import("../../logic/helpers/calabash.zig");
const EnergyHelper = @import("../../logic/helpers/energy.zig");
const RewardService = @import("../../logic/helpers/reward.zig");

const finished_by_default = [_]i32{
    10116, 10193, 10194,
    10120, 10175, 10270,
    10285,
};

const default_tutorial_reward_astrite: i32 = 1;
const tutorial_drop_package_id: i32 = 100001;

fn saveGuideInfo(arena: std.mem.Allocator, fs: *FileSystem, guide_comp: *const PlayerGuideComponent) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/{s}", .{ guide_comp.player_id, GuideInfo.data_path });
    try comp_util.saveStruct(fs, guide_comp.info, path, arena);
}

fn hasEntryId(entries: []const pb.IllustratedEntry, id: i32) bool {
    for (entries) |entry| {
        if (entry.Id == id) return true;
    }
    return false;
}

pub fn onGuideInfoRequest(
    txn: *Transaction(pb.GuideInfoRequest),
    guide_comp: *PlayerGuideComponent,
    assets: *const Assets,
    alloc: mem.Alloc,
) !void {
    var finished: std.ArrayList(i32) = .empty;
    if (Assets.DataTables.Config.autocomplete_guides) {
        for (assets.tables.guide_group.items) |group| {
            try finished.append(alloc.arena, group.Id);
        }
    } else {
        for (finished_by_default) |group_id| {
            if (!guide_comp.info.hasFinished(group_id)) try finished.append(alloc.arena, group_id);
        }
        try finished.appendSlice(alloc.arena, guide_comp.info.finished_groups);
    }
    txn.respond(.{ .GuideGroupFinishList = finished });
}

pub fn onGuideTriggerRequest(
    txn: *Transaction(pb.GuideTriggerRequest),
    qctx: QuestEngine.QuestContext,
) !void {
    if (txn.message.GroupId > 0) {
        try QuestEngine.onGuideFinish(qctx, txn.message.GroupId);
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onGuideFinishRequest(
    txn: *Transaction(pb.GuideFinishRequest),
    guide_comp: *PlayerGuideComponent,
    qctx: QuestEngine.QuestContext,
    fs: *FileSystem,
    alloc: mem.Alloc,
) !void {
    try guide_comp.info.addFinished(alloc.gpa, txn.message.GroupId);
    try saveGuideInfo(alloc.arena, fs, guide_comp);

    try QuestEngine.onGuideFinish(qctx, txn.message.GroupId);

    txn.respond(.{
        .ErrorCode = .Success,
        .ErrorParams = .empty,
    });
}

pub fn onIllustratedInfoRequest(
    txn: *Transaction(pb.IllustratedInfoRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
    cosmetic_comp: *const PlayerCosmeticComponent,
    echo_comp: *const PlayerEchoComponent,
    weapon_comp: *const PlayerWeaponComponent,
) !void {
    var classes: std.ArrayList(pb.IllustratedClass) = .empty;

    for (txn.message.TypeList.items) |illustrated_type| {
        const type_int = @intFromEnum(illustrated_type);
        var entries: std.ArrayList(pb.IllustratedEntry) = .empty;

        if (illustrated_type == .VocalCorpse) {
            for (assets.tables.calabash_develop_reward.items) |reward| {
                if (!reward.IsShow) continue;
                const max_q = CalabashHelper.discoveredMaxQuality(assets, echo_comp, reward.MonsterId);
                const saved = cosmetic_comp.info.findIllustrated(type_int, reward.MonsterId);
                if (max_q <= 0 and saved == null) continue;

                var unlocked_conds: i32 = 0;
                for (reward.DevelopCondition, 0..) |cond_id, idx| {
                    if (max_q >= CalabashHelper.requiredQualityForCondition(assets, cond_id, idx)) {
                        unlocked_conds += 1;
                    }
                }

                const min_num: i32 = if (max_q > 0) @max(@as(i32, 1), unlocked_conds) else 1;
                try entries.append(alloc.arena, .{
                    .Id = reward.MonsterId,
                    .CreateTime = if (saved) |rec| rec.create_time else 1,
                    .Num = if (saved) |rec| @max(min_num, rec.num) else min_num,
                    .IsRead = if (saved) |rec| rec.is_read else false,
                });
            }
        } else if (illustrated_type == .Weapon) {
            var it = weapon_comp.weapon_map.iterator();
            while (it.next()) |kv| {
                const weapon_id = kv.value_ptr.id;
                if (hasEntryId(entries.items, weapon_id)) continue;

                var copies: i32 = 0;
                var cnt_it = weapon_comp.weapon_map.iterator();
                while (cnt_it.next()) |cnt_kv| {
                    if (cnt_kv.value_ptr.id == weapon_id) copies += 1;
                }
                const saved = cosmetic_comp.info.findIllustrated(type_int, weapon_id);
                try entries.append(alloc.arena, .{
                    .Id = weapon_id,
                    .CreateTime = if (saved) |rec| rec.create_time else 1,
                    .Num = if (saved) |rec| @max(copies, rec.num) else copies,
                    .IsRead = if (saved) |rec| rec.is_read else true,
                });
            }
        }

        for (cosmetic_comp.info.illustrated_entries) |rec| {
            if (rec.entry_type != type_int) continue;
            if (hasEntryId(entries.items, rec.id)) continue;
            try entries.append(alloc.arena, .{
                .Id = rec.id,
                .CreateTime = rec.create_time,
                .Num = rec.num,
                .IsRead = rec.is_read,
            });
        }

        try classes.append(alloc.arena, .{
            .Type = illustrated_type,
            .IllustratedEntryList = entries,
        });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .IllustratedClassList = classes,
    });
}

pub fn onIllustratedRedDotRequest(
    txn: *Transaction(pb.IllustratedRedDotRequest),
    alloc: mem.Alloc,
    cosmetic_comp: *const PlayerCosmeticComponent,
) !void {
    var red_dots: std.ArrayList(pb.IllustratedType) = .empty;
    for (cosmetic_comp.info.illustrated_entries) |rec| {
        if (rec.is_read) continue;
        const itype = std.enums.fromInt(pb.IllustratedType, rec.entry_type) orelse continue;
        if (std.mem.indexOfScalar(pb.IllustratedType, red_dots.items, itype) == null) {
            try red_dots.append(alloc.arena, itype);
        }
    }
    txn.respond(.{ .RedDotList = red_dots });
}

pub fn onIllustratedReadRequest(
    txn: *Transaction(pb.IllustratedReadRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    cosmetic_comp: *PlayerCosmeticComponent,
) !void {
    if (txn.message.Type) |entry_type| {
        if (cosmetic_comp.info.markIllustratedRead(@intFromEnum(entry_type), txn.message.Id)) {
            try PlayerCosmeticComponent.save(alloc.gpa, fs, cosmetic_comp.player_id, cosmetic_comp.info);
        }
    }
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onRoleIllustratedInfoRequest(
    txn: *Transaction(pb.RoleIllustratedInfoRequest),
    alloc: mem.Alloc,
    role_comp: *const PlayerRoleComponent,
    weapon_comp: *const PlayerWeaponComponent,
) !void {
    var roles: std.ArrayList(pb.RoleHandbookInfo) = .empty;
    var role_it = role_comp.role_map.iterator();
    while (role_it.next()) |kv| {
        const role_id = kv.key_ptr.*;
        if (role_comp.isTrialRole(role_id)) continue;
        if (role_id < 1000 or role_id >= 2000) continue;
        try roles.append(alloc.arena, .{
            .RoleId = role_id,
            .UnlockTime = if (kv.value_ptr.create_time > 0) @intCast(kv.value_ptr.create_time) else 1,
        });
    }

    var weapons: std.ArrayList(pb.WeaponHandbookInfo) = .empty;
    var weapon_it = weapon_comp.weapon_map.iterator();
    while (weapon_it.next()) |kv| {
        const weapon_id = kv.value_ptr.id;
        var seen = false;
        for (weapons.items) |existing| {
            if (existing.WeaponId == weapon_id) {
                seen = true;
                break;
            }
        }
        if (seen) continue;
        try weapons.append(alloc.arena, .{
            .WeaponId = weapon_id,
            .UnlockTime = 1,
        });
    }

    txn.respond(.{
        .Roles = roles,
        .Weapons = weapons,
    });
}

pub fn onTutorialInfoRequest(
    txn: *Transaction(pb.TutorialInfoRequest),
    alloc: mem.Alloc,
    guide_comp: *const PlayerGuideComponent,
) !void {
    var list: std.ArrayList(pb.TutorialInfo) = .empty;
    for (guide_comp.info.unlocked_tutorials) |rec| {
        try list.append(alloc.arena, .{
            .Id = rec.id,
            .CreateTime = rec.create_time,
            .GetAward = rec.get_award,
        });
    }
    txn.respond(.{ .UnlockList = list });
}

pub fn onTutorialUnlockRequest(
    txn: *Transaction(pb.TutorialUnlockRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    guide_comp: *PlayerGuideComponent,
    io: std.Io,
) !void {
    const create_time: u32 = @intCast(std.math.clamp(EnergyHelper.nowSeconds(io), 1, std.math.maxInt(u32)));
    const rec = try guide_comp.info.unlockTutorial(alloc.gpa, txn.message.Id, create_time);
    try saveGuideInfo(alloc.arena, fs, guide_comp);

    txn.respond(.{
        .UnLockInfo = .{
            .Id = rec.id,
            .CreateTime = rec.create_time,
            .GetAward = rec.get_award,
        },
        .ErrorCode = .Success,
    });
}

pub fn onTutorialReceiveRequest(
    txn: *Transaction(pb.TutorialReceiveRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    basic_comp: *PlayerBasicComponent,
    inventory_comp: *PlayerInventoryComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    guide_comp: *PlayerGuideComponent,
    io: std.Io,
) !void {
    const tutorial_id = txn.message.Id;
    var item_map: std.ArrayList(pb.MapEntry(i32, i32)) = .empty;

    if (tutorial_id <= 0) {
        txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
        return;
    }

    const create_time: u32 = @intCast(std.math.clamp(EnergyHelper.nowSeconds(io), 1, std.math.maxInt(u32)));
    var rec = guide_comp.info.findTutorialPtr(tutorial_id) orelse blk: {
        _ = try guide_comp.info.unlockTutorial(alloc.gpa, tutorial_id, create_time);
        break :blk guide_comp.info.findTutorialPtr(tutorial_id) orelse {
            txn.respond(.{ .ErrorCode = .Success, .ItemMap = item_map });
            return;
        };
    };

    if (!rec.get_award) {
        rec.get_award = true;
        try saveGuideInfo(alloc.arena, fs, guide_comp);

        const reward_items = [_]RewardService.RewardItem{
            .{ .item_id = 3, .count = default_tutorial_reward_astrite },
        };
        _ = try RewardService.grantItems(
            .{
                .alloc = alloc,
                .io = io,
                .fs = fs,
                .assets = assets,
                .conn = conn,
                .basic_comp = basic_comp,
                .inventory_comp = inventory_comp,
                .weapon_comp = weapon_comp,
                .echo_comp = echo_comp,
            },
            tutorial_drop_package_id,
            false,
            &reward_items,
            1,
        );
        try item_map.append(alloc.arena, .{ .key = 3, .value = default_tutorial_reward_astrite });
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .ItemMap = item_map,
    });
}
