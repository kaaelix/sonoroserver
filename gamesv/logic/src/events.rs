#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingFlow {
    pub flow_list_name: String,
    pub flow_id: i32,
    pub state_id: i32,
    pub start_action_id: i32,
    pub source_quest_id: i32,
    pub source_quest_child_id: i32,
    pub pos_sender: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsmServerActionKind {
    EnterFight,
    CueParalysis,
    ResetStatus,
    SetRageFull,
    InstanceState(i32),
    ActivatePart {
        name: String,
        activate: bool,
    },
    ResetPart {
        name: String,
        reset_activate: bool,
        reset_life: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FsmBindBuffSource {
    pub fsm_id: i32,
    pub state: i32,
    pub bind_index: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuffAdditionEntry {
    pub id: i64,
    pub is_active: bool,
    pub stack_count: i32,
    pub duration_seconds: Option<f32>,
    pub fsm_bind_source: Option<FsmBindBuffSource>,
    pub message_id: i64,
    pub skill_id: i64,
    pub level: i32,
    pub notify_client: bool,
}

impl Default for BuffAdditionEntry {
    fn default() -> Self {
        Self {
            id: 0,
            is_active: false,
            stack_count: 0,
            duration_seconds: None,
            fsm_bind_source: None,
            message_id: 0,
            skill_id: 0,
            level: 1,
            notify_client: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityHandle {
    pub index: usize,
    pub net_id: i64,
}

#[derive(Debug, Clone)]
pub struct NaturalExpiration {
    pub now_ms: i64,
    pub instigator: EntityHandle,
    pub follow_up_buffs: Vec<BuffAdditionEntry>,
}

#[derive(Debug, Clone)]
pub enum Event {
    EnterGame {
        player_id: i32,
    },
    PushData {
        player_id: i32,
        map_config_id: i32,
        instance_id: i32,
    },
    PushDataComplete {
        player_id: i32,
    },
    ReconnectResync {
        player_id: i32,
        reason: String,
    },
    ConditionStateRefresh {
        player_id: i32,
        entity_config_id: i32,
    },
    InitialSceneJoin {
        player_id: i32,
        map_config_id: i32,
        instance_id: i32,
        location: Option<[f32; 3]>,
        rotation: Option<[f32; 3]>,
        pending_flow: Option<PendingFlow>,
    },
    SceneSwitch {
        player_id: i32,
        instance_id: i32,
        x: f32,
        y: f32,
        z: f32,
        yaw: f32,
        keep_team: bool,
        pending_flow: Option<PendingFlow>,
    },
    AfterSceneJoin {
        player_id: i32,
        instance_id: i32,
        reason: String,
        pending_flow: Option<PendingFlow>,
        suppress_area_events: bool,
    },
    UpdateFormations {
        player_id: i32,
        formation_id: i32,
        role_ids: Vec<i32>,
    },
    RoleInfoModified {
        player_id: i32,
        role_id: i32,
    },
    WeaponInfoModified {
        player_id: i32,
        incr_id: i32,
    },
    EntityMovement {
        player_id: i32,
        entity_net_id: i64,
        pos: [f32; 3],
        rot: [f32; 3],
    },
    BuffRemovalByFsmBind {
        entity: EntityHandle,
        source: FsmBindBuffSource,
    },
    BuffRemoval {
        entity: EntityHandle,
        handle_ids: Vec<i32>,
        natural_expiration: Option<NaturalExpiration>,
    },
    BuffRemovalById {
        entity: EntityHandle,
        buff_id: i64,
    },
    BuffAddition {
        target: EntityHandle,
        instigator: EntityHandle,
        buffs: Vec<BuffAdditionEntry>,
    },
    BuffChange {
        player_id: i32,
        entity: EntityHandle,
        buff_id: i64,
        stack_count: i32,
        is_add: bool,
        instigator: EntityHandle,
    },
    BuffTimersDue {
        now_ms: i64,
    },
    FsmWake {
        entity_id: i64,
        now_ms: i64,
    },
    FsmServerAction {
        entity: EntityHandle,
        kind: FsmServerActionKind,
    },
    FsmLifecycleComplete {
        entity: EntityHandle,
        now_ms: i64,
        recheck: bool,
    },
    LevelPlayTimersDue {
        now_ms: i64,
    },
    QuestTimersDue {
        now_ms: i64,
    },
    ScenePendingUpdates,
    DirtySavesDue {
        now_ms: i64,
    },
    GameplayTagChange {
        entity: EntityHandle,
        add_tag_ids: Vec<i32>,
        remove_tag_ids: Vec<i32>,
        remove_before_add: bool,
    },
    ChatCommandResponse {
        player_id: i32,
        message: String,
    },
    SpatialSpawnsCheck {
        player_id: i32,
    },
}
