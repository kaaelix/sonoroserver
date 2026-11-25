use sonoro_gamesv_logic::commands;
use sonoro_gamesv_logic::event_queue::EventQueue;
use sonoro_gamesv_logic::events::Event;
use sonoro_gamesv_logic::scene::now_millis;
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    ChannelChatRequest, ChannelChatResponse, ChatContentProto, ChatNotify, ChatRequest,
    ChatResponse, PrivateChatDataRequest, PrivateChatDataResponse, PrivateChatHistoryContentProto,
    PrivateChatHistoryNotify, PrivateChatHistoryRequest, PrivateChatHistoryResponse,
    PrivateChatRequest, PrivateChatResponse,
};

use super::State;

pub fn handle_private_chat_data_request(
    state: &mut State,
    _req: PrivateChatDataRequest,
) -> Result<PrivateChatDataResponse, String> {
    let _ = state.conn.push_msg(
        PrivateChatHistoryNotify::MSG_ID,
        &PrivateChatHistoryNotify {
            AllChats: Vec::new(),
        },
    );
    Ok(PrivateChatDataResponse { LoadSucc: true })
}

pub fn handle_private_chat_history_request(
    _state: &mut State,
    req: PrivateChatHistoryRequest,
) -> Result<PrivateChatHistoryResponse, String> {
    Ok(PrivateChatHistoryResponse {
        ErrorCode: Some(ErrorCode::Success),
        Data: Some(PrivateChatHistoryContentProto {
            TargetUid: req.TargetUid,
            Chats: Vec::new(),
            HistoryIsEnd: true,
            TotalNums: 0,
        }),
    })
}

pub fn handle_private_chat_request(
    _state: &mut State,
    req: PrivateChatRequest,
) -> Result<PrivateChatResponse, String> {
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
        ..Default::default()
    };
    use sonoro_proto::PbMessage;
    let _ = state
        .conn
        .push_msg(ChatNotify::MSG_ID, &ChatNotify {
            Content: content.encode_to_vec(),
        });
    Ok(ChannelChatResponse {
        ErrorCode: Some(ErrorCode::Success),
        ..Default::default()
    })
}
