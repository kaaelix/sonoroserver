const IntMap = @import("int_map.zig").IntMap;

Id: i32,
ShopId: i32 = 0,
SortIndex: i32 = 0,
ItemId: i32 = 0,
ItemNum: i32 = 1,
CondId: i32 = 0,
LimitNum: i32 = -1,
OriginalPrice: IntMap(i32, i32) = .{},
Price: IntMap(i32, i32) = .{},
Show: bool = true,
Label: []const u8 = "",
