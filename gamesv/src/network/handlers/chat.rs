use sonoro_gamesv_logic::commands;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::helpers::pb_kv;
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    ChannelChatRequest, ChannelChatResponse, ChatContentProto, ChatNotify, ChatRequest,
    ChatResponse, PrivateChatDataRequest, PrivateChatDataResponse, PrivateChatHistoryContentProto,
    PrivateChatHistoryNotify, PrivateChatHistoryRequest, PrivateChatHistoryResponse,
    PrivateChatRequest, PrivateChatResponse,
};

use super::State;

fn private_chat_key(target_uid: i32) -> String {
    format!("chat:priv:{target_uid}")
}

fn load_private_chat_history(state: &State, target_uid: i32) -> PrivateChatHistoryContentProto {
    pb_kv::load_msg::<PrivateChatHistoryContentProto>(
        &state.player_components.progress,
        &private_chat_key(target_uid),
    )
    .unwrap_or_else(|| PrivateChatHistoryContentProto {
        TargetUid: target_uid,
        Chats: Vec::new(),
        HistoryIsEnd: true,
        TotalNums: 0,
    })
}

pub fn handle_private_chat_data_request(
    state: &mut State,
    _req: PrivateChatDataRequest,
) -> Result<PrivateChatDataResponse, String> {
    let mut all_chats = Vec::new();
    let pc = &state.player_components.progress;
    for entry in &pc.info.dynamic.strs {
        let Some(rest) = entry.key.strip_prefix("chat:priv:") else {
            continue;
        };
        if rest.parse::<i32>().is_err() {
            continue;
        }
        if let Some(mut history) = pb_kv::load_msg::<PrivateChatHistoryContentProto>(pc, &entry.key) {
            history.HistoryIsEnd = true;
            history.TotalNums = history.Chats.len() as i32;
            all_chats.push(history);
        }
    }
    let _ = state.conn.push_msg(
        PrivateChatHistoryNotify::MSG_ID,
        &PrivateChatHistoryNotify {
            AllChats: all_chats,
        },
    );
    Ok(PrivateChatDataResponse { LoadSucc: true })
}

pub fn handle_private_chat_history_request(
    state: &mut State,
    req: PrivateChatHistoryRequest,
) -> Result<PrivateChatHistoryResponse, String> {
    let mut history = load_private_chat_history(state, req.TargetUid);
    let total = history.Chats.len() as i32;
    if req.StartIndex > 0 {
        let start = (req.StartIndex as usize).min(history.Chats.len());
        history.Chats = history.Chats.split_off(start);
    }
    history.TargetUid = req.TargetUid;
    history.HistoryIsEnd = true;
    history.TotalNums = total;
    Ok(PrivateChatHistoryResponse {
        ErrorCode: Some(ErrorCode::Success),
        Data: Some(history),
    })
}

pub fn handle_private_chat_request(
    state: &mut State,
    req: PrivateChatRequest,
) -> Result<PrivateChatResponse, String> {
    let sender_uid = state.player_components.basic.player_id;
    let chat_msg = ChatContentProto {
        SenderUid: sender_uid,
        ChatContentType: req.ChatContentType,
        Content: req.Content.clone(),
        UtcTime: (now_millis() / 1000) as i64,
        ..Default::default()
    };
    let mut history = load_private_chat_history(state, req.TargetUid);
    history.TargetUid = req.TargetUid;
    history.Chats.push(chat_msg.clone());
    if history.Chats.len() > 50 {
        let drain = history.Chats.len() - 50;
        history.Chats.drain(..drain);
    }
    history.HistoryIsEnd = true;
    history.TotalNums = history.Chats.len() as i32;
    pb_kv::save_msg(
        &state.fs,
        &mut state.player_components.progress,
        &private_chat_key(req.TargetUid),
        &history,
    )
    .map_err(|e| e.to_string())?;
    if let Some(coop) = state.coop_service.as_ref() {
        let _ = coop.deliver_private_chat(&state.fs, sender_uid, req.TargetUid, &chat_msg);
    }
    Ok(PrivateChatResponse {
        TargetUid: req.TargetUid,
        ErrorCode: Some(ErrorCode::Success),
        FilterMsg: req.Content,
        ..Default::default()
    })
}

pub fn handle_chat_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ChatRequest,
) -> Result<ChatResponse, String> {
    let text = String::from_utf8_lossy(&req.Content);
    tracing::info!(target: "chat", "player {}: {}", state.player_id, text);
    let trimmed = text.trim();
    if let Some(cmd) = trimmed.strip_prefix('/') {
        let now_ms = now_millis();
        let mut rt = state.as_runtime(events, None);
        if let Err(err) = commands::dispatch(&mut rt, cmd, now_ms) {
            events.enqueue(Event::ChatCommandResponse {
                player_id: state.player_id,
                message: format!("error: {err}"),
            });
        }
    }
    Ok(ChatResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}

pub fn handle_channel_chat_request(
    state: &mut State,
    events: &mut EventQueue,
    req: ChannelChatRequest,
) -> Result<ChannelChatResponse, String> {
    let text = String::from_utf8_lossy(&req.Content);
    tracing::info!(
        target: "chat",
        "channel {:?} player {}: {}",
        req.ChannelType,
        state.player_id,
        text
    );
    let trimmed = text.trim();
    if let Some(cmd) = trimmed.strip_prefix('/') {
        let now_ms = now_millis();
        let mut rt = state.as_runtime(events, None);
        if let Err(err) = commands::dispatch(&mut rt, cmd, now_ms) {
            events.enqueue(Event::ChatCommandResponse {
                player_id: state.player_id,
                message: format!("error: {err}"),
            });
        }
    }

    let sender_uid = state.player_components.basic.player_id;
    let content = ChatContentProto {
        SenderUid: sender_uid,
        Content: req.Content,
        ChatContentType: req.ChatContentType,
        UtcTime: (now_millis() / 1000) as i64,
        ..Default::default()
    };
    use sonoro_proto::PbMessage;
    let notify = ChatNotify {
        Content: content.encode_to_vec(),
    };
    let _ = state.conn.push_msg(ChatNotify::MSG_ID, &notify);
    if let Some(coop) = state.coop_service.as_ref() {
        let is_team = req.ChannelType == Some(sonoro_proto::enums::ChatChannelType::Team);
        let room_host = coop
            .with_online_player(sender_uid, |p| p.room_host_id)
            .unwrap_or(0);
        if is_team && room_host > 0 {
            coop.broadcast_room(room_host, sender_uid, ChatNotify::MSG_ID, &notify);
        } else {
            let _ = coop.broadcast_all(sender_uid, ChatNotify::MSG_ID, &notify);
        }
    }
    Ok(ChannelChatResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}
