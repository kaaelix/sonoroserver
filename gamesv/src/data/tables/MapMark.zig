MarkId: i32,
MapId: i32 = 0,
RelativeType: i32 = 0,
RelativeSubType: i32 = 0,
RelativeId: i32 = 0,
EntityConfigId: i64 = 0,
FogHide: i32 = 0,
ObjectType: i32 = 0,
ShowCondition: i32 = 0,
CreateOnStart: i32 = 0,
FogShow: i32 = 0,
IsMonster: i32 = 0,
MarkVector: Vec3 = .{},

pub const Vec3 = struct {
    X: f32 = 0,
    Y: f32 = 0,
    Z: f32 = 0,
};
