const IntMap = @import("int_map.zig").IntMap;

Id: i32,
Type: i32 = 0,
Content: IntMap(i32, i32) = .{},
Weight: IntMap(i32, i32) = .{},
AvailableNum: i32 = 0,
ShowType: i32 = 0,
CheckGoodsMethod: i32 = 0,
