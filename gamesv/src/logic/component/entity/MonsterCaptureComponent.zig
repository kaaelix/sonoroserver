const Component = @This();
const pb = @import("proto").pb;

template_id: i32 = 0,
entity_id: i32 = 0,
monster_id: i32 = 0,

pub fn toProto(comp: Component) pb.MonsterCaptureComponentPb {
    return .{
        .TemplateId = comp.template_id,
        .EntityId = comp.entity_id,
        .MonsterId = comp.monster_id,
    };
}
