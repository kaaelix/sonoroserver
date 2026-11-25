use sonoro_proto::pb::LogicStateComponentPb;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LogicStateComponent {
    pub position_state: i32,
    pub move_state: i32,
    pub direction_state: i32,
    pub position_sub_state: i32,
}

impl LogicStateComponent {
    pub fn to_proto(&self) -> LogicStateComponentPb {
        LogicStateComponentPb {
            PositionState: self.position_state,
            MoveState: self.move_state,
            DirectionState: self.direction_state,
            PositionSubState: self.position_sub_state,
        }
    }
}
