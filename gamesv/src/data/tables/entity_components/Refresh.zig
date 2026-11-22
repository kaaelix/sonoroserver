const std = @import("std");

Disabled: ?bool = null,
IsDisableRefreshAfterDroppingReward: ?bool = null,
RefreshRule: std.json.Value = .null,
