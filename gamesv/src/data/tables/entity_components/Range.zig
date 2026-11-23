pub const Vec3 = struct {
    X: ?f32 = null,
    Y: ?f32 = null,
    Z: ?f32 = null,
};

pub const ShapeConfig = struct {
    Type: ?[]const u8 = null,
    Radius: ?f32 = null,
    Height: ?f32 = null,
    InnerRadius: ?f32 = null,
    Angle: ?f32 = null,
    Size: ?Vec3 = null,
    Center: ?Vec3 = null,
    Rotator: ?Vec3 = null,
    CombinationShapes: ?[]const @This() = null,
};

Shape: ?ShapeConfig = null,
