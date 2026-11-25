use serde::Serialize;
use tokio::sync::oneshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GmAction {
    GetState,
    ExecCmd,
    GiveItem,
    SetUnionLevel,
    SetWorldLevel,
    SetRoleLevel,
    UnlockRoles,
    RoleMax,
    RoleChain,
    WeaponMax,
    QuestPick,
    QuestStep,
    QuestSkip,
    UnlockTp,
    UnlockFunc,
    TimeSet,
    HealTeam,
    ResetCd,
    NoCdMode,
    SpawnEcho,
    RepairAccount,
    SwitchRover,
    ResetPlayer,
    RotateGmKey,
}

#[derive(Debug)]
pub struct GmJob {
    pub action: GmAction,
    pub target_str: String,
    pub int_a: i32,
    pub int_b: i32,
    #[allow(dead_code)]
    pub int_c: i32,
    pub flag: bool,
    pub raw_cmd: String,
    pub reply_tx: oneshot::Sender<GmJobResult>,
}

#[derive(Debug, Clone)]
pub struct GmJobResult {
    #[allow(dead_code)]
    pub ok: bool,
    pub response_json: String,
}

#[derive(Serialize)]
struct ErrPayload<'a> {
    ok: bool,
    message: &'a str,
}

impl GmJob {
    pub fn fail_and_signal(self, msg: &str) {
        let json = serde_json::to_string(&ErrPayload {
            ok: false,
            message: msg,
        })
        .unwrap_or_else(|_| "{\"ok\":false,\"message\":\"internal error\"}".to_string());
        let _ = self.reply_tx.send(GmJobResult {
            ok: false,
            response_json: json,
        });
    }

    pub fn succeed_and_signal(self, response_json: String) {
        let _ = self.reply_tx.send(GmJobResult {
            ok: true,
            response_json,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use sonoro_gamesv_logic::commands::cheat;
    use tokio::sync::{Notify, oneshot};

    use super::*;
    use crate::network::kcp_transport::ConnectionHandle;
    use crate::network::mod_state::SessionManager;
    use crate::network::connection::GameplayState;
    use crate::network::session_supervisor::cleanup_gameplay;

    #[test]
    fn cheat_breach_and_gm_bridge_cleanup_behave_cleanly() {
        assert_eq!(cheat::breach_for_level(1), 0);
        assert_eq!(cheat::breach_for_level(21), 1);
        assert_eq!(cheat::breach_for_level(90), 6);

        let (tx1, mut rx1) = oneshot::channel();
        let job1 = GmJob {
            action: GmAction::GetState,
            target_str: String::new(),
            int_a: 0,
            int_b: 0,
            int_c: 0,
            flag: false,
            raw_cmd: String::new(),
            reply_tx: tx1,
        };
        job1.fail_and_signal("offline test");
        let res1 = rx1.try_recv().expect("offline signal received");
        assert!(!res1.ok);
        assert!(res1.response_json.contains("offline test"));

        let addr = "127.0.0.1:9999".parse().unwrap();
        let mut channels = ConnectionHandle::open(1, addr, Arc::new(Notify::new()));
        let handle = Arc::new(channels.handle);
        let mut gameplay = GameplayState::new(channels.outbound_tx, Arc::clone(&handle.transport_work));
        let sessions = SessionManager::new();

        let (tx2, mut rx2) = oneshot::channel();
        handle
            .gm_inbox_tx
            .try_send(GmJob {
                action: GmAction::ExecCmd,
                target_str: String::new(),
                int_a: 0,
                int_b: 0,
                int_c: 0,
                flag: false,
                raw_cmd: String::new(),
                reply_tx: tx2,
            })
            .expect("enqueue gm job");
        handle.close(crate::network::CloseReason::Shutdown);
        cleanup_gameplay(&mut gameplay, &sessions, &handle, &mut channels.gm_inbox_rx);

        let res2 = rx2.try_recv().expect("drained signal received");
        assert!(!res2.ok);
        assert!(res2.response_json.contains("session closed"));
    }
}
