const std = @import("std");
const pb = @import("proto").pb;
const Transaction = @import("../handlers.zig").Transaction;
const mem = @import("../../mem.zig");
const FileSystem = @import("common").FileSystem;
const Assets = @import("../../data/Assets.zig");
const comp_util = @import("../../logic/component/comp_util.zig");
const GachaInfo = @import("../../fs/GachaInfo.zig");
const GachaViewInfo = @import("../../data/tables/GachaViewInfo.zig");
const PlayerGachaComponent = @import("../../logic/component/player/PlayerGachaComponent.zig");
const PlayerInventoryComponent = @import("../../logic/component/player/PlayerInventoryComponent.zig");
const PlayerRoleComponent = @import("../../logic/component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../../logic/component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../../logic/component/player/PlayerEchoComponent.zig");
const PlayerBasicComponent = @import("../../logic/component/player/PlayerBasicComponent.zig");
const InventoryInfo = @import("../../fs/InventoryInfo.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const special_item_incr = @import("../../fs/special_item_incr.zig");
const RoleHelper = @import("../../logic/helpers/role.zig");
const RoleStats = @import("../../logic/helpers/role_stats.zig");
const Connection = @import("../Connection.zig");
const inventory_helper = @import("../../logic/helpers/inventory.zig");
const ConsumeHelper = @import("../../logic/helpers/consume.zig");

const Io = std.Io;
const gacha_rates = Assets.DataTables.Config.gacha_rates;

const item_id_afterglow_coral: i32 = 50003;
const item_id_oscillated_coral: i32 = 50004;

fn saveInventoryInfo(
    alloc: mem.Alloc,
    fs: *FileSystem,
    inventory_comp: *PlayerInventoryComponent,
) !void {
    const path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}", .{ inventory_comp.player_id, InventoryInfo.data_path });
    try comp_util.saveStruct(fs, inventory_comp.info, path, alloc.arena);
}

fn saveWeaponInfo(
    alloc: mem.Alloc,
    fs: *FileSystem,
    weapon_comp: *PlayerWeaponComponent,
    incr_id: i32,
    weapon: WeaponItem,
) !void {
    const path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}/{}", .{ weapon_comp.player_id, WeaponItem.data_dir, incr_id });
    try comp_util.saveStruct(fs, weapon, path, alloc.arena);
}

fn roleWavebandItem(assets: *const Assets, role_id: i32) ?i32 {
    const role = assets.tables.role_info.getDataById(role_id) orelse return null;
    if (role.ResonantChainGroupId == 0) return null;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId != role.ResonantChainGroupId or chain.GroupIndex != 1) continue;
        var iterator = chain.ActivateConsume.map.iterator();
        if (iterator.next()) |entry| return entry.key_ptr.*;
        return null;
    }
    return null;
}

fn pushUpdatedRoleList(
    conn: *Connection,
    alloc: mem.Alloc,
    assets: *const Assets,
    role_comp: *PlayerRoleComponent,
    basic_comp: *PlayerBasicComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    io: std.Io,
) !void {
    const selected_main_role = RoleHelper.selectedMainRoleId(
        assets,
        role_comp,
        basic_comp.info.attributes.sex,
        basic_comp.info.selected_main_role_id,
    );
    var notify: pb.PbGetRoleListNotify = .default;
    try notify.RoleList.ensureTotalCapacity(alloc.arena, role_comp.role_map.count());
    var iterator = role_comp.role_map.iterator();
    while (iterator.next()) |role| {
        if (role.key_ptr.* > 1999 or role.key_ptr.* < 1000) continue;
        if (selected_main_role) |selected| {
            if (RoleHelper.isMainRole(assets, role.key_ptr.*) and role.key_ptr.* != selected) continue;
        }
        notify.RoleList.appendAssumeCapacity(try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            role.key_ptr.*,
            role.value_ptr,
            weapon_comp,
            echo_comp,
        ));
    }
    try conn.push(alloc.gpa, io, notify);
}

const GachaViewType = struct {
    const novice = 1;
    const featured_role = 2;
    const featured_weapon = 3;
    const standard_role = 4;
    const standard_weapon = 5;
    const beginner_choice = 6;
    const anniversary_role = 7;
    const anniversary_weapon = 8;
    const new_voyage_role = 9;
    const new_voyage_weapon = 10;
    const collab_role = 11;
    const collab_weapon = 12;
    const rerun_role = 13;
    const rerun_weapon = 14;
};

fn saveGachaInfo(alloc: mem.Alloc, fs: *FileSystem, comp: *PlayerGachaComponent) !void {
    const path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}", .{ comp.player_id, GachaInfo.data_path });
    try comp_util.saveStruct(fs, comp.info, path, alloc.arena);
}

fn firstPoolId(assets: *const Assets, gacha_id: i32) i32 {
    var best_id: i32 = 0;
    var best_sort: i32 = std.math.maxInt(i32);
    for (assets.tables.gacha_pool.items) |pool| {
        if (pool.GachaId != gacha_id) continue;
        if (pool.Sort < best_sort or (pool.Sort == best_sort and (best_id == 0 or pool.Id < best_id))) {
            best_id = pool.Id;
            best_sort = pool.Sort;
        }
    }
    return best_id;
}

fn gachaRuleGroupId(assets: *const Assets, gacha_id: i32) i32 {
    if (assets.tables.gacha.getDataById(gacha_id)) |gacha| {
        if (gacha.RuleGroupId != 0) return gacha.RuleGroupId;
    }
    return gacha_id;
}

fn poolBelongsToGacha(assets: *const Assets, gacha_id: i32, pool_id: i32) bool {
    for (assets.tables.gacha_pool.items) |pool| {
        if (pool.GachaId == gacha_id and pool.Id == pool_id) return true;
    }
    return false;
}

fn selectedPoolView(assets: *const Assets, banner: *GachaInfo.Banner) ?GachaViewInfo {
    const pool_id = if (banner.selected_pool_id != 0 and poolBelongsToGacha(assets, banner.id, banner.selected_pool_id))
        banner.selected_pool_id
    else
        firstPoolId(assets, banner.id);
    if (pool_id == 0) return null;
    banner.selected_pool_id = pool_id;
    return assets.tables.gacha_view_info.getDataById(pool_id);
}

fn appendI32List(list: *std.ArrayList(i32), arena: std.mem.Allocator, values: []const i32) !void {
    try list.ensureUnusedCapacity(arena, values.len);
    for (values) |value| list.appendAssumeCapacity(value);
}

fn poolTitle(view_type: i32) []const u8 {
    return switch (view_type) {
        GachaViewType.novice => "Novice Convene",
        GachaViewType.featured_role, GachaViewType.anniversary_role, GachaViewType.new_voyage_role, GachaViewType.collab_role, GachaViewType.rerun_role => "Featured Resonator Convene",
        GachaViewType.featured_weapon, GachaViewType.anniversary_weapon, GachaViewType.new_voyage_weapon, GachaViewType.collab_weapon, GachaViewType.rerun_weapon => "Featured Weapon Convene",
        GachaViewType.standard_role => "Standard Resonator Convene",
        GachaViewType.standard_weapon => "Standard Weapon Convene",
        GachaViewType.beginner_choice => "Beginner's Choice Convene",
        else => "Convene",
    };
}

fn poolDescription(view_type: i32) []const u8 {
    return switch (view_type) {
        GachaViewType.novice => "Novice Convene",
        GachaViewType.featured_role, GachaViewType.anniversary_role, GachaViewType.new_voyage_role, GachaViewType.collab_role, GachaViewType.rerun_role => "Every 10 Convenes guarantees a\n4-Star or above item.\nA 5-Star Character is guaranteed\nwithin 80 Convenes.",
        GachaViewType.featured_weapon, GachaViewType.anniversary_weapon, GachaViewType.new_voyage_weapon, GachaViewType.collab_weapon, GachaViewType.rerun_weapon => "Featured Weapon Convene Event",
        GachaViewType.standard_role => "Standard Resonator Convene",
        GachaViewType.standard_weapon => "Standard Weapon Convene",
        GachaViewType.beginner_choice => "Beginner's Choice Convene",
        else => "Convene Event",
    };
}

fn displayText(key: []const u8, fallback: []const u8) []const u8 {
    return if (std.mem.startsWith(u8, key, "Text_")) fallback else key;
}

fn buildPoolInfo(arena: std.mem.Allocator, view: GachaViewInfo) !pb.GachaPoolInfo {
    var show_ids: std.ArrayList(i32) = .empty;
    var up_ids: std.ArrayList(i32) = .empty;
    var preview_ids: std.ArrayList(i32) = .empty;
    try appendI32List(&show_ids, arena, view.ShowIdList);
    try appendI32List(&up_ids, arena, view.UpList);
    try appendI32List(&preview_ids, arena, view.PreviewIdList);

    return .{
        .Id = view.Id,
        .BeginTime = 0,
        .EndTime = 0,
        .Title = displayText(view.SummaryTitle, poolTitle(view.Type)),
        .Description = displayText(view.SummaryDescribe, poolDescription(view.Type)),
        .UiType = view.Type,
        .ThemeColor = view.ThemeColor,
        .ShowIdList = show_ids,
        .UpList = up_ids,
        .PreviewIdList = preview_ids,
        .ComplianceDetail = "",
    };
}

fn isBeginnerChoice(gacha_id: i32, view_type: i32) bool {
    return view_type == GachaViewType.beginner_choice and gacha_id == 5;
}

fn isNoviceBanner(assets: *const Assets, gacha_id: i32, view_type: i32) bool {
    return view_type == GachaViewType.novice or gachaRuleGroupId(assets, gacha_id) == 5;
}

fn bannerTotalLimit(assets: *const Assets, gacha_id: i32, view_type: i32) i32 {
    if (isBeginnerChoice(gacha_id, view_type)) return 1;
    if (isNoviceBanner(assets, gacha_id, view_type)) return 50;
    if (view_type == GachaViewType.beginner_choice or gachaRuleGroupId(assets, gacha_id) == 6) return 80;
    return 0;
}

fn consumeCountForPull(assets: *const Assets, gacha_id: i32, view_type: i32, gacha_times: i32) ?i32 {
    if (isBeginnerChoice(gacha_id, view_type)) {
        return if (gacha_times == 1) 1 else null;
    }
    if (isNoviceBanner(assets, gacha_id, view_type)) {
        return if (gacha_times == 10) 8 else null;
    }
    if (gacha_times == 1 or gacha_times == 10) return gacha_times;
    return null;
}

fn gachaConsumes(arena: std.mem.Allocator, gacha_id: i32, view_type: i32) !std.ArrayList(pb.GachaConsume) {
    var consumes: std.ArrayList(pb.GachaConsume) = .empty;
    if (view_type == GachaViewType.novice) {
        try consumes.append(arena, .{ .Times = 10, .Consume = 8 });
    } else if (isBeginnerChoice(gacha_id, view_type)) {
        try consumes.append(arena, .{ .Times = 1, .Consume = 1 });
    } else {
        try consumes.append(arena, .{ .Times = 1, .Consume = 1 });
        try consumes.append(arena, .{ .Times = 10, .Consume = 10 });
    }
    return consumes;
}

fn resourcesId(view_type: i32) []const u8 {
    return switch (view_type) {
        GachaViewType.featured_role, GachaViewType.anniversary_role, GachaViewType.new_voyage_role, GachaViewType.collab_role, GachaViewType.rerun_role => "UiItem_RoleUpGachaPool",
        GachaViewType.featured_weapon, GachaViewType.anniversary_weapon, GachaViewType.new_voyage_weapon, GachaViewType.collab_weapon, GachaViewType.rerun_weapon => "UiItem_WeaponGachaPool",
        GachaViewType.novice => "UiItem_NewPlayerGachaPool",
        else => "UiItem_BaseGachaPool",
    };
}

fn buildGachaInfo(
    alloc: mem.Alloc,
    assets: *const Assets,
    gacha_comp: *PlayerGachaComponent,
    gacha_id: i32,
) !?pb.GachaInfo {
    const first_pool_id = firstPoolId(assets, gacha_id);
    const banner = try gacha_comp.info.ensureBanner(alloc.gpa, gacha_id, first_pool_id);
    const view = selectedPoolView(assets, banner) orelse return null;

    var pools: std.ArrayList(pb.GachaPoolInfo) = .empty;
    for (assets.tables.gacha_pool.items) |pool| {
        if (pool.GachaId != gacha_id) continue;
        const pool_view = assets.tables.gacha_view_info.getDataById(pool.Id) orelse continue;
        try pools.append(alloc.arena, try buildPoolInfo(alloc.arena, pool_view));
    }
    if (pools.items.len == 0) return null;

    return .{
        .Id = gacha_id,
        .TodayTimes = banner.daily_pulls,
        .TotalTimes = banner.total_pulls,
        .ItemId = inventory_helper.gachaCurrencyItemId(gacha_id, view.Type),
        .GachaConsumes = try gachaConsumes(alloc.arena, gacha_id, view.Type),
        .UsePoolId = banner.selected_pool_id,
        .Pools = pools,
        .BeginTime = 0,
        .EndTime = 0,
        .DailyLimitTimes = 0,
        .TotalLimitTimes = bannerTotalLimit(assets, gacha_id, view.Type),
        .ResourcesId = resourcesId(view.Type),
    };
}

pub fn onGachaInfoRequest(
    txn: *Transaction(pb.GachaInfoRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    gacha_comp: *PlayerGachaComponent,
) !void {
    var infos: std.ArrayList(pb.GachaInfo) = .empty;
    for (assets.tables.gacha.items) |gacha| {
        if (try buildGachaInfo(alloc, assets, gacha_comp, gacha.Id)) |info| {
            try infos.append(alloc.arena, info);
        }
    }
    std.mem.sort(pb.GachaInfo, infos.items, assets, gachaInfoOlderFirst);
    try saveGachaInfo(alloc, fs, gacha_comp);

    txn.respond(.{
        .ErrorCode = .Success,
        .GachaInfos = infos,
        .DailyTotalLeftTimes = -1,
    });
}

fn gachaSortValue(assets: *const Assets, gacha_id: i32) i32 {
    if (assets.tables.gacha.getDataById(gacha_id)) |gacha| return gacha.Sort;
    return std.math.maxInt(i32);
}

fn gachaViewType(assets: *const Assets, gacha_id: i32) i32 {
    const pool_id = firstPoolId(assets, gacha_id);
    if (pool_id == 0) return 0;
    if (assets.tables.gacha_view_info.getDataById(pool_id)) |view| return view.Type;
    return 0;
}

fn gachaFamilyRank(view_type: i32) i32 {
    return switch (view_type) {
        GachaViewType.featured_role,
        GachaViewType.anniversary_role,
        GachaViewType.new_voyage_role,
        GachaViewType.collab_role,
        GachaViewType.rerun_role,
        => 10,
        GachaViewType.featured_weapon,
        GachaViewType.anniversary_weapon,
        GachaViewType.new_voyage_weapon,
        GachaViewType.collab_weapon,
        GachaViewType.rerun_weapon,
        => 20,
        GachaViewType.novice => 30,
        GachaViewType.beginner_choice => 40,
        GachaViewType.standard_role => 50,
        GachaViewType.standard_weapon => 60,
        else => 100,
    };
}

fn gachaSubRank(view_type: i32) i32 {
    return switch (view_type) {
        GachaViewType.collab_role,
        GachaViewType.collab_weapon,
        => 1,
        else => 0,
    };
}

fn gachaInfoOlderFirst(assets: *const Assets, lhs: pb.GachaInfo, rhs: pb.GachaInfo) bool {
    const lhs_type = gachaViewType(assets, lhs.Id);
    const rhs_type = gachaViewType(assets, rhs.Id);
    const lhs_family = gachaFamilyRank(lhs_type);
    const rhs_family = gachaFamilyRank(rhs_type);
    if (lhs_family != rhs_family) return lhs_family < rhs_family;

    const lhs_sub = gachaSubRank(lhs_type);
    const rhs_sub = gachaSubRank(rhs_type);
    if (lhs_sub != rhs_sub) return lhs_sub < rhs_sub;

    const lhs_sort = gachaSortValue(assets, lhs.Id);
    const rhs_sort = gachaSortValue(assets, rhs.Id);
    if (lhs_sort != rhs_sort) return lhs_sort < rhs_sort;

    return lhs.Id < rhs.Id;
}

pub fn onGachaUsePoolRequest(
    txn: *Transaction(pb.GachaUsePoolRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    gacha_comp: *PlayerGachaComponent,
) !void {
    const request = txn.message;
    if (!poolBelongsToGacha(assets, request.GachaId, request.PoolId)) {
        txn.respond(.{ .ErrorCode = .ErrGachaPoolConfigNotFound });
        return;
    }

    const banner = try gacha_comp.info.ensureBanner(alloc.gpa, request.GachaId, request.PoolId);
    banner.selected_pool_id = request.PoolId;
    try saveGachaInfo(alloc, fs, gacha_comp);

    txn.respond(.{ .ErrorCode = .Success });
}

fn appendDetailItems(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(pb.GachaItem),
    values: []const i32,
    view_type: i32,
    quality: i32,
    is_up: bool,
) !void {
    for (values) |item_id| {
        if (containsDetailItem(list.items, item_id)) continue;
        if (itemQuality(assets, item_id) == quality and itemMatchesBanner(assets, item_id, view_type, quality)) {
            try list.append(arena, .{ .ItemId = item_id, .IsUp = is_up });
        }
    }
}

fn appendDetailItemsExcluding(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(pb.GachaItem),
    values: []const i32,
    exclude_a: []const i32,
    exclude_b: []const i32,
    view_type: i32,
    quality: i32,
    is_up: bool,
) !void {
    for (values) |item_id| {
        if (containsI32(exclude_a, item_id) or containsI32(exclude_b, item_id)) continue;
        if (containsDetailItem(list.items, item_id)) continue;
        if (itemQuality(assets, item_id) == quality and itemMatchesBanner(assets, item_id, view_type, quality)) {
            try list.append(arena, .{ .ItemId = item_id, .IsUp = is_up });
        }
    }
}

fn buildUpDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
) !std.ArrayList(pb.GachaItem) {
    var list: std.ArrayList(pb.GachaItem) = .empty;
    if (!isFeaturedView(view.Type)) return list;
    try appendDetailItems(arena, assets, &list, view.ShowIdList, view.Type, quality, true);
    try appendDetailItems(arena, assets, &list, view.UpList, view.Type, quality, true);
    return list;
}

fn buildRegularDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
) !std.ArrayList(pb.GachaItem) {
    var list: std.ArrayList(pb.GachaItem) = .empty;
    const exclude_show = if (isFeaturedView(view.Type) and quality >= 4) view.ShowIdList else &.{};
    const exclude_up = if (isFeaturedView(view.Type) and quality >= 4) view.UpList else &.{};
    try appendDetailItemsExcluding(arena, assets, &list, view.PreviewIdList, exclude_show, exclude_up, view.Type, quality, false);
    if (list.items.len == 0) {
        try appendFallbackDetailItems(arena, assets, &list, view.Type, quality, exclude_show, exclude_up);
    }
    return list;
}

fn buildConfiguredDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
    is_up: bool,
) !std.ArrayList(pb.GachaItem) {
    var list: std.ArrayList(pb.GachaItem) = .empty;
    try appendDetailItems(arena, assets, &list, view.ShowIdList, view.Type, quality, is_up);
    try appendDetailItems(arena, assets, &list, view.UpList, view.Type, quality, is_up);
    try appendDetailItems(arena, assets, &list, view.PreviewIdList, view.Type, quality, is_up);
    return list;
}

fn buildStandardFiveStarResonatorDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
) !std.ArrayList(pb.GachaItem) {
    var list: std.ArrayList(pb.GachaItem) = .empty;
    const standard = try buildStandardFiveStarResonators(arena, assets);
    for (standard.items) |item_id| {
        try list.append(arena, .{ .ItemId = item_id, .IsUp = false });
    }
    return list;
}

fn buildFiveStarRoleDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
) !std.ArrayList(pb.GachaItem) {
    if (isFeaturedResonatorView(view.Type)) return buildUpDetailList(arena, assets, view, 5);
    return switch (view.Type) {
        GachaViewType.standard_role,
        GachaViewType.novice,
        GachaViewType.beginner_choice,
        => buildConfiguredDetailList(arena, assets, view, 5, false),
        else => .empty,
    };
}

fn buildFiveStarWeaponDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
) !std.ArrayList(pb.GachaItem) {
    if (isFeaturedWeaponView(view.Type)) return buildUpDetailList(arena, assets, view, 5);
    if (isFeaturedResonatorView(view.Type)) return buildStandardFiveStarResonatorDetailList(arena, assets);
    if (view.Type == GachaViewType.standard_weapon) return buildConfiguredDetailList(arena, assets, view, 5, false);
    return .empty;
}

fn buildThreeStarDetailList(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
) !std.ArrayList(pb.GachaItem) {
    var list: std.ArrayList(pb.GachaItem) = .empty;
    try appendDetailItems(arena, assets, &list, view.PreviewIdList, view.Type, 3, false);
    if (list.items.len == 0) {
        try appendFallbackDetailItems(arena, assets, &list, view.Type, 3, &.{}, &.{});
    }
    return list;
}

pub fn onGachaPoolDetailRequest(
    txn: *Transaction(pb.GachaPoolDetailRequest),
    alloc: mem.Alloc,
    assets: *const Assets,
) !void {
    const view = assets.tables.gacha_view_info.getDataById(txn.message.PoolId) orelse {
        txn.respond(.{ .ErrorCode = .ErrGachaPoolConfigNotFound });
        return;
    };

    txn.respond(.{
        .ErrorCode = .Success,
        .GachaPoolDetail = .{
            .Text = displayText(view.SummaryDescribe, poolDescription(view.Type)),
            .FiveStarRoles = try buildFiveStarRoleDetailList(alloc.arena, assets, view),
            .FiveStarWeapons = try buildFiveStarWeaponDetailList(alloc.arena, assets, view),
            .FourStarRoles = try buildUpDetailList(alloc.arena, assets, view, 4),
            .FourStarWeapons = try buildRegularDetailList(alloc.arena, assets, view, 4),
            .ThreeStarRoles = try buildThreeStarDetailList(alloc.arena, assets, view),
            .FiveStarTitle = "5-Star Convene Items",
            .FileStarDetail = "Featured 5-Star",
            .FourStarTitle = "4-Star Convene Items",
            .FourStarDetail = "Featured 4-Star",
            .ThreeStarTitle = "3-Star Convene Items",
            .ThreeStarDetail = "Available 3-Star",
        },
    });
}

fn secureRandomLessThan(io: Io, less_than: u64) !u64 {
    if (less_than == 0) return 0;

    const bound: u128 = less_than;
    const range = @as(u128, 1) << 64;
    const limit = range - (range % bound);

    while (true) {
        var value: u64 = undefined;
        try io.randomSecure(std.mem.asBytes(&value));
        const wide_value: u128 = value;
        if (wide_value < limit) return @intCast(wide_value % bound);
    }
}

fn roll(io: Io, less_than: i32) !i32 {
    return @intCast(try secureRandomLessThan(io, @intCast(less_than)));
}

fn chooseFrom(io: Io, values: []const i32) !i32 {
    if (values.len == 0) return 0;
    return values[@intCast(try secureRandomLessThan(io, @intCast(values.len)))];
}

fn rollRatio(io: Io, numerator: i32, denominator: i32) !bool {
    if (denominator <= 0 or numerator <= 0) return false;
    if (numerator >= denominator) return true;
    return (try roll(io, denominator)) < numerator;
}

fn containsI32(values: []const i32, value: i32) bool {
    for (values) |item| {
        if (item == value) return true;
    }
    return false;
}

fn containsDetailItem(values: []const pb.GachaItem, item_id: i32) bool {
    for (values) |item| {
        if (item.ItemId == item_id) return true;
    }
    return false;
}

fn fiveStarHardPity(view_type: i32) i32 {
    return if (view_type == GachaViewType.novice) gacha_rates.novice.five_star_hard_pity_pull else gacha_rates.normal.five_star_hard_pity_pull;
}

fn normalFiveStarRate(pull_number: i32) i32 {
    if (pull_number >= gacha_rates.normal.five_star_soft_pity_force_pull) return gacha_rates.rate_denominator;
    if (pull_number < gacha_rates.normal.five_star_soft_pity_start_pull) return gacha_rates.five_star_base_rate;

    if (pull_number <= gacha_rates.normal.five_star_soft_pity_phase_one_end_pull) {
        return gacha_rates.five_star_base_rate + gacha_rates.normal.five_star_soft_pity_phase_one_step * (pull_number - gacha_rates.normal.five_star_soft_pity_start_pull + 1);
    }
    if (pull_number <= gacha_rates.normal.five_star_soft_pity_phase_two_end_pull) {
        const phase_one_rate = gacha_rates.five_star_base_rate + gacha_rates.normal.five_star_soft_pity_phase_one_step * (gacha_rates.normal.five_star_soft_pity_phase_one_end_pull - gacha_rates.normal.five_star_soft_pity_start_pull + 1);
        return phase_one_rate + gacha_rates.normal.five_star_soft_pity_phase_two_step * (pull_number - gacha_rates.normal.five_star_soft_pity_phase_one_end_pull);
    }

    const phase_one_rate = gacha_rates.five_star_base_rate + gacha_rates.normal.five_star_soft_pity_phase_one_step * (gacha_rates.normal.five_star_soft_pity_phase_one_end_pull - gacha_rates.normal.five_star_soft_pity_start_pull + 1);
    const phase_two_rate = phase_one_rate + gacha_rates.normal.five_star_soft_pity_phase_two_step * (gacha_rates.normal.five_star_soft_pity_phase_two_end_pull - gacha_rates.normal.five_star_soft_pity_phase_one_end_pull);
    return phase_two_rate + gacha_rates.normal.five_star_soft_pity_phase_three_step * (pull_number - gacha_rates.normal.five_star_soft_pity_phase_two_end_pull);
}

fn noviceFiveStarRate(pull_number: i32) i32 {
    if (pull_number < gacha_rates.novice.five_star_soft_pity_start_pull) return gacha_rates.five_star_base_rate;
    return @min(gacha_rates.rate_denominator, gacha_rates.five_star_base_rate + gacha_rates.novice.five_star_soft_pity_step * (pull_number - gacha_rates.novice.five_star_soft_pity_start_pull + 1));
}

fn fiveStarRate(view_type: i32, pull_number: i32) i32 {
    return if (view_type == GachaViewType.novice)
        noviceFiveStarRate(pull_number)
    else
        normalFiveStarRate(pull_number);
}

fn determineRarity(io: Io, bucket: *GachaInfo.Bucket, gacha_id: i32, view_type: i32) !i32 {
    if (isBeginnerChoice(gacha_id, view_type)) return 5;
    const pull_number = bucket.pull_count + 1;
    if (pull_number >= fiveStarHardPity(view_type)) return 5;
    if (bucket.pity_four + 1 >= 10) return 4;

    const five_rate = fiveStarRate(view_type, pull_number);
    const value = try roll(io, gacha_rates.rate_denominator);
    if (value < five_rate) return 5;
    if (value < five_rate + gacha_rates.four_star_base_rate) return 4;
    return 3;
}

fn updatePity(bucket: *GachaInfo.Bucket, banner: *GachaInfo.Banner, rarity: i32) void {
    if (rarity == 5) {
        bucket.pull_count = 0;
        bucket.pity_four += 1;
    } else {
        bucket.pull_count += 1;
        bucket.pity_four = if (rarity == 4) 0 else bucket.pity_four + 1;
    }
    banner.daily_pulls += 1;
    banner.total_pulls += 1;
}

fn isFeaturedResonatorView(view_type: i32) bool {
    return view_type == GachaViewType.featured_role or
        view_type == GachaViewType.anniversary_role or
        view_type == GachaViewType.new_voyage_role or
        view_type == GachaViewType.collab_role or
        view_type == GachaViewType.rerun_role;
}

fn isFeaturedWeaponView(view_type: i32) bool {
    return view_type == GachaViewType.featured_weapon or
        view_type == GachaViewType.anniversary_weapon or
        view_type == GachaViewType.new_voyage_weapon or
        view_type == GachaViewType.collab_weapon or
        view_type == GachaViewType.rerun_weapon;
}

fn isFeaturedView(view_type: i32) bool {
    return isFeaturedResonatorView(view_type) or isFeaturedWeaponView(view_type);
}

fn itemQuality(assets: *const Assets, item_id: i32) i32 {
    if (assets.tables.role_info.getDataById(item_id)) |role| return role.QualityId;
    if (assets.tables.weapon_conf.getDataById(item_id)) |weapon| return weapon.QualityId;
    return 0;
}

fn hasGachaTexture(assets: *const Assets, item_id: i32) bool {
    return assets.tables.gacha_texture_info.getDataById(item_id) != null;
}

fn itemMatchesBanner(assets: *const Assets, item_id: i32, view_type: i32, quality: i32) bool {
    const is_role = assets.tables.role_info.getDataById(item_id) != null;
    const is_weapon = assets.tables.weapon_conf.getDataById(item_id) != null;
    if (!hasGachaTexture(assets, item_id)) return false;
    if (quality < 5) return is_role or is_weapon;

    return switch (view_type) {
        GachaViewType.featured_role,
        GachaViewType.standard_role,
        GachaViewType.anniversary_role,
        GachaViewType.new_voyage_role,
        GachaViewType.collab_role,
        GachaViewType.rerun_role,
        GachaViewType.novice,
        GachaViewType.beginner_choice,
        => is_role,
        GachaViewType.featured_weapon,
        GachaViewType.standard_weapon,
        GachaViewType.anniversary_weapon,
        GachaViewType.new_voyage_weapon,
        GachaViewType.collab_weapon,
        GachaViewType.rerun_weapon,
        => is_weapon,
        else => is_role or is_weapon,
    };
}

fn appendQualityItems(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(i32),
    values: []const i32,
    view_type: i32,
    quality: i32,
) !void {
    for (values) |item_id| {
        if (containsI32(list.items, item_id)) continue;
        if (itemQuality(assets, item_id) == quality and itemMatchesBanner(assets, item_id, view_type, quality)) {
            try list.append(arena, item_id);
        }
    }
}

fn appendQualityItemsExcluding(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(i32),
    values: []const i32,
    exclude_a: []const i32,
    exclude_b: []const i32,
    view_type: i32,
    quality: i32,
) !void {
    for (values) |item_id| {
        if (containsI32(exclude_a, item_id) or containsI32(exclude_b, item_id)) continue;
        if (containsI32(list.items, item_id)) continue;
        if (itemQuality(assets, item_id) == quality and itemMatchesBanner(assets, item_id, view_type, quality)) {
            try list.append(arena, item_id);
        }
    }
}

fn appendFallbackQualityItems(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(i32),
    view_type: i32,
    quality: i32,
    exclude_a: []const i32,
    exclude_b: []const i32,
) !void {
    if (quality == 3) {
        for (assets.tables.weapon_conf.items) |weapon| {
            if (containsI32(exclude_a, weapon.ItemId) or containsI32(exclude_b, weapon.ItemId)) continue;
            if (containsI32(list.items, weapon.ItemId)) continue;
            if (weapon.QualityId == quality and hasGachaTexture(assets, weapon.ItemId)) {
                try list.append(arena, weapon.ItemId);
            }
        }
        return;
    }

    if (quality == 4) {
        for (assets.tables.role_info.items) |role| {
            if (containsI32(exclude_a, role.Id) or containsI32(exclude_b, role.Id)) continue;
            if (containsI32(list.items, role.Id)) continue;
            if (!role.IsTrial and itemQuality(assets, role.Id) == quality and hasGachaTexture(assets, role.Id)) {
                try list.append(arena, role.Id);
            }
        }
        for (assets.tables.weapon_conf.items) |weapon| {
            if (containsI32(exclude_a, weapon.ItemId) or containsI32(exclude_b, weapon.ItemId)) continue;
            if (containsI32(list.items, weapon.ItemId)) continue;
            if (weapon.QualityId == quality and hasGachaTexture(assets, weapon.ItemId)) {
                try list.append(arena, weapon.ItemId);
            }
        }
        return;
    }

    switch (view_type) {
        GachaViewType.featured_role,
        GachaViewType.standard_role,
        GachaViewType.anniversary_role,
        GachaViewType.new_voyage_role,
        GachaViewType.collab_role,
        GachaViewType.rerun_role,
        GachaViewType.novice,
        GachaViewType.beginner_choice,
        => for (assets.tables.role_info.items) |role| {
            if (containsI32(exclude_a, role.Id) or containsI32(exclude_b, role.Id)) continue;
            if (containsI32(list.items, role.Id)) continue;
            if (!role.IsTrial and itemQuality(assets, role.Id) == quality and hasGachaTexture(assets, role.Id)) {
                try list.append(arena, role.Id);
            }
        },
        GachaViewType.featured_weapon,
        GachaViewType.standard_weapon,
        GachaViewType.anniversary_weapon,
        GachaViewType.new_voyage_weapon,
        GachaViewType.collab_weapon,
        GachaViewType.rerun_weapon,
        => for (assets.tables.weapon_conf.items) |weapon| {
            if (containsI32(exclude_a, weapon.ItemId) or containsI32(exclude_b, weapon.ItemId)) continue;
            if (containsI32(list.items, weapon.ItemId)) continue;
            if (weapon.QualityId == quality and hasGachaTexture(assets, weapon.ItemId)) {
                try list.append(arena, weapon.ItemId);
            }
        },
        else => {},
    }
}

fn appendFallbackDetailItems(
    arena: std.mem.Allocator,
    assets: *const Assets,
    list: *std.ArrayList(pb.GachaItem),
    view_type: i32,
    quality: i32,
    exclude_a: []const i32,
    exclude_b: []const i32,
) !void {
    var candidates: std.ArrayList(i32) = .empty;
    try appendFallbackQualityItems(arena, assets, &candidates, view_type, quality, exclude_a, exclude_b);
    for (candidates.items) |item_id| {
        try list.append(arena, .{ .ItemId = item_id, .IsUp = false });
    }
}

fn buildFeaturedRewardCandidates(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
) !std.ArrayList(i32) {
    var candidates: std.ArrayList(i32) = .empty;
    if (!isFeaturedView(view.Type)) return candidates;

    if (quality == 5) {
        try appendQualityItems(arena, assets, &candidates, view.ShowIdList, view.Type, quality);
        try appendQualityItems(arena, assets, &candidates, view.UpList, view.Type, quality);
    } else if (quality == 4) {
        try appendQualityItems(arena, assets, &candidates, view.UpList, view.Type, quality);
    }
    return candidates;
}

fn buildRegularRewardCandidates(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
) !std.ArrayList(i32) {
    var candidates: std.ArrayList(i32) = .empty;
    const exclude_show = if (isFeaturedView(view.Type) and quality >= 4) view.ShowIdList else &.{};
    const exclude_up = if (isFeaturedView(view.Type) and quality >= 4) view.UpList else &.{};

    try appendQualityItemsExcluding(arena, assets, &candidates, view.PreviewIdList, exclude_show, exclude_up, view.Type, quality);
    if (candidates.items.len == 0) {
        try appendFallbackQualityItems(arena, assets, &candidates, view.Type, quality, exclude_show, exclude_up);
    }
    return candidates;
}

fn buildRewardCandidates(
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    quality: i32,
) !std.ArrayList(i32) {
    if (isFeaturedView(view.Type) and quality == 5) {
        var candidates = try buildFeaturedRewardCandidates(arena, assets, view, quality);
        if (candidates.items.len == 0) {
            try appendFallbackQualityItems(arena, assets, &candidates, view.Type, quality, &.{}, &.{});
        }
        return candidates;
    }

    return buildRegularRewardCandidates(arena, assets, view, quality);
}

fn buildStandardFiveStarResonators(arena: std.mem.Allocator, assets: *const Assets) !std.ArrayList(i32) {
    var candidates: std.ArrayList(i32) = .empty;
    for (assets.tables.gacha_view_info.items) |view| {
        if (view.Type != GachaViewType.standard_role) continue;
        try appendQualityItems(arena, assets, &candidates, view.PreviewIdList, view.Type, 5);
        if (candidates.items.len != 0) break;
    }
    return candidates;
}

fn chooseFeaturedResonatorFiveStar(
    io: Io,
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    bucket: *GachaInfo.Bucket,
) !i32 {
    const featured = try buildRewardCandidates(arena, assets, view, 5);
    if (featured.items.len == 0) return error.GachaRewardCandidateNotFound;

    if (bucket.guarantee_five or try rollRatio(io, gacha_rates.featured_five_star_win_numerator, gacha_rates.featured_five_star_win_denominator)) {
        bucket.guarantee_five = false;
        return chooseFrom(io, featured.items);
    }

    const standard = try buildStandardFiveStarResonators(arena, assets);
    if (standard.items.len == 0) {
        bucket.guarantee_five = false;
        return chooseFrom(io, featured.items);
    }

    bucket.guarantee_five = true;
    return chooseFrom(io, standard.items);
}

fn chooseFeaturedFourStar(
    io: Io,
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    bucket: *GachaInfo.Bucket,
) !i32 {
    const up = try buildFeaturedRewardCandidates(arena, assets, view, 4);
    if (up.items.len == 0) {
        const regular = try buildRegularRewardCandidates(arena, assets, view, 4);
        if (regular.items.len == 0) return error.GachaRewardCandidateNotFound;
        return chooseFrom(io, regular.items);
    }

    if (bucket.guarantee_four or try rollRatio(io, gacha_rates.featured_four_star_win_numerator, gacha_rates.featured_four_star_win_denominator)) {
        bucket.guarantee_four = false;
        return chooseFrom(io, up.items);
    }

    const regular = try buildRegularRewardCandidates(arena, assets, view, 4);
    if (regular.items.len == 0) {
        bucket.guarantee_four = false;
        return chooseFrom(io, up.items);
    }

    bucket.guarantee_four = true;
    return chooseFrom(io, regular.items);
}

const RolledPull = struct {
    rarity: i32,
    reward_id: i32,
};

fn maxResonantChainIndex(assets: *const Assets, role_id: i32) i32 {
    const role = assets.tables.role_info.getDataById(role_id) orelse return 6;
    if (role.ResonantChainGroupId == 0) return 6;
    var max_index: i32 = 0;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId == role.ResonantChainGroupId and chain.GroupIndex > max_index) {
            max_index = chain.GroupIndex;
        }
    }
    return if (max_index > 0) max_index else 6;
}

fn recordUpdatedNormalItem(
    arena: std.mem.Allocator,
    added_items: *std.ArrayList(pb.NormalItem),
    updated_items: *std.ArrayList(pb.NormalItem),
    old_count: i32,
    inventory_comp: *const PlayerInventoryComponent,
    item_id: i32,
) !void {
    const new_count = inventory_comp.info.normalItemCount(item_id);
    const target_list = if (old_count <= 0) added_items else updated_items;
    for (target_list.items) |*entry| {
        if (entry.Id == item_id) {
            entry.Count = new_count;
            return;
        }
    }
    try target_list.append(arena, .{
        .Id = item_id,
        .Count = new_count,
        .ExpireTime = 0,
    });
}

fn grantBonusItem(
    alloc: mem.Alloc,
    inventory_comp: *PlayerInventoryComponent,
    reward_list: *std.ArrayList(pb.GachaReward),
    added_items: *std.ArrayList(pb.NormalItem),
    updated_items: *std.ArrayList(pb.NormalItem),
    item_id: i32,
    count: i32,
) !void {
    if (item_id <= 0 or count <= 0) return;
    const old_count = inventory_comp.info.normalItemCount(item_id);
    try InventoryInfo.addNormalItem(&inventory_comp.info, alloc.gpa, item_id, count);
    try reward_list.append(alloc.arena, .{ .ItemId = item_id, .ItemCount = count });
    try recordUpdatedNormalItem(alloc.arena, added_items, updated_items, old_count, inventory_comp, item_id);
}

fn chooseReward(
    io: Io,
    arena: std.mem.Allocator,
    assets: *const Assets,
    view: GachaViewInfo,
    _: *const GachaInfo.Banner,
    bucket: *GachaInfo.Bucket,
    rarity: i32,
) !i32 {
    if (view.Type == GachaViewType.beginner_choice and rarity == 5 and view.ShowIdList.len != 0) {
        return view.ShowIdList[0];
    }

    if (rarity == 5 and isFeaturedResonatorView(view.Type)) {
        return try chooseFeaturedResonatorFiveStar(io, arena, assets, view, bucket);
    }

    if (rarity == 4 and isFeaturedView(view.Type)) {
        return try chooseFeaturedFourStar(io, arena, assets, view, bucket);
    }

    const candidates = try buildRewardCandidates(arena, assets, view, rarity);
    if (candidates.items.len == 0) return error.GachaRewardCandidateNotFound;
    return chooseFrom(io, candidates.items);
}

pub fn onGachaRequest(
    txn: *Transaction(pb.GachaRequest),
    alloc: mem.Alloc,
    io: Io,
    fs: *FileSystem,
    assets: *const Assets,
    gacha_comp: *PlayerGachaComponent,
    inventory_comp: *PlayerInventoryComponent,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    basic_comp: *PlayerBasicComponent,
) !void {
    const request = txn.message;
    const banner = try gacha_comp.info.ensureBanner(alloc.gpa, request.GachaId, firstPoolId(assets, request.GachaId));
    const bucket = try gacha_comp.info.ensureBucket(alloc.gpa, gachaRuleGroupId(assets, request.GachaId));
    const view = selectedPoolView(assets, banner) orelse {
        txn.respond(.{ .ErrorCode = .ErrGachaConfigNotFound });
        return;
    };

    const consume_count = consumeCountForPull(assets, request.GachaId, view.Type, request.GachaTimes) orelse {
        txn.respond(.{ .ErrorCode = .ErrGachaTimesNonsupport });
        return;
    };

    const total_limit = bannerTotalLimit(assets, request.GachaId, view.Type);
    if (total_limit > 0 and banner.total_pulls + request.GachaTimes > total_limit) {
        txn.respond(.{ .ErrorCode = .ErrGachaTotalTimesLimit });
        return;
    }

    const currency_id = inventory_helper.gachaCurrencyItemId(request.GachaId, view.Type);
    const costs = [_]ConsumeHelper.CostItem{.{ .id = currency_id, .count = consume_count }};
    if (!(try ConsumeHelper.hasCosts(alloc.arena, basic_comp, inventory_comp, &costs))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    // Pre-roll against copies of bucket & banner so pool config errors abort cleanly before spending currency.
    var next_bucket = bucket.*;
    var next_banner = banner.*;
    var rolled: std.ArrayList(RolledPull) = .empty;
    try rolled.ensureTotalCapacity(alloc.arena, @intCast(request.GachaTimes));

    for (0..@intCast(request.GachaTimes)) |_| {
        const rarity = try determineRarity(io, &next_bucket, request.GachaId, view.Type);
        const reward_id = chooseReward(io, alloc.arena, assets, view, &next_banner, &next_bucket, rarity) catch {
            txn.respond(.{ .ErrorCode = .ErrGachaPoolConfigNotFound });
            return;
        };
        updatePity(&next_bucket, &next_banner, rarity);
        rolled.appendAssumeCapacity(.{ .rarity = rarity, .reward_id = reward_id });
    }

    if (!(try ConsumeHelper.consumeCosts(txn.conn, alloc, fs, basic_comp, inventory_comp, &costs, io))) {
        txn.respond(.{ .ErrorCode = .ErrItemNotEnough });
        return;
    }

    bucket.* = next_bucket;
    banner.* = next_banner;

    var results: std.ArrayList(pb.GachaResult) = .empty;
    var added_weapons: std.ArrayList(pb.WeaponItem) = .empty;
    var starter_weapons: std.ArrayList(pb.WeaponItem) = .empty;
    var added_items: std.ArrayList(pb.NormalItem) = .empty;
    var updated_items: std.ArrayList(pb.NormalItem) = .empty;
    var has_inventory_updates = false;
    var has_new_roles = false;

    for (rolled.items) |pull| {
        const reward_id = pull.reward_id;
        var gacha_result: pb.GachaResult = .{
            .GachaReward = .{ .ItemId = reward_id, .ItemCount = 1 },
        };

        if (assets.tables.weapon_conf.getDataById(reward_id) != null) {
            const incr_id = try special_item_incr.next(alloc.gpa, fs, weapon_comp.player_id);
            const weapon: WeaponItem = .{
                .id = reward_id,
                .func_value = WeaponItem.defaultFuncValue(assets, reward_id),
            };
            try weapon_comp.weapon_map.put(alloc.gpa, incr_id, weapon);
            try saveWeaponInfo(alloc, fs, weapon_comp, incr_id, weapon);
            try added_weapons.append(alloc.arena, weapon.toProto(incr_id));

            const quality = itemQuality(assets, reward_id);
            if (quality >= 5) {
                try grantBonusItem(alloc, inventory_comp, &gacha_result.ExtraRewards, &added_items, &updated_items, item_id_afterglow_coral, 15);
            } else if (quality == 4) {
                try grantBonusItem(alloc, inventory_comp, &gacha_result.ExtraRewards, &added_items, &updated_items, item_id_afterglow_coral, 3);
            } else {
                try grantBonusItem(alloc, inventory_comp, &gacha_result.ExtraRewards, &added_items, &updated_items, item_id_oscillated_coral, 15);
            }
            has_inventory_updates = true;
        } else if (assets.tables.role_info.getDataById(reward_id) != null) {
            if (role_comp.role_map.get(reward_id)) |existing_role| {
                const is_five_star = itemQuality(assets, reward_id) >= 5;
                const waveband_opt = roleWavebandItem(assets, reward_id);
                const owned_wavebands = if (waveband_opt) |wb_id| inventory_comp.info.normalItemCount(wb_id) else 0;
                const max_chain = maxResonantChainIndex(assets, reward_id);

                if (existing_role.resonant_chain_group_index + owned_wavebands < max_chain) {
                    if (waveband_opt) |waveband_id| {
                        try grantBonusItem(alloc, inventory_comp, &gacha_result.TransformRewards, &added_items, &updated_items, waveband_id, 1);
                        has_inventory_updates = true;
                    }
                    const dup_coral: i32 = if (is_five_star) 15 else 3;
                    try grantBonusItem(alloc, inventory_comp, &gacha_result.TransformRewards, &added_items, &updated_items, item_id_afterglow_coral, dup_coral);
                    has_inventory_updates = true;
                } else {
                    const overflow_coral: i32 = if (is_five_star) 45 else 8;
                    try grantBonusItem(alloc, inventory_comp, &gacha_result.TransformRewards, &added_items, &updated_items, item_id_afterglow_coral, overflow_coral);
                    has_inventory_updates = true;
                }
            } else {
                const new_role = try RoleInfo.createDefault(alloc.gpa, assets, reward_id);
                try role_comp.role_map.put(alloc.gpa, reward_id, new_role);
                const role_path = try std.fmt.allocPrint(alloc.arena, "player/{}/role/{}", .{ role_comp.player_id, reward_id });
                try comp_util.saveStruct(fs, new_role, role_path, alloc.arena);
                if (RoleHelper.ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, reward_id) catch false) {
                    if (role_comp.role_map.get(reward_id)) |rinfo| {
                        if (weapon_comp.weapon_map.get(rinfo.weapon)) |witem| {
                            try starter_weapons.append(alloc.arena, witem.toProto(rinfo.weapon));
                        }
                    }
                }
                has_new_roles = true;
            }
        } else {
            const old_cnt = inventory_comp.info.normalItemCount(reward_id);
            try InventoryInfo.addNormalItem(&inventory_comp.info, alloc.gpa, reward_id, 1);
            try recordUpdatedNormalItem(alloc.arena, &added_items, &updated_items, old_cnt, inventory_comp, reward_id);
            has_inventory_updates = true;
        }

        try results.append(alloc.arena, gacha_result);
    }

    try saveGachaInfo(alloc, fs, gacha_comp);

    if (added_weapons.items.len > 0) {
        try txn.conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
            .WeaponItemList = added_weapons,
            .AddFromRole = false,
            .Reason = 0,
        });
    }

    if (starter_weapons.items.len > 0) {
        try txn.conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
            .WeaponItemList = starter_weapons,
            .AddFromRole = true,
            .Reason = 0,
        });
    }

    if (has_inventory_updates) {
        try saveInventoryInfo(alloc, fs, inventory_comp);
        if (added_items.items.len > 0) {
            try txn.conn.push(alloc.gpa, io, pb.NormalItemAddNotify{
                .NormalItemList = added_items,
                .NoTips = false,
            });
        }
        if (updated_items.items.len > 0) {
            try txn.conn.push(alloc.gpa, io, pb.NormalItemUpdateNotify{
                .NormalItemList = updated_items,
                .NoTips = false,
            });
        }
    }

    if (has_new_roles) {
        try pushUpdatedRoleList(txn.conn, alloc, assets, role_comp, basic_comp, weapon_comp, echo_comp, io);
    }

    txn.respond(.{
        .ErrorCode = .Success,
        .GachaResults = results,
    });
}
