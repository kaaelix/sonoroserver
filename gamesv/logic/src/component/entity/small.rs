use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_proto::enums::{
    EEntityType, ESummonType, EntityConfigType, EntityState as PbEntityState, FollowerType,
    MoveStatus, SceneItemBBKey, VehicleSource,
};
use sonoro_proto::pb::{
    CalabashSkinComponentPb, CharacterAttachComponentPb, CharacterAttachInfo, CombatCommon,
    ConcomitantsComponentPb, EntityMoveSplineComponentPb,
    EntityMoveSplineComponentPbRuntimeDataOneof, EquipComponentPb, FollowEntityComponentPb,
    FollowShooterComponentPb, FollowerComponentPb, FollowerList, LiftComponentPb,
    MonsterAiComponentPb, MonsterCaptureComponentPb, MotorDaCtxComponentPb, MotorDiyEquippedPb,
    MoveSplineConfig, MoveToPointComponentPb, NpcDriveVehicleComponentPb, OrnamentComponentPb,
    PassiveGaSkillComponentPb, PassiveSkillComponentPb, PassiveSkillItemPb, PbMoveToPointConfig,
    PlayerSceneComponentPb, Rotator, SceneItemBlackboardParam,
    SceneItemBlackboardParamValueOneof, SceneItemComponentPb, SceneItemSplineRuntimeData,
    SkillComponentPb, StateTagComponentPb, SummonerComponentPb, Transform, Vector, VehiclePb,
    VisionSkillComponentPb, VisionSkillInformation, WeaponSkinComponentPb,
};
use sonoro_proto::wire::PbEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EntityType {
    Player = 0,
    Npc = 1,
    Monster = 2,
    SceneItem = 5,
    Custom = 6,
    Vision = 7,
    Animal = 8,
    ClientOnly = 9,
    Vehicle = 10,
    PlayerEntity = 11,
    SceneEntity = 12,
}

impl EntityType {
    pub fn to_proto(self) -> EEntityType {
        EEntityType::from_i32_or_default(self as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum EntityState {
    #[default]
    Default = 0,
    Sleep = 1,
    Born = 2,
    Other = 3,
}

impl EntityState {
    pub fn to_proto(self) -> PbEntityState {
        PbEntityState::from_i32_or_default(self as i32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ConfigType {
    OldEntity = 0,
    Level = 1,
    Global = 2,
    Character = 3,
    Template = 4,
    Prefab = 5,
}

impl ConfigType {
    pub fn to_proto(self) -> EntityConfigType {
        EntityConfigType::from_i32_or_default(self as i32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConfigComponent {
    pub camp: i32,
    pub config_id: i32,
    pub config_type: ConfigType,
    pub entity_type: EntityType,
    pub state: EntityState,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PositionComponent {
    pub location: [f32; 3],
    pub rotation: [f32; 3],
}

impl PositionComponent {
    pub fn to_proto(self) -> Transform {
        Transform {
            Pos: Some(Vector {
                X: self.location[0],
                Y: self.location[1],
                Z: self.location[2],
            }),
            Rot: Some(Rotator {
                Roll: self.rotation[0],
                Pitch: self.rotation[1],
                Yaw: self.rotation[2],
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GravityComponent {
    pub location: [f32; 3],
}

impl Default for GravityComponent {
    fn default() -> Self {
        Self {
            location: [0.0, 0.0, -1.0],
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BaseSkinComponent {
    pub role_skin_id: i32,
    pub paragliding_skin_id: i32,
    pub soar_skin_id: i32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CalabashSkinComponent {
    pub skin_id: i32,
}

impl CalabashSkinComponent {
    pub fn to_proto(self) -> CalabashSkinComponentPb {
        CalabashSkinComponentPb {
            CalabashSkinId: self.skin_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct WeaponSkinComponent {
    pub skin_id: i32,
}

impl WeaponSkinComponent {
    pub fn to_proto(self) -> WeaponSkinComponentPb {
        WeaponSkinComponentPb {
            WeaponSkinId: self.skin_id,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OrnamentComponent {
    pub ornament_ids: Vec<i32>,
}

impl OrnamentComponent {
    pub fn to_proto(&self) -> OrnamentComponentPb {
        OrnamentComponentPb {
            OrnamentIds: self.ornament_ids.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StateTagComponent {
    pub state_tag_id: i32,
}

impl StateTagComponent {
    pub fn to_proto(self) -> StateTagComponentPb {
        StateTagComponentPb {
            StateTagId: self.state_tag_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct EquipComponent {
    pub weapon_id: i32,
    pub weapon_breach_level: i32,
}

impl EquipComponent {
    pub fn to_proto(self) -> EquipComponentPb {
        EquipComponentPb {
            WeaponId: self.weapon_id,
            WeaponBreachLevel: self.weapon_breach_level,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FollowEntityComponent {
    pub entity_id: i64,
}

impl FollowEntityComponent {
    pub fn to_proto(self) -> FollowEntityComponentPb {
        FollowEntityComponentPb {
            EntityId: self.entity_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FollowShooterComponent {
    pub player_entity_id: i64,
    pub summon_config_id: i32,
}

impl FollowShooterComponent {
    pub fn to_proto(self) -> FollowShooterComponentPb {
        FollowShooterComponentPb {
            PlayerEntityId: self.player_entity_id,
            SummonConfigId: self.summon_config_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FollowerEntry {
    pub kind: i32,
    pub entity_id: i64,
}

#[derive(Debug, Clone, Default)]
pub struct FollowerComponent {
    pub list: Vec<FollowerEntry>,
}

impl FollowerComponent {
    pub fn to_proto(&self) -> FollowerComponentPb {
        FollowerComponentPb {
            FollowerList: self
                .list
                .iter()
                .map(|item| FollowerList {
                    Type: Some(FollowerType::from_i32_or_default(item.kind)),
                    EntityId: item.entity_id,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CharacterAttachComponent {
    pub pb_combine_part_info_list: Vec<CharacterAttachInfo>,
    pub pb_combine_target_server_id: i64,
}

impl CharacterAttachComponent {
    pub fn to_proto(&self) -> CharacterAttachComponentPb {
        CharacterAttachComponentPb {
            PbCombinePartInfoList: self.pb_combine_part_info_list.clone(),
            PbCombineTargetServerId: self.pb_combine_target_server_id,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConcomitantComponent {
    pub vision_entity_id: Vec<i64>,
    pub custom_entity_ids: Vec<i64>,
    pub phantom_role_id: i64,
    pub boss_rush_id: i64,
}

impl ConcomitantComponent {
    pub fn to_proto(&self) -> ConcomitantsComponentPb {
        ConcomitantsComponentPb {
            VisionEntityId: self.vision_entity_id.clone(),
            CustomEntityIds: self.custom_entity_ids.clone(),
            PhantomRoleId: self.phantom_role_id,
            BossRushId: self.boss_rush_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LiftComponent {
    pub location: i32,
}

impl LiftComponent {
    pub fn to_proto(self) -> LiftComponentPb {
        LiftComponentPb {
            Location: self.location,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MonsterAiComponent {
    pub weapon_id: i32,
    pub hatred_group_id: i64,
    pub ai_team_init_id: i32,
    pub combat_message_id: i64,
    pub hatred_id: i64,
    pub perception_ids: [i32; 8],
    pub perception_len: u8,
}

impl MonsterAiComponent {
    pub fn to_proto(&self) -> MonsterAiComponentPb {
        let len = usize::from(self.perception_len).min(self.perception_ids.len());
        MonsterAiComponentPb {
            WeaponId: self.weapon_id,
            HatredGroupId: self.hatred_group_id,
            AiTeamInitId: self.ai_team_init_id,
            CombatMessageId: self.combat_message_id,
            BasicPerceptionIds: self.perception_ids[..len]
                .iter()
                .copied()
                .filter(|&pid| pid != 0)
                .collect(),
            HatredId: self.hatred_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MonsterCaptureComponent {
    pub template_id: i32,
    pub entity_id: i32,
    pub monster_id: i32,
}

impl MonsterCaptureComponent {
    pub fn to_proto(self) -> MonsterCaptureComponentPb {
        MonsterCaptureComponentPb {
            TemplateId: self.template_id,
            EntityId: self.entity_id,
            MonsterId: self.monster_id,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MotorDaCtxComponent {
    pub context_id: i64,
}

impl MotorDaCtxComponent {
    pub fn to_proto(self) -> MotorDaCtxComponentPb {
        MotorDaCtxComponentPb {
            MotorDaCtxId: self.context_id,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MotorOutlookComponent {
    pub skin: i32,
    pub stickers: Vec<i32>,
    pub decorations: Vec<i32>,
    pub frame: i32,
}

impl MotorOutlookComponent {
    pub fn to_proto(&self) -> MotorDiyEquippedPb {
        MotorDiyEquippedPb {
            SkinEquipped: self.skin,
            StickerEquipped: self.stickers.clone(),
            DecorationsEquipped: self.decorations.clone(),
            FrameEquipped: self.frame,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MoveSplineComponent {
    pub move_status: Option<MoveStatus>,
    pub spline_entity_id: i32,
    pub runtime_data: Option<SceneItemSplineRuntimeData>,
    pub move_spline_config: Option<MoveSplineConfig>,
}

impl MoveSplineComponent {
    pub fn to_proto(&self) -> EntityMoveSplineComponentPb {
        EntityMoveSplineComponentPb {
            RuntimeData: self
                .runtime_data
                .clone()
                .map(EntityMoveSplineComponentPbRuntimeDataOneof::SceneItemSplineRuntimeData),
            SplineEntityId: self.spline_entity_id,
            MoveSplineConfig: self.move_spline_config.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MoveToPointComponent {
    pub target_pos: [f32; 3],
    pub move_type: i32,
    pub is_exact: bool,
    pub move_time: f32,
    pub stay_time: f32,
    pub max_speed: f32,
    pub acceleration: f32,
}

impl MoveToPointComponent {
    pub fn to_proto(self) -> MoveToPointComponentPb {
        MoveToPointComponentPb {
            PbMoveToPointConfig: Some(PbMoveToPointConfig {
                TargetPos: Some(Vector {
                    X: self.target_pos[0],
                    Y: self.target_pos[1],
                    Z: self.target_pos[2],
                }),
                MoveType: self.move_type,
                IsExact: self.is_exact,
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NpcDriveVehicleComponent {
    pub vehicle_entity_id: i64,
    pub seat: i32,
}

impl NpcDriveVehicleComponent {
    pub fn to_proto(self) -> NpcDriveVehicleComponentPb {
        NpcDriveVehicleComponentPb {
            VehicleCreatureId: self.vehicle_entity_id,
            Seat: self.seat,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PassiveGaSkillComponent {
    pub skill_component_pb: Vec<SkillComponentPb>,
}

impl PassiveGaSkillComponent {
    pub fn to_proto(&self) -> PassiveGaSkillComponentPb {
        PassiveGaSkillComponentPb {
            SkillInfoList: Vec::new(),
            SkillComponentPb: self.skill_component_pb.clone(),
        }
    }

    pub fn to_passive_skill_proto(&self, assets: &Assets, net_id: i64) -> PassiveSkillComponentPb {
        let mut list = Vec::new();
        for entry in &self.skill_component_pb {
            if assets.tables.passive_skill.get_data_by_id(&i64::from(entry.SkillId)).is_none() {
                continue;
            }
            list.push(PassiveSkillItemPb {
                CombatCommon: Some(CombatCommon {
                    EntityId: net_id,
                    MessageId: -1,
                    PreMessageId: -1,
                    ..Default::default()
                }),
                SkillId: i64::from(entry.SkillId),
            });
        }
        PassiveSkillComponentPb {
            PassiveSkillItemPbList: list,
        }
    }

    pub fn add_skill(&mut self, skill_id: i32) -> bool {
        if skill_id == 0 || self.skill_component_pb.iter().any(|s| s.SkillId == skill_id) {
            return false;
        }
        self.skill_component_pb.push(SkillComponentPb {
            SkillId: skill_id,
            ConstateId: -1,
        });
        true
    }

    pub fn remove_skill(&mut self, skill_id: i32) -> bool {
        let prev = self.skill_component_pb.len();
        self.skill_component_pb.retain(|s| s.SkillId != skill_id);
        self.skill_component_pb.len() != prev
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerBattleBinder {
    pub player_entity_id: i64,
    pub battle_scene_entity_id: i64,
}

impl PlayerBattleBinder {
    pub fn new(player_entity_id: i64, battle_scene_entity_id: i64) -> Self {
        Self {
            player_entity_id: player_entity_id.max(0),
            battle_scene_entity_id: battle_scene_entity_id.max(0),
        }
    }

    pub fn to_proto(self) -> PlayerSceneComponentPb {
        PlayerSceneComponentPb {
            EntityIds: vec![
                self.player_entity_id,
                self.battle_scene_entity_id,
                self.player_entity_id,
                self.battle_scene_entity_id,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SceneItemComponent {
    pub pos_sender: i32,
    pub manipulatable_state: Option<i32>,
}

impl SceneItemComponent {
    pub fn from_progress(info: &ProgressInfo, player_id: i32, config_id: i32) -> Self {
        let key = format!("item_bb:{config_id}:0");
        let val = info
            .dynamic
            .ints
            .iter()
            .find(|e| e.key == key)
            .and_then(|e| i32::try_from(e.val).ok());
        Self {
            pos_sender: player_id,
            manipulatable_state: val,
        }
    }

    pub fn to_proto(self) -> SceneItemComponentPb {
        let mut blackboards = Vec::new();
        if let Some(state) = self.manipulatable_state {
            blackboards.push(SceneItemBlackboardParam {
                Key: Some(SceneItemBBKey::ManipulatableState),
                Value: Some(SceneItemBlackboardParamValueOneof::IntValue(state)),
            });
        }
        SceneItemComponentPb {
            PosSender: self.pos_sender,
            BlackBoards: blackboards,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SummonerComponent {
    pub summoner_id: i64,
    pub summon_cfg_id: i32,
    pub summon_skill_id: i32,
    pub player_id: i32,
    pub summon_type: ESummonType,
}

impl Default for SummonerComponent {
    fn default() -> Self {
        Self {
            summoner_id: 0,
            summon_cfg_id: 0,
            summon_skill_id: 0,
            player_id: 0,
            summon_type: ESummonType::Default,
        }
    }
}

impl SummonerComponent {
    pub fn to_proto(self) -> SummonerComponentPb {
        SummonerComponentPb {
            SummonerId: self.summoner_id,
            SummonCfgId: self.summon_cfg_id,
            SummonSkillId: self.summon_skill_id,
            PlayerId: self.player_id,
            Type: Some(self.summon_type),
            AvatarCfgId: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct VehicleComponent {
    pub source: Option<VehicleSource>,
    pub occupied: bool,
}

impl VehicleComponent {
    pub fn to_proto(self) -> VehiclePb {
        VehiclePb {
            Source: self.source,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VisionSkillComponent {
    pub vision_skills: Vec<VisionSkillInformation>,
}

impl VisionSkillComponent {
    pub fn to_proto(&self) -> VisionSkillComponentPb {
        VisionSkillComponentPb {
            VisionSkillInfos: self.vision_skills.clone(),
            PhantomSkillInfo: None,
        }
    }
}
