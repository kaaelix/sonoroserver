const std = @import("std");
const pb = @import("proto").pb;
const Assets = @import("../../data/Assets.zig");
const FileSystem = @import("common").FileSystem;
const Connection = @import("../../network/Connection.zig");
const State = @import("../../network/State.zig");
const EventQueue = @import("../EventQueue.zig");
const Scene = @import("../Scene.zig");
const RoleInfo = @import("../../fs/RoleInfo.zig");
const WeaponItem = @import("../../fs/WeaponItem.zig");
const PlayerBasicComponent = @import("../component/player/PlayerBasicComponent.zig");
const PlayerRoleComponent = @import("../component/player/PlayerRoleComponent.zig");
const PlayerWeaponComponent = @import("../component/player/PlayerWeaponComponent.zig");
const PlayerEchoComponent = @import("../component/player/PlayerEchoComponent.zig");
const PlayerMapComponent = @import("../component/player/PlayerMapComponent.zig");
const PlayerQuestComponent = @import("../component/player/PlayerQuestComponent.zig");
const comp_util = @import("../component/comp_util.zig");
const mem = @import("../../mem.zig");
const respond = @import("../commands.zig").respond;
const RoleStats = @import("../helpers/role_stats.zig");
const RoleHelper = @import("../helpers/role.zig");
const QuestEngine = @import("../helpers/quest_engine.zig");
const QuestActions = @import("../helpers/quest_actions.zig");
const RewardService = @import("../helpers/reward.zig");
const RoleAttributeSync = @import("../../network/helpers/role_attribute_sync.zig");

const ConnProxy = struct { conn: *Connection };

fn pushAllOwnedRoles(
    conn: *Connection,
    alloc: mem.Alloc,
    assets: *const Assets,
    basic_comp: *PlayerBasicComponent,
    role_comp: *PlayerRoleComponent,
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
    var it = role_comp.role_map.iterator();
    while (it.next()) |kv| {
        const rid = kv.key_ptr.*;
        if (rid > 1999 or rid < 1000) continue;
        if (selected_main_role) |selected| {
            if (RoleHelper.isMainRole(assets, rid) and rid != selected) continue;
        }
        notify.RoleList.appendAssumeCapacity(try RoleStats.toClientRoleInfo(
            alloc.gpa,
            alloc.arena,
            assets,
            role_comp,
            rid,
            kv.value_ptr,
            weapon_comp,
            echo_comp,
        ));
    }
    if (notify.RoleList.items.len > 0) {
        try conn.push(alloc.gpa, io, notify);
    }
}

fn saveRole(fs: *FileSystem, arena: std.mem.Allocator, player_id: i32, role_id: i32, role: RoleInfo) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/{s}/{}", .{ player_id, RoleInfo.data_dir, role_id });
    try comp_util.saveStruct(fs, role, path, arena);
}

fn saveBasic(fs: *FileSystem, arena: std.mem.Allocator, basic_comp: *const PlayerBasicComponent) !void {
    const path = try std.fmt.allocPrint(arena, "player/{}/basic_info", .{basic_comp.player_id});
    try comp_util.saveStruct(fs, basic_comp.info, path, arena);
}

fn maxResonantChainIndex(assets: *const Assets, group_id: i32) i32 {
    if (group_id <= 0) return 6;
    var max_idx: i32 = 0;
    for (assets.tables.resonant_chain.items) |chain| {
        if (chain.GroupId == group_id and chain.GroupIndex > max_idx) {
            max_idx = chain.GroupIndex;
        }
    }
    return if (max_idx > 0) max_idx else 6;
}

fn maxSkillLevelForId(assets: *const Assets, skill_id: i32) i32 {
    const sk = assets.tables.skill.getDataById(skill_id) orelse return 10;
    return if (sk.MaxSkillLevel > 0) sk.MaxSkillLevel else 10;
}

fn setRoleMaxSkillsAndChain(
    gpa: std.mem.Allocator,
    assets: *const Assets,
    role_id: i32,
    role: *RoleInfo,
    chain_opt: ?i32,
) !void {
    const rc = assets.tables.resolveBaseRoleInfo(role_id) orelse return;
    const max_chain = maxResonantChainIndex(assets, rc.ResonantChainGroupId);
    role.resonant_chain_group_index = if (chain_opt) |c| std.math.clamp(c, 0, max_chain) else max_chain;

    if (role.skill_node_state.len == 0) {
        try role.addDefaultSkills(gpa, assets, role_id);
    }
    for (role.skill_node_state) |*sn| {
        sn.active = true;
    }

    var skill_pairs: std.ArrayList(struct { i32, i32 }) = .empty;
    errdefer skill_pairs.deinit(gpa);
    for (assets.tables.skill_tree.items) |node| {
        if (node.NodeGroup != rc.SkillTreeGroupId or node.SkillId <= 0) continue;
        var duplicate = false;
        for (skill_pairs.items) |existing| {
            if (existing[0] == node.SkillId) {
                duplicate = true;
                break;
            }
        }
        if (!duplicate) {
            try skill_pairs.append(gpa, .{ node.SkillId, maxSkillLevelForId(assets, node.SkillId) });
        }
    }
    if (skill_pairs.items.len > 0) {
        if (role.skills.len != 0) gpa.free(role.skills);
        role.skills = try skill_pairs.toOwnedSlice(gpa);
    } else {
        skill_pairs.deinit(gpa);
        for (role.skills) |*s| {
            s[1] = maxSkillLevelForId(assets, s[0]);
        }
    }
}

fn syncRoleOnSceneOrNotify(
    events: *EventQueue,
    alloc: mem.Alloc,
    fs: *FileSystem,
    assets: *const Assets,
    conn: *Connection,
    scene_opt: *?Scene,
    role_comp: *PlayerRoleComponent,
    weapon_comp: *PlayerWeaponComponent,
    echo_comp: *PlayerEchoComponent,
    role_id: i32,
    io: std.Io,
) !void {
    const proxy: ConnProxy = .{ .conn = conn };
    try events.enqueue(.role_info_modified, .{ .role_id = role_id });
    if (scene_opt.*) |*scene| {
        const StatQuery = Scene.Query(&.{ Scene.Entity, *Scene.Entity.ConfigComponent, *Scene.Entity.AttributeComponent });
        const q: StatQuery = .{ .iterator = .{ .entities = &scene.entities, .net_id_map = &scene.net_id_map } };
        try RoleAttributeSync.refreshRole(proxy, alloc, fs, scene, assets, role_comp, weapon_comp, echo_comp, q, role_id, io);
    } else {
        try RoleAttributeSync.pushRoleInfoNotify(proxy, alloc, assets, role_comp, weapon_comp, echo_comp, role_id, io);
    }
}

pub const helpers = struct {
    pub fn breachForLevel(lv: i32) i32 {
        if (lv > 80) return 6;
        if (lv > 70) return 5;
        if (lv > 60) return 4;
        if (lv > 50) return 3;
        if (lv > 40) return 2;
        if (lv > 20) return 1;
        return 0;
    }
};

fn breachForLevel(lv: i32) i32 {
    return helpers.breachForLevel(lv);
}

pub const world_level = struct {
    pub const alias = "wl";
    pub const description = "sets SOL3 Phase / World Level (1..8).\nusage: world_level [1..8]";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        conn: *Connection,
        basic_comp: *PlayerBasicComponent,
        alloc: mem.Alloc,
        io: std.Io,
        target_wl: i32,
    ) !void {
        const wl = std.math.clamp(target_wl, 1, 8);
        basic_comp.info.attributes.origin_world_level = wl;
        basic_comp.info.attributes.cur_world_level = wl;
        try saveBasic(fs, alloc.arena, basic_comp);

        var attrs: std.ArrayList(pb.PlayerAttr) = .empty;
        try attrs.append(alloc.arena, .{ .Key = .OriginWorldLevel, .ValueType = .Int32, .Value = .{ .Int32Value = wl } });
        try attrs.append(alloc.arena, .{ .Key = .CurWorldLevel, .ValueType = .Int32, .Value = .{ .Int32Value = wl } });
        try conn.push(alloc.gpa, io, pb.PlayerAttrNotify{ .Attributes = attrs });
        try respond(events, alloc.arena, "set SOL3 / World Level to {d}", .{wl});
    }
};

pub const unlock_roles = struct {
    pub const alias = "uroles";
    pub const description = "unlocks all playable resonators or one role by id.\nusage: unlock_roles [all|role_id?]";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        basic_comp: *PlayerBasicComponent,
        role_comp: *PlayerRoleComponent,
        weapon_comp: *PlayerWeaponComponent,
        echo_comp: *PlayerEchoComponent,
        alloc: mem.Alloc,
        io: std.Io,
        target_opt: ?[]const u8,
    ) !void {
        const token = target_opt orelse "all";
        var added: usize = 0;

        var starter_weapons: std.ArrayList(pb.WeaponItem) = .empty;

        if (!std.ascii.eqlIgnoreCase(token, "all")) {
            const raw_id = std.fmt.parseInt(i32, token, 10) catch {
                try respond(events, alloc.arena, "invalid role_id: {s}", .{token});
                return;
            };
            const rid = if (raw_id >= 10000000) raw_id - 10000000 else raw_id;
            if (assets.tables.role_info.getDataById(rid) == null) {
                try respond(events, alloc.arena, "unknown role id: {d}", .{rid});
                return;
            }
            const rc = assets.tables.role_info.getDataById(rid) orelse {
                try respond(events, alloc.arena, "unknown role id {d}", .{rid});
                return;
            };
            if (rc.IsTrial or rc.RoleType != 1 or rid < 1000 or rid > 1999) {
                try respond(events, alloc.arena, "role id {d} is not a playable character", .{rid});
                return;
            }
            if (role_comp.isTrialRole(rid)) {
                _ = role_comp.trial_roles.remove(rid);
            }
            if (RoleHelper.isMainRole(assets, rid)) {
                const target_main = RoleHelper.mainRoleIdForElement(assets, basic_comp.info.attributes.sex, rc.ElementId) orelse rid;
                const fallback_main: i32 = if (basic_comp.info.attributes.sex == 0) 1502 else 1501;
                const src_main = RoleHelper.selectedMainRoleId(assets, role_comp, basic_comp.info.attributes.sex, basic_comp.info.selected_main_role_id) orelse fallback_main;
                _ = try RoleHelper.transferMainRoleState(
                    alloc,
                    fs,
                    assets,
                    null,
                    basic_comp,
                    role_comp,
                    weapon_comp,
                    echo_comp,
                    src_main,
                    target_main,
                );
                try RoleHelper.saveBasicInfo(fs, alloc.arena, basic_comp);
                added = 1;
            } else if (!role_comp.role_map.contains(rid)) {
                const new_role = try RoleInfo.createDefault(alloc.gpa, assets, rid);
                try role_comp.role_map.put(alloc.gpa, rid, new_role);
                try saveRole(fs, alloc.arena, role_comp.player_id, rid, new_role);
                added = 1;
            }
            if (RoleHelper.ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, rid) catch false) {
                if (role_comp.role_map.get(rid)) |rinfo| {
                    if (weapon_comp.weapon_map.get(rinfo.weapon)) |witem| {
                        try starter_weapons.append(alloc.arena, witem.toProto(rinfo.weapon));
                    }
                }
            }
            if (starter_weapons.items.len > 0) {
                try conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
                    .WeaponItemList = starter_weapons,
                    .AddFromRole = true,
                    .Reason = 1,
                });
            }
            try pushAllOwnedRoles(conn, alloc, assets, basic_comp, role_comp, weapon_comp, echo_comp, io);
            try respond(events, alloc.arena, "unlocked role {d} (new={d})", .{ rid, added });
            return;
        }

        const selected_main = RoleHelper.selectedMainRoleId(
            assets,
            role_comp,
            basic_comp.info.attributes.sex,
            basic_comp.info.selected_main_role_id,
        );
        for (assets.tables.role_info.items) |rc| {
            if (rc.Id < 1000 or rc.Id > 1999) continue;
            if (rc.IsTrial or rc.RoleType != 1) continue;
            if (RoleHelper.isMainRole(assets, rc.Id)) {
                if (selected_main) |sel| {
                    if (rc.Id != sel) continue;
                } else if (!RoleHelper.isMainRoleForGender(assets, basic_comp.info.attributes.sex, rc.Id)) {
                    continue;
                }
            }
            if (role_comp.isTrialRole(rc.Id)) {
                _ = role_comp.trial_roles.remove(rc.Id);
            }
            if (!role_comp.role_map.contains(rc.Id)) {
                const new_role = RoleInfo.createDefault(alloc.gpa, assets, rc.Id) catch continue;
                try role_comp.role_map.put(alloc.gpa, rc.Id, new_role);
                try saveRole(fs, alloc.arena, role_comp.player_id, rc.Id, new_role);
                added += 1;
            }
            if (RoleHelper.ensureRoleWeapon(alloc, fs, assets, role_comp, weapon_comp, rc.Id) catch false) {
                if (role_comp.role_map.get(rc.Id)) |rinfo| {
                    if (weapon_comp.weapon_map.get(rinfo.weapon)) |witem| {
                        try starter_weapons.append(alloc.arena, witem.toProto(rinfo.weapon));
                    }
                }
            }
        }
        if (starter_weapons.items.len > 0) {
            try conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
                .WeaponItemList = starter_weapons,
                .AddFromRole = true,
                .Reason = 1,
            });
        }
        try pushAllOwnedRoles(conn, alloc, assets, basic_comp, role_comp, weapon_comp, echo_comp, io);
        try respond(events, alloc.arena, "unlocked {d} new resonator(s) (total owned={d})", .{ added, role_comp.role_map.count() });
    }
};

pub const role_max = struct {
    pub const alias = "rmax";
    pub const description = "maxes level, breakthrough, resonance chain S6, and skills.\nusage: role_max [current|team|all|role_id?] [level?]";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        scene_opt: *?Scene,
        role_comp: *PlayerRoleComponent,
        weapon_comp: *PlayerWeaponComponent,
        echo_comp: *PlayerEchoComponent,
        alloc: mem.Alloc,
        io: std.Io,
        subject_opt: ?[]const u8,
        lv_opt: ?i32,
    ) !void {
        const subject = subject_opt orelse "team";
        const lv = std.math.clamp(lv_opt orelse 90, 1, 90);
        const breach = breachForLevel(lv);
        var updated: usize = 0;

        if (std.ascii.eqlIgnoreCase(subject, "all")) {
            var it = role_comp.role_map.iterator();
            while (it.next()) |kv| {
                const rid = kv.key_ptr.*;
                if (rid < 1000 or rid > 1999) continue;
                kv.value_ptr.level = lv;
                kv.value_ptr.breakthrough = breach;
                kv.value_ptr.exp = 0;
                try setRoleMaxSkillsAndChain(alloc.gpa, assets, rid, kv.value_ptr, null);
                _ = try kv.value_ptr.refreshScaledProperties(alloc.gpa, assets, rid);
                try saveRole(fs, alloc.arena, role_comp.player_id, rid, kv.value_ptr.*);
                try syncRoleOnSceneOrNotify(events, alloc, fs, assets, conn, scene_opt, role_comp, weapon_comp, echo_comp, rid, io);
                updated += 1;
            }
        } else {
            var ids: [3]i32 = undefined;
            var count: usize = 0;
            if (std.ascii.eqlIgnoreCase(subject, "current")) {
                if (scene_opt.*) |*scene| {
                    if (scene.formation_info.cur_formation >= 0) {
                        const idx: usize = @intCast(scene.formation_info.cur_formation);
                        if (idx < scene.formation_info.formations.len and scene.formation_info.formations[idx].cur_role > 0) {
                            ids[0] = scene.formation_info.formations[idx].cur_role;
                            count = 1;
                        }
                    }
                }
            } else if (std.ascii.eqlIgnoreCase(subject, "team")) {
                if (scene_opt.*) |*scene| {
                    if (scene.formation_info.cur_formation >= 0) {
                        const idx: usize = @intCast(scene.formation_info.cur_formation);
                        if (idx < scene.formation_info.formations.len) {
                            for (scene.formation_info.formations[idx].roles) |mr| {
                                if (mr) |r| {
                                    ids[count] = r.role_id;
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            } else if (std.fmt.parseInt(i32, subject, 10)) |parsed| {
                ids[0] = if (parsed >= 10000000) parsed - 10000000 else parsed;
                count = 1;
            } else |_| {}

            for (ids[0..count]) |rid| {
                const role = role_comp.role_map.getPtr(rid) orelse continue;
                role.level = lv;
                role.breakthrough = breach;
                role.exp = 0;
                try setRoleMaxSkillsAndChain(alloc.gpa, assets, rid, role, null);
                _ = try role.refreshScaledProperties(alloc.gpa, assets, rid);
                try saveRole(fs, alloc.arena, role_comp.player_id, rid, role.*);
                try syncRoleOnSceneOrNotify(events, alloc, fs, assets, conn, scene_opt, role_comp, weapon_comp, echo_comp, rid, io);
                updated += 1;
            }
        }

        if (updated > 0) try events.enqueue(.update_formations, .{});
        try respond(events, alloc.arena, "maxxed {d} resonator(s) to Lv.{d} S6 + Max Skills", .{ updated, lv });
    }
};

pub const role_chain = struct {
    pub const alias = "s";
    pub const description = "sets resonance chain rank (0..6).\nusage: role_chain [current|team|all|role_id] [0..6]";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        scene_opt: *?Scene,
        role_comp: *PlayerRoleComponent,
        weapon_comp: *PlayerWeaponComponent,
        echo_comp: *PlayerEchoComponent,
        alloc: mem.Alloc,
        io: std.Io,
        subject: []const u8,
        rank: i32,
    ) !void {
        const clamped = std.math.clamp(rank, 0, 6);
        var updated: usize = 0;

        var it = role_comp.role_map.iterator();
        while (it.next()) |kv| {
            const rid = kv.key_ptr.*;
            if (rid < 1000 or rid > 1999) continue;
            var match = std.ascii.eqlIgnoreCase(subject, "all");
            if (!match and std.ascii.eqlIgnoreCase(subject, "current")) {
                if (scene_opt.*) |*scene| {
                    if (scene.formation_info.cur_formation >= 0) {
                        const idx: usize = @intCast(scene.formation_info.cur_formation);
                        if (idx < scene.formation_info.formations.len) {
                            match = scene.formation_info.formations[idx].cur_role == rid;
                        }
                    }
                }
            } else if (!match and std.ascii.eqlIgnoreCase(subject, "team")) {
                if (scene_opt.*) |*scene| {
                    if (scene.formation_info.cur_formation >= 0) {
                        const idx: usize = @intCast(scene.formation_info.cur_formation);
                        if (idx < scene.formation_info.formations.len) {
                            for (scene.formation_info.formations[idx].roles) |mr| {
                                if (mr) |r| if (r.role_id == rid) {
                                    match = true;
                                };
                            }
                        }
                    }
                }
            } else if (!match) {
                if (std.fmt.parseInt(i32, subject, 10)) |parsed| {
                    const normalized = if (parsed >= 10000000) parsed - 10000000 else parsed;
                    match = normalized == rid;
                } else |_| {}
            }
            if (!match) continue;

            kv.value_ptr.resonant_chain_group_index = clamped;
            try saveRole(fs, alloc.arena, role_comp.player_id, rid, kv.value_ptr.*);
            try syncRoleOnSceneOrNotify(events, alloc, fs, assets, conn, scene_opt, role_comp, weapon_comp, echo_comp, rid, io);
            updated += 1;
        }
        try respond(events, alloc.arena, "set Resonance Chain S{d} on {d} role(s)", .{ clamped, updated });
    }
};

pub const weapon_max = struct {
    pub const alias = "wmax";
    pub const description = "maxes equipped or all weapons to Lv.90 R5.\nusage: weapon_max [equipped|current|all] [level?] [rank?]";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        scene_opt: *?Scene,
        role_comp: *PlayerRoleComponent,
        weapon_comp: *PlayerWeaponComponent,
        echo_comp: *PlayerEchoComponent,
        alloc: mem.Alloc,
        io: std.Io,
        scope_opt: ?[]const u8,
        lv_opt: ?i32,
        rank_opt: ?i32,
    ) !void {
        const scope = scope_opt orelse "equipped";
        const lv = std.math.clamp(lv_opt orelse 90, 1, 90);
        const rank = std.math.clamp(rank_opt orelse 5, 1, 5);
        const breach = breachForLevel(lv);

        var updated_weapons: std.ArrayList(pb.WeaponItem) = .empty;
        var touched_roles: std.ArrayList(i32) = .empty;

        const cur_weapon_incr: i32 = blk: {
            if (scene_opt.*) |*scene| {
                if (scene.formation_info.cur_formation >= 0) {
                    const idx: usize = @intCast(scene.formation_info.cur_formation);
                    if (idx < scene.formation_info.formations.len) {
                        const rid = scene.formation_info.formations[idx].cur_role;
                        if (role_comp.role_map.get(rid)) |role| break :blk role.weapon;
                    }
                }
            }
            break :blk 0;
        };

        var it = weapon_comp.weapon_map.iterator();
        while (it.next()) |kv| {
            const incr_id = kv.key_ptr.*;
            const w = kv.value_ptr;
            const match = if (std.ascii.eqlIgnoreCase(scope, "all"))
                true
            else if (std.ascii.eqlIgnoreCase(scope, "current"))
                incr_id == cur_weapon_incr
            else
                w.role_id != null;
            if (!match) continue;

            const cfg = assets.tables.weapon_conf.getDataById(w.id);
            const max_reson = if (cfg) |c| if (c.ResonLevelLimit > 0) c.ResonLevelLimit else 5 else 5;
            w.level = lv;
            w.breach = breach;
            w.exp = 0;
            w.reson_level = @min(rank, max_reson);

            const path = try std.fmt.allocPrint(alloc.arena, "player/{}/{s}/{}", .{ weapon_comp.player_id, WeaponItem.data_dir, incr_id });
            try comp_util.saveStruct(fs, w.*, path, alloc.arena);
            try events.enqueue(.weapon_info_modified, .{ .incr_id = incr_id });
            try updated_weapons.append(alloc.arena, w.toProto(incr_id));
            if (w.role_id) |rid| try touched_roles.append(alloc.arena, rid);
        }

        if (updated_weapons.items.len > 0) {
            try conn.push(alloc.gpa, io, pb.WeaponItemAddNotify{
                .WeaponItemList = updated_weapons,
                .AddFromRole = false,
                .Reason = 0,
            });
        }
        for (touched_roles.items) |rid| {
            try syncRoleOnSceneOrNotify(events, alloc, fs, assets, conn, scene_opt, role_comp, weapon_comp, echo_comp, rid, io);
        }
        try respond(events, alloc.arena, "upgraded {d} weapon(s) to Lv.{d} R{d}", .{ updated_weapons.items.len, lv, rank });
    }
};

pub const quest_pick = struct {
    pub const alias = "qpick";
    pub const description = "picks/starts and tracks a quest by ID.\nusage: quest_pick [quest_id] [force?]";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
        quest_id: i32,
        force_opt: ?bool,
    ) !void {
        const ctx = state.questContext(alloc.gpa, io, events);
        const ok = try QuestEngine.gmPickQuest(ctx, quest_id, force_opt orelse true);
        if (!ok) {
            try respond(events, alloc.arena, "unknown quest id {d}", .{quest_id});
            return;
        }
        try respond(events, alloc.arena, "picked and tracked quest {d}", .{quest_id});
    }
};

pub const quest_step = struct {
    pub const alias = "qstep";
    pub const description = "skips the active objective node of current/target quest.\nusage: quest_step [quest_id?]";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
        quest_id_opt: ?i32,
    ) !void {
        const ctx = state.questContext(alloc.gpa, io, events);
        const qid = try QuestEngine.gmSkipQuestNode(ctx, quest_id_opt);
        if (qid == 0) {
            try respond(events, alloc.arena, "no active quest node found to step", .{});
            return;
        }
        try respond(events, alloc.arena, "skipped current step of quest {d}", .{qid});
    }
};

pub const quest_skip = struct {
    pub const alias = "qskip";
    pub const description = "skips/completes entire tracked or specified quest.\nusage: quest_skip [quest_id?]";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
        quest_id_opt: ?i32,
    ) !void {
        const ctx = state.questContext(alloc.gpa, io, events);
        const qid = try QuestEngine.gmSkipQuest(ctx, quest_id_opt);
        if (qid == 0) {
            try respond(events, alloc.arena, "no matching quest found to skip", .{});
            return;
        }
        try respond(events, alloc.arena, "completed/skipped quest {d}", .{qid});
    }
};

pub const unlock_tp = struct {
    pub const alias = "tpall";
    pub const description = "unlocks all Resonance Beacons / Nexus teleporters and map fog.\nusage: unlock_tp";

    pub fn call(
        events: *EventQueue,
        fs: *FileSystem,
        assets: *const Assets,
        conn: *Connection,
        map_comp: *PlayerMapComponent,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        var tp_added: usize = 0;
        var fog_added: usize = 0;
        for (assets.tables.teleporter.items) |tp| {
            if (try map_comp.info.unlockTeleporter(alloc.gpa, tp.Id)) {
                tp_added += 1;
            }
            if (tp.FogId > 0 and try map_comp.info.unlockFog(alloc.gpa, tp.FogId)) {
                fog_added += 1;
                conn.pushBestEffort(alloc.gpa, io, pb.MapUnlockFieldNotify{ .FieldId = tp.FogId });
            }
        }
        for (assets.tables.map_fog.items) |fog| {
            if (fog.Fog > 0 and try map_comp.info.unlockFog(alloc.gpa, fog.Fog)) {
                fog_added += 1;
                conn.pushBestEffort(alloc.gpa, io, pb.MapUnlockFieldNotify{ .FieldId = fog.Fog });
            }
        }
        if (tp_added > 0 or fog_added > 0) {
            try map_comp.save(fs, alloc.arena);
        }
        var tp_ids: std.ArrayList(i32) = .empty;
        for (map_comp.info.unlocked_teleporters) |id| {
            try tp_ids.append(alloc.arena, id);
        }
        try conn.push(alloc.gpa, io, pb.TeleportUpdateNotify{ .Ids = tp_ids });
        try respond(events, alloc.arena, "unlocked +{d} teleporters (+{d} map fog zones, total={d})", .{ tp_added, fog_added, tp_ids.items.len });
    }
};

pub const unlock_func = struct {
    pub const alias = "funcall";
    pub const description = "unlocks all system menus/functions (Convene, Events, Tower, Co-op, Synthesis).\nusage: unlock_func";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        const ctx = state.questContext(alloc.gpa, io, events);
        var added: usize = 0;
        for (state.assets.tables.function_condition.items) |fc| {
            if (fc.FunctionId <= 0 or !fc.IsOn) continue;
            if (try ctx.quest_comp.info.unlockSystem(alloc.gpa, fc.FunctionId)) {
                added += 1;
            }
        }
        try QuestActions.pushFuncOpen(ctx);
        try ctx.quest_comp.save(ctx.fs, alloc.arena);
        try respond(events, alloc.arena, "unlocked +{d} functions (total={d})", .{ added, ctx.quest_comp.info.unlocked_systems.len });
    }
};

pub const money = struct {
    pub const alias = "cash";
    pub const description = "grants Astrite, Shell Credit, Lunite, Tides, and Waveplates.\nusage: money";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        const items = [_]RewardService.RewardItem{
            .{ .item_id = 3, .count = 160000 },
            .{ .item_id = 2, .count = 10000000 },
            .{ .item_id = 4, .count = 64800 },
            .{ .item_id = 50002, .count = 500 },
            .{ .item_id = 50005, .count = 500 },
            .{ .item_id = 50001, .count = 500 },
            .{ .item_id = 50013, .count = 5000 },
            .{ .item_id = 5, .count = 1000 },
        };
        _ = try RewardService.grantItems(state.rewardContext(alloc.gpa, io), 0, true, &items, 1);
        try respond(events, alloc.arena, "Granted currencies: +160k Astrite, +10M Shell Credit, +64.8k Lunite, +500 Tides, +1000 Waveplates", .{});
    }
};

pub const time_set = struct {
    pub const alias = "time";
    pub const description = "sets in-game clock hour and minute.\nusage: time_set [0..23] [0..59?]";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
        hour: i32,
        minute_opt: ?i32,
    ) !void {
        const h = @mod(hour, 24);
        const m = @mod(minute_opt orelse 0, 60);
        const ctx = state.questContext(alloc.gpa, io, events);
        try QuestEngine.setSceneTime(ctx, h, m, true);
        try ctx.quest_comp.save(ctx.fs, alloc.arena);
        try respond(events, alloc.arena, "set in-game time to {d:0>2}:{d:0>2}", .{ h, m });
    }
};

pub const repair_account = struct {
    pub const alias = "repair";
    pub const description = "repairs broken quest unlocks, orphaned trial roles/locks, Rover & RoleInfo weapons.\nusage: repair_account";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
    ) !void {
        const ctx = state.questContext(alloc.gpa, io, events);
        const rep = try QuestEngine.reconcileAccountState(ctx, true);
        try RoleHelper.pushRoleMetadataNotifies(
            state.conn,
            alloc,
            state.assets,
            &state.player_components.basic,
            &state.player_components.role,
            io,
        );
        try pushAllOwnedRoles(
            state.conn,
            alloc,
            state.assets,
            &state.player_components.basic,
            &state.player_components.role,
            &state.player_components.weapon,
            &state.player_components.echo,
            io,
        );
        try respond(
            events,
            alloc.arena,
            "repaired account: quests={d}, systems=+{d}, trials_cleared={d}, locks_cleared={d}, leaked_roles_removed={d}, roles_fixed={d}, weapons_fixed={d}",
            .{
                rep.repaired_quests,
                rep.unlocked_systems,
                rep.removed_trial_roles,
                rep.unlocked_locks,
                rep.role_summary.removed_leaks,
                rep.role_summary.fixed_roles,
                rep.role_summary.fixed_weapons,
            },
        );
    }
};

pub const switch_rover = struct {
    pub const alias = "rover";
    pub const description = "switches Rover element safely (spectro|havoc|aero|electro).\nusage: switch_rover [spectro|havoc|aero|electro]";

    pub fn call(
        state: *State,
        events: *EventQueue,
        alloc: mem.Alloc,
        io: std.Io,
        element_arg: []const u8,
    ) !void {
        const elem_id: i32 = if (std.ascii.eqlIgnoreCase(element_arg, "spectro") or std.mem.eql(u8, element_arg, "1501") or std.mem.eql(u8, element_arg, "1502"))
            5
        else if (std.ascii.eqlIgnoreCase(element_arg, "havoc") or std.mem.eql(u8, element_arg, "1604") or std.mem.eql(u8, element_arg, "1605"))
            6
        else if (std.ascii.eqlIgnoreCase(element_arg, "aero") or std.mem.eql(u8, element_arg, "1406") or std.mem.eql(u8, element_arg, "1408"))
            4
        else if (std.ascii.eqlIgnoreCase(element_arg, "electro") or std.mem.eql(u8, element_arg, "1309") or std.mem.eql(u8, element_arg, "1310"))
            3
        else {
            try respond(events, alloc.arena, "unknown Rover element '{s}' (use spectro|havoc|aero|electro)", .{element_arg});
            return;
        };

        const basic_comp = &state.player_components.basic;
        const role_comp = &state.player_components.role;
        const weapon_comp = &state.player_components.weapon;
        const echo_comp = &state.player_components.echo;
        const gender = if (basic_comp.info.attributes.sex == 0) @as(i32, 0) else @as(i32, 1);

        const target_id = RoleHelper.mainRoleIdForElement(state.assets, gender, elem_id) orelse {
            try respond(events, alloc.arena, "Rover element {d} not found for gender {d}", .{ elem_id, gender });
            return;
        };
        const default_main: i32 = if (gender == 0) 1502 else 1501;
        const source_id = RoleHelper.selectedMainRoleId(
            state.assets,
            role_comp,
            gender,
            basic_comp.info.selected_main_role_id,
        ) orelse default_main;
        const scene_ptr: ?*Scene = if (state.scene) |*sc| sc else null;
        const res = try RoleHelper.transferMainRoleState(
            alloc,
            state.fs,
            state.assets,
            scene_ptr,
            basic_comp,
            role_comp,
            weapon_comp,
            echo_comp,
            source_id,
            target_id,
        );
        if (res.basic_changed) try RoleHelper.saveBasicInfo(state.fs, alloc.arena, basic_comp);
        if (res.formation_changed) {
            if (scene_ptr) |sc| sc.savePersistent(state.fs, alloc.gpa) catch {};
        }

        try RoleHelper.pushRoleMetadataNotifies(state.conn, alloc, state.assets, basic_comp, role_comp, io);
        try pushAllOwnedRoles(state.conn, alloc, state.assets, basic_comp, role_comp, weapon_comp, echo_comp, io);
        try respond(events, alloc.arena, "switched Rover from {d} to {d} ({s})", .{ source_id, target_id, element_arg });
    }
};
