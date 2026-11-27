use std::collections::BTreeMap;

use sonoro_gamesv_data::inline_tables::PayGiftCfg;
use sonoro_gamesv_data::reward_index::composite_key;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::component::player::PlayerProgressComponent;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::{consume, energy, pb_kv, quest_engine, reward};
use sonoro_proto::enums::{BattlePassPayStatus, BattlePassType, ErrorCode, PayUpdateType};
use sonoro_proto::pb::{
    BattlePassEnterPush, BattlePassExpUpdateNotify, BattlePassLevelUpRequest,
    BattlePassLevelUpResponse, BattlePassRecurringTakeRequest, BattlePassRecurringTakeResponse,
    BattlePassRequest, BattlePassResponse, BattlePassTakeAllRewardRequest,
    BattlePassTakeAllRewardResponse, BattlePassTakeRewardRequest, BattlePassTakeRewardResponse,
    BattlePassTaskRequest, BattlePassTaskResponse, BattlePassTaskTakeRequest,
    BattlePassTaskTakeResponse, BirthDayCardRoleRequest, BirthDayCardRoleResponse,
    BirthDayRewardRequest, BirthDayRewardResponse, ClientStorageInfoDataOneof,
    EffectiveWeekCardInfoNotify,
    GachaAccumulateGroupInfo, GachaAccumulateInfo, GachaAccumulateInfoRequest,
    GachaAccumulateInfoResponse, GachaAccumulateRewardInfo, GachaAccumulateRewardRequest,
    GachaAccumulateRewardResponse, InfluenceInfo, InfluenceInfoRequest, InfluenceInfoResponse,
    InfluenceRewardRequest, InfluenceRewardResponse, MonthCardRequest, MonthCardResponse,
    PayGiftInfo, PayGiftInfoRequest, PayGiftInfoResponse, PayGiftRequest, PayGiftResponse,
    PayGiftShopInfo, PayGiftSuccessNotify, PayItemRequest, PayItemResponse,
    PayItemSuccessNotify, PayShopDirectBuyNotify, PayShopDirectBuyRequest,
    PayShopDirectBuyResponse, PbBattlePass, PbBattlePassReward, PbBattlePassTask,
    RoleFavorUnLockRequest, RoleFavorUnLockResponse, RoleGiftRewardRequest,
    RoleGiftRewardResponse, RoleMotionUnLockRequest, RoleMotionUnLockResponse, RoleTrainRequest,
    RoleTrainResponse, RoleUpdateDevelopTargetRequest, RoleUpdateDevelopTargetResponse,
    ShortMessageChatCharmChangeRequest, ShortMessageChatCharmChangeResponse,
    ShortMessageChatThemeChangeRequest, ShortMessageChatThemeChangeResponse, ShortMessageInfo,
    ShortMessageInfoRequest, ShortMessageInfoResponse, ShortMessageReadRequest,
    ShortMessageReadResponse, ShortMessageReceivedRequest, ShortMessageReceivedResponse,
    ShortMessageReplyRequest, ShortMessageReplyResponse, ShortMessageShowChangeRequest,
    ShortMessageShowChangeResponse, ShortMessageUpdateRequest, ShortMessageUpdateResponse,
    StorageInfoRequest, StorageInfoResponse, StorageInfoUpdateRequest, StorageInfoUpdateResponse,
    TotalTopUpRewardRequest, TotalTopUpRewardResponse, TotalTopUpScoreInfo,
    TotalTopUpScoreInfoRequest, TotalTopUpScoreInfoResponse, WeekCardContentInfo, WeekCardInfo,
    WeekCardInfoRequest, WeekCardInfoResponse, WeekCardRewardRequest, WeekCardRewardResponse,
};

use super::State;
use sonoro_gamesv_logic::sink::PushSink;

pub fn handle_role_favor_unlock_request(
    state: &mut State,
    req: RoleFavorUnLockRequest,
) -> Result<RoleFavorUnLockResponse, String> {
    let itype = req.ItemType.map_or(0, |t| t as i32);
    let k = format!("favor_unlock:{}:{itype}:{}", req.RoleId, req.UnLockId);
    let _ = state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleFavorUnLockResponse {
        RoleId: req.RoleId,
        ItemType: req.ItemType,
        UnLockId: req.UnLockId,
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_gift_reward_request(
    state: &mut State,
    req: RoleGiftRewardRequest,
) -> Result<RoleGiftRewardResponse, String> {
    let k = format!("role_gift_clm:{}", req.TaskId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 40,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 10000,
                },
            ],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(RoleGiftRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_motion_unlock_request(
    state: &mut State,
    req: RoleMotionUnLockRequest,
) -> Result<RoleMotionUnLockResponse, String> {
    let k = format!("role_motion:{}:{}", req.RoleId, req.UnLockId);
    let _ = state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleMotionUnLockResponse {
        RoleId: req.RoleId,
        UnLockId: req.UnLockId,
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_train_request(
    state: &mut State,
    req: RoleTrainRequest,
) -> Result<RoleTrainResponse, String> {
    let k = format!("role_train:{}:{}", req.RoleId, req.TrainType);
    let _ = state.player_components.progress.info.add_key_int(&k, 1);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleTrainResponse {
        GreatSucc: true,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_role_update_develop_target_request(
    state: &mut State,
    req: RoleUpdateDevelopTargetRequest,
) -> Result<RoleUpdateDevelopTargetResponse, String> {
    let k = format!("role_dev_target:{}", req.RoleId);
    state
        .player_components
        .progress
        .info
        .set_key_int(&k, i64::from(req.PlanId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(RoleUpdateDevelopTargetResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn build_short_message_infos(
    assets: &Assets,
    progress_comp: &PlayerProgressComponent,
) -> Vec<ShortMessageInfo> {
    let rows = assets.reward_index.get_rows("ShortMessage");
    let mut infos = Vec::new();
    for row in rows.iter() {
        let cid = row.key as i32;
        if (38000..=38999).contains(&cid) {
            continue;
        }
        if progress_comp.info.has_claim(&format!("sm:del:{cid}")) {
            continue;
        }
        let unlocked = progress_comp.info.has_claim(&format!("sm:unlock:{cid}"))
            || progress_comp.info.has_claim(&format!("sm:read:{cid}"))
            || progress_comp.info.has_claim(&format!("sm:recv:{cid}"))
            || progress_comp.info.has_claim(&format!("sm:fin:{cid}"));
        if !unlocked {
            continue;
        }
        let saved_last = progress_comp.info.get_key_int(&format!("sm:last:{cid}"));
        let last_cid = if saved_last > 0 { saved_last as i32 } else { cid };
        let unlock_time = progress_comp.info.get_key_int(&format!("sm:time:{cid}"));
        infos.push(ShortMessageInfo {
            ConfigId: cid,
            LastConfigId: last_cid,
            IsRead: progress_comp.info.has_claim(&format!("sm:read:{cid}")),
            IsReceived: progress_comp.info.has_claim(&format!("sm:recv:{cid}")),
            IsFinish: progress_comp.info.has_claim(&format!("sm:fin:{cid}")),
            UnlockTime: if unlock_time > 0 { unlock_time } else { 1 },
            ..Default::default()
        });
    }
    infos
}

pub fn handle_short_message_info_request(
    state: &mut State,
    _req: ShortMessageInfoRequest,
) -> Result<ShortMessageInfoResponse, String> {
    let pc = &state.player_components.progress;
    let infos = build_short_message_infos(&state.assets, pc);
    let charm_saved = pc.info.get_key_int("sm:charm");
    let theme_saved = pc.info.get_key_int("sm:theme");
    let bg_saved = pc.info.get_key_int("sm:bg");
    let bubble_saved = pc.info.get_key_int("sm:bubble");

    Ok(ShortMessageInfoResponse {
        ShortMessageInfos: infos,
        ChatCharmId: if charm_saved > 0 { charm_saved as i32 } else { 80892001 },
        ChatCharmIds: vec![80892001, 80892002],
        ChatThemeId: if theme_saved > 0 { theme_saved as i32 } else { 80891001 },
        ChatThemeIds: vec![80891001, 80891002],
        ChatBgId: if bg_saved > 0 { bg_saved as i32 } else { 80820001 },
        ChatBgIds: vec![80820001, 80820002, 80820004],
        BubbleId: if bubble_saved > 0 { bubble_saved as i32 } else { 80810001 },
        BubbleIds: vec![80810001, 80810002, 80810003],
        ErrCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_short_message_read_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ShortMessageReadRequest,
) -> Result<ShortMessageReadResponse, String> {
    let k = format!("sm:read:{}", req.ConfigId);
    let _ = state.player_components.progress.info.mark_claim(&k);
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if req.ConfigId > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_phone_message_read(&mut qctx, req.ConfigId);
    }
    Ok(ShortMessageReadResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_received_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ShortMessageReceivedRequest,
) -> Result<ShortMessageReceivedResponse, String> {
    let k = format!("sm:recv:{}", req.ConfigId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_table_reward(
            &mut rctx,
            "ShortMessage",
            i64::from(req.ConfigId),
            false,
            true,
            19500,
            &[reward::RewardItem {
                item_id: 3,
                count: 20,
            }],
            None,
        )
        .map_err(|e| e.to_string())?;
    }
    if req.ConfigId > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_phone_message_event(&mut qctx, req.ConfigId, req.ConfigId);
    }
    Ok(ShortMessageReceivedResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_reply_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ShortMessageReplyRequest,
) -> Result<ShortMessageReplyResponse, String> {
    let k = format!("sm:opt:{}:{}", req.ConfigId, req.CurOptionId);
    state
        .player_components
        .progress
        .info
        .set_key_int(&k, i64::from(req.SelectId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if req.ConfigId > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_phone_message_event(&mut qctx, req.ConfigId, req.ConfigId);
    }
    Ok(ShortMessageReplyResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_update_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ShortMessageUpdateRequest,
) -> Result<ShortMessageUpdateResponse, String> {
    if req.IsFinish {
        let _ = state
            .player_components
            .progress
            .info
            .mark_claim(&format!("sm:fin:{}", req.ConfigId));
    }
    state
        .player_components
        .progress
        .info
        .set_key_int(&format!("sm:last:{}", req.ConfigId), i64::from(req.LastConfigId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if req.IsFinish && req.ConfigId > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_phone_message_event(&mut qctx, req.ConfigId, req.ConfigId);
    }
    Ok(ShortMessageUpdateResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_chat_charm_change_request(
    state: &mut State,
    req: ShortMessageChatCharmChangeRequest,
) -> Result<ShortMessageChatCharmChangeResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("sm:charm", i64::from(req.ChatCharmId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(ShortMessageChatCharmChangeResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_chat_theme_change_request(
    state: &mut State,
    req: ShortMessageChatThemeChangeRequest,
) -> Result<ShortMessageChatThemeChangeResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("sm:theme", i64::from(req.ChatThemeId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(ShortMessageChatThemeChangeResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_short_message_show_change_request(
    state: &mut State,
    req: ShortMessageShowChangeRequest,
) -> Result<ShortMessageShowChangeResponse, String> {
    state
        .player_components
        .progress
        .info
        .set_key_int("sm:bg", i64::from(req.ChatBgId));
    state
        .player_components
        .progress
        .info
        .set_key_int("sm:bubble", i64::from(req.BubbleId));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(ShortMessageShowChangeResponse {
        ErrCode: Some(ErrorCode::Success),
    })
}

pub fn handle_birthday_card_role_request(
    state: &mut State,
    req: BirthDayCardRoleRequest,
) -> Result<BirthDayCardRoleResponse, String> {
    let k = format!("birthday:role:{}", req.YearId);
    state
        .player_components
        .progress
        .info
        .set_key_int(&k, i64::from(req.RoleID));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(BirthDayCardRoleResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_birthday_reward_request(
    state: &mut State,
    req: BirthDayRewardRequest,
) -> Result<BirthDayRewardResponse, String> {
    let k = format!("birthday:clm:{}", req.YearId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .info
            .set_key_int("birthday:reward_year", i64::from(req.YearId));
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 160,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 20000,
                },
            ],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(BirthDayRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn current_battle_pass_id(assets: &Assets) -> i32 {
    let rows = assets.reward_index.get_rows("BattlePass");
    rows.last().map_or(2, |r| r.key as i32)
}

fn get_battle_pass_item(assets: &Assets, bp_id: i32, level: i32, is_pay: bool) -> i32 {
    let keys = [
        composite_key(bp_id, level),
        composite_key(13, level),
        composite_key(0, level),
        composite_key(1, level),
    ];
    for k in keys {
        if let Some(row) = assets.reward_index.get_row("BattlePassReward", k) {
            if is_pay {
                if let Some(first) = row.pay_items.first() {
                    return first.item_id;
                }
            } else if let Some(first) = row.items.first() {
                return first.item_id;
            }
        }
    }
    0
}

pub fn handle_battle_pass_request(
    state: &mut State,
    _req: BattlePassRequest,
) -> Result<BattlePassResponse, String> {
    let now_s = energy::now_seconds();
    let bp_id = current_battle_pass_id(&state.assets);
    let pc = &state.player_components.progress;
    let lvl = (pc.info.get_key_int("bp:level") as i32).max(1);
    let exp = pc.info.get_key_int("bp:exp").max(0) as i32;
    let paid_raw = pc.info.get_key_int("bp:paid");
    let pay_status = if paid_raw >= 2 {
        BattlePassPayStatus::Advanced
    } else if paid_raw >= 1 {
        BattlePassPayStatus::Paid
    } else {
        BattlePassPayStatus::NoPaid
    };

    let mut taken = Vec::new();
    for l in 1..=70 {
        if pc.info.has_claim(&format!("bp:clm:0:{l}")) {
            taken.push(PbBattlePassReward {
                Level: l,
                Type: Some(BattlePassType::Free),
                ItemId: get_battle_pass_item(&state.assets, bp_id, l, false),
            });
        }
        if pc.info.has_claim(&format!("bp:clm:1:{l}")) {
            taken.push(PbBattlePassReward {
                Level: l,
                Type: Some(BattlePassType::Pay),
                ItemId: get_battle_pass_item(&state.assets, bp_id, l, true),
            });
        }
    }

    Ok(BattlePassResponse {
        ErrorCode: Some(ErrorCode::Success),
        BattlePass: Some(PbBattlePass {
            InTimeRange: true,
            Id: bp_id,
            Level: lvl,
            Exp: exp,
            WeeklyTotalExp: exp,
            PayStatus: Some(pay_status),
            TakenRewards: taken,
            BeginTime: now_s - 86_400,
            EndTime: now_s + 30 * 86_400,
            RecurringRewards: Vec::new(),
            HadEnter: pc.info.has_claim("bp:entered"),
        }),
    })
}

pub fn handle_battle_pass_enter_push(
    state: &mut State,
    _req: BattlePassEnterPush,
) -> Result<(), String> {
    let _ = state.player_components.progress.info.mark_claim("bp:entered");
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())
}

pub fn handle_battle_pass_level_up_request(
    state: &mut State,
    req: BattlePassLevelUpRequest,
) -> Result<BattlePassLevelUpResponse, String> {
    let cur = (state.player_components.progress.info.get_key_int("bp:level") as i32).max(1);
    let requested = if req.Level > 0 { req.Level } else { cur + 1 };
    let next = requested.clamp(cur, 70);
    let levels_gained = next - cur;
    if levels_gained <= 0 {
        return Ok(BattlePassLevelUpResponse {
            ErrorCode: Some(ErrorCode::Success),
        });
    }

    let costs = [consume::CostItem {
        id: 3,
        count: levels_gained * 100,
    }];
    let ok = {
        let State {
            conn,
            fs,
            player_components,
            ..
        } = state;
        consume::consume_costs(
            conn,
            fs,
            &mut player_components.basic,
            &mut player_components.inventory,
            &costs,
        )
        .map_err(|e| e.to_string())?
    };
    if !ok {
        return Ok(BattlePassLevelUpResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
        });
    }

    state
        .player_components
        .progress
        .info
        .set_key_int("bp:level", i64::from(next));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    state.conn.push(
        BattlePassExpUpdateNotify::MSG_ID,
        &BattlePassExpUpdateNotify {
            Level: next,
            Exp: 0,
            WeeklyTotalExp: next * 1000,
            RecurringRewards: Vec::new(),
        },
    );
    Ok(BattlePassLevelUpResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn grant_battle_pass_level_reward(
    state: &mut State,
    bp_id: i32,
    level: i32,
    is_pay: bool,
) -> Result<bool, String> {
    let keys = [
        composite_key(bp_id, level),
        composite_key(0, level),
        composite_key(1, level),
    ];
    for k in keys {
        if state
            .assets
            .reward_index
            .get_row("BattlePassReward", k)
            .is_some()
        {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "BattlePassReward",
                k,
                is_pay,
                false,
                18500,
                &[reward::RewardItem {
                    item_id: 2,
                    count: 10000,
                }],
                None,
            )
            .map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn handle_battle_pass_take_reward_request(
    state: &mut State,
    req: BattlePassTakeRewardRequest,
) -> Result<BattlePassTakeRewardResponse, String> {
    let t_val = req.Type.map_or(0, |t| t as i32);
    let is_pay = t_val == 1;
    let cur_lvl = (state.player_components.progress.info.get_key_int("bp:level") as i32).clamp(1, 70);
    let paid = state.player_components.progress.info.get_key_int("bp:paid") != 0;
    if req.Level < 1 || req.Level > cur_lvl || (is_pay && !paid) {
        return Ok(BattlePassTakeRewardResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }
    let k = format!("bp:clm:{t_val}:{}", req.Level);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let bp_id = current_battle_pass_id(&state.assets);
        if !grant_battle_pass_level_reward(state, bp_id, req.Level, is_pay)? {
            let mut rctx = state.reward_context();
            reward::grant_items(
                &mut rctx,
                0,
                true,
                &[reward::RewardItem {
                    item_id: 2,
                    count: 10000,
                }],
                18500,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(BattlePassTakeRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_battle_pass_take_all_reward_request(
    state: &mut State,
    _req: BattlePassTakeAllRewardRequest,
) -> Result<BattlePassTakeAllRewardResponse, String> {
    let cur_lvl = (state.player_components.progress.info.get_key_int("bp:level") as i32).clamp(1, 70);
    let paid = state.player_components.progress.info.get_key_int("bp:paid") != 0;
    let bp_id = current_battle_pass_id(&state.assets);
    let mut taken = Vec::new();
    let mut dirty = false;

    for l in 1..=cur_lvl {
        for tp in [0i32, 1i32] {
            if tp == 1 && !paid {
                continue;
            }
            let k = format!("bp:clm:{tp}:{l}");
            if state.player_components.progress.info.mark_claim(&k) {
                dirty = true;
                let _ = grant_battle_pass_level_reward(state, bp_id, l, tp == 1)?;
                let it_id = get_battle_pass_item(&state.assets, bp_id, l, tp == 1);
                taken.push(PbBattlePassReward {
                    Level: l,
                    Type: Some(if tp == 1 {
                        BattlePassType::Pay
                    } else {
                        BattlePassType::Free
                    }),
                    ItemId: it_id,
                });
            }
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(BattlePassTakeAllRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        TakenRewards: taken,
        ..Default::default()
    })
}

pub fn handle_battle_pass_recurring_take_request(
    state: &mut State,
    req: BattlePassRecurringTakeRequest,
) -> Result<BattlePassRecurringTakeResponse, String> {
    let bp_id = current_battle_pass_id(&state.assets);
    let cur_lvl = state.player_components.progress.info.get_key_int("bp:level").max(1) as i32;
    let cur_exp = state.player_components.progress.info.get_key_int("bp:exp").max(0) as i32;
    let bp_cfg_opt = state.assets.tables.battle_pass.get_data_by_id(&bp_id);
    let bp_cfg = bp_cfg_opt
        .as_deref()
        .or_else(|| state.assets.tables.battle_pass.items.last());
    let level_limit = bp_cfg.map_or(70, |c| c.LevelLimit.max(1));
    if cur_lvl < level_limit {
        return Ok(BattlePassRecurringTakeResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }

    let is_pay = req.Type.map_or(0, |t| t as i32) == 1;
    let paid = state.player_components.progress.info.get_key_int("bp:paid") != 0;
    if is_pay && !paid {
        return Ok(BattlePassRecurringTakeResponse {
            ErrorCode: Some(ErrorCode::RequestParamError),
        });
    }

    let step_exp = bp_cfg
        .as_ref()
        .map_or(1000, |c| c.RecurringLevelExp.max(1));
    let earned_steps = ((cur_lvl - level_limit).max(0) + (cur_exp / step_exp)).max(1);
    let t_val = if is_pay { 1 } else { 0 };
    let mut granted = Vec::new();
    if let Some(cfg) = bp_cfg {
        let map = if is_pay {
            &cfg.PayRecurringReward
        } else {
            &cfg.FreeRecurringReward
        };
        if req.ItemId > 0
            && let Some(&cnt) = map.iter().find(|(k, _)| **k == req.ItemId).map(|(_, v)| v)
            && cnt > 0
        {
            granted.push(reward::RewardItem {
                item_id: req.ItemId,
                count: cnt,
            });
        } else {
            for (&item_id, &cnt) in map.iter() {
                if item_id > 0 && cnt > 0 {
                    granted.push(reward::RewardItem { item_id, count: cnt });
                }
            }
        }
    }
    if granted.is_empty() {
        granted.push(reward::RewardItem {
            item_id: if req.ItemId > 0 { req.ItemId } else { 2 },
            count: 5000,
        });
    }

    let mut dirty = false;
    for step in 1..=earned_steps {
        let k = format!("bp:rec:{t_val}:{step}");
        if state.player_components.progress.info.mark_claim(&k) {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_items(&mut rctx, 0, true, &granted, 18500)
                .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(BattlePassRecurringTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_battle_pass_task_request(
    state: &mut State,
    _req: BattlePassTaskRequest,
) -> Result<BattlePassTaskResponse, String> {
    let now_s = energy::now_seconds();
    let rows = state.assets.reward_index.get_rows("BattlePassTask");
    let tasks = rows
        .iter()
        .filter_map(|row| {
            let tid = row.key as i32;
            (tid > 0).then(|| PbBattlePassTask {
                Id: tid,
                Current: 1,
                Target: 1,
                IsFinished: true,
                IsTaken: state
                    .player_components
                    .progress
                    .info
                    .has_claim(&format!("bp:task:{tid}")),
            })
        })
        .collect();
    Ok(BattlePassTaskResponse {
        ErrorCode: Some(ErrorCode::Success),
        Tasks: tasks,
        DayEnd: now_s + 86_400,
        WeekEnd: now_s + 7 * 86_400,
    })
}

pub fn handle_battle_pass_task_take_request(
    state: &mut State,
    req: BattlePassTaskTakeRequest,
) -> Result<BattlePassTaskTakeResponse, String> {
    let mut taken_ids = Vec::new();
    let mut exp_gain = 0i32;

    for tid in req.Ids {
        let k = format!("bp:task:{tid}");
        if !state.player_components.progress.info.mark_claim(&k) {
            continue;
        }
        taken_ids.push(tid);
        if let Some(row) = state.assets.reward_index.get_row("BattlePassTask", i64::from(tid)) {
            for it in &row.items {
                exp_gain += it.count;
            }
        } else {
            exp_gain += 250;
        }
    }

    if !taken_ids.is_empty() {
        let mut cur_lvl =
            (state.player_components.progress.info.get_key_int("bp:level") as i32).max(1);
        let mut cur_exp =
            state.player_components.progress.info.get_key_int("bp:exp").max(0) as i32 + exp_gain;
        while cur_exp >= 1000 && cur_lvl < 70 {
            cur_exp -= 1000;
            cur_lvl += 1;
        }
        state
            .player_components
            .progress
            .info
            .set_key_int("bp:level", i64::from(cur_lvl));
        state
            .player_components
            .progress
            .info
            .set_key_int("bp:exp", i64::from(cur_exp));
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        state.conn.push(
            BattlePassExpUpdateNotify::MSG_ID,
            &BattlePassExpUpdateNotify {
                Level: cur_lvl,
                Exp: cur_exp,
                WeeklyTotalExp: cur_lvl * 1000 + cur_exp,
                RecurringRewards: Vec::new(),
            },
        );
    }

    Ok(BattlePassTaskTakeResponse {
        ErrorCode: Some(ErrorCode::Success),
        Ids: taken_ids,
    })
}

pub fn handle_month_card_request(
    state: &mut State,
    _req: MonthCardRequest,
) -> Result<MonthCardResponse, String> {
    let mut days = state
        .player_components
        .progress
        .info
        .get_key_int("month_card:days")
        .max(0) as i32;
    let mut dirty = false;
    if days <= 0
        && state
            .player_components
            .progress
            .info
            .mark_claim("month_card:init")
    {
        days = 30;
        state
            .player_components
            .progress
            .info
            .set_key_int("month_card:days", i64::from(days));
        dirty = true;
    }
    let day_bucket = energy::now_seconds().div_euclid(86_400);
    let last_got_day = state
        .player_components
        .progress
        .info
        .get_key_int("month_card:last_day");
    let is_got_today = last_got_day == day_bucket;
    let mut granted_today = false;
    if !is_got_today && days > 0 {
        days -= 1;
        state
            .player_components
            .progress
            .info
            .set_key_int("month_card:days", i64::from(days));
        state
            .player_components
            .progress
            .info
            .set_key_int("month_card:last_day", day_bucket);
        dirty = true;
        granted_today = true;
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    if granted_today {
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            false,
            &[reward::RewardItem {
                item_id: 3,
                count: 90,
            }],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(MonthCardResponse {
        Days: days,
        IsDailyGot: is_got_today || granted_today,
        ErrorCode: Some(ErrorCode::Success),
    })
}

const DAY_MS: i64 = 86_400_000;

fn week_card_meta(assets: &Assets, wid: i32) -> (i32, i32) {
    for row in assets.raw_json_cached("WeekCard").iter() {
        if row.get("Id").and_then(|v| v.as_i64()) == Some(i64::from(wid)) {
            let pay_gift_id = row
                .get("PayGiftId")
                .and_then(|v| v.as_i64())
                .and_then(|v| i32::try_from(v).ok())
                .unwrap_or(if wid == 2 { 48 } else { 47 });
            let days = row
                .get("Days")
                .and_then(|v| v.as_i64())
                .and_then(|v| i32::try_from(v).ok())
                .filter(|&d| d > 0)
                .unwrap_or(15);
            return (pay_gift_id, days);
        }
    }
    match wid {
        2 => (48, 15),
        _ => (47, 15),
    }
}

fn week_card_id_for_gift(assets: &Assets, gift_id: i32) -> Option<(i32, i32)> {
    for row in assets.raw_json_cached("WeekCard").iter() {
        if row.get("PayGiftId").and_then(|v| v.as_i64()) == Some(i64::from(gift_id)) {
            let wid = row
                .get("Id")
                .and_then(|v| v.as_i64())
                .and_then(|v| i32::try_from(v).ok())?;
            let days = row
                .get("Days")
                .and_then(|v| v.as_i64())
                .and_then(|v| i32::try_from(v).ok())
                .filter(|&d| d > 0)
                .unwrap_or(15);
            return Some((wid, days));
        }
    }
    match gift_id {
        47 => Some((1, 15)),
        48 => Some((2, 15)),
        _ => None,
    }
}

fn week_card_default_rewards() -> BTreeMap<i32, i32> {
    BTreeMap::from([(3, 100), (2, 15_000)])
}

fn build_week_card_info(state: &State, wid: i32, now_ms: i64) -> WeekCardInfo {
    let (pay_gift_id, days) = week_card_meta(&state.assets, wid);
    let info = &state.player_components.progress.info;
    let stored_buy_ms = info.get_key_int(&format!("week_card:buy_ms:{wid}"));
    let stored_end_ms = info.get_key_int(&format!("week_card:end_ms:{wid}"));
    let legacy_bought = info.get_key_int(&format!("paygift:{pay_gift_id}")) > 0;

    let (buy_ms, end_ms) = if stored_end_ms > 0 {
        let buy = if stored_buy_ms > 0 {
            stored_buy_ms
        } else {
            now_ms
        };
        (buy, stored_end_ms)
    } else if legacy_bought {
        (now_ms, now_ms + i64::from(days) * DAY_MS)
    } else {
        (0, 0)
    };

    let bought = end_ms > 0;
    let rewards = week_card_default_rewards();
    let mut contents = Vec::with_capacity(3);
    for day in 1..=3 {
        let cid = wid * 100 + day;
        let k = format!("week_card:{cid}");
        let unlock_ms = if bought {
            buy_ms + i64::from(day - 1) * DAY_MS
        } else {
            0
        };
        let status = if !bought {
            0
        } else if info.has_claim(&k) {
            2
        } else if now_ms >= unlock_ms {
            1
        } else {
            0
        };
        contents.push(WeekCardContentInfo {
            WeekCardContentId: cid,
            UnlockDay: day,
            RewardContent: rewards.clone(),
            UnlockTimeStamp: unlock_ms,
            Status: status,
        });
    }

    WeekCardInfo {
        WeekCardId: wid,
        Days: days,
        BuyTimeStamp: buy_ms,
        EndTimeStamp: end_ms,
        WeekCardContentInfos: contents,
    }
}

pub fn handle_week_card_info_request(
    state: &mut State,
    req: WeekCardInfoRequest,
) -> Result<WeekCardInfoResponse, String> {
    let now_ms = energy::now_seconds() * 1000;
    let target_ids = if req.WeekCardIds.is_empty() && req.WeekCardId > 0 {
        vec![req.WeekCardId]
    } else {
        req.WeekCardIds
    };
    let mut infos = Vec::with_capacity(target_ids.len());
    for wid in target_ids {
        if wid > 0 {
            infos.push(build_week_card_info(state, wid, now_ms));
        }
    }
    let first_info = infos.first().cloned().unwrap_or_else(|| WeekCardInfo {
        WeekCardId: req.WeekCardId,
        Days: 0,
        BuyTimeStamp: 0,
        EndTimeStamp: 0,
        WeekCardContentInfos: Vec::new(),
    });
    Ok(WeekCardInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        WeekCardInfos: infos,
        Infos: Some(first_info),
    })
}

pub fn handle_week_card_reward_request(
    state: &mut State,
    req: WeekCardRewardRequest,
) -> Result<WeekCardRewardResponse, String> {
    let k = format!("week_card:{}", req.WeekCardContentId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                reward::RewardItem {
                    item_id: 3,
                    count: 100,
                },
                reward::RewardItem {
                    item_id: 2,
                    count: 15000,
                },
            ],
            1,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(WeekCardRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

struct PayGiftMeta {
    shop_id: i32,
    tab_id: i32,
    gift_type: i32,
    pay_id: i32,
    buy_limit: i32,
    update_type: PayUpdateType,
    is_active: bool,
}

/// Formats a USD price as a plain numeric string without currency symbol prefixes
/// because the client prepends `$` itself in `PayItemConfig.GetPayShow`.
pub fn format_pay_amount(amount: f64) -> String {
    let rounded = (amount * 100.0).round() / 100.0;
    let s = format!("{rounded:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Resolves `(ProductId, Amount)` for `pay_id`, preferring global USD (`Default`,
/// then `SEA` / `America`) and `Amount` before `GlobalAmount` placeholders.
pub fn resolve_pay_pricing(
    assets: &Assets,
    pay_id: i32,
    fallback_prefix: &str,
) -> (String, String) {
    let items = &assets.tables.pay.items;
    let pay_cfg = items
        .iter()
        .find(|p| p.PayId == pay_id && p.Region == "Default")
        .or_else(|| items.iter().find(|p| p.PayId == pay_id && p.Region == "SEA"))
        .or_else(|| {
            items
                .iter()
                .find(|p| p.PayId == pay_id && p.Region == "America")
        })
        .or_else(|| items.iter().find(|p| p.PayId == pay_id));
    match pay_cfg {
        Some(p) => {
            let prod = if p.ProductId.is_empty() {
                format!("{fallback_prefix}{pay_id}")
            } else {
                p.ProductId.clone()
            };
            let amt = if p.Amount > 0.0 {
                p.Amount
            } else if p.GlobalAmount > 0.0 {
                p.GlobalAmount
            } else {
                0.99
            };
            (prod, format_pay_amount(amt))
        }
        None => (format!("{fallback_prefix}{pay_id}"), "0.99".to_string()),
    }
}

fn infer_bundle_pay_id(assets: &Assets, item_id: i32) -> i32 {
    let pkg = assets
        .tables
        .item_info
        .get_data_by_id(&item_id)
        .and_then(|cfg| {
            cfg.Parameters
                .get(&4)
                .copied()
                .or_else(|| cfg.Parameters.get(&2).copied())
        })
        .and_then(|pkg_id| assets.tables.gift_package.get_data_by_id(&pkg_id));
    let Some(pkg) = pkg else {
        return 23;
    };
    let mut tides = 0i32;
    let mut astrite = 0i32;
    for (&k, &v) in pkg.Content.iter() {
        match k {
            50001 | 50002 | 50005 => tides += v,
            3 => astrite += v,
            _ => {}
        }
    }
    if tides >= 15 {
        26
    } else if tides >= 10 {
        25
    } else if tides >= 5 {
        24
    } else if tides >= 2 || astrite >= 500 {
        23
    } else if astrite >= 300 {
        22
    } else if astrite >= 120 {
        21
    } else {
        20
    }
}

fn classify_pay_gift(assets: &Assets, cfg: &PayGiftCfg) -> PayGiftMeta {
    match cfg.Id {
        42 => PayGiftMeta {
            shop_id: 3,
            tab_id: 0,
            gift_type: 1,
            pay_id: 15,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        43 => PayGiftMeta {
            shop_id: 3,
            tab_id: 0,
            gift_type: 2,
            pay_id: 16,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        44 => PayGiftMeta {
            shop_id: 3,
            tab_id: 0,
            gift_type: 2,
            pay_id: 17,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        45 => PayGiftMeta {
            shop_id: 3,
            tab_id: 0,
            gift_type: 2,
            pay_id: 18,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        46 => PayGiftMeta {
            shop_id: 3,
            tab_id: 0,
            gift_type: 6,
            pay_id: 16,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: false,
        },
        47 => PayGiftMeta {
            shop_id: 7,
            tab_id: 1,
            gift_type: 7,
            pay_id: 21,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        48 => PayGiftMeta {
            shop_id: 7,
            tab_id: 1,
            gift_type: 7,
            pay_id: 23,
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: true,
        },
        105_100_101 => PayGiftMeta {
            shop_id: 0,
            tab_id: 2,
            gift_type: 3,
            pay_id: 24,
            buy_limit: 1,
            update_type: PayUpdateType::Forever,
            is_active: true,
        },
        105_100_102..=105_100_999 => PayGiftMeta {
            shop_id: 0,
            tab_id: 3,
            gift_type: 3,
            pay_id: if matches!(cfg.Id, 105_100_103 | 105_100_108 | 105_100_109) {
                26
            } else {
                24
            },
            buy_limit: 1,
            update_type: PayUpdateType::Forever,
            is_active: true,
        },
        107_100_000..=107_100_999 => PayGiftMeta {
            shop_id: 0,
            tab_id: 4,
            gift_type: 3,
            pay_id: 22,
            buy_limit: 1,
            update_type: PayUpdateType::Forever,
            is_active: true,
        },
        103_102_001..=103_102_999 => PayGiftMeta {
            shop_id: 3,
            tab_id: 2,
            gift_type: 3,
            pay_id: match cfg.Id {
                103_102_001 => 26,
                103_102_002 => 25,
                103_102_003 => 23,
                _ => 24,
            },
            buy_limit: 1,
            update_type: PayUpdateType::Forever,
            is_active: true,
        },
        103_101_021..=103_101_099 | 103_103_079..=103_103_999 | 108_100_000..=108_100_999 => {
            PayGiftMeta {
                shop_id: 3,
                tab_id: 1,
                gift_type: 3,
                pay_id: infer_bundle_pay_id(assets, cfg.ItemId),
                buy_limit: if matches!(cfg.Id, 103_103_079 | 103_103_090 | 108_100_101) {
                    1
                } else {
                    3
                },
                update_type: PayUpdateType::Monthly,
                is_active: true,
            }
        }
        104_100_113..=104_100_999 | 106_100_103 => PayGiftMeta {
            shop_id: 3,
            tab_id: 4,
            gift_type: 3,
            pay_id: infer_bundle_pay_id(assets, cfg.ItemId),
            buy_limit: 3,
            update_type: PayUpdateType::Monthly,
            is_active: true,
        },
        _ => PayGiftMeta {
            shop_id: 3,
            tab_id: 3,
            gift_type: 3,
            pay_id: infer_bundle_pay_id(assets, cfg.ItemId),
            buy_limit: 1,
            update_type: PayUpdateType::None,
            is_active: false,
        },
    }
}

fn build_pay_gift_info(state: &State, cfg: &PayGiftCfg, sort: i32) -> PayGiftInfo {
    let meta = classify_pay_gift(&state.assets, cfg);
    let bought = state
        .player_components
        .progress
        .info
        .get_key_int(&format!("paygift:{}", cfg.Id))
        .max(0) as i32;
    let (product_id, amount) =
        resolve_pay_pricing(&state.assets, meta.pay_id, "com.kurogame.aki.gift");
    let sold_out = meta.buy_limit > 0 && bought >= meta.buy_limit;
    let (begin_time, end_time) = if meta.is_active {
        (1, 2_000_000_000)
    } else {
        (1, 2)
    };
    let update_time = if matches!(meta.update_type, PayUpdateType::Monthly) && meta.is_active {
        energy::now_seconds() + 30 * 86_400
    } else {
        0
    };
    PayGiftInfo {
        Id: cfg.Id,
        PayId: meta.pay_id,
        ItemId: cfg.ItemId,
        ItemCount: 1,
        Sort: sort,
        BuyLimit: meta.buy_limit,
        BoughtCount: bought,
        BeginTime: begin_time,
        EndTime: end_time,
        ProductId: product_id.into_bytes(),
        Amount: amount.into_bytes(),
        TabId: meta.tab_id,
        Type: meta.gift_type,
        IsCanBuy: meta.is_active && !sold_out,
        UpdateType: Some(meta.update_type),
        UpdateTime: update_time,
        ShopId: meta.shop_id,
        ..Default::default()
    }
}

/// Builds the full `PayGiftShopInfo` list embedded in `PayShopInfoResponse` and `PayGiftInfoResponse`.
pub fn build_pay_gift_shop_info(state: &State, version: Vec<u8>) -> PayGiftShopInfo {
    let mut rows: Vec<&PayGiftCfg> = state.assets.tables.pay_gift.items.iter().collect();
    rows.sort_by_key(|cfg| cfg.Id);
    let infos = rows
        .into_iter()
        .enumerate()
        .map(|(idx, cfg)| build_pay_gift_info(state, cfg, (idx + 1) as i32))
        .collect();
    PayGiftShopInfo {
        Gifts: infos,
        Version: if version.is_empty() {
            b"3.8.11".to_vec()
        } else {
            version
        },
    }
}

pub fn handle_pay_gift_info_request(
    state: &mut State,
    req: PayGiftInfoRequest,
) -> Result<PayGiftInfoResponse, String> {
    let gift_shop = build_pay_gift_shop_info(state, req.Version);
    Ok(PayGiftInfoResponse {
        Infos: gift_shop.Gifts,
        Version: gift_shop.Version,
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_pay_gift_request(
    state: &mut State,
    req: PayGiftRequest,
) -> Result<PayGiftResponse, String> {
    let k = format!("paygift:{}", req.Id);
    let _ = state.player_components.progress.info.add_key_int(&k, 1);
    let _ = state
        .player_components
        .progress
        .info
        .add_key_int("topup:score", 300);
    if req.Id == 42 {
        let _ = state
            .player_components
            .progress
            .info
            .add_key_int("month_card:days", 30);
    }
    let now_ms = energy::now_seconds() * 1000;
    let wk_card = week_card_id_for_gift(&state.assets, req.Id);
    if let Some((wid, days)) = wk_card {
        let buy_key = format!("week_card:buy_ms:{wid}");
        let end_key = format!("week_card:end_ms:{wid}");
        let prev_end = state.player_components.progress.info.get_key_int(&end_key);
        let duration_ms = i64::from(days) * DAY_MS;
        let new_end = if prev_end > now_ms {
            prev_end + duration_ms
        } else {
            state
                .player_components
                .progress
                .info
                .set_key_int(&buy_key, now_ms);
            now_ms + duration_ms
        };
        state
            .player_components
            .progress
            .info
            .set_key_int(&end_key, new_end);
    }
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if let Some((wid, _)) = wk_card {
        let wc_info = build_week_card_info(state, wid, now_ms);
        state.conn.push(
            EffectiveWeekCardInfoNotify::MSG_ID,
            &EffectiveWeekCardInfoNotify {
                WeekCardInfos: vec![wc_info.clone()],
                Infos: Some(wc_info),
            },
        );
    }
    let gift_cfg = state
        .assets
        .tables
        .pay_gift
        .get_data_by_id(&req.Id)
        .map(|c| c.into_owned());
    let grant_items = if let Some(ref cfg) = gift_cfg
        && cfg.ItemId > 0
    {
        vec![reward::RewardItem {
            item_id: cfg.ItemId,
            count: 1,
        }]
    } else {
        vec![
            reward::RewardItem {
                item_id: 4,
                count: 300,
            },
            reward::RewardItem {
                item_id: 2,
                count: 50000,
            },
        ]
    };
    let first_grant = grant_items[0];
    let mut rctx = state.reward_context();
    reward::grant_items(&mut rctx, 0, true, &grant_items, 8001).map_err(|e| e.to_string())?;
    let updated_gift = gift_cfg
        .as_ref()
        .map(|cfg| build_pay_gift_info(state, cfg, 1));
    state.conn.push(
        PayGiftSuccessNotify::MSG_ID,
        &PayGiftSuccessNotify {
            ReceiptId: b"local_receipt".to_vec(),
            ItemId: first_grant.item_id,
            ItemCount: first_grant.count,
            Id: req.Id,
            PayGiftInfo: updated_gift,
        },
    );
    Ok(PayGiftResponse {
        ErrorCode: Some(ErrorCode::Success),
        ReceiptId: b"local_receipt".to_vec(),
        ..Default::default()
    })
}

pub fn handle_pay_item_request(
    state: &mut State,
    req: PayItemRequest,
) -> Result<PayItemResponse, String> {
    let k = format!("payitem:{}", req.Id);
    let prev_count = state.player_components.progress.info.get_key_int(&k);
    let _ = state.player_components.progress.info.add_key_int(&k, 1);
    let (item_id, total_count) = if let Some(cfg) = state.assets.tables.pay_item.get_data_by_id(&req.Id)
    {
        let bonus = if prev_count == 0 && cfg.SpecialBonusItemCount > 0 {
            cfg.SpecialBonusItemCount
        } else {
            cfg.BonusItemCount
        };
        (
            if cfg.ItemId > 0 { cfg.ItemId } else { 4 },
            (cfg.ItemCount + bonus).max(1),
        )
    } else {
        (4, 648)
    };
    let _ = state
        .player_components
        .progress
        .info
        .add_key_int("topup:score", i64::from(total_count));
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut rctx = state.reward_context();
    reward::grant_items(
        &mut rctx,
        0,
        true,
        &[reward::RewardItem {
            item_id,
            count: total_count,
        }],
        8001,
    )
    .map_err(|e| e.to_string())?;
    state.conn.push(
        PayItemSuccessNotify::MSG_ID,
        &PayItemSuccessNotify {
            ReceiptId: b"local_receipt".to_vec(),
            Id: req.Id,
            ItemCount: total_count,
            ItemId: item_id,
        },
    );
    Ok(PayItemResponse {
        ErrorCode: Some(ErrorCode::Success),
        ReceiptId: b"local_receipt".to_vec(),
        ..Default::default()
    })
}

pub fn handle_pay_shop_direct_buy_request(
    state: &mut State,
    req: PayShopDirectBuyRequest,
) -> Result<PayShopDirectBuyResponse, String> {
    let shop_item_id = req.Id;
    let (item_id, item_count) = if let Some(cfg) = state.assets.tables.pay_gift.get_data_by_id(&shop_item_id)
        && cfg.ItemId > 0
    {
        (cfg.ItemId, 1)
    } else {
        (3, 60)
    };
    let receipt_id = format!(
        "direct_{}_{}_{}",
        state.player_id,
        shop_item_id,
        energy::now_seconds()
    );
    if shop_item_id > 0 {
        let _ = state
            .player_components
            .progress
            .info
            .add_key_int(&format!("paygift:{shop_item_id}"), 1);
        let _ = state
            .player_components
            .progress
            .info
            .add_key_int("topup:score", i64::from(item_count));
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[reward::RewardItem {
                item_id,
                count: item_count,
            }],
            8001,
        )
        .map_err(|e| e.to_string())?;
    }
    state.conn.push(
        PayShopDirectBuyNotify::MSG_ID,
        &PayShopDirectBuyNotify {
            ReceiptId: receipt_id.as_bytes().to_vec(),
            ShopItemId: shop_item_id,
            ItemId: item_id,
            ItemCount: item_count,
        },
    );
    Ok(PayShopDirectBuyResponse {
        ErrorCode: Some(ErrorCode::Success),
        ReceiptId: receipt_id.into_bytes(),
        ..Default::default()
    })
}

pub fn handle_gacha_accumulate_info_request(
    state: &mut State,
    req: GachaAccumulateInfoRequest,
) -> Result<GachaAccumulateInfoResponse, String> {
    let cur_gacha_num: i32 = state
        .player_components
        .gacha
        .info
        .banners
        .iter()
        .map(|b| b.total_pulls.max(0))
        .sum();
    let group_id = if req.GachaAccumulateId >= 200 {
        2
    } else {
        1
    };
    let reward_infos = state
        .assets
        .tables
        .gacha_accumulate_reward
        .items
        .iter()
        .enumerate()
        .map(|(idx, r)| {
            let need = ((idx as i32) + 1) * 10;
            let k = format!("gacha_acc:{}:{}", req.GachaAccumulateId, r.Id);
            let claimed = state.player_components.progress.info.has_claim(&k);
            let status = if claimed {
                2
            } else if cur_gacha_num >= need {
                1
            } else {
                0
            };
            GachaAccumulateRewardInfo {
                Id: r.Id,
                GroupId: group_id,
                GachaNum: need,
                Status: status,
                ..Default::default()
            }
        })
        .collect();
    Ok(GachaAccumulateInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        GachaAccumulateInfo: Some(GachaAccumulateInfo {
            GachaAccumulateId: req.GachaAccumulateId,
            GachaAccumulateGroupInfo: Some(GachaAccumulateGroupInfo {
                GroupId: group_id,
                CurGachaNum: cur_gacha_num,
                GachaAccumulateRewardInfos: reward_infos,
            }),
        }),
    })
}

pub fn handle_gacha_accumulate_reward_request(
    state: &mut State,
    req: GachaAccumulateRewardRequest,
) -> Result<GachaAccumulateRewardResponse, String> {
    let mut dirty = false;
    for rid in req.RewardId {
        let k = format!("gacha_acc:{}:{rid}", req.GachaAccumulateId);
        if state.player_components.progress.info.mark_claim(&k) {
            dirty = true;
            let mut rctx = state.reward_context();
            reward::grant_items(
                &mut rctx,
                0,
                true,
                &[reward::RewardItem {
                    item_id: 3,
                    count: 160,
                }],
                19500,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(GachaAccumulateRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_total_top_up_score_info_request(
    state: &mut State,
    _req: TotalTopUpScoreInfoRequest,
) -> Result<TotalTopUpScoreInfoResponse, String> {
    let score = state
        .player_components
        .progress
        .info
        .get_key_int("topup:score")
        .max(0) as i32;
    Ok(TotalTopUpScoreInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        Infos: vec![TotalTopUpScoreInfo {
            Id: 1,
            Score: score,
            ..Default::default()
        }],
    })
}

pub fn handle_total_top_up_reward_request(
    state: &mut State,
    req: TotalTopUpRewardRequest,
) -> Result<TotalTopUpRewardResponse, String> {
    let k = format!("topup:clm:{}", req.TotalTopUpRewardId);
    if state.player_components.progress.info.mark_claim(&k) {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        let mut rctx = state.reward_context();
        reward::grant_items(
            &mut rctx,
            0,
            true,
            &[reward::RewardItem {
                item_id: 3,
                count: 300,
            }],
            19500,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(TotalTopUpRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_influence_info_request(
    state: &mut State,
    _req: InfluenceInfoRequest,
) -> Result<InfluenceInfoResponse, String> {
    let infos = state
        .player_components
        .progress
        .info
        .dynamic
        .ints
        .iter()
        .filter_map(|entry| {
            let rest = entry.key.strip_prefix("inf:idx:")?;
            let inf_id = rest.parse::<i32>().ok()?;
            let val = entry.val.max(0) as i32;
            Some(InfluenceInfo {
                InfluenceId: inf_id,
                RewardIndex: val,
                Relation: val,
            })
        })
        .collect();
    Ok(InfluenceInfoResponse {
        InfluenceInfos: infos,
    })
}

pub fn handle_influence_reward_request(
    state: &mut State,
    req: InfluenceRewardRequest,
) -> Result<InfluenceRewardResponse, String> {
    let k = format!("inf:idx:{}", req.InfluenceId);
    let next_idx = state.player_components.progress.info.add_key_int(&k, 1) as i32;
    state
        .player_components
        .progress
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    let mut rctx = state.reward_context();
    reward::grant_items(
        &mut rctx,
        0,
        true,
        &[
            reward::RewardItem {
                item_id: 3,
                count: 50,
            },
            reward::RewardItem {
                item_id: 2,
                count: 15000,
            },
        ],
        19500,
    )
    .map_err(|e| e.to_string())?;
    let mut items = std::collections::BTreeMap::new();
    items.insert(3, 50);
    Ok(InfluenceRewardResponse {
        Code: Some(ErrorCode::Success),
        InfluenceId: req.InfluenceId,
        RewardIndex: next_idx,
        RewardItems: items,
    })
}

pub fn handle_storage_info_request(
    state: &mut State,
    _req: StorageInfoRequest,
) -> Result<StorageInfoResponse, String> {
    let mut infos = Vec::new();
    pb_kv::append_stored_client_storage_infos(&state.player_components.progress, &mut infos);
    Ok(StorageInfoResponse { Infos: infos })
}

pub fn handle_storage_info_update_request(
    state: &mut State,
    req: StorageInfoUpdateRequest,
) -> Result<StorageInfoUpdateResponse, String> {
    let mut dirty = false;
    for info in &req.Infos {
        let Some(data) = info.Data.as_ref() else {
            continue;
        };
        let mkey = format!("cstorage:msg:{}", info.SystemId);
        pb_kv::save_msg(&state.fs, &mut state.player_components.progress, &mkey, info)
            .map_err(|e| e.to_string())?;
        dirty = true;
        match data {
            ClientStorageInfoDataOneof::IntData(v) => {
                state.player_components.progress.info.set_key_int(
                    &format!("cstorage:i:{}", info.SystemId),
                    i64::from(v.Data),
                );
            }
            ClientStorageInfoDataOneof::LongData(v) => {
                state
                    .player_components
                    .progress
                    .info
                    .set_key_int(&format!("cstorage:i:{}", info.SystemId), v.Data);
            }
            ClientStorageInfoDataOneof::BoolData(v) => {
                state.player_components.progress.info.set_key_int(
                    &format!("cstorage:i:{}", info.SystemId),
                    i64::from(v.Data),
                );
            }
            ClientStorageInfoDataOneof::StringData(v) => {
                let s = String::from_utf8_lossy(&v.Data);
                state
                    .player_components
                    .progress
                    .info
                    .set_key_str(&format!("cstorage:s:{}", info.SystemId), &s);
            }
            _ => {}
        }
    }
    if dirty {
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }
    Ok(StorageInfoUpdateResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
