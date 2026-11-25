use std::collections::BTreeMap;

use sonoro_gamesv_fs::mail_info::state as mail_state;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::helpers::reward::{self, RewardItem};
use sonoro_gamesv_logic::helpers::{energy, quest_engine};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    MailBind, MailBindInfoRequest, MailBindInfoResponse, MailBindRequest, MailBindResponse,
    MailBindRewardRequest, MailBindRewardResponse, MailDeleteRequest, MailDeleteResponse,
    MailFavoriteRequest, MailFavoriteResponse, MailGetAttachmentRequest, MailGetAttachmentResponse,
    MailInfosNotify, MailReadRequest, MailReadResponse,
};

use super::State;

pub fn handle_mail_bind_info_request(
    state: &mut State,
    _req: MailBindInfoRequest,
) -> Result<MailBindInfoResponse, String> {
    let bind_rewarded = state.player_components.mail.info.bind_rewarded;
    Ok(MailBindInfoResponse {
        MailBind: Some(MailBind {
            IsBind: bind_rewarded,
            IsReward: bind_rewarded,
            CloseTime: 0,
        }),
    })
}

pub fn handle_mail_read_request(
    state: &mut State,
    events: &mut EventQueue,
    req: MailReadRequest,
) -> Result<MailReadResponse, String> {
    let now_s = energy::now_seconds();
    let pruned = state.player_components.mail.info.prune_expired(now_s);
    let req_id = String::from_utf8_lossy(&req.Id);
    let Some(idx) = state
        .player_components
        .mail
        .info
        .mails
        .iter()
        .position(|m| m.id == req_id.as_ref())
    else {
        if pruned {
            state
                .player_components
                .mail
                .save(&state.fs)
                .map_err(|e| e.to_string())?;
        }
        return Ok(MailReadResponse {
            ErrorCode: Some(ErrorCode::ErrMailNotExist),
            ..Default::default()
        });
    };

    let (id_bytes, new_state, expiry_time, read_time, mail_mid) = {
        let entry = &mut state.player_components.mail.info.mails[idx];
        entry.state |= mail_state::READ;
        if entry.read_time == 0 {
            entry.read_time = now_s;
        }
        (
            entry.id.as_bytes().to_vec(),
            entry.wire_state(),
            entry.expiry_time,
            entry.read_time,
            entry.id.parse::<i32>().unwrap_or(0),
        )
    };
    state
        .player_components
        .mail
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    if mail_mid > 0 {
        let mut qctx = state.quest_context(events);
        let _ = quest_engine::events::on_mail_read(&mut qctx, mail_mid);
    }

    Ok(MailReadResponse {
        ExpiryTime: expiry_time,
        Id: id_bytes,
        State: new_state,
        ErrorCode: Some(ErrorCode::Success),
        ReadTime: read_time,
    })
}

pub fn handle_mail_get_attachment_request(
    state: &mut State,
    req: MailGetAttachmentRequest,
) -> Result<MailGetAttachmentResponse, String> {
    let now_s = energy::now_seconds();
    let pruned = state.player_components.mail.info.prune_expired(now_s);
    let mut granted = Vec::new();
    let mut success_map = BTreeMap::new();
    let mut changed = false;

    for raw_id in &req.MailIds {
        let req_id = String::from_utf8_lossy(raw_id);
        let Some(entry) = state
            .player_components
            .mail
            .info
            .mails
            .iter_mut()
            .find(|m| m.id == req_id.as_ref())
        else {
            continue;
        };
        if entry.is_claimed() {
            success_map.insert(raw_id.clone(), 0);
            continue;
        }
        entry.state |= mail_state::READ | mail_state::CLAIMED;
        if entry.read_time == 0 {
            entry.read_time = now_s;
        }
        for att in &entry.attachments {
            if att.item_id > 0 && att.count > 0 {
                granted.push(RewardItem {
                    item_id: att.item_id,
                    count: att.count,
                });
            }
        }
        success_map.insert(raw_id.clone(), 0);
        changed = true;
    }

    if !granted.is_empty() {
        let mut rctx = state.reward_context();
        let _ = reward::grant_items(&mut rctx, 0, true, &granted, 7000)
            .map_err(|e| e.to_string())?;
    }
    if changed || pruned {
        state
            .player_components
            .mail
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        if changed {
            let notify = state.player_components.mail.info.to_proto_notify();
            let _ = state.conn.push_msg(MailInfosNotify::MSG_ID, &notify);
        }
    }

    Ok(MailGetAttachmentResponse {
        ErrorCode: Some(ErrorCode::Success),
        SuccessIdMap: success_map,
    })
}

pub fn handle_mail_delete_request(
    state: &mut State,
    req: MailDeleteRequest,
) -> Result<MailDeleteResponse, String> {
    let now_s = energy::now_seconds();
    let pruned = state.player_components.mail.info.prune_expired(now_s);
    let mut deleted_ids = Vec::new();
    let mut write_idx = 0usize;
    let mails = &mut state.player_components.mail.info.mails;
    for read_idx in 0..mails.len() {
        let want_delete = req
            .MailIds
            .iter()
            .any(|id| String::from_utf8_lossy(id).as_ref() == mails[read_idx].id);
        let can_delete = want_delete
            && !mails[read_idx].is_favorite()
            && (mails[read_idx].attachments.is_empty() || mails[read_idx].is_claimed());
        if can_delete {
            deleted_ids.push(mails[read_idx].id.as_bytes().to_vec());
        } else {
            if write_idx != read_idx {
                mails.swap(write_idx, read_idx);
            }
            write_idx += 1;
        }
    }
    mails.truncate(write_idx);

    if !deleted_ids.is_empty() || pruned {
        state
            .player_components
            .mail
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
    }

    Ok(MailDeleteResponse {
        ErrorCode: Some(ErrorCode::Success),
        SuccessIds: deleted_ids,
    })
}

pub fn handle_mail_favorite_request(
    state: &mut State,
    req: MailFavoriteRequest,
) -> Result<MailFavoriteResponse, String> {
    let req_id = String::from_utf8_lossy(&req.Id);
    let Some(entry) = state
        .player_components
        .mail
        .info
        .mails
        .iter_mut()
        .find(|m| m.id == req_id.as_ref())
    else {
        return Ok(MailFavoriteResponse {
            ErrorCode: Some(ErrorCode::ErrMailNotExist),
            ..Default::default()
        });
    };

    entry.is_favorite = req.IsFavorite;
    if req.IsFavorite {
        entry.state |= mail_state::FAVORITE;
    } else {
        entry.state &= !mail_state::FAVORITE;
    }
    let id_bytes = entry.id.as_bytes().to_vec();
    let wire_state = entry.wire_state();
    let expiry_time = entry.expiry_time;

    state
        .player_components
        .mail
        .save(&state.fs)
        .map_err(|e| e.to_string())?;

    Ok(MailFavoriteResponse {
        ErrorCode: Some(ErrorCode::Success),
        ExpiryTime: expiry_time,
        Id: id_bytes,
        State: wire_state,
    })
}

pub fn handle_mail_bind_request(
    state: &mut State,
    _req: MailBindRequest,
) -> Result<MailBindResponse, String> {
    state.player_components.mail.info.bind_rewarded = true;
    state
        .player_components
        .mail
        .save(&state.fs)
        .map_err(|e| e.to_string())?;
    Ok(MailBindResponse {})
}

pub fn handle_mail_bind_reward_request(
    state: &mut State,
    _req: MailBindRewardRequest,
) -> Result<MailBindRewardResponse, String> {
    let newly_claimed = if !state.player_components.mail.info.bind_rewarded {
        state.player_components.mail.info.bind_rewarded = true;
        state
            .player_components
            .mail
            .save(&state.fs)
            .map_err(|e| e.to_string())?;
        true
    } else {
        false
    };
    if newly_claimed {
        let mut rctx = state.reward_context();
        let _ = reward::grant_items(
            &mut rctx,
            0,
            true,
            &[
                RewardItem {
                    item_id: 3,
                    count: 100,
                },
                RewardItem {
                    item_id: 2,
                    count: 10_000,
                },
            ],
            7000,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(MailBindRewardResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}
