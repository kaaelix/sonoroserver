pub const Entry = struct {
    Type: i32 = 0,
    LocationEntityId: i64 = 0,
};

Disabled: ?bool = null,
DefaultGravity: ?i32 = null,
Config: []const Entry = &.{},
