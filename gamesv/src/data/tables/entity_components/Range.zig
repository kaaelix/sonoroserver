pub const Vec3 = struct {
    X: ?f32 = null,
    Y: ?f32 = null,
    Z: ?f32 = null,
};

pub const ShapeConfig = struct {
    Type: ?[]const u8 = null,
    Radius: ?f32 = null,
    Size: ?Vec3 = null,
    Center: ?Vec3 = null,
};

Shape: ?ShapeConfig = null,
