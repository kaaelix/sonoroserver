const pb = @import("proto").pb;

location: i32 = 0,

pub fn toProto(component: @This()) pb.LiftComponentPb {
    return .{ .Location = component.location };
}
