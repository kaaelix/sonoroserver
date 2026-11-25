use sonoro_gamesv_data::fsm_graph_registry::MAX_STATE_DEPTH;

pub use crate::events::FsmBindBuffSource;

pub const EVAL_FLAG_WAIT_CLIENT: u32 = 1 << 0;
pub const MAX_PART_STATE_LEN: usize = 128;

pub type WakeMask = u16;

pub mod wake_reason {
    use super::WakeMask;

    pub const INITIAL: WakeMask = 1 << 0;
    pub const STATE: WakeMask = 1 << 1;
    pub const ATTRIBUTE: WakeMask = 1 << 2;
    pub const TAG: WakeMask = 1 << 3;
    pub const BUFF: WakeMask = 1 << 4;
    pub const PART: WakeMask = 1 << 5;
    pub const BLACKBOARD: WakeMask = 1 << 6;
    pub const EVENT: WakeMask = 1 << 7;
    pub const POSITION: WakeMask = 1 << 8;
    pub const CLIENT_PASS: WakeMask = 1 << 9;
    pub const DISSOLVE_COMBINE: WakeMask = 1 << 10;
    pub const TIMER: WakeMask = 1 << 11;
    pub const SKILL_END: WakeMask = 1 << 12;
    pub const HATE: WakeMask = 1 << 13;

    pub const ALL: WakeMask = INITIAL
        | STATE
        | ATTRIBUTE
        | TAG
        | BUFF
        | PART
        | BLACKBOARD
        | EVENT
        | POSITION
        | CLIENT_PASS
        | DISSOLVE_COMBINE
        | TIMER
        | SKILL_END
        | HATE;
}

#[derive(Debug, Clone)]
pub struct FsmNode {
    pub fsm_id: i32,
    pub active_path: [i32; MAX_STATE_DEPTH],
    pub active_since_ms: [i64; MAX_STATE_DEPTH],
    pub active_len: usize,
    pub previous_path: [i32; MAX_STATE_DEPTH],
    pub previous_len: usize,
    pub active_skill_id: Option<i32>,
    pub active_skill_ended: bool,
    pub pending_path: [i32; MAX_STATE_DEPTH],
    pub pending_since_ms: [i64; MAX_STATE_DEPTH],
    pub pending_len: usize,
    pub pending_from: Option<i32>,
    pub pending_to: Option<i32>,
    pub pending_started_ms: i64,
    pub pending_skill_id: Option<i32>,
    pub pending_skill_ended: bool,
    pub wake_dependencies: WakeMask,
    pub dirty_reasons: WakeMask,
    pub next_timer_due_ms: Option<i64>,
    pub delayed_suicide_state: Option<i32>,
    pub delayed_suicide_due_ms: Option<i64>,
    pub delayed_destroy_due_ms: Option<i64>,
    pub delayed_suicide_fired: bool,
}

impl Default for FsmNode {
    fn default() -> Self {
        Self {
            fsm_id: 0,
            active_path: [0; MAX_STATE_DEPTH],
            active_since_ms: [0; MAX_STATE_DEPTH],
            active_len: 0,
            previous_path: [0; MAX_STATE_DEPTH],
            previous_len: 0,
            active_skill_id: None,
            active_skill_ended: false,
            pending_path: [0; MAX_STATE_DEPTH],
            pending_since_ms: [0; MAX_STATE_DEPTH],
            pending_len: 0,
            pending_from: None,
            pending_to: None,
            pending_started_ms: 0,
            pending_skill_id: None,
            pending_skill_ended: false,
            wake_dependencies: 0,
            dirty_reasons: 0,
            next_timer_due_ms: None,
            delayed_suicide_state: None,
            delayed_suicide_due_ms: None,
            delayed_destroy_due_ms: None,
            delayed_suicide_fired: false,
        }
    }
}

impl FsmNode {
    pub fn active(&self) -> &[i32] {
        &self.active_path[..self.active_len]
    }

    pub fn pending(&self) -> &[i32] {
        &self.pending_path[..self.pending_len]
    }

    pub fn previous(&self) -> &[i32] {
        &self.previous_path[..self.previous_len]
    }

    pub fn leaf(&self) -> Option<i32> {
        self.active().last().copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionKey {
    pub fsm_id: i32,
    pub from: i32,
    pub to: i32,
    pub index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagCount {
    pub id: i64,
    pub count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PartState {
    pub name: [u8; MAX_PART_STATE_LEN],
    pub name_len: u8,
    pub life: f32,
    pub life_max: f32,
    pub activated: bool,
}

impl Default for PartState {
    fn default() -> Self {
        Self {
            name: [0u8; MAX_PART_STATE_LEN],
            name_len: 0,
            life: 0.0,
            life_max: 0.0,
            activated: false,
        }
    }
}

impl PartState {
    pub fn slice(&self) -> &[u8] {
        &self.name[..usize::from(self.name_len)]
    }
}

#[derive(Clone, Default)]
pub struct EvalContext<'a> {
    pub now_ms: i64,
    pub flags: u32,
    pub position_state: Option<i32>,
    pub attributes: &'a [i32],
    pub tags: &'a [i32],
    pub parent_tags: Option<&'a sonoro_gamesv_data::data_tables::Table<sonoro_gamesv_data::tables::gameplay_tag_parent::GameplayTagParent, i32>>,
    pub buff_stacks: &'a [(i64, i32)],
    pub parts: &'a [PartState],
    pub instance_state: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transition {
    pub fsm_id: i32,
    pub from: i32,
    pub to: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TagMirror {
    pub tag_id: i32,
    pub delta: i32,
    pub clear: bool,
    pub was_present: bool,
    pub is_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindBuffAddition {
    pub source: FsmBindBuffSource,
    pub buff_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartActivation {
    pub name: String,
    pub activate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartReset {
    pub name: String,
    pub reset_activate: bool,
    pub reset_life: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleEffect {
    EnterFight,
    AddBuff(i64),
    RemoveBuff(i64),
    BindBuffAdd(BindBuffAddition),
    BindBuffRemove(FsmBindBuffSource),
    CueParalysis,
    ResetStatus,
    SetRageFull,
    SetInstanceState(i32),
    ActivatePart(PartActivation),
    ResetPart(PartReset),
    TagMirror(TagMirror),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmResult {
    Confirmed,
    Accepted,
    Mismatch(i32),
    MachineNotFound,
    NoPending,
    InvalidSource,
    InvalidTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientPassResult {
    Updated,
    MachineNotFound,
    InvalidSource,
    InactiveSource,
    InvalidTarget,
    TransitionNotFound,
    ConditionNotFound,
    ConditionNotClient,
    ConditionContextInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientConditionRequirement {
    None,
    Task,
    Montage,
    GroupPatrol,
    GroupPerform,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientConditionLookup {
    Valid(ClientConditionRequirement),
    TransitionNotFound,
    ConditionNotFound,
    ConditionNotClient,
    ConditionInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorValidation {
    Ok,
    MachineNotFound,
    InvalidState,
    InactiveState,
    NodeNotFound,
    MetadataNotFound,
    Mismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedStates {
    pub from: i32,
    pub to: i32,
}
