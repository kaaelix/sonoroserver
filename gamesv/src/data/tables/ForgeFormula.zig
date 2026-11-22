const CostEntry = @import("common_types.zig").CostEntry;

Id: i32,
FormulaItemId: i32 = 0,
ItemId: i32 = 0,
TypeId: i32 = 0,
Unlock: bool = true,
ConsumeItems: []const CostEntry = &.{},
RoleList: []const i32 = &.{},
