const CostEntry = @import("common_types.zig").CostEntry;

Id: i32,
FormulaItemId: i32 = 0,
FoodItemId: i32 = 0,
TypeId: i32 = 0,
Unlock: bool = true,
ConsumeItems: []const CostEntry = &.{},
Proficiency: i32 = 0,
MaxProficiencyCount: i32 = 0,
RoleList: []const i32 = &.{},
