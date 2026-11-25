//! Handler registry mapping protocol message IDs to Rust handler functions.

pub mod achievement;
pub mod activity;
pub mod battle_pass;
pub mod extra;
pub mod liveness;
pub mod settings_guide;
pub mod store_card;

use std::collections::HashMap;

use crate::ipc::AkiResult;

/// Function signature implemented by every IPC message handler.
pub type HandlerFn = fn(i32, &[u8]) -> AkiResult;

/// Returns the number of explicit gameplay/system handlers (excluding ping `0`).
pub fn rich_handler_count() -> usize {
    build_handlers().len().saturating_sub(1)
}

/// Builds the complete `msg_id -> handler` dispatch table matching `IpcServer.cs`.
pub fn build_handlers() -> HashMap<u16, HandlerFn> {
    let mut map: HashMap<u16, HandlerFn> = HashMap::new();

    // Ping / Health
    map.insert(0, |_, _| AkiResult::ok_raw(0, vec![1]));

    // Delegate gameplay, shop, battle pass, cards, achievements, liveness, music,
    // and detection natively to gamesv so ZON/Scene state never split-brains.
    extra::register_all(&mut map);
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use sonoro_proto::PbMessage;
    use sonoro_proto::enums::ErrorCode;
    use sonoro_proto::pb::{
        GatewayEchoRequest, GatewayEchoResponse, KcpConvRequest, KcpConvResponse,
    };
    use sonoro_proto::registry::msg_id;

    #[test]
    fn registers_expected_rich_handlers_and_roundtrips_echo_kcp() {
        let map = build_handlers();
        assert!(map.len() > 75);

        let echo_req = GatewayEchoRequest {
            Content: b"sonoro-rust".to_vec(),
        };
        let echo_res = map[&msg_id::GATEWAY_ECHO_REQUEST](0, &echo_req.encode_to_vec());
        assert_eq!(echo_res.resp_msg_id, msg_id::GATEWAY_ECHO_RESPONSE);
        let echo_dec = GatewayEchoResponse::decode(&echo_res.resp_bytes).expect("decode echo");
        assert_eq!(echo_dec.Content, b"sonoro-rust");

        let kcp_req = KcpConvRequest {
            DisableCrc: false,
            ..Default::default()
        };
        let kcp_res = map[&msg_id::KCP_CONV_REQUEST](0, &kcp_req.encode_to_vec());
        assert_eq!(kcp_res.resp_msg_id, msg_id::KCP_CONV_RESPONSE);
        let kcp_dec = KcpConvResponse::decode(&kcp_res.resp_bytes).expect("decode kcp");
        assert_eq!(kcp_dec.Code, Some(ErrorCode::Success));
        assert_eq!(kcp_dec.Conv, 1);
        assert_eq!(kcp_dec.CrcCheck, 1);
        assert_eq!(kcp_dec.KcpMtu, 1400);

        for unshadowed in [
            msg_id::TRACE_ENTITY_POS_REQUEST,
            msg_id::SNEAK_FINISH_REQUEST,
            msg_id::MAIN_PHANTOM_RECOMMEND_REQUEST,
            msg_id::EXCHANGE_REWARD_INFO_REQUEST,
            msg_id::DIRECT_TRAIN_INFO_REQUEST,
            msg_id::ACHIEVEMENT_INFO_REQUEST,
            msg_id::BATTLE_PASS_REQUEST,
            msg_id::PHANTOM_MUSIC_SAVE_REQUEST,
            msg_id::MONSTER_DETECTION_INFO_REQUEST,
            msg_id::PAY_INFO_REQUEST,
        ] {
            assert!(!map.contains_key(&unshadowed), "unexpected shadow {unshadowed}");
        }
    }
}
