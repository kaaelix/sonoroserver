const std = @import("std");

Id: i32,
Type: []const u8 = "",
NeedNum: i32 = 1,
LimitParams: ?std.json.Value = null,
LimitParamsOpe: ?std.json.Value = null,
IsClient: bool = false,
