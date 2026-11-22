const pb = @import("proto").pb;

pub const transient = true;

target_pos: [3]f32 = @splat(0),
move_type: i32 = 0,
is_exact: bool = false,
move_time: f32 = 0,
stay_time: f32 = 0,
max_speed: f32 = 0,
acceleration: f32 = 0,

pub fn toProto(component: @This()) pb.MoveToPointComponentPb {
    return .{
        .PbMoveToPointConfig = .{
            .TargetPos = .{
                .X = component.target_pos[0],
                .Y = component.target_pos[1],
                .Z = component.target_pos[2],
            },
            .MoveType = component.move_type,
            .IsExact = component.is_exact,
        },
    };
}
