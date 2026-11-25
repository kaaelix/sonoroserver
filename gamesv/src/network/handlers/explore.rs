use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::blueprint_config::EntityLogic;
use sonoro_gamesv_data::tables::buff::buff_list_from_ids;
use sonoro_gamesv_logic::component::entity::{
    CharacterAttachComponent, ConfigComponent, ConfigType, EntityComponentStorage, EntityState,
    EntityType, EquipComponent, FightBuffComponent, LogicStateComponent, MonsterAiComponent,
    PositionComponent, SummonerComponent,
};
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{entity_proto, phantom_projector, quest_engine};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_gamesv_logic::templates::role_entity_templates;
use sonoro_proto::enums::{ESummonType, ErrorCode, RouletteType};
use sonoro_proto::pb::{
    EnterAreaRequest, EnterAreaResponse, EntityAccessCasketRequest, EntityAccessCasketResponse,
    EntityAddNotify, EntityRemoveInfo, EntityRemoveNotify, ExploreEntityTraceRequest,
    ExploreEntityTraceResponse, ExploreSkillRoulette, ExploreSkillRouletteSetRequest,
    ExploreSkillRouletteSetResponse, ExploreSkillRouletteUpdateNotify, MoralePosRequest,
    MoralePosResponse, MoraleTreasureBoxTraceRequest, MoraleTreasureBoxTraceResponse,
    PlayerAccessCasketRequest, PlayerAccessCasketResponse, RemoveSummonEntityRequest,
    RemoveSummonEntityResponse, Summon3Request, Summon3Response, SummonRequest, SummonResponse,
    VisionExploreSkillSetRequest, VisionExploreSkillSetResponse, VisionSkillChangeNotify,
};

use super::State;

const ROULETTE_SLOT_COUNT: usize = 8;

fn roulette_skill_ids(ids: &[i32]) -> Vec<i32> {
    let mut out = vec![0i32; ROULETTE_SLOT_COUNT];
    let count = ids.len().min(ROULETTE_SLOT_COUNT);
    out[..count].copy_from_slice(&ids[..count]);
    out
}

pub fn handle_enter_area_request(
    state: &mut State,
    events: &mut EventQueue,
    req: EnterAreaRequest,
) -> Result<EnterAreaResponse, String> {
    if req.Id > 0 {
        if state.player_components.map.info.unlock_area(req.Id) {
            state
                .player_components
                .map
                .save(&state.fs)
                .map_err(|e| e.to_string())?;
        }
        if let Some(loc) = state
            .scene
            .as_ref()
            .and_then(|s| s.instance.players.first().map(|p| p.location))
        {
            let mut qctx = state.quest_context(events);
            if let Err(err) = quest_engine::on_player_move(&mut qctx, loc[0], loc[1], loc[2]) {
                tracing::warn!("on_player_move error: {err}");
            }
        }
    }
    Ok(EnterAreaResponse {
        ErrorCode: Some(ErrorCode::Success),
        Id: req.Id,
    })
}

pub fn handle_vision_explore_skill_set_request(
    state: &mut State,
    events: &mut EventQueue,
    req: VisionExploreSkillSetRequest,
) -> Result<VisionExploreSkillSetResponse, String> {
    let now_ms = quest_engine::tree::now_ms_pub();
    let State {
        conn,
        fs,
        assets,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let Some(scene) = scene_opt.as_mut() else {
        return Err("no active scene".to_string());
    };
    {
        let old_skill = scene.explore_tools_info.active_explore_skill;
        scene.explore_tools_info.active_explore_skill = req.SkillId;

        let target_ids: Vec<i64> = scene
            .entities
            .iter()
            .filter_map(|s| s.vision_skills.as_ref().map(|_| s.net_id))
            .collect();

        for net_id in target_ids {
            let mut infos = Vec::new();
            if let Some(s) = scene.entity_mut(net_id)
                && let Some(vs) = s.vision_skills.as_mut()
            {
                for skill in &mut vs.vision_skills {
                    if skill.SkillId == old_skill {
                        skill.SkillId = req.SkillId;
                    }
                }
                infos = vs.vision_skills.clone();
            }
            buff_timers.ensure_entity_registered(scene, assets, net_id, now_ms);
            buff_timers.sync_entity_left_durations(scene, net_id, now_ms);
            scene
                .save_entity_by_net_id(fs, net_id)
                .map_err(|e| e.to_string())?;
            conn.push(
                VisionSkillChangeNotify::MSG_ID,
                &VisionSkillChangeNotify {
                    EntityId: net_id,
                    VisionSkillInfos: infos,
                    PhantomSkillInfo: None,
                },
            );
        }
        scene.save_persistent(fs).map_err(|e| e.to_string())?;
    }

    if req.SkillId != 0 {
        let mut qctx = state.quest_context(events);
        if let Err(err) = quest_engine::on_vision_system_event(&mut qctx, "VisionSetActivated") {
            tracing::warn!("on_vision_system_event(VisionSetActivated) error: {err}");
        }
    }

    Ok(VisionExploreSkillSetResponse {
        SkillId: req.SkillId,
        ErrorCode: None,
    })
}

pub fn handle_explore_skill_roulette_set_request(
    state: &mut State,
    req: ExploreSkillRouletteSetRequest,
) -> Result<ExploreSkillRouletteSetResponse, String> {
    let now_ms = quest_engine::tree::now_ms_pub();
    let roulette_type = req.RouletteType.unwrap_or(RouletteType::Explore);
    let selected_roulette = req
        .SkillRoulette
        .clone()
        .or_else(|| req.SkillRoulettes.first().cloned());

    let State {
        conn,
        fs,
        assets,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let scene = scene_opt.as_mut().ok_or("no active scene")?;

    if let Some(ref roulette) = selected_roulette {
        let info = &mut scene.explore_tools_info;
        match roulette_type {
            RouletteType::Explore => {
                info.roulette = roulette_skill_ids(&roulette.SkillIds);
                info.explore_extra_item_id = roulette.ExtraItemId;
                info.active_explore_skill = roulette.ExploreSkill;
            }
            RouletteType::Function => {
                info.function_roulette = roulette_skill_ids(&roulette.SkillIds);
                info.function_extra_item_id = roulette.ExtraItemId;
                info.active_function_skill = roulette.ExploreSkill;
            }
            RouletteType::Motorcycle => {
                info.motorcycle_roulette = roulette_skill_ids(&roulette.SkillIds);
                info.motorcycle_extra_item_id = roulette.ExtraItemId;
                info.active_motorcycle_skill = roulette.ExploreSkill;
            }
            RouletteType::TrapDefense => {
                info.trap_defense_roulette = roulette_skill_ids(&roulette.SkillIds);
                info.trap_defense_extra_item_id = roulette.ExtraItemId;
                info.active_trap_defense_skill = roulette.ExploreSkill;
            }
        }
        buff_timers.ensure_all_registered(scene, assets, now_ms);
        buff_timers.sync_all_left_durations(scene, now_ms);
        scene.save_persistent(fs).map_err(|e| e.to_string())?;
    }

    let tools = &scene.explore_tools_info;
    let roulette_info = vec![
        ExploreSkillRoulette {
            SkillIds: roulette_skill_ids(&tools.roulette),
            ExtraItemId: tools.explore_extra_item_id,
            ExploreSkill: tools.active_explore_skill,
        },
        ExploreSkillRoulette {
            SkillIds: roulette_skill_ids(&tools.function_roulette),
            ExtraItemId: tools.function_extra_item_id,
            ExploreSkill: tools.active_function_skill,
        },
        ExploreSkillRoulette {
            SkillIds: roulette_skill_ids(&tools.trap_defense_roulette),
            ExtraItemId: tools.trap_defense_extra_item_id,
            ExploreSkill: tools.active_trap_defense_skill,
        },
        ExploreSkillRoulette {
            SkillIds: roulette_skill_ids(&tools.motorcycle_roulette),
            ExtraItemId: tools.motorcycle_extra_item_id,
            ExploreSkill: tools.active_motorcycle_skill,
        },
    ];

    conn.push(
        ExploreSkillRouletteUpdateNotify::MSG_ID,
        &ExploreSkillRouletteUpdateNotify {
            RouletteInfo: roulette_info.clone(),
        },
    );

    Ok(ExploreSkillRouletteSetResponse {
        ErrorCode: Some(ErrorCode::Success),
        SkillRoulette: selected_roulette,
        RouletteType: Some(roulette_type),
        SkillRoulettes: roulette_info,
    })
}

fn summon_entity_type_from_blueprint(assets: &Assets, blueprint_type: &str) -> EntityType {
    let Some(bp) = assets.find_blueprint_config(blueprint_type) else {
        return EntityType::Monster;
    };
    match bp.EntityLogic {
        EntityLogic::Vision => EntityType::Vision,
        EntityLogic::Monster => EntityType::Monster,
        EntityLogic::Npc => EntityType::Npc,
        EntityLogic::Vehicle => EntityType::Vehicle,
        EntityLogic::Item => EntityType::SceneItem,
        EntityLogic::ClientOnly => EntityType::ClientOnly,
        EntityLogic::Custom => EntityType::Custom,
        _ => EntityType::Monster,
    }
}

pub fn handle_summon_request(
    state: &mut State,
    req: SummonRequest,
) -> Result<SummonResponse, String> {
    let now_ms = quest_engine::tree::now_ms_pub();
    let Some(info) = req.SummonInfo else {
        return Ok(SummonResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    if info.SummonEntityId <= 0 {
        return Ok(SummonResponse {
            ErrorCode: Some(ErrorCode::ErrSummonAddEntityFail),
        });
    }
    let State {
        conn,
        assets,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let scene = scene_opt.as_mut().ok_or("no active scene")?;

    if scene.net_id_map.contains_key(&info.SummonEntityId) {
        return Ok(SummonResponse {
            ErrorCode: Some(ErrorCode::ErrSummonEntityIdAlreadyExist),
        });
    }
    let Some(summon_cfg) = assets
        .tables
        .summon_cfg
        .get_data_by_id(&info.SummonConfigId)
        .map(|r| r.into_owned())
    else {
        return Ok(SummonResponse {
            ErrorCode: Some(ErrorCode::ErrSummonCfgNotFound),
        });
    };

    let template_config = if !summon_cfg.BlueprintType.is_empty() {
        assets.find_template_config(&summon_cfg.BlueprintType)
    } else {
        None
    };
    let cfg_id = template_config.as_ref().map_or(summon_cfg.Id, |tc| tc.Id);
    let ent_type = if !summon_cfg.BlueprintType.is_empty() {
        summon_entity_type_from_blueprint(assets, &summon_cfg.BlueprintType)
    } else {
        EntityType::Monster
    };

    let attr_comp = scene
        .entity(req.SummonerEntityId)
        .and_then(|s| s.attribute.clone())
        .unwrap_or_default();
    let pos = info.Pos.unwrap_or_default();
    let rot = info.Rot.unwrap_or_default();

    let mut storage = EntityComponentStorage::new(
        info.SummonEntityId,
        ConfigComponent {
            camp: 0,
            state: EntityState::Default,
            entity_type: ent_type,
            config_type: if template_config.is_some() {
                ConfigType::Template
            } else {
                ConfigType::OldEntity
            },
            config_id: cfg_id,
        },
    );
    storage.player_id = Some(scene.player_id);
    storage.position = Some(PositionComponent {
        location: [pos.X, pos.Y, pos.Z],
        rotation: [rot.Roll, rot.Pitch, rot.Yaw],
    });
    storage.actor_visible = info.IsVisible;
    storage.attribute = Some(attr_comp);
    storage.summoner = Some(SummonerComponent {
        player_id: scene.player_id,
        summon_cfg_id: summon_cfg.Id,
        summon_skill_id: info.SkillId,
        summon_type: ESummonType::ConcomitantCustom,
        summoner_id: req.SummonerEntityId,
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

    let born_buffs: Vec<sonoro_gamesv_logic::events::BuffAdditionEntry> =
        buff_list_from_ids(&summon_cfg.BornBuffId)
            .into_iter()
            .map(|b| sonoro_gamesv_logic::events::BuffAdditionEntry {
                id: b.id,
                stack_count: b.stack_count,
                is_active: b.is_active,
                ..Default::default()
            })
            .collect();
    let mut buffs_comp = FightBuffComponent::default();
    if !born_buffs.is_empty() {
        buffs_comp.buffs = scene.build_fight_buff_infos(&born_buffs, info.SummonEntityId);
    }
    storage.buffs = Some(buffs_comp);

    let entity = match scene.spawn_with_net_id(info.SummonEntityId, storage) {
        Ok(e) => e,
        Err("EntityIdAlreadyExists") => {
            return Ok(SummonResponse {
                ErrorCode: Some(ErrorCode::ErrSummonEntityIdAlreadyExist),
            });
        }
        Err(err) => return Err(err.to_string()),
    };

    let pb = entity_proto::build(assets, scene, buff_timers, entity.net_id, now_ms)
        .map_err(|e| e.to_string())?;
    conn.push(
        EntityAddNotify::MSG_ID,
        &EntityAddNotify {
            RemoveTagIds: false,
            EntityPbs: vec![pb],
        },
    );
    Ok(SummonResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_summon3_request(
    state: &mut State,
    req: Summon3Request,
) -> Result<Summon3Response, String> {
    let now_ms = quest_engine::tree::now_ms_pub();
    let Some(info) = req.SummonInfo else {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    };
    if info.SummonEntityId <= 0 {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::ErrSummonAddEntityFail),
        });
    }
    let State {
        conn,
        assets,
        player_components,
        scene: scene_opt,
        buff_timers,
        ..
    } = state;
    let scene = scene_opt.as_mut().ok_or("no active scene")?;

    if scene.net_id_map.contains_key(&info.SummonEntityId) {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::ErrSummonEntityIdAlreadyExist),
        });
    }
    if !phantom_projector::is_unlocked_monster(assets, info.SummonConfigId) {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::ErrPhantomInteractionNotUnlock),
        });
    }
    let Some(item) = phantom_projector::displayed_phantom_item(
        assets,
        &player_components.echo.calabash_info,
        info.SummonConfigId,
    ) else {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::ErrPhantomInteractionConfigNotFind),
        });
    };
    let summon_id = if item.MeshId != 0 {
        item.MeshId
    } else {
        item.SkillId
    };
    if summon_id == 0 {
        return Ok(Summon3Response {
            ErrorCode: Some(ErrorCode::ErrSummonCfgNotFound),
        });
    }

    let pos = info.Pos.unwrap_or_default();
    let rot = info.Rot.unwrap_or_default();
    let entity = match role_entity_templates::create_projector_vision_entity(
        scene,
        scene.player_id,
        assets,
        summon_id,
        req.SummonerEntityId,
        info.SummonEntityId,
        [pos.X, pos.Y, pos.Z],
        [rot.Roll, rot.Pitch, rot.Yaw],
    ) {
        Ok(Some(e)) => e,
        Ok(None) => {
            return Ok(Summon3Response {
                ErrorCode: Some(ErrorCode::ErrSummonCfgNotFound),
            });
        }
        Err("EntityIdAlreadyExists") => {
            return Ok(Summon3Response {
                ErrorCode: Some(ErrorCode::ErrSummonEntityIdAlreadyExist),
            });
        }
        Err(err) => return Err(err.to_string()),
    };

    let pb = entity_proto::build(assets, scene, buff_timers, entity.net_id, now_ms)
        .map_err(|e| e.to_string())?;
    conn.push(
        EntityAddNotify::MSG_ID,
        &EntityAddNotify {
            RemoveTagIds: false,
            EntityPbs: vec![pb],
        },
    );
    Ok(Summon3Response {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_remove_summon_entity_request(
    state: &mut State,
    req: RemoveSummonEntityRequest,
) -> Result<RemoveSummonEntityResponse, String> {
    let State {
        conn,
        fs,
        scene: scene_opt,
        ..
    } = state;
    let scene = scene_opt.as_mut().ok_or("no active scene")?;

    let mut to_remove = Vec::new();
    for entity_id in req.RemoveENtityIds {
        let Some(summoner) = scene.entity(entity_id).and_then(|s| s.summoner) else {
            continue;
        };
        if summoner.summoner_id != req.SummonerId {
            continue;
        }
        if matches!(
            summoner.summon_type,
            ESummonType::ConcomitantCustom | ESummonType::ConcomitantVision
        ) {
            continue;
        }
        to_remove.push(entity_id);
    }

    let mut remove_infos = Vec::with_capacity(to_remove.len());
    for entity_id in to_remove {
        scene.remove(fs, entity_id).map_err(|e| e.to_string())?;
        remove_infos.push(EntityRemoveInfo {
            EntityId: entity_id,
            Type: None,
        });
    }

    if !remove_infos.is_empty() {
        conn.push(
            EntityRemoveNotify::MSG_ID,
            &EntityRemoveNotify {
                IsRemove: true,
                RemoveInfos: remove_infos,
            },
        );
    }

    Ok(RemoveSummonEntityResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_player_access_casket_request(
    state: &mut State,
    req: PlayerAccessCasketRequest,
) -> Result<PlayerAccessCasketResponse, String> {
    let val = if req.IsEnter { 1 } else { 0 };
    if !req.AccessEntityIncIds.is_empty() {
        for &inc_id in &req.AccessEntityIncIds {
            state
                .player_components
                .progress
                .info
                .set_key_int(&format!("casket:player:{inc_id}"), val);
        }
    } else if req.CasketEntityIncId != 0 {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("casket:player:{}", req.CasketEntityIncId), val);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let resp_inc = req
        .AccessEntityIncIds
        .first()
        .copied()
        .unwrap_or(req.CasketEntityIncId);
    Ok(PlayerAccessCasketResponse {
        ErrorCode: Some(ErrorCode::Success),
        CasketEntityIncId: resp_inc,
    })
}

pub fn handle_entity_access_casket_request(
    state: &mut State,
    req: EntityAccessCasketRequest,
) -> Result<EntityAccessCasketResponse, String> {
    let val = if req.IsEnter { 1 } else { 0 };
    if !req.AccessEntityIncIds.is_empty() {
        for &inc_id in &req.AccessEntityIncIds {
            state.player_components.progress.info.set_key_int(
                &format!("casket:ent:{}:{inc_id}", req.EntityIncId),
                val,
            );
        }
    } else {
        state
            .player_components
            .progress
            .info
            .set_key_int(&format!("casket:ent:{}", req.EntityIncId), val);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let resp_inc = req
        .AccessEntityIncIds
        .first()
        .copied()
        .unwrap_or(req.CasketEntityIncId);
    Ok(EntityAccessCasketResponse {
        ErrorCode: Some(ErrorCode::Success),
        CasketEntityIncId: resp_inc,
    })
}

pub fn handle_explore_entity_trace_request(
    state: &mut State,
    req: ExploreEntityTraceRequest,
) -> Result<ExploreEntityTraceResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("explore:trace_area", i64::from(req.AreaId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let mut eids = Vec::new();
    if req.ExploratoryDegree > 0 {
        if let Some(db) = state.assets.tables.assets_db() {
            let found = db
                .query_entity_ids_by_area_explore(req.AreaId, req.ExploratoryDegree)
                .map_err(|e| e.to_string())?;
            eids.extend(found.into_iter().take(32));
        } else {
            for lcfg in &state.assets.tables.level_entity_config.items {
                if req.AreaId != 0 && lcfg.AreaId != req.AreaId {
                    continue;
                }
                let Some(ref bi) = lcfg.ComponentsData.BaseInfoComponent else {
                    continue;
                };
                let Some(ref cat) = bi.Category else {
                    continue;
                };
                if cat.ExploratoryDegree.unwrap_or(0) != req.ExploratoryDegree {
                    continue;
                }
                if lcfg.EntityId > 0 && lcfg.EntityId <= i64::from(i32::MAX) {
                    eids.push(lcfg.EntityId as i32);
                }
                if eids.len() >= 32 {
                    break;
                }
            }
        }
    }
    if eids.is_empty() && req.ExploratoryDegree == 0 {
        for m in &state.assets.tables.map_mark.items {
            if (req.AreaId == 0 || m.RelativeId == req.AreaId)
                && m.EntityConfigId > 0
                && m.EntityConfigId <= i64::from(i32::MAX)
            {
                eids.push(m.EntityConfigId as i32);
                if eids.len() >= 32 {
                    break;
                }
            }
        }
    }

    Ok(ExploreEntityTraceResponse {
        EntityIds: eids,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_morale_pos_request(
    state: &mut State,
    _req: MoralePosRequest,
) -> Result<MoralePosResponse, String> {
    let pos = state
        .player_components
        .progress
        .info
        .get_key_int("morale:area_pos") as i32;
    Ok(MoralePosResponse { AreaPos: pos })
}

pub fn handle_morale_treasure_box_trace_request(
    state: &mut State,
    req: MoraleTreasureBoxTraceRequest,
) -> Result<MoraleTreasureBoxTraceResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("morale:area_pos", i64::from(req.AreaPos));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    let mut eids = Vec::new();
    for m in &state.assets.tables.map_mark.items {
        if m.EntityConfigId > 0
            && m.EntityConfigId <= i64::from(i32::MAX)
            && (req.AreaPos == 0 || m.RelativeId == req.AreaPos)
        {
            eids.push(m.EntityConfigId as i32);
            if eids.len() >= 16 {
                break;
            }
        }
    }

    Ok(MoraleTreasureBoxTraceResponse {
        ErrorCode: Some(ErrorCode::Success),
        AreaPos: req.AreaPos,
        EntityIds: eids,
    })
}
