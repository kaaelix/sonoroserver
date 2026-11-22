const std = @import("std");

Disabled: ?bool = null,
MaxTriggerTimes: ?i32 = null,
ChangeRoleTrigger: ?bool = null,
Condition: std.json.Value = .null,
Actions: std.json.Value = .null,
ExitConfig: std.json.Value = .null,
