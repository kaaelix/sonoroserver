const IntMap = @import("int_map.zig").IntMap;

Id: i32,
BreachId: i32,
Level: i32,
ConditionId: i32 = 0,
LevelLimit: i32,
Consume: IntMap(i32, i32),
GoldConsume: i32 = 0,
