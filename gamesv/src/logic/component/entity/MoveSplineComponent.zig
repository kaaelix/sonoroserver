const pb = @import("proto").pb;

pub const transient = true;

move_status: pb.MoveStatus = .Stop,
spline_entity_id: i32 = 0,
runtime_data: ?pb.SceneItemSplineRuntimeData = null,
move_spline_config: ?pb.MoveSplineConfig = null,

pub fn toProto(component: @This()) pb.EntityMoveSplineComponentPb {
    return .{
        .RuntimeData = if (component.runtime_data) |runtime| .{
            .SceneItemSplineRuntimeData = runtime,
        } else null,
        .SplineEntityId = component.spline_entity_id,
        .MoveSplineConfig = component.move_spline_config,
    };
}
