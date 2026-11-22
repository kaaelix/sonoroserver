pub const PropEntry = struct {
    Id: i32 = 0,
    Value: i32 = 0,
    IsRatio: bool = false,
};

Id: i32,
Level: i32 = 0,
MainProp: PropEntry = .{},
SubProp: []const PropEntry = &.{},
MainProps: []const i32 = &.{},
SubPropList: []const i32 = &.{},
FetterGroupId: i32 = 0,
SkinItemId: i32 = 0,
