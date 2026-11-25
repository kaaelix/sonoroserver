use std::collections::BTreeMap;

use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_gamesv_logic::component::player::PlayerEchoComponent;
use sonoro_gamesv_logic::helpers::{calabash, reward};
use sonoro_gamesv_logic::sink::PushSink;
use sonoro_proto::PbEnum;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    CalabashLevelRewardRequest, CalabashLevelRewardResponse, CalabashLevelsRewardNotify,
    CalabashMsgRequest, CalabashMsgResponse, MulCalabashLevelRewardRequest,
    MulCalabashLevelRewardResponse, PhantomCollectReceiveRequest, PhantomCollectReceiveResponse,
};

use super::super::State;

pub fn handle_calabash_level_reward_request(
    state: &mut State,
    req: CalabashLevelRewardRequest,
) -> Result<CalabashLevelRewardResponse, String> {
    let level = req.Level;
    if level <= 0 {
        return Ok(CalabashLevelRewardResponse {
            ErrorCode: Some(ErrorCode::ErrCalabashLevelRequest),
        });
    }
    let Some(cfg) = state
        .assets
        .tables
        .calabash_level
        .get_data_by_id(&level)
        .map(|r| r.into_owned())
    else {
        return Ok(CalabashLevelRewardResponse {
            ErrorCode: Some(ErrorCode::ErrCalabashLevelConfig),
        });
    };
    let current_level = calabash::current_level(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    );
    if level > current_level {
        return Ok(CalabashLevelRewardResponse {
            ErrorCode: Some(ErrorCode::ErrCalabashLevelUp),
        });
    }
    if state
        .player_components
        .echo
        .calabash_info
        .rewarded_levels
        .contains(&level)
    {
        return Ok(CalabashLevelRewardResponse {
            ErrorCode: Some(ErrorCode::ErrCalabashLevelRewardDone),
        });
    }

    state
        .player_components
        .echo
        .calabash_info
        .rewarded_levels
        .push(level);
    PlayerEchoComponent::save_calabash(
        &state.fs,
        state.player_id,
        &state.player_components.echo.calabash_info,
    )
    .map_err(|e| e.to_string())?;

    if cfg.RewardId > 0 {
        let mut rctx = state.reward_context();
        reward::grant_drop_package(&mut rctx, cfg.RewardId, 0).map_err(|e| e.to_string())?;
    }

    let rewarded_levels = state
        .player_components
        .echo
        .calabash_info
        .rewarded_levels
        .clone();
    state.conn.push(
        CalabashLevelsRewardNotify::MSG_ID,
        &CalabashLevelsRewardNotify {
            RewardedLevels: rewarded_levels,
        },
    );

    Ok(CalabashLevelRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

pub fn handle_mul_calabash_level_reward_request(
    state: &mut State,
    req: MulCalabashLevelRewardRequest,
) -> Result<MulCalabashLevelRewardResponse, String> {
    let current_level = calabash::current_level(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    );
    let mut claimed_levels = Vec::new();
    let mut dirty = false;

    for level in req.Level {
        if level <= 0 || level > current_level {
            continue;
        }
        if state
            .player_components
            .echo
            .calabash_info
            .rewarded_levels
            .contains(&level)
        {
            continue;
        }
        let Some(cfg) = state
            .assets
            .tables
            .calabash_level
            .get_data_by_id(&level)
            .map(|r| r.into_owned())
        else {
            continue;
        };

        state
            .player_components
            .echo
            .calabash_info
            .rewarded_levels
            .push(level);
        dirty = true;
        claimed_levels.push(level);
        if cfg.RewardId > 0 {
            let mut rctx = state.reward_context();
            reward::grant_drop_package_popup(&mut rctx, cfg.RewardId, 19500)
                .map_err(|e| e.to_string())?;
        }
    }

    if dirty {
        PlayerEchoComponent::save_calabash(
            &state.fs,
            state.player_id,
            &state.player_components.echo.calabash_info,
        )
        .map_err(|e| e.to_string())?;
        let rewarded_levels = state
            .player_components
            .echo
            .calabash_info
            .rewarded_levels
            .clone();
        state.conn.push(
            CalabashLevelsRewardNotify::MSG_ID,
            &CalabashLevelsRewardNotify {
                RewardedLevels: rewarded_levels,
            },
        );
    }

    Ok(MulCalabashLevelRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
        Level: claimed_levels,
    })
}

pub fn handle_phantom_collect_receive_request(
    state: &mut State,
    req: PhantomCollectReceiveRequest,
) -> Result<PhantomCollectReceiveResponse, String> {
    let reward_type = req.Type.map_or(0, |t| t.to_i32());
    let key = format!("phantom_collect:{}:{reward_type}", req.ActivityId);
    let mut item_map = BTreeMap::new();

    if state.player_components.progress.info.mark_claim(&key) {
        let _ = &ProgressInfo::append_unique_id;
        state
            .player_components
            .progress
            .save(&state.fs)
            .map_err(|e| e.to_string())?;

        let mut granted = Vec::new();
        {
            let mut rctx = state.reward_context();
            reward::grant_table_reward(
                &mut rctx,
                "PhantomCollect",
                i64::from(reward_type + 1),
                false,
                true,
                19500,
                &[
                    reward::RewardItem {
                        item_id: 3,
                        count: 60,
                    },
                    reward::RewardItem {
                        item_id: 2,
                        count: 20000,
                    },
                ],
                Some(&mut granted),
            )
            .map_err(|e| e.to_string())?;
        }
        for it in granted {
            *item_map.entry(it.item_id).or_insert(0) += it.count;
        }
    }

    Ok(PhantomCollectReceiveResponse {
        ErrorCode: Some(ErrorCode::Success),
        ItemMap: item_map,
    })
}

pub fn handle_calabash_msg_request(
    state: &mut State,
    _req: CalabashMsgRequest,
) -> Result<CalabashMsgResponse, String> {
    let notify = calabash::build_msg_notify(
        &state.assets,
        &state.player_components.basic,
        &state.player_components.echo,
    );
    Ok(CalabashMsgResponse {
        ErrCode: Some(ErrorCode::Success),
        CalabashMsg: notify.CalabashMsg,
        CalabashCfg: notify.CalabashCfg,
    })
}
