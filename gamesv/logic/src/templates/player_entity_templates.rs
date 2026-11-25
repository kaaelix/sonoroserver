use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::buff_list_from_ids;
use sonoro_gamesv_data::tables::template_config::TemplateConfig;
use sonoro_proto::enums::{ESummonType, FollowerType};
use sonoro_proto::pb::CombatReceiveData;
use sonoro_proto::wire::PbEnum;

use crate::component::entity::small::{
    CharacterAttachComponent, ConfigComponent, ConfigType, EntityState, EntityType, EquipComponent,
    FollowEntityComponent, FollowShooterComponent, FollowerComponent, FollowerEntry,
    MonsterAiComponent, MotorDaCtxComponent, MotorOutlookComponent, NpcDriveVehicleComponent,
    PositionComponent, SummonerComponent, VehicleComponent,
};
use crate::component::entity::tag::TagEntry;
use crate::component::entity::{
    AttributeComponent, EntityComponentStorage, FightBuffComponent, LogicStateComponent,
    TagComponent,
};
use crate::component::player::PlayerMotorComponent;
use crate::event_queue::EventQueue;
use crate::events::{BuffAdditionEntry, EntityHandle, Event};
use crate::scene::{Scene, now_millis};

const MOTORCYCLE_COMPANION_SUMMON_CFG_ID: i32 = 24_000_030;
const MOTORCYCLE_COMPANION_TEMPLATE_ID: i32 = 36_700_001;
const MOTORCYCLE_COMPANION_PROPERTY_ID: i32 = 90_000_004;

const FORCED_MOTORCYCLE_BORN_BUFF_IDS: &[i64] = &[3_000_000_801, 3_000_020_024];
const FORCED_MOTORCYCLE_GAMEPLAY_TAG_IDS: &[i32] = &[-849_916_103];

#[derive(Debug, Clone, Copy)]
pub struct MotorcycleCompanionResult {
    pub entity: EntityHandle,
    pub created: bool,
}

pub fn create_player_scene_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    motorcycle_entity_id: i64,
) -> EntityHandle {
    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::PlayerEntity,
            config_type: ConfigType::Template,
            config_id: 14_750_001,
        },
    );
    storage.follower = Some(FollowerComponent {
        list: vec![
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerDefault.to_i32(),
                entity_id: 0,
            },
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerExploreSkill.to_i32(),
                entity_id: 0,
            },
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerAuxiliary.to_i32(),
                entity_id: 0,
            },
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerSpecialItem.to_i32(),
                entity_id: 0,
            },
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerMotor.to_i32(),
                entity_id: motorcycle_entity_id,
            },
            FollowerEntry {
                kind: FollowerType::EPlayerFollowerMax.to_i32(),
                entity_id: 0,
            },
        ],
    });
    storage.attribute = Some(AttributeComponent::create(
        &assets.tables.get_props(390_077_025),
    ));
    storage.buffs = Some(FightBuffComponent::default());
    storage.position = Some(PositionComponent {
        location: [-10000.0, -10000.0, -10000.0],
        rotation: [0.0, 0.0, 0.0],
    });
    storage.player_id = Some(player_id);
    storage.visible = true;
    storage.actor_visible = true;

    scene.spawn(storage)
}

pub fn create_motorcycle_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    motor_comp: &PlayerMotorComponent,
) -> Result<EntityHandle, &'static str> {
    let props = assets.tables.get_props(90_000_015);
    let motorcycle_config_id = assets
        .get_motorcycle_config_id()
        .ok_or("MotorcycleConfigNotFound")?;
    let born_buffs = build_motorcycle_born_buffs(assets);
    let gameplay_tags = build_motorcycle_gameplay_tags();

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::Vehicle,
            config_type: ConfigType::Template,
            config_id: motorcycle_config_id,
        },
    );
    storage.attribute = Some(AttributeComponent::create(&props));
    storage.vehicle = Some(VehicleComponent::default());
    storage.tag = Some(TagComponent {
        gameplay_tags,
        init_gameplay_tag: true,
        ..Default::default()
    });
    storage.motor_outlook = Some(MotorOutlookComponent {
        skin: motor_comp.info.equipped_skin,
        stickers: motor_comp.info.equipped_stickers.clone(),
        decorations: motor_comp.info.equipped_decorations.clone(),
        frame: motor_comp.info.equipped_frame,
    });
    storage.motor_da_ctx = Some(MotorDaCtxComponent::default());
    storage.buffs = Some(FightBuffComponent::default());
    storage.position = Some(PositionComponent {
        location: [-10000.0, -10000.0, -10000.0],
        rotation: [0.0, 0.0, 0.0],
    });
    storage.player_id = Some(player_id);
    storage.visible = true;
    storage.actor_visible = true;

    let entity = scene.spawn(storage);
    let infos = scene.build_fight_buff_infos(&born_buffs, entity.net_id);
    if let Some(buffs) = scene.entities[entity.index].buffs.as_mut() {
        buffs.buffs = infos;
    }
    Ok(entity)
}

pub fn find_player_scene_entity(scene: &Scene, player_id: i32) -> Option<EntityHandle> {
    for (i, e) in scene.entities.iter().enumerate() {
        if e.config.entity_type == EntityType::PlayerEntity && e.player_id == Some(player_id) {
            return Some(EntityHandle {
                index: i,
                net_id: e.net_id,
            });
        }
    }
    None
}

fn find_motorcycle_companion_entity(
    scene: &Scene,
    motorcycle_entity: EntityHandle,
) -> Option<EntityHandle> {
    let follow_entity = scene.entities.get(motorcycle_entity.index)?.follow_entity?;
    if follow_entity.entity_id == 0 {
        return None;
    }
    scene.entity_handle(follow_entity.entity_id)
}

pub fn ensure_motorcycle_companion_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    motorcycle_entity: EntityHandle,
    player_scene_entity: EntityHandle,
) -> Result<MotorcycleCompanionResult, &'static str> {
    if let Some(entity) = find_motorcycle_companion_entity(scene, motorcycle_entity) {
        return Ok(MotorcycleCompanionResult {
            entity,
            created: false,
        });
    }

    let summon_cfg = assets
        .tables
        .summon_cfg
        .get_data_by_id(&MOTORCYCLE_COMPANION_SUMMON_CFG_ID)
        .ok_or("MotorcycleCompanionSummonCfgNotFound")?;
    let template_config = assets
        .tables
        .template_config
        .get_data_by_id(&MOTORCYCLE_COMPANION_TEMPLATE_ID)
        .ok_or("MotorcycleCompanionTemplateNotFound")?;

    let born_buffs: Vec<BuffAdditionEntry> = buff_list_from_ids(&summon_cfg.BornBuffId)
        .into_iter()
        .map(|b| BuffAdditionEntry {
            id: b.id,
            stack_count: b.stack_count,
            is_active: b.is_active,
            ..Default::default()
        })
        .collect();

    let (location, rotation) = if let Some(first) = scene.instance.players.first() {
        (first.location, first.rotation)
    } else {
        ([-10000.0, -10000.0, -10000.0], [0.0, 0.0, 0.0])
    };

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::Monster,
            config_type: ConfigType::Template,
            config_id: template_config.Id,
        },
    );
    storage.player_id = Some(player_id);
    storage.position = Some(PositionComponent { location, rotation });
    storage.actor_visible = true;
    storage.buffs = Some(FightBuffComponent::default());
    storage.attribute = Some(AttributeComponent::create(
        &assets.tables.get_props(MOTORCYCLE_COMPANION_PROPERTY_ID),
    ));
    storage.summoner = Some(SummonerComponent {
        player_id,
        summon_cfg_id: summon_cfg.Id,
        summon_skill_id: 0,
        summon_type: ESummonType::ConcomitantMotorcycle,
        summoner_id: motorcycle_entity.net_id,
    });
    storage.follow_shooter = Some(FollowShooterComponent {
        player_entity_id: player_scene_entity.net_id,
        summon_config_id: summon_cfg.Id,
    });
    storage.logic_state = Some(LogicStateComponent::default());
    storage.monster_ai = Some(MonsterAiComponent {
        ai_team_init_id: 0,
        combat_message_id: 1,
        hatred_group_id: 0,
        weapon_id: 0,
        ..Default::default()
    });
    storage.equip = Some(EquipComponent::default());
    storage.character_attach = Some(CharacterAttachComponent::default());

    let entity = scene.spawn(storage);
    let infos = scene.build_fight_buff_infos(&born_buffs, entity.net_id);
    if let Some(buffs) = scene.entities[entity.index].buffs.as_mut() {
        buffs.buffs = infos;
    }
    scene.entities[motorcycle_entity.index].follow_entity = Some(FollowEntityComponent {
        entity_id: entity.net_id,
    });

    Ok(MotorcycleCompanionResult {
        entity,
        created: true,
    })
}

fn contains_ignore_case_ascii(haystack: &str, needle: &str) -> bool {
    if needle.len() < 3 || needle.len() > haystack.len() {
        return false;
    }
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn role_asset_token(ui_scene_performance_abp: &str) -> &str {
    let mut previous = "";
    for part in ui_scene_performance_abp.split('/') {
        if part.starts_with("R2T1") {
            return previous;
        }
        previous = part;
    }
    ""
}

fn passenger_template_matches_role(assets: &Assets, blueprint_type: &str, role_id: i32) -> bool {
    let role_text = role_id.to_string();
    if blueprint_type.ends_with(&role_text) {
        let start = blueprint_type.len() - role_text.len();
        if start > 0 && blueprint_type.as_bytes()[start - 1] == b'_' {
            return true;
        }
    }
    let needle = format!("_{role_text}_");
    if blueprint_type.contains(&needle) {
        return true;
    }
    let Some(role) = assets.tables.role_info.get_data_by_id(&role_id) else {
        return false;
    };
    contains_ignore_case_ascii(
        blueprint_type,
        role_asset_token(&role.UiScenePerformanceABP),
    )
}

fn is_npc_passenger_blueprint(assets: &Assets, blueprint_type: &str) -> bool {
    use sonoro_gamesv_data::tables::blueprint_config::EntityLogic;
    let Some(bp) = assets.find_blueprint_config(blueprint_type) else {
        return false;
    };
    bp.EntityLogic == EntityLogic::Npc && bp.BlueprintType.contains("Passenger")
}

fn find_motorcycle_passenger_template(
    assets: &Assets,
    role_id: i32,
) -> Option<TemplateConfig> {
    if let Some(db) = assets.tables.assets_db()
        && let Ok(rows) = db.query_passenger_blueprint_types()
    {
        let mut matched_bp = None;
        for bp in rows {
            if !passenger_template_matches_role(assets, &bp, role_id) {
                continue;
            }
            if !is_npc_passenger_blueprint(assets, &bp) {
                continue;
            }
            matched_bp = Some(bp);
        }
        let bp = matched_bp?;
        return assets.find_template_config(&bp).map(|c| c.into_owned());
    }

    let mut candidate = None;
    for template in &assets.tables.template_config.items {
        if !passenger_template_matches_role(assets, &template.BlueprintType, role_id) {
            continue;
        }
        if !is_npc_passenger_blueprint(assets, &template.BlueprintType) {
            continue;
        }
        candidate = Some(template.clone());
    }
    candidate
}

pub fn find_motorcycle_passenger_entity(scene: &Scene, assets: &Assets) -> Option<EntityHandle> {
    for (i, e) in scene.entities.iter().enumerate() {
        if e.config.entity_type != EntityType::Npc || e.config.config_type != ConfigType::Template {
            continue;
        }
        let Some(template) = assets.tables.template_config.get_data_by_id(&e.config.config_id)
        else {
            continue;
        };
        if !is_npc_passenger_blueprint(assets, &template.BlueprintType) {
            continue;
        }
        return Some(EntityHandle {
            index: i,
            net_id: e.net_id,
        });
    }
    None
}

pub fn create_motorcycle_passenger_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    role_id: i32,
    motorcycle_entity: EntityHandle,
    seat: i32,
) -> Result<EntityHandle, &'static str> {
    let template = find_motorcycle_passenger_template(assets, role_id)
        .ok_or("MotorcyclePassengerTemplateNotFound")?;
    let base_info = template.ComponentsData.BaseInfoComponent.as_ref();

    let (location, rotation) = if let Some(first) = scene.instance.players.first() {
        (first.location, first.rotation)
    } else {
        ([-10000.0, -10000.0, -10000.0], [0.0, 0.0, 0.0])
    };
    let property_id = base_info.and_then(|i| i.EntityPropertyId).unwrap_or(2);

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: base_info.and_then(|i| i.Camp).unwrap_or(0),
            state: EntityState::Default,
            entity_type: EntityType::Npc,
            config_type: ConfigType::Template,
            config_id: template.Id,
        },
    );
    storage.player_id = Some(player_id);
    storage.position = Some(PositionComponent { location, rotation });
    storage.visible = true;
    storage.actor_visible = true;
    storage.buffs = Some(FightBuffComponent::default());
    storage.attribute = Some(AttributeComponent::create(
        &assets.tables.get_props(property_id),
    ));
    storage.npc_drive_vehicle = Some(NpcDriveVehicleComponent {
        vehicle_entity_id: motorcycle_entity.net_id,
        seat,
    });

    Ok(scene.spawn(storage))
}

pub fn build_motorcycle_born_buffs(assets: &Assets) -> Vec<BuffAdditionEntry> {
    FORCED_MOTORCYCLE_BORN_BUFF_IDS
        .iter()
        .copied()
        .filter(|id| assets.tables.buff.get_data_by_id(id).is_some())
        .map(|id| BuffAdditionEntry {
            id,
            is_active: true,
            ..Default::default()
        })
        .collect()
}

pub fn build_motorcycle_gameplay_tags() -> Vec<TagEntry> {
    FORCED_MOTORCYCLE_GAMEPLAY_TAG_IDS
        .iter()
        .copied()
        .map(|id| TagEntry { id, count: 1 })
        .collect()
}

pub fn refresh_motorcycle_born_buffs(
    events: &mut EventQueue,
    scene: &mut Scene,
    assets: &Assets,
    entity: EntityHandle,
) {
    let Some(storage) = scene.entities.get_mut(entity.index) else {
        return;
    };

    if let Some(buff_comp) = storage.buffs.as_ref() {
        let desired = build_motorcycle_born_buffs(assets);
        let add_buffs: Vec<BuffAdditionEntry> = desired
            .iter()
            .copied()
            .filter(|entry| buff_comp.get_by_buff_id(entry.id).is_none())
            .collect();
        let remove_handles: Vec<i32> = buff_comp
            .buffs
            .iter()
            .filter(|info| {
                FORCED_MOTORCYCLE_BORN_BUFF_IDS.contains(&info.BuffId)
                    && !desired.iter().any(|d| d.id == info.BuffId)
            })
            .map(|info| info.HandleId)
            .collect();

        if !add_buffs.is_empty() {
            events.enqueue(Event::BuffAddition {
                target: entity,
                instigator: entity,
                buffs: add_buffs,
            });
        }
        if !remove_handles.is_empty() {
            events.enqueue(Event::BuffRemoval {
                entity,
                handle_ids: remove_handles,
                natural_expiration: None,
            });
        }
    }

    if let Some(tag_comp) = storage.tag.as_mut() {
        refresh_motorcycle_gameplay_tags(events, entity, tag_comp);
    }
}

fn refresh_motorcycle_gameplay_tags(
    events: &mut EventQueue,
    entity: EntityHandle,
    tag_comp: &mut TagComponent,
) {
    let desired = build_motorcycle_gameplay_tags();
    let add_tag_ids: Vec<i32> = desired
        .iter()
        .filter(|entry| tag_comp.tag_count(entry.id) == 0)
        .map(|entry| entry.id)
        .collect();
    let remove_tag_ids: Vec<i32> = tag_comp
        .gameplay_tags
        .iter()
        .filter(|entry| {
            FORCED_MOTORCYCLE_GAMEPLAY_TAG_IDS.contains(&entry.id)
                && !desired.iter().any(|d| d.id == entry.id && d.count > 0)
        })
        .map(|entry| entry.id)
        .collect();

    for &tag_id in &add_tag_ids {
        tag_comp.add_tag(tag_id, 1);
    }
    for &tag_id in &remove_tag_ids {
        tag_comp.remove_tag(tag_id);
    }

    if !add_tag_ids.is_empty() || !remove_tag_ids.is_empty() {
        events.enqueue(Event::GameplayTagChange {
            entity,
            add_tag_ids,
            remove_tag_ids,
            remove_before_add: false,
        });
    }
}

pub fn create_scene_battle_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
) -> EntityHandle {
    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::SceneEntity,
            config_type: ConfigType::Template,
            config_id: 14_000_169,
        },
    );
    storage.attribute = Some(AttributeComponent::create(
        &assets.tables.get_props(390_077_025),
    ));
    storage.buffs = Some(FightBuffComponent::default());
    storage.position = Some(PositionComponent {
        location: [-10000.0, -10000.0, -10000.0],
        rotation: [0.0, 0.0, 0.0],
    });
    storage.player_id = Some(player_id);
    storage.visible = true;
    storage.actor_visible = true;

    let _ = now_millis();
    let _unused: Option<&CombatReceiveData> = None;
    let _ = _unused;
    scene.spawn(storage)
}
