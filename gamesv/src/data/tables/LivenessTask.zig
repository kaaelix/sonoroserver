const IntMap = @import("int_map.zig").IntMap;

TaskId: i32,
UpdateType: i32 = 0,
TaskReward: IntMap(i32, i32) = .{},
UnlockCondition: i32 = 0,
