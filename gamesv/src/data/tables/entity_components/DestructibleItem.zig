const std = @import("std");

Disabled: ?bool = null,
Durability: ?i32 = null,
DurabilityId: ?i32 = null,
DurabilityStateConfig: std.json.Value = .null,
DestructionActions: std.json.Value = .null,
