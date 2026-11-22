const std = @import("std");

pub const VarDef = struct {
    Name: []const u8 = "",
    Type: []const u8 = "",
    Access: i32 = 0,
    Value: std.json.Value = .null,
};

Disabled: ?bool = null,
Vars: []const VarDef = &.{},
