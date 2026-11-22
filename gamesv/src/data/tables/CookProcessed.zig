const CostEntry = @import("common_types.zig").CostEntry;

Id: i32,
FinalItemId: i32 = 0,
Unlock: bool = true,
ConsumeItemsId: []const CostEntry = &.{},
InterationId: []const i32 = &.{},
