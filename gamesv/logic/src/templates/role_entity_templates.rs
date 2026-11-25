use std::collections::HashSet;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::buff::buff_list_from_ids;
use sonoro_gamesv_data::tables::blueprint_config::{BlueprintConfig, EntityType as BpEntityType};
use sonoro_gamesv_data::tables::instance_dungeon::InstanceDungeon;
use sonoro_gamesv_data::tables::role_be_hit_map::RoleBeHitMap;
use sonoro_gamesv_data::tables::summon_cfg::SummonCfg;
use sonoro_gamesv_data::tables::template_config::TemplateConfig;
use sonoro_gamesv_data::data_tables::WeaponEquipInfo;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_proto::enums::ESummonType;
use sonoro_proto::pb::{SkillComponentPb, VisionSkillInformation};

use crate::component::entity::fsm::FsmComponent;
use crate::component::entity::small::{
    BaseSkinComponent, CalabashSkinComponent, CharacterAttachComponent, ConcomitantComponent,
    ConfigComponent, ConfigType, EntityState, EntityType, EquipComponent, MonsterAiComponent,
    OrnamentComponent, PassiveGaSkillComponent, PlayerBattleBinder, PositionComponent,
    SummonerComponent, VisionSkillComponent, WeaponSkinComponent,
};
use crate::component::entity::{
    AttributeComponent, EntityComponentStorage, FightBuffComponent, LogicStateComponent,
    TagComponent,
};
use crate::component::player::{PlayerEchoComponent, PlayerRoleComponent, PlayerWeaponComponent};
use crate::events::{BuffAdditionEntry, EntityHandle};
use crate::helpers::{attributes, cosmetics, role_stats};
use crate::scene::{Scene, now_millis};

fn append_passive_skill(skills: &mut Vec<SkillComponentPb>, skill_id: i32) {
    if skill_id == 0 || skills.iter().any(|s| s.SkillId == skill_id) {
        return;
    }
    skills.push(SkillComponentPb {
        SkillId: skill_id,
        ConstateId: -1,
    });
}

fn append_vision_passive_skills(
    assets: &Assets,
    skills: &mut Vec<SkillComponentPb>,
    summon_ids: &[i32],
    vision_skills: &[VisionSkillInformation],
) {
    let mut has_vision_summon = false;
    for &summon_id in summon_ids {
        if summon_id == 0 {
            continue;
        }
        has_vision_summon = true;
        append_vision_entity_passive_skills(assets, skills, summon_id);
    }
    if !has_vision_summon {
        return;
    }

    for skill in vision_skills {
        if skill.Index >= 5 {
            continue;
        }
        append_passive_skill(skills, skill.SkillId);
    }

    for entry in &assets.tables.common_skill_preload.items {
        if !entry.IsCommon {
            continue;
        }
        let is_vision_skill = entry.Others.iter().any(|path| {
            path.contains("/Character/Vision/GA/GA_")
                || path.contains("/Character/Vision/GA/Role/GA_Role_Bianshen.")
                || path.contains("/Character/Vision/GA/Role/GA_Role_Zhaohuan.")
        });
        if is_vision_skill {
            append_passive_skill(skills, entry.Id);
        }
    }
}

fn append_vision_entity_passive_skills(
    assets: &Assets,
    skills: &mut Vec<SkillComponentPb>,
    summon_id: i32,
) {
    let Some(actor_path) = vision_summon_actor_path(assets, summon_id) else {
        return;
    };
    if let Some(db) = assets.tables.assets_db() {
        if let Ok(skill_ids) = db.query_skill_ids_by_actor_blueprint(&actor_path) {
            for sid in skill_ids {
                append_passive_skill(skills, sid);
            }
        }
    } else {
        for entry in &assets.tables.entity_skill_preload.items {
            if !entry.ActorBlueprint.is_empty() && entry.ActorBlueprint == actor_path {
                append_passive_skill(skills, entry.SkillId);
            }
        }
    }
}

fn vision_summon_actor_path(assets: &Assets, summon_id: i32) -> Option<String> {
    let summon_cfg = assets.tables.summon_cfg.get_data_by_id(&summon_id)?;
    let template_config = assets.find_template_config(&summon_cfg.BlueprintType)?;
    let model_comp = template_config.ComponentsData.ModelComponent.as_ref()?;
    let model_id = model_comp.ModelType.ModelId;
    if model_id == 0 {
        return None;
    }
    let model = assets.tables.model_config_preload.get_data_by_id(&model_id)?;
    Some(model.ActorClassPath.clone())
}

fn player_or_born_pose(scene: &Scene, instance_dungeon: &InstanceDungeon) -> ([f32; 3], [f32; 3]) {
    if let Some(first) = scene.instance.players.first() {
        return (first.location, first.rotation);
    }
    let bp = &instance_dungeon.BornPosition;
    let br = &instance_dungeon.BornRotation;
    let loc = [
        bp.first().copied().unwrap_or(0) as f32,
        bp.get(1).copied().unwrap_or(0) as f32,
        bp.get(2).copied().unwrap_or(0) as f32,
    ];
    let rot = [
        br.first().copied().unwrap_or(0) as f32,
        br.get(1).copied().unwrap_or(0) as f32,
        br.get(2).copied().unwrap_or(0) as f32,
    ];
    (loc, rot)
}

fn set_entity_born_buffs(
    scene: &mut Scene,
    entity: EntityHandle,
    born_buffs: &[BuffAdditionEntry],
) {
    let infos = scene.build_fight_buff_infos(born_buffs, entity.net_id);
    if let Some(buffs) = scene.entities[entity.index].buffs.as_mut() {
        buffs.buffs = infos;
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_concomitant_entity(
    scene: &mut Scene,
    player_id: i32,
    role_comp: &PlayerRoleComponent,
    instance_dungeon: &InstanceDungeon,
    summon_cfg: &SummonCfg,
    template_config: &TemplateConfig,
    role: i32,
    summoner_id: i64,
) -> Result<EntityHandle, &'static str> {
    let role_info = role_comp.role_map.get(&role).ok_or("RoleNotFound")?;
    let born_buffs: Vec<BuffAdditionEntry> = buff_list_from_ids(&summon_cfg.BornBuffId)
        .into_iter()
        .map(|b| BuffAdditionEntry {
            id: b.id,
            stack_count: b.stack_count,
            is_active: b.is_active,
            ..Default::default()
        })
        .collect();
    let (location, rotation) = player_or_born_pose(scene, instance_dungeon);

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
    storage.attribute = Some(AttributeComponent::create(&role_info.base_prop));
    storage.summoner = Some(SummonerComponent {
        player_id,
        summon_cfg_id: summon_cfg.Id,
        summon_skill_id: 0,
        summon_type: ESummonType::ConcomitantCustom,
        summoner_id,
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
    set_entity_born_buffs(scene, entity, &born_buffs);
    Ok(entity)
}

pub fn create_vision_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    role_info: &RoleInfo,
    instance_dungeon: &InstanceDungeon,
    summon_skill_id: i32,
    summoner_id: i64,
) -> Option<EntityHandle> {
    let summon_cfg = assets.tables.summon_cfg.get_data_by_id(&summon_skill_id)?;
    if summon_cfg.BlueprintType.is_empty() {
        return None;
    }
    let template_config = assets.find_template_config(&summon_cfg.BlueprintType)?;
    let (location, rotation) = player_or_born_pose(scene, instance_dungeon);

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::Vision,
            config_type: ConfigType::Template,
            config_id: template_config.Id,
        },
    );
    storage.player_id = Some(player_id);
    storage.position = Some(PositionComponent { location, rotation });
    storage.actor_visible = true;
    storage.buffs = Some(FightBuffComponent::default());
    storage.attribute = Some(AttributeComponent::create(&role_info.base_prop));
    storage.summoner = Some(SummonerComponent {
        player_id,
        summon_cfg_id: summon_cfg.Id,
        summon_skill_id,
        summon_type: ESummonType::ConcomitantVision,
        summoner_id,
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
    storage.vision_skills = Some(VisionSkillComponent::default());

    let entity = scene.spawn(storage);
    let born_buffs: Vec<BuffAdditionEntry> = buff_list_from_ids(&summon_cfg.BornBuffId)
        .into_iter()
        .map(|b| BuffAdditionEntry {
            id: b.id,
            stack_count: b.stack_count,
            is_active: b.is_active,
            ..Default::default()
        })
        .collect();
    set_entity_born_buffs(scene, entity, &born_buffs);
    Some(entity)
}

#[allow(clippy::too_many_arguments)]
pub fn create_projector_vision_entity(
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    summon_skill_id: i32,
    summoner_id: i64,
    net_id: i64,
    pos: [f32; 3],
    rot: [f32; 3],
) -> Result<Option<EntityHandle>, &'static str> {
    let Some(summon_cfg) = assets.tables.summon_cfg.get_data_by_id(&summon_skill_id) else {
        return Ok(None);
    };
    if summon_cfg.BlueprintType.is_empty() {
        return Ok(None);
    }
    let Some(template_config) = assets.find_template_config(&summon_cfg.BlueprintType) else {
        return Ok(None);
    };

    let mut passive_skills = Vec::new();
    append_vision_passive_skills(assets, &mut passive_skills, &[summon_skill_id], &[]);

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::Vision,
            config_type: ConfigType::Template,
            config_id: template_config.Id,
        },
    );
    storage.player_id = Some(player_id);
    storage.position = Some(PositionComponent {
        location: pos,
        rotation: rot,
    });
    storage.actor_visible = true;
    storage.buffs = Some(FightBuffComponent::default());
    storage.attribute = Some(AttributeComponent::default());
    storage.summoner = Some(SummonerComponent {
        player_id,
        summon_cfg_id: summon_cfg.Id,
        summon_skill_id,
        summon_type: ESummonType::ConcomitantVision,
        summoner_id,
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
    storage.vision_skills = Some(VisionSkillComponent::default());
    storage.passive_ga_skill = Some(PassiveGaSkillComponent {
        skill_component_pb: passive_skills,
    });

    let entity = scene.spawn_with_net_id(net_id, storage)?;
    let born_buffs: Vec<BuffAdditionEntry> = buff_list_from_ids(&summon_cfg.BornBuffId)
        .into_iter()
        .map(|b| BuffAdditionEntry {
            id: b.id,
            stack_count: b.stack_count,
            is_active: b.is_active,
            ..Default::default()
        })
        .collect();
    set_entity_born_buffs(scene, entity, &born_buffs);
    Ok(Some(entity))
}

#[allow(clippy::too_many_arguments)]
pub fn create_role_entity(
    fs: &FileSystem,
    scene: &mut Scene,
    player_id: i32,
    assets: &Assets,
    role_comp: &mut PlayerRoleComponent,
    weapon_comp: &PlayerWeaponComponent,
    echo_comp: &PlayerEchoComponent,
    instance_dungeon: &InstanceDungeon,
    role: i32,
) -> Result<EntityHandle, &'static str> {
    let role_info = role_comp.role_map.get(&role).ok_or("RoleNotFound")?.clone();
    let resolved_role_id = assets.tables.resolve_role_id(role);
    let role_config = assets.tables.resolve_base_role_info(role);
    let chain_group_id = role_config
        .as_ref()
        .map_or(resolved_role_id, |rc| rc.ResonantChainGroupId);

    let weapon = weapon_comp
        .weapon_map
        .get(&role_info.weapon)
        .ok_or("WeaponNotFound")?;

    let mut role_base_buffs: Vec<BuffAdditionEntry> = assets
        .tables
        .get_role_auto_buffs(
            role,
            WeaponEquipInfo {
                id: weapon.id,
                reson_level: weapon.reson_level,
            },
            role_info.skill_branch,
        )?
        .into_iter()
        .map(|b| BuffAdditionEntry {
            id: b.id,
            stack_count: b.stack_count,
            is_active: b.is_active,
            ..Default::default()
        })
        .collect();

    for chain in &assets.tables.resonant_chain.items {
        if chain.GroupId != chain_group_id || chain.GroupIndex > role_info.resonant_chain_group_index
        {
            continue;
        }
        for &buff_id in &chain.BuffIds {
            role_base_buffs.push(BuffAdditionEntry {
                id: buff_id,
                is_active: true,
                ..Default::default()
            });
        }
    }

    let ornament_ids = cosmetics::build_ornament_ids_for_role_skin(&role_info, role_info.role_skin_id);
    let ornament_buffs = cosmetics::build_ornament_buffs_for_ids(assets, &ornament_ids);
    role_base_buffs.extend(ornament_buffs);

    let active_echo_buffs = echo_comp.active_echo_buff_effects(assets, role);
    for &buff_id in &active_echo_buffs {
        role_base_buffs.push(BuffAdditionEntry {
            id: buff_id,
            is_active: true,
            ..Default::default()
        });
    }

    let ornament_born_buff_ids =
        cosmetics::build_ornament_born_buff_ids_for_ids(assets, &ornament_ids);

    let formation_idx = usize::try_from(scene.formation_info.cur_formation)
        .ok()
        .filter(|&i| i < scene.formation_info.formations.len())
        .unwrap_or(0);
    let is_cur_role = scene.formation_info.formations[formation_idx].cur_role == role;

    let be_hit_entry = assets
        .tables
        .role_be_hit_map
        .get_data_by_id(&resolved_role_id);
    let be_hit_state_machine = RoleBeHitMap::state_machine_name(be_hit_entry.as_deref());

    let role_snapshot =
        role_stats::build_snapshot(assets, role_comp, weapon_comp, echo_comp, role)?;

    let mut role_attrs = AttributeComponent::create_with_increments(
        &role_snapshot.base_props,
        &role_snapshot.add_props,
    );
    if let Some(saved_hp) = role_info.current_hp {
        let _ = attributes::set_life_unconditional(&mut role_attrs, saved_hp);
    }

    let mut player_entity_id = 2i64;
    let mut battle_scene_entity_id = 4i64;
    for e in &scene.entities {
        if e.config.entity_type == EntityType::PlayerEntity {
            player_entity_id = e.net_id;
        }
        if e.config.entity_type == EntityType::SceneEntity {
            battle_scene_entity_id = e.net_id;
        }
    }

    let now_ms = now_millis();
    let (location, rotation) = if let Some(first) = scene.instance.players.first() {
        (first.location, first.rotation)
    } else {
        ([0.0; 3], [0.0; 3])
    };

    let mut storage = EntityComponentStorage::new(
        0,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: EntityType::Player,
            config_type: ConfigType::Character,
            config_id: role,
        },
    );
    storage.player_battle_binder =
        Some(PlayerBattleBinder::new(player_entity_id, battle_scene_entity_id));
    storage.tag = Some(TagComponent::default());
    storage.buffs = Some(FightBuffComponent::default());
    storage.position = Some(PositionComponent { location, rotation });
    storage.player_id = Some(player_id);
    storage.actor_visible = true;
    storage.direct_control = true;
    storage.attribute = Some(role_attrs);
    storage.concomitant = Some(ConcomitantComponent::default());
    storage.passive_ga_skill = Some(PassiveGaSkillComponent::default());
    storage.fsm = FsmComponent::from_state_machine_id(assets, be_hit_state_machine, now_ms)?;
    storage.vision_skills = Some(VisionSkillComponent::default());
    storage.equip = Some(EquipComponent {
        weapon_id: weapon.id,
        weapon_breach_level: weapon.breach,
    });
    storage.logic_state = Some(LogicStateComponent::default());
    storage.base_skin = Some(BaseSkinComponent {
        role_skin_id: role_info.role_skin_id,
        paragliding_skin_id: role_info.paragliding_skin_id,
        soar_skin_id: role_info.soar_skin_id,
    });
    storage.weapon_skin = Some(WeaponSkinComponent {
        skin_id: role_info.weapon_skin_id,
    });
    storage.ornament = Some(OrnamentComponent { ornament_ids });
    storage.calabash_skin = Some(CalabashSkinComponent {
        skin_id: role_info.calabash_skin_id,
    });

    let entity = scene.spawn(storage);

    let mut role_vision_entity_id = 0i64;
    let mut combo_vision_entity_id = 0i64;
    let mut role_vision_summon_id = 0i32;
    let mut combo_vision_summon_id = 0i32;
    if let Some(summon_id) = echo_comp.main_echo_summon_id(assets, role) {
        role_vision_summon_id = summon_id;
        if let Some(vision_entity) = create_vision_entity(
            scene,
            player_id,
            assets,
            &role_info,
            instance_dungeon,
            summon_id,
            entity.net_id,
        ) {
            role_vision_entity_id = vision_entity.net_id;
        }
    }
    if let Some(summon_id) = echo_comp.combo_echo_summon_id(assets, role) {
        combo_vision_summon_id = summon_id;
        if let Some(vision_entity) = create_vision_entity(
            scene,
            player_id,
            assets,
            &role_info,
            instance_dungeon,
            summon_id,
            entity.net_id,
        ) {
            combo_vision_entity_id = vision_entity.net_id;
        }
    }

    let mut blueprint_configs: Vec<&BlueprintConfig> = Vec::new();
    for cfg in &assets.tables.blueprint_config.items {
        if cfg.EntityType != BpEntityType::Monster {
            continue;
        }
        if !cfg.BlueprintType.contains("Player") || !cfg.BlueprintType.contains('_') {
            continue;
        }
        let Some(role_skin) = assets
            .tables
            .role_skin
            .items
            .iter()
            .find(|r| r.RoleId == resolved_role_id)
        else {
            continue;
        };
        let Some(model) = assets.tables.model_config_preload.get_data_by_id(&cfg.ModelId) else {
            continue;
        };
        let Some(model_name) = role_skin.UiScenePerformanceABP.split('/').nth(6) else {
            continue;
        };
        if !model
            .ActorClassPath
            .to_ascii_lowercase()
            .contains(&model_name.to_ascii_lowercase())
        {
            continue;
        }
        blueprint_configs.push(cfg);
    }

    let mut concomitants: Vec<EntityHandle> = Vec::new();
    let mut concomitant_buff_ids: HashSet<i64> = HashSet::new();
    for bp_config in blueprint_configs {
        let Some(summon_cfg) = assets
            .tables
            .summon_cfg
            .items
            .iter()
            .find(|s| s.BlueprintType == bp_config.BlueprintType)
        else {
            continue;
        };
        let Some(template_config) = assets.find_template_config(&bp_config.BlueprintType) else {
            continue;
        };
        for &buff_id in &summon_cfg.BornBuffId {
            concomitant_buff_ids.insert(buff_id);
        }
        let count = SummonCfg::get_concom_count(resolved_role_id).unwrap_or(1);
        for _ in 0..count {
            concomitants.push(create_concomitant_entity(
                scene,
                player_id,
                role_comp,
                instance_dungeon,
                summon_cfg,
                &template_config,
                role,
                entity.net_id,
            )?);
        }
    }

    let mut fight_buff_infos = scene.build_fight_buff_infos(&role_base_buffs, entity.net_id);
    fight_buff_infos.sort_by_key(|b| b.BuffId);
    fight_buff_infos.retain(|b| !concomitant_buff_ids.contains(&b.BuffId));

    let vision_skills = echo_comp.build_vision_skills(
        assets,
        role,
        role_vision_entity_id,
        combo_vision_entity_id,
        scene.explore_tools_info.active_explore_skill,
    );

    let mut passive_skills = Vec::new();
    let role_str = resolved_role_id.to_string();
    if let Some(db) = assets.tables.assets_db() {
        if let Ok(skill_ids) = db.query_skill_ids_by_role_prefix(resolved_role_id) {
            for sid in skill_ids {
                append_passive_skill(&mut passive_skills, sid);
            }
        }
    } else {
        for entry in &assets.tables.entity_skill_preload.items {
            if entry.SkillId.to_string().starts_with(&role_str) {
                append_passive_skill(&mut passive_skills, entry.SkillId);
            }
        }
    }

    for buff_info in &fight_buff_infos {
        let Some(buff) = assets.tables.buff.get_data_by_id(&buff_info.BuffId) else {
            continue;
        };
        if buff.ExtraEffectID != 35 {
            continue;
        }
        let Some(skill_id_list) = buff.ExtraEffectParameters.first() else {
            continue;
        };
        for s in skill_id_list.split('|') {
            if let Ok(id) = s.trim().parse::<i32>() {
                append_passive_skill(&mut passive_skills, id);
            }
        }
    }

    append_passive_skill(&mut passive_skills, 100_005);
    append_vision_passive_skills(
        assets,
        &mut passive_skills,
        &[role_vision_summon_id, combo_vision_summon_id],
        &vision_skills,
    );

    let storage = &mut scene.entities[entity.index];
    if let Some(concomitant) = storage.concomitant.as_mut() {
        if role_vision_entity_id != 0 && combo_vision_entity_id != 0 {
            concomitant.vision_entity_id = vec![role_vision_entity_id, combo_vision_entity_id];
        } else if role_vision_entity_id != 0 {
            concomitant.vision_entity_id = vec![role_vision_entity_id];
        }
        concomitant.custom_entity_ids = concomitants.iter().map(|e| e.net_id).collect();
    }
    if let Some(vs) = storage.vision_skills.as_mut() {
        vs.vision_skills = vision_skills;
    }
    if let Some(buff_comp) = storage.buffs.as_mut() {
        buff_comp.buffs = fight_buff_infos;
        let handles_to_mark: Vec<i32> = buff_comp
            .buffs
            .iter()
            .filter(|b| b.IsActive && active_echo_buffs.contains(&b.BuffId))
            .map(|b| b.HandleId)
            .collect();
        for handle_id in handles_to_mark {
            buff_comp.mark_prebaked_handle(handle_id);
        }
        buff_comp.client_born_buff_ids = ornament_born_buff_ids;
        buff_comp.client_born_message_id = entity.net_id;
    }
    if let Some(passive) = storage.passive_ga_skill.as_mut() {
        passive.skill_component_pb = passive_skills;
    }
    if is_cur_role {
        storage.visible = true;
    }
    let _ = storage.save(fs, player_id, scene.instance_id);
    Ok(entity)
}
