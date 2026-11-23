const QuestInfo = @This();
const std = @import("std");

const Allocator = std.mem.Allocator;

pub const default: QuestInfo = .{};
pub const data_path = "quest_info";

pub const ActiveQuest = struct {
    quest_id: i32,
    active_nodes: []i32 = &.{},
    finished_nodes: []i32 = &.{},
    killed_entities: []i32 = &.{},
};

pub const QuestVar = struct {
    key: []const u8,
    int_val: i32 = 0,
    bool_val: bool = false,
    str_val: []const u8 = "",
};

pub const ChildQuestRecord = struct {
    quest_id: i32,
    node_id: i32,
};

pub const EntityStateRecord = struct {
    entity_id: i64,
    state_name: []const u8 = "",
};

pub const AreaStateRecord = struct {
    area_id: i32,
    state: bool = false,
};

pub const AreaTimeRecord = struct {
    area_id: i32,
    hour: i32 = 12,
    minute: i32 = 0,
    locked: bool = false,
};

pub const AudioStateRecord = struct {
    group: []const u8 = "",
    state: []const u8 = "",
};

pub const QuestBranchRecord = struct {
    page_id: i32,
    cur_branch: i32 = 0,
    complete_branches: []i32 = &.{},
};

pub const HiddenGroupRecord = struct {
    group_key: []const u8,
    entity_ids: []i32 = &.{},
    data_layers: []i32 = &.{},
    level_plays: []i32 = &.{},
};

pub const WorldState = struct {
    loaded_datalayers: []i32 = &.{},
    unloaded_datalayers: []i32 = &.{},
    loaded_sublevels: [][]const u8 = &.{},
    hidden_sublevels: [][]const u8 = &.{},
    hidden_groups: []HiddenGroupRecord = &.{},
    hidden_entities: []i32 = &.{},
    shown_entities: []i32 = &.{},
    locked_entities: []i32 = &.{},
    unlocked_map_marks: []i32 = &.{},
    hidden_map_marks: []i32 = &.{},
    entity_states: []EntityStateRecord = &.{},
    active_guests: []i32 = &.{},
};

pub const EnvState = struct {
    area_states: []AreaStateRecord = &.{},
    area_times: []AreaTimeRecord = &.{},
    audio_states: []AudioStateRecord = &.{},
    weather_id: i32 = 0,
    weather_locked: bool = false,
    unlocked_weather_switches: []i32 = &.{},
    time_locked: bool = false,
    time_lock_hour: i32 = 8,
    time_lock_minute: i32 = 0,
    trial_roles: []i32 = &.{},
    pre_trial_roles: []i32 = &.{},
    pre_trial_cur_role: i32 = 0,
};

pub const DailyState = struct {
    daily_quests: []i32 = &.{},
    claimed_daily_rewards: []i32 = &.{},
    preferred_daily_area: i32 = 1,
    preferred_daily_influence: i32 = 0,
};

pub const ProgressState = struct {
    focus_deter_cond: bool = false,
    quest_branches: []QuestBranchRecord = &.{},
    completed_bvb: []i32 = &.{},
    read_mails: []i32 = &.{},
    read_phone_messages: []i32 = &.{},
    received_telecoms: []i32 = &.{},
    opened_activities: []i32 = &.{},
    completed_photo_nodes: []ChildQuestRecord = &.{},
    entered_level_plays: []i32 = &.{},
};

finished_quests: []i32 = &.{},
finished_child_quests: []ChildQuestRecord = &.{},
active_quests: []ActiveQuest = &.{},
tracked_quest_id: i32 = 139000025,
focus_quest_id: i32 = 0,
vars: []QuestVar = &.{},
unlocked_systems: []i32 = &.{},
completed_dungeons: []i32 = &.{},
completed_level_plays: []i32 = &.{},
opened_chests: []i32 = &.{},
next_flow_inc_id: i64 = 100,
// Grouped to stay well below Zig 0.16 ZON parser comptime eval branch limits.
world: WorldState = .{},
env: EnvState = .{},
daily: DailyState = .{},
progress: ProgressState = .{},

pub fn hasFinishedQuest(info: QuestInfo, quest_id: i32) bool {
    return containsInt(info.finished_quests, quest_id);
}

pub fn hasFinishedChildQuest(info: QuestInfo, quest_id: i32, node_id: i32) bool {
    for (info.finished_child_quests) |rec| {
        if (rec.quest_id == quest_id and rec.node_id == node_id) return true;
    }
    if (info.findActiveQuest(quest_id)) |aq| {
        return containsInt(aq.finished_nodes, node_id);
    }
    return info.hasFinishedQuest(quest_id);
}

pub fn hasActiveQuest(info: QuestInfo, quest_id: i32) bool {
    return info.findActiveQuest(quest_id) != null;
}

pub fn findActiveQuest(info: QuestInfo, quest_id: i32) ?ActiveQuest {
    for (info.active_quests) |aq| {
        if (aq.quest_id == quest_id) return aq;
    }
    return null;
}

pub fn findActiveQuestPtr(info: *QuestInfo, quest_id: i32) ?*ActiveQuest {
    for (info.active_quests) |*aq| {
        if (aq.quest_id == quest_id) return aq;
    }
    return null;
}

pub fn getOrAddActiveQuest(info: *QuestInfo, gpa: Allocator, quest_id: i32) !*ActiveQuest {
    if (info.findActiveQuestPtr(quest_id)) |ptr| return ptr;

    const grown = try gpa.alloc(ActiveQuest, info.active_quests.len + 1);
    @memcpy(grown[0..info.active_quests.len], info.active_quests);
    grown[info.active_quests.len] = .{ .quest_id = quest_id };
    if (info.active_quests.len != 0) gpa.free(info.active_quests);
    info.active_quests = grown;
    return &info.active_quests[info.active_quests.len - 1];
}

pub fn finishQuest(info: *QuestInfo, gpa: Allocator, quest_id: i32) !void {
    if (!containsInt(info.finished_quests, quest_id)) {
        info.finished_quests = try appendInt(info.finished_quests, gpa, quest_id);
    }

    if (info.findActiveQuest(quest_id)) |aq| {
        for (aq.finished_nodes) |nid| {
            try info.recordChildQuestFinished(gpa, quest_id, nid);
        }
        try info.removeActiveQuest(gpa, quest_id);
    }
}

pub fn recordChildQuestFinished(info: *QuestInfo, gpa: Allocator, quest_id: i32, node_id: i32) !void {
    for (info.finished_child_quests) |rec| {
        if (rec.quest_id == quest_id and rec.node_id == node_id) return;
    }
    const grown = try gpa.alloc(ChildQuestRecord, info.finished_child_quests.len + 1);
    @memcpy(grown[0..info.finished_child_quests.len], info.finished_child_quests);
    grown[info.finished_child_quests.len] = .{ .quest_id = quest_id, .node_id = node_id };
    if (info.finished_child_quests.len != 0) gpa.free(info.finished_child_quests);
    info.finished_child_quests = grown;
}

pub fn removeActiveQuest(info: *QuestInfo, gpa: Allocator, quest_id: i32) !void {
    for (info.active_quests, 0..) |aq, idx| {
        if (aq.quest_id != quest_id) continue;
        if (aq.active_nodes.len != 0) gpa.free(aq.active_nodes);
        if (aq.finished_nodes.len != 0) gpa.free(aq.finished_nodes);
        if (aq.killed_entities.len != 0) gpa.free(aq.killed_entities);

        if (info.active_quests.len == 1) {
            gpa.free(info.active_quests);
            info.active_quests = &.{};
            return;
        }
        const shrunk = try gpa.alloc(ActiveQuest, info.active_quests.len - 1);
        @memcpy(shrunk[0..idx], info.active_quests[0..idx]);
        @memcpy(shrunk[idx..], info.active_quests[idx + 1 ..]);
        gpa.free(info.active_quests);
        info.active_quests = shrunk;
        return;
    }
}

pub fn isNodeActive(aq: ActiveQuest, node_id: i32) bool {
    return containsInt(aq.active_nodes, node_id);
}

pub fn isNodeFinished(aq: ActiveQuest, node_id: i32) bool {
    return containsInt(aq.finished_nodes, node_id);
}

pub fn markNodeActive(aq: *ActiveQuest, gpa: Allocator, node_id: i32) !bool {
    if (containsInt(aq.active_nodes, node_id)) return false;
    aq.active_nodes = try appendInt(aq.active_nodes, gpa, node_id);
    return true;
}

pub fn markNodeInactive(aq: *ActiveQuest, gpa: Allocator, node_id: i32) !bool {
    const new_slice = try removeInt(aq.active_nodes, gpa, node_id) orelse return false;
    aq.active_nodes = new_slice;
    return true;
}

pub fn markNodeFinished(info: *QuestInfo, aq: *ActiveQuest, gpa: Allocator, node_id: i32) !bool {
    _ = try markNodeInactive(aq, gpa, node_id);
    if (containsInt(aq.finished_nodes, node_id)) return false;
    aq.finished_nodes = try appendInt(aq.finished_nodes, gpa, node_id);
    try info.recordChildQuestFinished(gpa, aq.quest_id, node_id);
    return true;
}

pub fn recordKilledEntity(aq: *ActiveQuest, gpa: Allocator, entity_id: i32) !bool {
    if (containsInt(aq.killed_entities, entity_id)) return false;
    aq.killed_entities = try appendInt(aq.killed_entities, gpa, entity_id);
    return true;
}

pub fn hasKilledEntity(aq: ActiveQuest, entity_id: i32) bool {
    return containsInt(aq.killed_entities, entity_id);
}

pub fn setVarInt(info: *QuestInfo, gpa: Allocator, key: []const u8, val: i32) !void {
    for (info.vars) |*v| {
        if (std.mem.eql(u8, v.key, key)) {
            v.int_val = val;
            return;
        }
    }
    const grown = try gpa.alloc(QuestVar, info.vars.len + 1);
    @memcpy(grown[0..info.vars.len], info.vars);
    grown[info.vars.len] = .{ .key = try gpa.dupe(u8, key), .int_val = val };
    if (info.vars.len != 0) gpa.free(info.vars);
    info.vars = grown;
}

pub fn hasVar(info: QuestInfo, key: []const u8) bool {
    for (info.vars) |v| {
        if (std.mem.eql(u8, v.key, key)) return true;
    }
    return false;
}

pub fn getVarInt(info: QuestInfo, key: []const u8) i32 {
    for (info.vars) |v| {
        if (std.mem.eql(u8, v.key, key)) return v.int_val;
    }
    return 0;
}

pub fn setVarBool(info: *QuestInfo, gpa: Allocator, key: []const u8, val: bool) !void {
    for (info.vars) |*v| {
        if (std.mem.eql(u8, v.key, key)) {
            v.bool_val = val;
            return;
        }
    }
    const grown = try gpa.alloc(QuestVar, info.vars.len + 1);
    @memcpy(grown[0..info.vars.len], info.vars);
    grown[info.vars.len] = .{ .key = try gpa.dupe(u8, key), .bool_val = val };
    if (info.vars.len != 0) gpa.free(info.vars);
    info.vars = grown;
}

pub fn getVarBool(info: QuestInfo, key: []const u8) bool {
    for (info.vars) |v| {
        if (std.mem.eql(u8, v.key, key)) return v.bool_val;
    }
    return false;
}

pub fn setVarStr(info: *QuestInfo, gpa: Allocator, key: []const u8, val: []const u8) !void {
    for (info.vars) |*v| {
        if (std.mem.eql(u8, v.key, key)) {
            if (v.str_val.len != 0) gpa.free(v.str_val);
            v.str_val = if (val.len != 0) try gpa.dupe(u8, val) else "";
            return;
        }
    }
    const grown = try gpa.alloc(QuestVar, info.vars.len + 1);
    @memcpy(grown[0..info.vars.len], info.vars);
    grown[info.vars.len] = .{
        .key = try gpa.dupe(u8, key),
        .str_val = if (val.len != 0) try gpa.dupe(u8, val) else "",
    };
    if (info.vars.len != 0) gpa.free(info.vars);
    info.vars = grown;
}

pub fn getVarStr(info: QuestInfo, key: []const u8) []const u8 {
    for (info.vars) |v| {
        if (std.mem.eql(u8, v.key, key)) return v.str_val;
    }
    return "";
}

pub fn setEntityState(info: *QuestInfo, gpa: Allocator, entity_id: i64, state_name: []const u8) !void {
    for (info.world.entity_states) |*rec| {
        if (rec.entity_id == entity_id) {
            if (rec.state_name.len != 0) gpa.free(rec.state_name);
            rec.state_name = if (state_name.len != 0) try gpa.dupe(u8, state_name) else "";
            return;
        }
    }
    _ = try appendItem(EntityStateRecord, &info.world.entity_states, gpa, .{
        .entity_id = entity_id,
        .state_name = if (state_name.len != 0) try gpa.dupe(u8, state_name) else "",
    });
}

pub fn getEntityState(info: QuestInfo, entity_id: i64) ?[]const u8 {
    for (info.world.entity_states) |rec| {
        if (rec.entity_id == entity_id) return rec.state_name;
    }
    return null;
}

pub fn setAreaState(info: *QuestInfo, gpa: Allocator, area_id: i32, state: bool) !void {
    for (info.env.area_states) |*rec| {
        if (rec.area_id == area_id) {
            rec.state = state;
            return;
        }
    }
    _ = try appendItem(AreaStateRecord, &info.env.area_states, gpa, .{ .area_id = area_id, .state = state });
}

pub fn setAreaTime(info: *QuestInfo, gpa: Allocator, area_id: i32, hour: i32, minute: i32, locked: bool) !void {
    for (info.env.area_times) |*rec| {
        if (rec.area_id == area_id) {
            rec.hour = hour;
            rec.minute = minute;
            rec.locked = locked;
            return;
        }
    }
    _ = try appendItem(AreaTimeRecord, &info.env.area_times, gpa, .{ .area_id = area_id, .hour = hour, .minute = minute, .locked = locked });
}

pub fn selectQuestBranch(info: *QuestInfo, gpa: Allocator, page_id: i32, branch_id: i32) !*QuestBranchRecord {
    for (info.progress.quest_branches) |*rec| {
        if (rec.page_id == page_id) {
            rec.cur_branch = branch_id;
            _ = try appendUniqueInt(&rec.complete_branches, gpa, branch_id);
            return rec;
        }
    }
    var init_done: []i32 = &.{};
    _ = try appendUniqueInt(&init_done, gpa, branch_id);
    return try appendItem(QuestBranchRecord, &info.progress.quest_branches, gpa, .{
        .page_id = page_id,
        .cur_branch = branch_id,
        .complete_branches = init_done,
    });
}

pub fn markPhotoNodeCompleted(info: *QuestInfo, gpa: Allocator, quest_id: i32, node_id: i32) !bool {
    for (info.progress.completed_photo_nodes) |rec| {
        if (rec.quest_id == quest_id and rec.node_id == node_id) return false;
    }
    _ = try appendItem(ChildQuestRecord, &info.progress.completed_photo_nodes, gpa, .{ .quest_id = quest_id, .node_id = node_id });
    return true;
}

pub fn hasPhotoNodeCompleted(info: QuestInfo, quest_id: i32, node_id: i32) bool {
    for (info.progress.completed_photo_nodes) |rec| {
        if (rec.quest_id == quest_id and rec.node_id == node_id) return true;
    }
    return false;
}

pub fn setHiddenGroup(
    info: *QuestInfo,
    gpa: Allocator,
    group_key: []const u8,
    entity_ids: []const i32,
    data_layers: []const i32,
    level_plays: []const i32,
) !void {
    if (group_key.len == 0) return;
    if (popHiddenGroup(info, gpa, group_key)) |old| {
        freeHiddenGroupRecord(old, gpa);
    }
    const owned_key = try gpa.dupe(u8, group_key);
    errdefer gpa.free(owned_key);
    const owned_entities = if (entity_ids.len != 0) try gpa.dupe(i32, entity_ids) else @as([]i32, &.{});
    errdefer if (owned_entities.len != 0) gpa.free(owned_entities);
    const owned_layers = if (data_layers.len != 0) try gpa.dupe(i32, data_layers) else @as([]i32, &.{});
    errdefer if (owned_layers.len != 0) gpa.free(owned_layers);
    const owned_lps = if (level_plays.len != 0) try gpa.dupe(i32, level_plays) else @as([]i32, &.{});
    errdefer if (owned_lps.len != 0) gpa.free(owned_lps);

    _ = try appendItem(HiddenGroupRecord, &info.world.hidden_groups, gpa, .{
        .group_key = owned_key,
        .entity_ids = owned_entities,
        .data_layers = owned_layers,
        .level_plays = owned_lps,
    });
}

pub fn popHiddenGroup(info: *QuestInfo, gpa: Allocator, group_key: []const u8) ?HiddenGroupRecord {
    const groups = info.world.hidden_groups;
    for (groups, 0..) |rec, idx| {
        if (!std.mem.eql(u8, rec.group_key, group_key)) continue;
        const removed = rec;
        if (groups.len == 1) {
            gpa.free(groups);
            info.world.hidden_groups = &.{};
            return removed;
        }
        if (idx + 1 < groups.len) {
            std.mem.copyForwards(HiddenGroupRecord, groups[idx .. groups.len - 1], groups[idx + 1 ..]);
        }
        if (gpa.realloc(groups, groups.len - 1)) |shrunk| {
            info.world.hidden_groups = shrunk;
        } else |_| {
            groups[groups.len - 1] = .{ .group_key = "" };
        }
        return removed;
    }
    return null;
}

pub fn freeHiddenGroupRecord(rec: HiddenGroupRecord, gpa: Allocator) void {
    if (rec.group_key.len != 0) gpa.free(rec.group_key);
    if (rec.entity_ids.len != 0) gpa.free(rec.entity_ids);
    if (rec.data_layers.len != 0) gpa.free(rec.data_layers);
    if (rec.level_plays.len != 0) gpa.free(rec.level_plays);
}

pub fn markDungeonCompleted(info: *QuestInfo, gpa: Allocator, dungeon_id: i32) !void {
    if (dungeon_id > 0) _ = try appendUniqueInt(&info.completed_dungeons, gpa, dungeon_id);
}

pub fn markLevelPlayCompleted(info: *QuestInfo, gpa: Allocator, level_play_id: i32) !void {
    if (level_play_id > 0) _ = try appendUniqueInt(&info.completed_level_plays, gpa, level_play_id);
}

pub fn setAudioState(info: *QuestInfo, gpa: Allocator, group: []const u8, state: []const u8) !void {
    if (group.len == 0) return;
    for (info.env.audio_states) |*rec| {
        if (!std.mem.eql(u8, rec.group, group)) continue;
        const next_state = try gpa.dupe(u8, state);
        if (rec.state.len != 0) gpa.free(rec.state);
        rec.state = next_state;
        return;
    }
    const owned_group = try gpa.dupe(u8, group);
    errdefer if (owned_group.len != 0) gpa.free(owned_group);
    const owned_state = try gpa.dupe(u8, state);
    errdefer if (owned_state.len != 0) gpa.free(owned_state);
    _ = try appendItem(AudioStateRecord, &info.env.audio_states, gpa, .{
        .group = owned_group,
        .state = owned_state,
    });
}

pub fn hasUnlockedSystem(info: QuestInfo, system_id: i32) bool {
    return containsInt(info.unlocked_systems, system_id);
}

pub fn unlockSystem(info: *QuestInfo, gpa: Allocator, system_id: i32) !bool {
    if (system_id <= 0) return false;
    return try appendUniqueInt(&info.unlocked_systems, gpa, system_id);
}

pub fn hasOpenedChest(info: QuestInfo, entity_config_id: i32) bool {
    return containsInt(info.opened_chests, entity_config_id);
}

pub fn markChestOpened(info: *QuestInfo, gpa: Allocator, entity_config_id: i32) !bool {
    if (entity_config_id <= 0) return false;
    return try appendUniqueInt(&info.opened_chests, gpa, entity_config_id);
}

pub fn allocFlowIncId(info: *QuestInfo) i64 {
    const id = info.next_flow_inc_id;
    info.next_flow_inc_id += 1;
    return id;
}

pub fn containsInt(ids: []const i32, needle: i32) bool {
    for (ids) |id| {
        if (id == needle) return true;
    }
    return false;
}

pub fn containsStr(items: []const []const u8, needle: []const u8) bool {
    for (items) |item| {
        if (std.mem.eql(u8, item, needle)) return true;
    }
    return false;
}

pub fn appendUniqueInt(slice: *[]i32, gpa: Allocator, val: i32) !bool {
    if (containsInt(slice.*, val)) return false;
    slice.* = try appendInt(slice.*, gpa, val);
    return true;
}

pub fn removeIntValue(slice: *[]i32, gpa: Allocator, val: i32) !bool {
    const new_slice = try removeInt(slice.*, gpa, val) orelse return false;
    slice.* = new_slice;
    return true;
}

pub fn appendUniqueStr(slice: *[][]const u8, gpa: Allocator, val: []const u8) !bool {
    if (val.len == 0 or containsStr(slice.*, val)) return false;
    const grown = try gpa.alloc([]const u8, slice.len + 1);
    @memcpy(grown[0..slice.len], slice.*);
    grown[slice.len] = try gpa.dupe(u8, val);
    if (slice.len != 0) gpa.free(slice.*);
    slice.* = grown;
    return true;
}

pub fn removeStrValue(slice: *[][]const u8, gpa: Allocator, val: []const u8) !bool {
    for (slice.*, 0..) |existing, idx| {
        if (!std.mem.eql(u8, existing, val)) continue;
        if (existing.len != 0) gpa.free(existing);
        if (slice.len == 1) {
            gpa.free(slice.*);
            slice.* = &.{};
            return true;
        }
        const shrunk = try gpa.alloc([]const u8, slice.len - 1);
        @memcpy(shrunk[0..idx], slice.*[0..idx]);
        @memcpy(shrunk[idx..], slice.*[idx + 1 ..]);
        gpa.free(slice.*);
        slice.* = shrunk;
        return true;
    }
    return false;
}

pub fn appendItem(comptime T: type, slice: *[]T, gpa: Allocator, val: T) !*T {
    const grown = try gpa.alloc(T, slice.len + 1);
    @memcpy(grown[0..slice.len], slice.*);
    grown[slice.len] = val;
    if (slice.len != 0) gpa.free(slice.*);
    slice.* = grown;
    return &slice.*[slice.len - 1];
}

fn appendInt(slice: []i32, gpa: Allocator, val: i32) ![]i32 {
    const grown = try gpa.alloc(i32, slice.len + 1);
    @memcpy(grown[0..slice.len], slice);
    grown[slice.len] = val;
    if (slice.len != 0) gpa.free(slice);
    return grown;
}

fn removeInt(slice: []i32, gpa: Allocator, val: i32) !?[]i32 {
    for (slice, 0..) |existing, idx| {
        if (existing != val) continue;
        if (slice.len == 1) {
            gpa.free(slice);
            return @as([]i32, &.{});
        }
        const shrunk = try gpa.alloc(i32, slice.len - 1);
        @memcpy(shrunk[0..idx], slice[0..idx]);
        @memcpy(shrunk[idx..], slice[idx + 1 ..]);
        gpa.free(slice);
        return shrunk;
    }
    return null;
}

pub fn deinit(info: QuestInfo, gpa: Allocator) void {
    std.zon.parse.free(gpa, info);
}

test "QuestInfo parses legacy ZON without new fields and frees mutated slices cleanly" {
    const gpa = std.testing.allocator;
    const legacy_zon: [:0]const u8 =
        \\.{
        \\    .finished_quests = .{ 139000025 },
        \\    .active_quests = .{},
        \\    .tracked_quest_id = 139000025,
        \\}
    ;
    var diagnostics: std.zon.parse.Diagnostics = .{};
    defer diagnostics.deinit(gpa);
    var info = try std.zon.parse.fromSliceAlloc(QuestInfo, gpa, legacy_zon, &diagnostics, .{});
    defer info.deinit(gpa);

    try std.testing.expect(info.hasFinishedQuest(139000025));
    try std.testing.expectEqual(@as(i32, 0), info.env.weather_id);
    try std.testing.expectEqual(@as(usize, 0), info.world.loaded_datalayers.len);

    try info.setVarStr(gpa, "dialog_choice", "A");
    try info.setVarStr(gpa, "dialog_choice", "B");
    try std.testing.expectEqualStrings("B", info.getVarStr("dialog_choice"));

    try info.setEntityState(gpa, 122006411, "关卡.Common.状态.常态");
    try std.testing.expectEqualStrings("关卡.Common.状态.常态", info.getEntityState(122006411).?);

    try info.setHiddenGroup(gpa, "q_125_a", &.{ 101, 102 }, &.{3}, &.{});
    try std.testing.expectEqual(@as(usize, 1), info.world.hidden_groups.len);
    const popped = info.popHiddenGroup(gpa, "q_125_a").?;
    defer QuestInfo.freeHiddenGroupRecord(popped, gpa);
    try std.testing.expectEqual(@as(usize, 0), info.world.hidden_groups.len);

    _ = try info.selectQuestBranch(gpa, 1000, 1);
    try std.testing.expectEqual(@as(i32, 1), info.progress.quest_branches[0].cur_branch);
    try info.setAreaState(gpa, 25, true);
    try info.setAreaTime(gpa, 25, 17, 40, true);
    try std.testing.expect(try info.markPhotoNodeCompleted(gpa, 1, 2));
}
