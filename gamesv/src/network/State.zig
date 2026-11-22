const State = @This();
const std = @import("std");
const common = @import("common");
const mem = @import("../mem.zig");

const Io = std.Io;
const Allocator = std.mem.Allocator;
const FileSystem = common.FileSystem;
const Assets = @import("../data/Assets.zig");
const Connection = @import("Connection.zig");
const ArenaAllocator = std.heap.ArenaAllocator;
const Scene = @import("../logic/Scene.zig");
const PlayerID = @import("../logic/PlayerID.zig");
const PlayerComponentStorage = @import("../logic/component/player/PlayerComponentStorage.zig");
const BuffTimerScheduler = @import("../logic/schedulers/BuffTimerScheduler.zig");
const DirtySaveQueue = @import("../logic/schedulers/DirtySaveQueue.zig");
const EventQueue = @import("../logic/EventQueue.zig");
const quest_types = @import("../logic/helpers/quest_types.zig");
const RewardService = @import("../logic/helpers/reward.zig");
const CoopService = @import("../logic/CoopService.zig");

const log = std.log.scoped(.state);

fs: *FileSystem,
conn: *Connection,
assets: *const Assets,
arena: ArenaAllocator,
player_id: PlayerID,
player_components: PlayerComponentStorage,
scene: ?Scene,
buff_timers: BuffTimerScheduler,
dirty_saves: DirtySaveQueue,
coop_service: ?*CoopService = null,
gm_capture: bool = false,
gm_output: std.ArrayList(u8) = .empty,

pub fn init(
    gpa: Allocator,
    fs: *FileSystem,
    conn: *Connection,
    assets: *const Assets,
    pcs: PlayerComponentStorage,
    player_id: i32,
) State {
    return .{
        .fs = fs,
        .conn = conn,
        .arena = .init(gpa),
        .assets = assets,
        .player_components = pcs,
        .player_id = .{ .id = player_id },
        .scene = null,
        .buff_timers = .{},
        .dirty_saves = .{},
    };
}

pub fn deinit(s: *State, gpa: Allocator, io: Io, fs: *FileSystem) void {
    const now_ms = (Io.Clock.awake).now(io).toMilliseconds();
    // Transient scene entities and buffs are wiped by scene.deinit -> clear_entities,
    // so only persist durable state here to keep session replacement fast.
    if (s.scene) |*scene| {
        scene.savePersistent(fs, gpa) catch |err| {
            log.err("failed to save persistent scene state during session cleanup: {t}", .{err});
        };
        scene.saveLastScene(&s.player_components.scene, fs, gpa) catch |err| {
            log.err("failed to save last scene during session cleanup: {t}", .{err});
        };
    }
    s.dirty_saves.flushPersistent(
        gpa,
        fs,
        &s.player_components.role,
        &s.player_components.weapon,
        now_ms,
    ) catch |err| {
        log.err("failed to flush persistent dirty saves during session cleanup: {t}", .{err});
    };

    s.dirty_saves.deinit(gpa);
    s.buff_timers.deinit(gpa);
    if (s.coop_service) |cs| cs.unregisterOnline(gpa, io, s.player_id.id);
    s.arena.deinit();
    s.player_components.deinit(gpa);
    if (s.scene) |*scene| scene.deinit(gpa, fs);
}

pub fn extract(s: *State, gpa: Allocator, io: Io, comptime T: type) !T {
    if (T == Io) return io;
    if (T == *FileSystem) return s.fs;
    if (T == *Connection) return s.conn;
    if (T == *const Assets) return s.assets;
    if (T == PlayerID) return s.player_id;
    if (T == mem.Alloc) return .{
        .gpa = gpa,
        .arena = s.arena.allocator(),
    };

    if (T == *?Scene) return &s.scene;
    if (T == *Scene) return &(s.scene orelse return error.NotInScene);
    if (T == *BuffTimerScheduler) return &s.buff_timers;
    if (T == *DirtySaveQueue) return &s.dirty_saves;
    if (T == *CoopService) return s.coop_service orelse return error.CoopUnavailable;
    if (T == ?*CoopService) return s.coop_service;

    const is_struct = comptime std.meta.activeTag(@typeInfo(T)) == .@"struct";

    if (is_struct and @hasDecl(T, "scene_query_types")) {
        if (s.scene) |*scene| {
            return .{
                .iterator = .{
                    .entities = &scene.entities,
                    .net_id_map = &scene.net_id_map,
                },
            };
        } else {
            return error.NotInScene;
        }
    }

    if (comptime PlayerComponentStorage.hasComponent(T)) return s.player_components.extract(T);
    if (T == *State) return s;

    @compileError("the type '" ++ @typeName(T) ++ "' can't be extracted out of State");
}

pub fn questContext(s: *State, gpa: Allocator, io: Io, events: *EventQueue) quest_types.QuestContext {
    return .{
        .assets = s.assets,
        .fs = s.fs,
        .conn = s.conn,
        .scene = if (s.scene) |*scene| scene else null,
        .scene_comp = &s.player_components.scene,
        .events = events,
        .alloc = .{
            .gpa = gpa,
            .arena = s.arena.allocator(),
        },
        .io = io,
        .basic_comp = &s.player_components.basic,
        .inventory_comp = &s.player_components.inventory,
        .weapon_comp = &s.player_components.weapon,
        .echo_comp = &s.player_components.echo,
        .map_comp = &s.player_components.map,
        .quest_comp = &s.player_components.quest,
        .guide_comp = &s.player_components.guide,
        .role_comp = &s.player_components.role,
        .mail_comp = &s.player_components.mail,
        .progress_comp = &s.player_components.progress,
    };
}

pub fn rewardContext(s: *State, gpa: Allocator, io: Io) RewardService.RewardContext {
    return .{
        .alloc = .{
            .gpa = gpa,
            .arena = s.arena.allocator(),
        },
        .io = io,
        .fs = s.fs,
        .assets = s.assets,
        .conn = s.conn,
        .basic_comp = &s.player_components.basic,
        .inventory_comp = &s.player_components.inventory,
        .weapon_comp = &s.player_components.weapon,
        .echo_comp = &s.player_components.echo,
    };
}
