const std = @import("std");
const pb = @import("proto").pb;
const common = @import("common");
const mem = @import("../../mem.zig");
const PlayerSettingsComponent = @import("../../logic/component/player/PlayerSettingsComponent.zig");
const PlayerProgressComponent = @import("../../logic/component/player/PlayerProgressComponent.zig");
const Transaction = @import("../handlers.zig").Transaction;
const pb_kv = @import("pb_kv.zig");

const FileSystem = common.FileSystem;

pub fn onLanguageSettingUpdateRequest(
    txn: *Transaction(pb.LanguageSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try progress_comp.info.setKeyInt(alloc.gpa, "setting:lang", txn.message.Language);
    try progress_comp.save(fs, alloc.arena);
    txn.respond(.{ .ErrorCode = .Success });
}

pub fn onInputSettingRequest(
    txn: *Transaction(pb.InputSettingRequest),
    alloc: mem.Alloc,
    progress_comp: *const PlayerProgressComponent,
) !void {
    txn.respond(.{
        .InputSettingData = pb_kv.loadMsg(alloc.arena, progress_comp, "setting:input_pb", pb.InputSettingData),
    });
}

pub fn onInputSettingUpdateRequest(
    txn: *Transaction(pb.InputSettingUpdateRequest),
    alloc: mem.Alloc,
    fs: *FileSystem,
    progress_comp: *PlayerProgressComponent,
) !void {
    try pb_kv.persistOptMsg(alloc, fs, progress_comp, "setting:input_pb", txn.message.InputSettingData);
    txn.respond(.{ .ErrorCode = .Success });
}
