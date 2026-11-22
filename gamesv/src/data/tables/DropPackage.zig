const IntMap = @import("int_map.zig").IntMap;

Id: i32,
ShowBg: bool = false,
Title: []const u8 = "",
DropPlan: i32 = 0,
DropPreview: IntMap(i32, i32),
