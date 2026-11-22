const CosmeticInfo = @This();
const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../data/Assets.zig");

const Allocator = std.mem.Allocator;

pub const data_path = "cosmetic_info";

pub const IllustratedRecord = struct {
    entry_type: i32,
    id: i32,
    create_time: u32 = 1,
    num: i32 = 1,
    is_read: bool = false,
};

role_skins: []i32 = &.{},
phantom_skins: []i32 = &.{},
fly_skins: []i32 = &.{},
weapon_skins: []i32 = &.{},
ornaments: []i32 = &.{},
viewed_ornaments: []i32 = &.{},
illustrated_entries: []IllustratedRecord = &.{},

pub fn intList(items: []const i32, arena: Allocator) !std.ArrayList(i32) {
    var list: std.ArrayList(i32) = .empty;
    try list.ensureTotalCapacity(arena, items.len);
    for (items) |id| {
        if (id == 0) continue;
        list.appendAssumeCapacity(id);
    }
    return list;
}

pub fn has(items: []const i32, id: i32) bool {
    if (id == 0) return true;
    for (items) |item| {
        if (item == id) return true;
    }
    return false;
}

pub fn isEmpty(info: CosmeticInfo) bool {
    return info.role_skins.len == 0 and
        info.phantom_skins.len == 0 and
        info.fly_skins.len == 0 and
        info.weapon_skins.len == 0 and
        info.ornaments.len == 0 and
        info.viewed_ornaments.len == 0;
}

pub fn addDefaults(cosmetics: *CosmeticInfo, gpa: Allocator, assets: *const Assets) !void {
    var role_skins: std.ArrayListUnmanaged(i32) = .empty;
    var phantom_skins: std.ArrayListUnmanaged(i32) = .empty;
    var fly_skins: std.ArrayListUnmanaged(i32) = .empty;
    var weapon_skins: std.ArrayListUnmanaged(i32) = .empty;
    var ornaments: std.ArrayListUnmanaged(i32) = .empty;
    var viewed_ornaments: std.ArrayListUnmanaged(i32) = .empty;

    for (assets.tables.fly_skin_config.items) |info| {
        try fly_skins.append(gpa, info.Id);
        try phantom_skins.append(gpa, info.Id);
    }

    for (assets.tables.role_skin.items) |info| {
        try role_skins.append(gpa, info.Id);
        try phantom_skins.append(gpa, info.Id);
    }

    for (assets.tables.weapon_skin.items) |info| {
        if (info.HideInSkinView) continue;
        try weapon_skins.append(gpa, info.Id);
        try phantom_skins.append(gpa, info.Id);
    }

    for (assets.tables.ornament.items) |info| {
        if (info.HideInUi) continue;
        try ornaments.append(gpa, info.Id);
        try phantom_skins.append(gpa, info.Id);
        try viewed_ornaments.append(gpa, info.Id);
        for (info.ItemAccess) |id| {
            try phantom_skins.append(gpa, id);
            try viewed_ornaments.append(gpa, id);
        }
    }

    for (assets.tables.phantom_item.items) |info| {
        if (info.ParentMonsterId != 0) {
            try phantom_skins.append(gpa, info.ItemId);
        }
    }

    cosmetics.role_skins = try role_skins.toOwnedSlice(gpa);
    cosmetics.phantom_skins = try phantom_skins.toOwnedSlice(gpa);
    cosmetics.fly_skins = try fly_skins.toOwnedSlice(gpa);
    cosmetics.weapon_skins = try weapon_skins.toOwnedSlice(gpa);
    cosmetics.ornaments = try ornaments.toOwnedSlice(gpa);
    cosmetics.viewed_ornaments = try viewed_ornaments.toOwnedSlice(gpa);
}

pub fn ensureDefaultPhantomSkins(info: *CosmeticInfo, gpa: Allocator, assets: *const Assets) !bool {
    var changed = false;
    var phantom_skins: std.ArrayListUnmanaged(i32) = .empty;
    errdefer phantom_skins.deinit(gpa);

    try phantom_skins.appendSlice(gpa, info.phantom_skins);
    for (assets.tables.phantom_item.items) |item| {
        if (item.ParentMonsterId == 0 or has(phantom_skins.items, item.ItemId)) continue;
        try phantom_skins.append(gpa, item.ItemId);
        changed = true;
    }

    if (!changed) {
        phantom_skins.deinit(gpa);
        return false;
    }

    if (info.phantom_skins.len != 0) gpa.free(info.phantom_skins);
    info.phantom_skins = try phantom_skins.toOwnedSlice(gpa);
    return true;
}

pub fn findIllustrated(info: CosmeticInfo, entry_type: i32, id: i32) ?IllustratedRecord {
    for (info.illustrated_entries) |rec| {
        if (rec.entry_type == entry_type and rec.id == id) return rec;
    }
    return null;
}

pub fn markIllustratedRead(info: *CosmeticInfo, entry_type: i32, id: i32) bool {
    var changed = false;
    for (info.illustrated_entries) |*rec| {
        if (rec.entry_type == entry_type and (id == 0 or rec.id == id) and !rec.is_read) {
            rec.is_read = true;
            changed = true;
        }
    }
    return changed;
}

pub fn unlockIllustrated(info: *CosmeticInfo, gpa: Allocator, entry_type: i32, id: i32, create_time: u32) !IllustratedRecord {
    for (info.illustrated_entries) |*rec| {
        if (rec.entry_type == entry_type and rec.id == id) {
            rec.num += 1;
            return rec.*;
        }
    }

    const record: IllustratedRecord = .{
        .entry_type = entry_type,
        .id = id,
        .create_time = if (create_time > 0) create_time else 1,
        .num = 1,
        .is_read = false,
    };
    const next = try gpa.alloc(IllustratedRecord, info.illustrated_entries.len + 1);
    @memcpy(next[0..info.illustrated_entries.len], info.illustrated_entries);
    next[info.illustrated_entries.len] = record;
    if (info.illustrated_entries.len != 0) gpa.free(info.illustrated_entries);
    info.illustrated_entries = next;
    return record;
}

pub fn deinit(info: CosmeticInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}

test "markIllustratedRead marks single or category entries read without losing unread others" {
    var info: CosmeticInfo = .{};
    defer if (info.illustrated_entries.len != 0) std.testing.allocator.free(info.illustrated_entries);

    _ = try info.unlockIllustrated(std.testing.allocator, 0, 1001, 10);
    _ = try info.unlockIllustrated(std.testing.allocator, 0, 1002, 20);
    _ = try info.unlockIllustrated(std.testing.allocator, 3, 2001, 30);

    try std.testing.expect(!info.findIllustrated(0, 1001).?.is_read);
    try std.testing.expect(info.markIllustratedRead(0, 1001));
    try std.testing.expect(info.findIllustrated(0, 1001).?.is_read);
    try std.testing.expect(!info.findIllustrated(0, 1002).?.is_read);
    try std.testing.expect(!info.markIllustratedRead(0, 1001));

    try std.testing.expect(info.markIllustratedRead(0, 0));
    try std.testing.expect(info.findIllustrated(0, 1002).?.is_read);
    try std.testing.expect(!info.findIllustrated(3, 2001).?.is_read);
}
