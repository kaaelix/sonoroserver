const CostEntry = @import("common_types.zig").CostEntry;

Id: i32,
FormulaItemId: i32 = 0,
ItemId: i32 = 0,
FormulaType: i32 = 0,
UnlockCondition: i32 = 0,
Proficiency: i32 = 0,
MaxProficiencyCount: i32 = 0,
TypeId: i32 = 0,
Unlock: bool = true,
LimitCount: i32 = 0,
ConsumeItems: []const CostEntry = &.{},
RoleList: []const i32 = &.{},
