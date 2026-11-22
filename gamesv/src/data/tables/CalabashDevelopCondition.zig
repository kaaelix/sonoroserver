const IntMap = @import("int_map.zig").IntMap;

Id: i32,
Condition: i32,
ConditionParam: []const i32 = &.{},
RewardExp: IntMap(i32, i32),
