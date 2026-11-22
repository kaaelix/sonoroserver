const std = @import("std");

LevelPlayId: i32,
Data: DataEntry,

pub const RewardConfig = struct {
    Type: []const u8 = "",
    RewardId: i32 = 0,
    RewardEntityId: i32 = 0,
};

pub const DataEntry = struct {
    Id: i32 = 0,
    LevelId: i32 = 0,
    Type: []const u8 = "",
    InstanceId: i32 = 0,
    LevelPlayEntityId: i32 = 0,
    EnterRadius: i32 = 0,
    LeaveRadius: i32 = 0,
    ExploratoryDegree: i32 = 0,
    LevelPlayRewardConfig: ?RewardConfig = null,
    Children: []const []const u8 = &.{},
    Reference: []const []const u8 = &.{},
};
