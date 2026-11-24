const std = @import("std");

pub const FlowRef = struct {
    FlowListName: []const u8 = "",
    FlowId: i32 = 0,
    StateId: i32 = 0,
};

pub const ActionParams = struct {
    FlowListName: []const u8 = "",
    FlowId: i32 = 0,
    StateId: i32 = 0,
    EntityState: []const u8 = "",
    State: []const u8 = "",
    Type: []const u8 = "",
    EntityId: i32 = 0,
    EntityIds: []const i32 = &.{},
    DungeonId: i32 = 0,
    LocationEntityId: i64 = 0,

    /// Custom JSON parser that accepts non-string `Type` payloads (for example
    /// `FireBullet` object configs) without aborting table load.
    pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) !@This() {
        if (.object_begin != try source.next()) return error.UnexpectedToken;
        var r: @This() = .{};
        const max_value_len = options.max_value_len orelse std.json.default_max_value_len;

        fields: while (true) {
            const name_token = try source.nextAllocMax(allocator, .alloc_if_needed, max_value_len);
            const field_name = switch (name_token) {
                .string, .allocated_string => |slice| slice,
                .object_end => break,
                else => return error.UnexpectedToken,
            };
            defer if (name_token == .allocated_string) allocator.free(field_name);

            inline for (std.meta.fields(@This())) |field| {
                if (std.mem.eql(u8, field_name, field.name)) {
                    const next_type = try source.peekNextTokenType();
                    if (comptime std.mem.eql(u8, field.name, "Type")) {
                        if (next_type == .string) {
                            r.Type = try std.json.innerParse([]const u8, allocator, source, options);
                        } else {
                            try source.skipValue();
                        }
                    } else if (next_type == .null) {
                        _ = try source.next();
                    } else {
                        @field(r, field.name) = try std.json.innerParse(field.type, allocator, source, options);
                    }
                    continue :fields;
                }
            }
            if (options.ignore_unknown_fields) {
                try source.skipValue();
            } else {
                return error.UnknownField;
            }
        }
        return r;
    }
};

pub const InteractAction = struct {
    Name: []const u8 = "",
    Params: ?ActionParams = null,
    Raw: ?std.json.Value = null,

    pub fn jsonParse(allocator: std.mem.Allocator, source: anytype, options: std.json.ParseOptions) !@This() {
        const val = try std.json.innerParse(std.json.Value, allocator, source, options);
        if (val != .object) return error.UnexpectedToken;
        var out: @This() = .{ .Raw = val };
        if (val.object.get("Name")) |nv| {
            if (nv == .string) out.Name = nv.string;
        }
        if (val.object.get("Params")) |pv| {
            if (pv == .object) {
                var p: ActionParams = .{};
                if (pv.object.get("FlowListName")) |v| if (v == .string) { p.FlowListName = v.string; };
                if (pv.object.get("FlowId")) |v| if (v == .integer) { p.FlowId = std.math.cast(i32, v.integer) orelse 0; };
                if (pv.object.get("StateId")) |v| if (v == .integer) { p.StateId = std.math.cast(i32, v.integer) orelse 0; };
                if (pv.object.get("EntityState")) |v| if (v == .string) { p.EntityState = v.string; };
                if (pv.object.get("State")) |v| if (v == .string) { p.State = v.string; };
                if (pv.object.get("Type")) |v| if (v == .string) { p.Type = v.string; };
                if (pv.object.get("EntityId")) |v| if (v == .integer) { p.EntityId = std.math.cast(i32, v.integer) orelse 0; };
                if (pv.object.get("DungeonId")) |v| if (v == .integer) { p.DungeonId = std.math.cast(i32, v.integer) orelse 0; };
                if (pv.object.get("LocationEntityId")) |v| if (v == .integer) { p.LocationEntityId = v.integer; };
                if (pv.object.get("EntityIds")) |v| if (v == .array and v.array.items.len > 0) {
                    const arr = try allocator.alloc(i32, v.array.items.len);
                    var n: usize = 0;
                    for (v.array.items) |item| {
                        if (item == .integer) {
                            if (std.math.cast(i32, item.integer)) |id| {
                                arr[n] = id;
                                n += 1;
                            }
                        }
                    }
                    out.Params = p;
                    out.Params.?.EntityIds = arr[0..n];
                    return out;
                };
                out.Params = p;
            }
        }
        return out;
    }
};

pub const OptionType = struct {
    Type: []const u8 = "",
    Flow: ?FlowRef = null,
    Actions: []const InteractAction = &.{},
};

pub const InteractOption = struct {
    Type: ?OptionType = null,
    Condition: ?std.json.Value = null,
};

pub const RandomInteractOption = struct {
    Weight: i32 = 0,
    Option: ?InteractOption = null,
};

pub const RandomInteractConfig = struct {
    RandomCount: i32 = 0,
    Options: []const RandomInteractOption = &.{},
};

Disabled: ?bool = null,
DoIntactType: []const u8 = "",
Range: i32 = 0,
Options: []const InteractOption = &.{},
RandomInteract: ?RandomInteractConfig = null,

test "Interact ActionParams skips non-string Type and accepts null fields" {
    const allocator = std.testing.allocator;
    const raw =
        \\{
        \\  "Options": [
        \\    {
        \\      "Type": {
        \\        "Type": "Actions",
        \\        "Actions": [
        \\          {
        \\            "Name": "ChangeEntityState",
        \\            "Params": { "Type": "Directly", "EntityId": 1001, "State": "Active", "EntityIds": null }
        \\          },
        \\          {
        \\            "Name": "FireBullet",
        \\            "Params": { "Type": { "Type": "TrackTarget", "Speed": 500 }, "ExtraUnknown": 1 }
        \\          }
        \\        ]
        \\      }
        \\    }
        \\  ]
        \\}
    ;
    const parsed = try std.json.parseFromSlice(@This(), allocator, raw, .{ .ignore_unknown_fields = true });
    defer parsed.deinit();

    const actions = parsed.value.Options[0].Type.?.Actions;
    try std.testing.expectEqual(@as(usize, 2), actions.len);
    try std.testing.expectEqualStrings("Directly", actions[0].Params.?.Type);
    try std.testing.expectEqual(@as(i32, 1001), actions[0].Params.?.EntityId);
    try std.testing.expectEqualStrings("Active", actions[0].Params.?.State);
    try std.testing.expectEqual(@as(usize, 0), actions[0].Params.?.EntityIds.len);
    try std.testing.expectEqualStrings("", actions[1].Params.?.Type);
}
