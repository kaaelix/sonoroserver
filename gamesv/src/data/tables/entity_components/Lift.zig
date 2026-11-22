const std = @import("std");

Disabled: ?bool = null,
InitialFloor: ?i32 = null,
MaxSpeed: ?f32 = null,
UniformMovement: ?bool = null,
StayPositions: std.json.Value = .null,
