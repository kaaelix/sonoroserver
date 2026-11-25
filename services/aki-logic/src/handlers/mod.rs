//! Handler registry mapping protocol message IDs to Rust handler functions.

pub mod achievement;
pub mod activity;
pub mod battle_pass;
pub mod extra;
pub mod liveness;
pub mod settings_guide;
pub mod store_card;

use std::collections::HashMap;

use sonoro_proto::registry::msg_id;

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

    // Activity
    map.insert(msg_id::ACTIVITY_REQUEST, activity::handle_activity_request);
    map.insert(
        msg_id::ACTIVITY_UPDATE_NOTIFY,
        activity::handle_activity_update_notify,
    );

    // BattlePass
    map.insert(
        msg_id::BATTLE_PASS_REQUEST,
        battle_pass::handle_battle_pass_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_TASK_REQUEST,
        battle_pass::handle_battle_pass_task_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_TAKE_REWARD_REQUEST,
        battle_pass::handle_battle_pass_take_reward_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_TAKE_ALL_REWARD_REQUEST,
        battle_pass::handle_battle_pass_take_all_reward_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_LEVEL_UP_REQUEST,
        battle_pass::handle_battle_pass_level_up_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_RECURRING_TAKE_REQUEST,
        battle_pass::handle_battle_pass_recurring_take_request,
    );
    map.insert(
        msg_id::BATTLE_PASS_TASK_TAKE_REQUEST,
        store_card::handle_battle_pass_task_take,
    );

    // MonthCard, WeekCard & SignActivity
    map.insert(msg_id::MONTH_CARD_REQUEST, store_card::handle_month_card);
    map.insert(
        msg_id::WEEK_CARD_INFO_REQUEST,
        store_card::handle_week_card_info,
    );
    map.insert(
        msg_id::WEEK_CARD_REWARD_REQUEST,
        store_card::handle_week_card_reward,
    );
    map.insert(
        msg_id::SIGN_ACTIVITY_REQUEST,
        store_card::handle_sign_activity,
    );
    map.insert(
        msg_id::SIGN_ACTIVITY_FREE_DROP_REQUEST,
        store_card::handle_sign_activity_free_drop,
    );

    // Liveness
    map.insert(msg_id::LIVENESS_REQUEST, liveness::handle_liveness_request);
    map.insert(
        msg_id::LIVENESS_TASK_TAKE_REQUEST,
        liveness::handle_liveness_task_take_request,
    );
    map.insert(
        msg_id::LIVENESS_TAKE_REQUEST,
        liveness::handle_liveness_take_request,
    );

    // Achievements
    map.insert(
        msg_id::ACHIEVEMENT_INFO_REQUEST,
        achievement::handle_achievement_info_request,
    );
    map.insert(
        msg_id::ACHIEVEMENT_UPDATE_REQUEST,
        achievement::handle_achievement_update_request,
    );
    map.insert(
        msg_id::ACHIEVEMENT_FINISH_REQUEST,
        achievement::handle_achievement_finish_request,
    );
    map.insert(
        msg_id::ACHIEVEMENT_RECEIVE_REQUEST,
        achievement::handle_achievement_receive_request,
    );
    map.insert(
        msg_id::MUL_ACHIEVEMENT_RECEIVE_REQUEST,
        achievement::handle_mul_achievement_receive_request,
    );

    // Settings, AccessPath, Titles & Music
    map.insert(
        msg_id::LANGUAGE_SETTING_UPDATE_REQUEST,
        settings_guide::handle_language_setting_update,
    );
    map.insert(
        msg_id::INPUT_SETTING_REQUEST,
        settings_guide::handle_input_setting_request,
    );
    map.insert(
        msg_id::INPUT_SETTING_UPDATE_REQUEST,
        settings_guide::handle_input_setting_update,
    );
    map.insert(
        msg_id::ACCESS_PATH_CONFIG_REQUEST,
        settings_guide::handle_access_path_config,
    );
    map.insert(
        msg_id::PLAYER_TITLE_REQUEST,
        settings_guide::handle_player_title_request,
    );
    map.insert(
        msg_id::PHANTOM_MUSIC_INFO_REQUEST,
        settings_guide::handle_phantom_music_info,
    );
    map.insert(
        msg_id::PHANTOM_MUSIC_UNLOCK_REQUEST,
        settings_guide::handle_phantom_music_unlock,
    );
    map.insert(
        msg_id::PHANTOM_MUSIC_SWITCH_REQUEST,
        settings_guide::handle_phantom_music_switch,
    );
    map.insert(
        msg_id::PHANTOM_MUSIC_COLLECT_UPDATE_REQUEST,
        settings_guide::handle_phantom_music_collect_update,
    );
    map.insert(
        msg_id::PHANTOM_MUSIC_SAVE_REQUEST,
        settings_guide::handle_phantom_music_save,
    );

    // Adventure, PayShop & MonsterDetection
    map.insert(
        msg_id::PAY_SHOP_DIRECT_BUY_REQUEST,
        store_card::handle_pay_shop_direct_buy,
    );
    map.insert(
        msg_id::PAY_INFO_REQUEST,
        store_card::handle_pay_info_request,
    );
    map.insert(
        msg_id::PAY_GIFT_INFO_REQUEST,
        store_card::handle_pay_gift_info_request,
    );
    map.insert(
        msg_id::MONSTER_DETECTION_INFO_REQUEST,
        settings_guide::handle_monster_detection_info_request,
    );
    map.insert(
        msg_id::GET_ADVENTURE_REWARD_REQUEST,
        store_card::handle_get_adventure_reward,
    );

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

    #[test]
    fn registers_expected_rich_handlers_and_roundtrips_echo_kcp() {
        let map = build_handlers();
        assert!(map.len() > 80);

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
    }
}
