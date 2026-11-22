const std = @import("std");

QuestId: i32,
Data: DataEntry,

pub const DataEntry = struct {
    Id: i32 = 0,
    Type: i32 = 0,
    RegionId: i32 = 0,
    ChapterId: i32 = 0,
    RewardId: i32 = 0,
    DungeonId: i32 = 0,
    ActivityId: i32 = 0,
    Key: []const u8 = "",
    DistributeType: []const u8 = "",
    IsAutoTrack: bool = false,
    IsHideInTaskList: bool = false,
    FunctionId: i32 = 0,
    RecommendationPriority: i32 = 0,
    ProvideType: ?std.json.Value = null,
    PreShowInfo: ?std.json.Value = null,
    ActiveActions: []const std.json.Value = &.{},
    AcceptActions: []const std.json.Value = &.{},
    FinishActions: []const std.json.Value = &.{},
    AddInteractOption: ?std.json.Value = null,
};
