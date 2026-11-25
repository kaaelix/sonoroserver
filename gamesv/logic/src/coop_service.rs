use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use serde::{Deserialize, Serialize};
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_fs::basic_info::BasicInfo;
use sonoro_gamesv_fs::file_util::{load_or_create_zon, parse_zon, serialize_zon};
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_proto::enums::{EMatchPlayerUiState, ErrorCode, MatchFailReason, MatchTeamState, WorldEnterPermission, WorldTeamLeaveReason};
use sonoro_proto::pb::{
    AgreeJoinResultNotify, ApplyJoinWorldNotify, CancelMatchNotify, ChatContentProto, ChatNotify,
    FriendAddedNotify, FriendApply, FriendApplyDeletedNotify, FriendApplyReceivedNotify,
    FriendDeletedNotify, FriendInfo, JoinWorldTeamNotify, MatchConfirmNotify, MatchFailNotify,
    MatchPlayerInfo, MatchTeamInfo, MatchTeamNotify, MatchTeamStateNotify, MovePackageNotify,
    MovingEntityData, PlayerDetails, PlayerEnterWorldTeamNotify, PlayerLeaveWorldTeamNotify,
    PrivateChatHistoryContentProto, StartMatchNotify, WorldTeamPlayerFightInfo,
    WorldTeamPlayerInfo, WorldTeamRoleInfo,
};
use sonoro_proto::wire::PbMessage;

pub trait CoopSender: Send + Sync {
    fn send_raw(&self, msg_id: u16, payload: Vec<u8>) -> bool;
}

#[derive(Default)]
pub struct VecCoopSender {
    pub packets: Mutex<Vec<(u16, Vec<u8>)>>,
}

impl CoopSender for VecCoopSender {
    fn send_raw(&self, msg_id: u16, payload: Vec<u8>) -> bool {
        if let Ok(mut guard) = self.packets.lock() {
            guard.push((msg_id, payload));
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct FriendRecord {
    pub id: i32,
    pub remark: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RecentTeamRecord {
    pub id: i32,
    pub team_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct SocialRecord {
    pub name: String,
    pub level: i32,
    pub origin_world_level: i32,
    pub cur_world_level: i32,
    pub head_id: i32,
    pub head_frame_id: i32,
    pub signature: String,
    pub sex: i32,
    pub friends: Vec<FriendRecord>,
    pub pending_applies: Vec<i32>,
    pub blocked: Vec<i32>,
    pub recent_teams: Vec<RecentTeamRecord>,
}

impl Default for SocialRecord {
    fn default() -> Self {
        Self {
            name: String::new(),
            level: 1,
            origin_world_level: 1,
            cur_world_level: 1,
            head_id: 82_001_209,
            head_frame_id: 0,
            signature: String::new(),
            sex: 0,
            friends: Vec::new(),
            pending_applies: Vec::new(),
            blocked: Vec::new(),
            recent_teams: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AdviceContentEntry {
    pub content_type: i32,
    pub id: i32,
    pub word: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AdviceRecord {
    pub id: i64,
    pub area_id: i32,
    pub owner_id: i32,
    pub up_vote: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_pitch: f32,
    pub rot_yaw: f32,
    pub rot_roll: f32,
    pub voter_ids: Vec<i32>,
    pub contents: Vec<AdviceContentEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AdviceStore {
    pub next_id: i64,
    pub advices: Vec<AdviceRecord>,
}

impl Default for AdviceStore {
    fn default() -> Self {
        Self {
            next_id: 1,
            advices: Vec::new(),
        }
    }
}

static SOCIAL_FILE_MUTEX: Mutex<()> = Mutex::new(());
static ADVICE_FILE_MUTEX: Mutex<()> = Mutex::new(());

fn now_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

#[derive(Clone)]
pub struct OnlinePlayer {
    pub player_id: i32,
    pub sender: Arc<dyn CoopSender>,
    pub name: String,
    pub level: i32,
    pub origin_world_level: i32,
    pub cur_world_level: i32,
    pub head_id: i32,
    pub head_frame_id: i32,
    pub signature: String,
    pub sex: i32,
    pub cur_role_id: i32,
    pub enter_permission: WorldEnterPermission,
    pub is_can_lobby_online: bool,
    pub room_host_id: i32,
    pub match_inst_id: i32,
    pub match_host_id: i32,
    pub match_ready: bool,
    pub match_confirm: bool,
    pub match_ui_state: EMatchPlayerUiState,
    pub match_roles: [i32; 4],
    pub area_id: i32,
}

impl OnlinePlayer {
    pub fn new(player_id: i32, sender: Arc<dyn CoopSender>, name: impl Into<String>) -> Self {
        Self {
            player_id,
            sender,
            name: name.into(),
            level: 1,
            origin_world_level: 1,
            cur_world_level: 1,
            head_id: 82_001_209,
            head_frame_id: 0,
            signature: String::new(),
            sex: 0,
            cur_role_id: 1104,
            enter_permission: WorldEnterPermission::ConfirmJoin,
            is_can_lobby_online: true,
            room_host_id: 0,
            match_inst_id: 0,
            match_host_id: 0,
            match_ready: false,
            match_confirm: false,
            match_ui_state: EMatchPlayerUiState::Wait,
            match_roles: [0; 4],
            area_id: 1,
        }
    }
}

#[derive(Default)]
pub struct CoopService {
    mutex: Mutex<()>,
    social_mutex: Mutex<()>,
    advices_mutex: Mutex<()>,
    online: Mutex<HashMap<i32, OnlinePlayer>>,
}

impl CoopService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn advices_lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.advices_mutex.lock().unwrap_or_else(|p| p.into_inner())
    }

    pub fn social_lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.social_mutex.lock().unwrap_or_else(|p| p.into_inner())
    }

    pub fn state_lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.mutex.lock().unwrap_or_else(|p| p.into_inner())
    }

    pub fn register_online(&self, mut entry: OnlinePlayer) {
        let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        if entry.room_host_id == 0 {
            entry.room_host_id = entry.player_id;
        }
        if let Some(prev) = online.get(&entry.player_id) {
            entry.room_host_id = prev.room_host_id;
            entry.enter_permission = prev.enter_permission;
        }
        online.insert(entry.player_id, entry);
    }

    pub fn unregister_online(&self, player_id: i32) {
        let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online.remove(&player_id);
    }

    pub fn with_online_player_mut<R>(&self, player_id: i32, f: impl FnOnce(&mut OnlinePlayer) -> R) -> Option<R> {
        let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online.get_mut(&player_id).map(f)
    }

    pub fn with_online_player<R>(&self, player_id: i32, f: impl FnOnce(&OnlinePlayer) -> R) -> Option<R> {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online.get(&player_id).map(f)
    }

    pub fn set_permission(&self, player_id: i32, perm: WorldEnterPermission) {
        let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(p) = online.get_mut(&player_id) {
            p.enter_permission = perm;
            p.is_can_lobby_online = perm != WorldEnterPermission::ForbidJoin;
        }
    }

    pub fn online_player_areas(&self) -> Vec<(i32, i32)> {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online
            .values()
            .map(|p| (p.player_id, p.area_id))
            .collect()
    }

    pub fn push_to_player<M: PbMessage>(&self, target_id: i32, msg_id: u16, msg: &M) -> bool {
        let sender = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            online.get(&target_id).map(|p| Arc::clone(&p.sender))
        };
        match sender {
            Some(s) => s.send_raw(msg_id, msg.encode_to_vec()),
            None => false,
        }
    }

    pub fn broadcast_room<M: PbMessage>(&self, host_id: i32, exclude_id: i32, msg_id: u16, msg: &M) {
        let senders: Vec<Arc<dyn CoopSender>> = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            online
                .values()
                .filter(|p| p.room_host_id == host_id && p.player_id != exclude_id)
                .take(8)
                .map(|p| Arc::clone(&p.sender))
                .collect()
        };
        if senders.is_empty() {
            return;
        }
        let payload = msg.encode_to_vec();
        for s in senders {
            let _ = s.send_raw(msg_id, payload.clone());
        }
    }

    pub fn broadcast_move_except(&self, sender_id: i32, moving: Vec<MovingEntityData>) {
        let senders: Vec<Arc<dyn CoopSender>> = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            let host_id = online.get(&sender_id).map_or(0, |s| s.room_host_id);
            if host_id <= 0 {
                return;
            }
            online
                .values()
                .filter(|p| p.room_host_id == host_id && p.player_id != sender_id)
                .take(8)
                .map(|p| Arc::clone(&p.sender))
                .collect()
        };
        if senders.is_empty() {
            return;
        }
        let notify = MovePackageNotify {
            MovingEntities: moving,
        };
        let payload = notify.encode_to_vec();
        for s in senders {
            let _ = s.send_raw(MovePackageNotify::MSG_ID, payload.clone());
        }
    }

    pub fn count_room_members(&self, host_id: i32) -> i32 {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        count_room_members_locked(&online, host_id)
    }

    pub fn build_lobby_list(&self, caller_id: i32) -> Vec<PlayerDetails> {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online
            .values()
            .filter(|p| {
                p.player_id != caller_id && p.enter_permission != WorldEnterPermission::ForbidJoin
            })
            .map(|p| to_details(p, count_room_members_locked(&online, p.room_host_id)))
            .collect()
    }

    pub fn get_online_details(&self, target_id: i32) -> Option<PlayerDetails> {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        let p = online.get(&target_id)?;
        let cnt = count_room_members_locked(&online, p.room_host_id);
        Some(to_details(p, cnt))
    }

    pub fn request_join_world(
        &self,
        fs: &FileSystem,
        requester_id: i32,
        host_id: i32,
    ) -> io::Result<ErrorCode> {
        let (host, req) = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            (online.get(&host_id).cloned(), online.get(&requester_id).cloned())
        };
        let Some(h) = host else {
            return Ok(ErrorCode::PlayerLoggedOut);
        };
        let Some(r) = req else {
            return Ok(ErrorCode::InternalError);
        };
        if h.enter_permission == WorldEnterPermission::ForbidJoin {
            return Ok(ErrorCode::ErrHostForbidJoin);
        }
        if h.enter_permission == WorldEnterPermission::DirectJoin {
            let _ = self.admit_to_room(fs, host_id, requester_id, true)?;
            return Ok(ErrorCode::Success);
        }
        let _ = self.push_to_player(
            host_id,
            ApplyJoinWorldNotify::MSG_ID,
            &ApplyJoinWorldNotify {
                PlayerId: r.player_id,
                PlayerName: r.name.into(),
                Level: r.level,
                HeadId: r.head_id,
                ..Default::default()
            },
        );
        Ok(ErrorCode::Success)
    }

    pub fn admit_to_room(
        &self,
        fs: &FileSystem,
        host_id: i32,
        guest_id: i32,
        accept: bool,
    ) -> io::Result<ErrorCode> {
        let (host_snap, room_members, guest_world_info) = {
            let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            let Some(host_snap) = online.get(&host_id).cloned() else {
                return Ok(ErrorCode::PlayerLoggedOut);
            };
            if !online.contains_key(&guest_id) {
                return Ok(ErrorCode::PlayerLoggedOut);
            }
            if !accept {
                drop(online);
                let _ = self.push_to_player(
                    guest_id,
                    AgreeJoinResultNotify::MSG_ID,
                    &AgreeJoinResultNotify {
                        ErrorCode: Some(ErrorCode::ErrHostRefuse),
                        PlayerName: host_snap.name.into(),
                    },
                );
                return Ok(ErrorCode::Success);
            }
            if let Some(g) = online.get_mut(&guest_id) {
                g.room_host_id = host_id;
            }
            let member_count = count_room_members_locked(&online, host_id);
            let mut room_members = Vec::new();
            let mut guest_world_info = None;
            for p in online.values() {
                if p.room_host_id != host_id {
                    continue;
                }
                let det = to_details(p, member_count);
                let wi = to_world_team_member(p, det);
                if p.player_id == guest_id {
                    guest_world_info = Some(wi.clone());
                }
                room_members.push(wi);
            }
            (host_snap, room_members, guest_world_info)
        };

        self.record_recent_team(fs, host_id, guest_id)?;
        self.record_recent_team(fs, guest_id, host_id)?;

        let _ = self.push_to_player(
            guest_id,
            AgreeJoinResultNotify::MSG_ID,
            &AgreeJoinResultNotify {
                ErrorCode: Some(ErrorCode::Success),
                PlayerName: host_snap.name.into(),
            },
        );
        let _ = self.push_to_player(
            guest_id,
            JoinWorldTeamNotify::MSG_ID,
            &JoinWorldTeamNotify {
                OwnerId: host_id,
                PlayerInfos: room_members,
            },
        );
        if let Some(gwi) = guest_world_info {
            self.broadcast_room(
                host_id,
                guest_id,
                PlayerEnterWorldTeamNotify::MSG_ID,
                &PlayerEnterWorldTeamNotify {
                    PlayerInfo: Some(gwi),
                },
            );
        }
        Ok(ErrorCode::Success)
    }

    pub fn leave_or_kick_room(&self, target_id: i32, reason: WorldTeamLeaveReason) {
        let old_host = {
            let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            if let Some(p) = online.get_mut(&target_id) {
                let h = p.room_host_id;
                p.room_host_id = target_id;
                h
            } else {
                0
            }
        };
        let notify = PlayerLeaveWorldTeamNotify {
            PlayerId: target_id,
            Reason: Some(reason),
        };
        if old_host > 0 {
            self.broadcast_room(old_host, 0, PlayerLeaveWorldTeamNotify::MSG_ID, &notify);
        }
        let _ = self.push_to_player(target_id, PlayerLeaveWorldTeamNotify::MSG_ID, &notify);
    }

    pub fn load_social(fs: &FileSystem, player_id: i32) -> io::Result<SocialRecord> {
        let _guard = SOCIAL_FILE_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        load_or_create_zon::<SocialRecord>(fs, &format!("social/{player_id}"))
    }

    pub fn save_social(fs: &FileSystem, player_id: i32, rec: &SocialRecord) -> io::Result<()> {
        let _guard = SOCIAL_FILE_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let bytes = serialize_zon(rec)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        fs.write_file(&format!("social/{player_id}"), &bytes)
    }

    pub fn record_recent_team(
        &self,
        fs: &FileSystem,
        player_id: i32,
        mate_id: i32,
    ) -> io::Result<()> {
        if player_id == mate_id {
            return Ok(());
        }
        let _guard = self.social_mutex.lock().unwrap_or_else(|p| p.into_inner());
        let mut rec = Self::load_social(fs, player_id)?;
        let now_s = now_seconds();
        if let Some(existing) = rec.recent_teams.iter_mut().find(|r| r.id == mate_id) {
            existing.team_time = now_s;
        } else {
            rec.recent_teams.push(RecentTeamRecord {
                id: mate_id,
                team_time: now_s,
            });
        }
        Self::save_social(fs, player_id, &rec)
    }

    pub fn send_friend_apply(
        &self,
        fs: &FileSystem,
        sender_id: i32,
        target_id: i32,
    ) -> io::Result<ErrorCode> {
        if sender_id == target_id {
            return Ok(ErrorCode::RequestParamError);
        }
        {
            let _guard = self.social_mutex.lock().unwrap_or_else(|p| p.into_inner());
            let mut target = Self::load_social(fs, target_id)?;
            if target.blocked.contains(&sender_id) {
                return Ok(ErrorCode::RequestParamError);
            }
            append_unique_i32(&mut target.pending_applies, sender_id);
            Self::save_social(fs, target_id, &target)?;
        }

        if let Some(det) = self.get_online_details(sender_id) {
            let _ = self.push_to_player(
                target_id,
                FriendApplyReceivedNotify::MSG_ID,
                &FriendApplyReceivedNotify {
                    FriendApply: Some(FriendApply {
                        Info: Some(det),
                        CreatedTime: now_seconds(),
                    }),
                },
            );
        }
        Ok(ErrorCode::Success)
    }

    pub fn handle_friend_apply(
        &self,
        fs: &FileSystem,
        player_id: i32,
        requester_id: i32,
        approve: bool,
    ) -> io::Result<()> {
        {
            let _guard = self.social_mutex.lock().unwrap_or_else(|p| p.into_inner());
            let mut mine = Self::load_social(fs, player_id)?;
            remove_i32(&mut mine.pending_applies, requester_id);
            if approve {
                add_friend(&mut mine, requester_id, "");
            }
            Self::save_social(fs, player_id, &mine)?;

            if approve {
                let mut other = Self::load_social(fs, requester_id)?;
                add_friend(&mut other, player_id, "");
                Self::save_social(fs, requester_id, &other)?;
            }
        }

        if approve {
            if let Some(det) = self.get_online_details(player_id) {
                let _ = self.push_to_player(
                    requester_id,
                    FriendAddedNotify::MSG_ID,
                    &FriendAddedNotify {
                        Info: Some(FriendInfo {
                            Info: Some(det),
                            Remark: Vec::new(),
                        }),
                    },
                );
            }
            if let Some(det) = self.get_online_details(requester_id) {
                let _ = self.push_to_player(
                    player_id,
                    FriendAddedNotify::MSG_ID,
                    &FriendAddedNotify {
                        Info: Some(FriendInfo {
                            Info: Some(det),
                            Remark: Vec::new(),
                        }),
                    },
                );
            }
        } else {
            let _ = self.push_to_player(
                requester_id,
                FriendApplyDeletedNotify::MSG_ID,
                &FriendApplyDeletedNotify { Id: player_id },
            );
        }
        Ok(())
    }

    pub fn delete_friend(
        &self,
        fs: &FileSystem,
        player_id: i32,
        target_id: i32,
    ) -> io::Result<()> {
        {
            let _guard = self.social_mutex.lock().unwrap_or_else(|p| p.into_inner());
            let mut mine = Self::load_social(fs, player_id)?;
            remove_friend(&mut mine, target_id);
            Self::save_social(fs, player_id, &mine)?;

            let mut other = Self::load_social(fs, target_id)?;
            remove_friend(&mut other, player_id);
            Self::save_social(fs, target_id, &other)?;
        }
        let _ = self.push_to_player(
            target_id,
            FriendDeletedNotify::MSG_ID,
            &FriendDeletedNotify { Id: player_id },
        );
        Ok(())
    }

    pub fn block_player(
        fs: &FileSystem,
        player_id: i32,
        target_id: i32,
        is_block: bool,
    ) -> io::Result<()> {
        let mut mine = Self::load_social(fs, player_id)?;
        if is_block {
            append_unique_i32(&mut mine.blocked, target_id);
        } else {
            remove_i32(&mut mine.blocked, target_id);
        }
        Self::save_social(fs, player_id, &mine)
    }

    pub fn load_advices(fs: &FileSystem) -> io::Result<AdviceStore> {
        let _guard = ADVICE_FILE_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        load_or_create_zon::<AdviceStore>(fs, "social/advices")
    }

    pub fn save_advices(fs: &FileSystem, store: &AdviceStore) -> io::Result<()> {
        let _guard = ADVICE_FILE_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
        let bytes = serialize_zon(store)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        fs.write_file("social/advices", &bytes)
    }

    pub fn is_room_host(&self, caller_id: i32, target_id: i32) -> bool {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        let Some(caller) = online.get(&caller_id) else {
            return false;
        };
        if caller.room_host_id != caller_id {
            return false;
        }
        online
            .get(&target_id)
            .is_some_and(|t| t.room_host_id == caller_id)
    }

    pub fn is_match_host(&self, caller_id: i32) -> bool {
        let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        online
            .get(&caller_id)
            .is_some_and(|c| c.match_host_id == caller_id || c.match_host_id == 0)
    }

    pub fn broadcast_match<M: PbMessage>(
        &self,
        player_id: i32,
        exclude_id: i32,
        msg_id: u16,
        msg: &M,
    ) {
        let senders: Vec<Arc<dyn CoopSender>> = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            let host_id = online.get(&player_id).map_or(0, |p| {
                if p.match_host_id > 0 {
                    p.match_host_id
                } else {
                    player_id
                }
            });
            if host_id <= 0 {
                return;
            }
            online
                .values()
                .filter(|p| {
                    let p_host = if p.match_host_id > 0 {
                        p.match_host_id
                    } else {
                        p.player_id
                    };
                    (p_host == host_id || p.player_id == player_id) && p.player_id != exclude_id
                })
                .take(8)
                .map(|p| Arc::clone(&p.sender))
                .collect()
        };
        if senders.is_empty() {
            return;
        }
        let payload = msg.encode_to_vec();
        for s in senders {
            let _ = s.send_raw(msg_id, payload.clone());
        }
    }

    pub fn confirm_match(&self, player_id: i32, is_accept: bool) {
        if !is_accept {
            self.broadcast_match(
                player_id,
                0,
                MatchFailNotify::MSG_ID,
                &MatchFailNotify {
                    Reason: Some(MatchFailReason::NotAccept),
                },
            );
            let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            let host_id = online.get(&player_id).map_or(0, |p| {
                if p.match_host_id > 0 {
                    p.match_host_id
                } else {
                    player_id
                }
            });
            for p in online.values_mut() {
                if p.player_id == player_id || (host_id > 0 && p.match_host_id == host_id) {
                    p.match_inst_id = 0;
                    p.match_host_id = 0;
                    p.match_ready = false;
                    p.match_confirm = false;
                    p.match_ui_state = EMatchPlayerUiState::Wait;
                }
            }
            return;
        }

        let all_confirmed = {
            let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            let mut host_id = 0;
            if let Some(me) = online.get_mut(&player_id) {
                me.match_confirm = true;
                host_id = if me.match_host_id > 0 {
                    me.match_host_id
                } else {
                    player_id
                };
            }
            host_id > 0
                && online
                    .values()
                    .all(|p| p.match_host_id != host_id || p.match_confirm)
        };

        if all_confirmed {
            self.broadcast_match(
                player_id,
                0,
                MatchTeamStateNotify::MSG_ID,
                &MatchTeamStateNotify {
                    TeamState: Some(MatchTeamState::EnterInst),
                },
            );
        }
    }

    pub fn start_match(&self, player_id: i32, inst_id: i32) {
        let (matched_ids, team_players) = {
            let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            if let Some(me) = online.get_mut(&player_id) {
                me.match_inst_id = inst_id;
                me.match_host_id = player_id;
                me.match_ready = true;
                me.match_confirm = false;
                me.match_ui_state = EMatchPlayerUiState::Confirm;
            }
            let mut matched_ids = vec![player_id];
            for p in online.values_mut() {
                if p.player_id == player_id {
                    continue;
                }
                if p.match_inst_id == inst_id && matched_ids.len() < 4 {
                    p.match_host_id = player_id;
                    matched_ids.push(p.player_id);
                }
            }
            let team_players: Vec<MatchPlayerInfo> = matched_ids
                .iter()
                .filter_map(|mid| online.get(mid))
                .map(|mp| MatchPlayerInfo {
                    PlayerId: mp.player_id,
                    PlayerName: mp.name.as_bytes().to_vec(),
                    IsConfirm: mp.match_confirm,
                    IsReady: mp.match_ready,
                    MatchUiState: Some(mp.match_ui_state),
                    ..Default::default()
                })
                .collect();
            (matched_ids, team_players)
        };

        for mid in matched_ids {
            let _ = self.push_to_player(mid, StartMatchNotify::MSG_ID, &StartMatchNotify { InstId: inst_id });
            let _ = self.push_to_player(
                mid,
                MatchTeamNotify::MSG_ID,
                &MatchTeamNotify {
                    TeamInfo: Some(MatchTeamInfo {
                        HostId: player_id,
                        PlayerInfos: team_players.clone(),
                        TeamState: Some(MatchTeamState::ReadyConfirm),
                        ..Default::default()
                    }),
                },
            );
            let _ = self.push_to_player(
                mid,
                MatchConfirmNotify::MSG_ID,
                &MatchConfirmNotify { ConfirmId: inst_id },
            );
        }
    }

    pub fn cancel_match(&self, player_id: i32) {
        self.broadcast_match(
            player_id,
            0,
            CancelMatchNotify::MSG_ID,
            &CancelMatchNotify {
                PlayerId: player_id,
            },
        );
        let mut online = self.online.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(me) = online.get_mut(&player_id) {
            me.match_inst_id = 0;
            me.match_host_id = 0;
            me.match_ready = false;
            me.match_confirm = false;
            me.match_ui_state = EMatchPlayerUiState::Wait;
        }
    }

    pub fn broadcast_all<M: PbMessage>(&self, exclude_id: i32, msg_id: u16, msg: &M) -> usize {
        let senders: Vec<Arc<dyn CoopSender>> = {
            let online = self.online.lock().unwrap_or_else(|p| p.into_inner());
            online
                .values()
                .filter(|p| p.player_id != exclude_id)
                .map(|p| Arc::clone(&p.sender))
                .collect()
        };
        if senders.is_empty() {
            return 0;
        }
        let payload = msg.encode_to_vec();
        let mut delivered = 0usize;
        for s in senders {
            if s.send_raw(msg_id, payload.clone()) {
                delivered += 1;
            }
        }
        delivered
    }

    pub fn deliver_private_chat(
        &self,
        fs: &FileSystem,
        sender_id: i32,
        target_id: i32,
        content: &ChatContentProto,
    ) -> bool {
        let delivered_live = if target_id > 0 && target_id != sender_id {
            self.push_to_player(
                target_id,
                ChatNotify::MSG_ID,
                &ChatNotify {
                    Content: content.encode_to_vec(),
                },
            )
        } else {
            false
        };

        if target_id > 0 && target_id != sender_id {
            let _guard = SOCIAL_FILE_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
            let rel_path = format!("player/{target_id}/progress_info");
            if let Ok(Some(bytes)) = fs.read_file(&rel_path)
                && let Ok(mut target_pc) = parse_zon::<ProgressInfo>(&bytes)
            {
                let priv_key = format!("chat:priv:{sender_id}");
                let b64 = target_pc.get_key_str(&priv_key);
                let mut hist = if b64.is_empty() {
                    None
                } else {
                    BASE64_STANDARD
                        .decode(b64)
                        .ok()
                        .and_then(|raw| PrivateChatHistoryContentProto::decode(&raw).ok())
                }
                .unwrap_or_else(|| PrivateChatHistoryContentProto {
                    TargetUid: sender_id,
                    Chats: Vec::new(),
                    HistoryIsEnd: true,
                    TotalNums: 0,
                });
                hist.TargetUid = sender_id;
                hist.Chats.push(content.clone());
                if hist.Chats.len() > 50 {
                    let drain = hist.Chats.len() - 50;
                    hist.Chats.drain(..drain);
                }
                hist.HistoryIsEnd = true;
                hist.TotalNums = hist.Chats.len() as i32;
                let encoded = BASE64_STANDARD.encode(hist.encode_to_vec());
                target_pc.set_key_str(&priv_key, &encoded);
                if let Ok(out) = serialize_zon(&target_pc) {
                    let _ = fs.write_file(&rel_path, &out);
                }
            }
        }
        delivered_live
    }

    pub fn collect_server_player_snapshots(
        fs: &FileSystem,
        coop: Option<&CoopService>,
        exclude_id: i32,
    ) -> Vec<ServerPlayerSnapshot> {
        let mut ids = Vec::new();
        if let Ok(Some(entries)) = fs.read_dir("player") {
            for ent in entries {
                if !ent.is_dir {
                    continue;
                }
                if let Some(pid_str) = ent.path.strip_prefix("player/")
                    && !pid_str.contains('/')
                    && let Ok(pid) = pid_str.parse::<i32>()
                    && pid > 0
                    && pid != exclude_id
                {
                    append_unique_i32(&mut ids, pid);
                }
            }
        }
        if let Some(svc) = coop {
            let online = svc.online.lock().unwrap_or_else(|p| p.into_inner());
            for &pid in online.keys() {
                if pid > 0 && pid != exclude_id {
                    append_unique_i32(&mut ids, pid);
                }
            }
        }
        ids.sort_unstable();

        let mut snapshots = Vec::new();
        for pid in ids.into_iter().take(50) {
            let online_entry = coop.and_then(|svc| svc.with_online_player(pid, Clone::clone));
            let basic_opt = fs
                .read_file(&format!("player/{pid}/basic_info"))
                .ok()
                .flatten()
                .and_then(|b| parse_zon::<BasicInfo>(&b).ok());
            let progress = fs
                .read_file(&format!("player/{pid}/progress_info"))
                .ok()
                .flatten()
                .and_then(|b| parse_zon::<ProgressInfo>(&b).ok())
                .unwrap_or_default();

            let (name, head_id, is_online) = if let Some(ref onl) = online_entry {
                (onl.name.clone(), onl.head_id, true)
            } else if let Some(ref b) = basic_opt {
                (
                    b.attributes.name.clone(),
                    b.attributes.head_photo.max(82_001_209),
                    false,
                )
            } else {
                continue;
            };

            snapshots.push(ServerPlayerSnapshot {
                player_id: pid,
                name: if name.is_empty() {
                    format!("Rover {pid}")
                } else {
                    name
                },
                head_id,
                progress,
                is_online,
            });
        }
        snapshots
    }
}

#[derive(Debug, Clone)]
pub struct ServerPlayerSnapshot {
    pub player_id: i32,
    pub name: String,
    pub head_id: i32,
    pub progress: ProgressInfo,
    pub is_online: bool,
}

pub fn append_unique_i32(list: &mut Vec<i32>, val: i32) -> bool {
    if list.contains(&val) {
        return false;
    }
    list.push(val);
    true
}

pub fn remove_i32(list: &mut Vec<i32>, val: i32) -> bool {
    if let Some(pos) = list.iter().position(|&v| v == val) {
        list.remove(pos);
        true
    } else {
        false
    }
}

pub fn add_friend(rec: &mut SocialRecord, target_id: i32, remark: &str) -> bool {
    if rec.friends.iter().any(|f| f.id == target_id) {
        return false;
    }
    rec.friends.push(FriendRecord {
        id: target_id,
        remark: remark.to_string(),
    });
    true
}

pub fn remove_friend(rec: &mut SocialRecord, target_id: i32) -> bool {
    if let Some(pos) = rec.friends.iter().position(|f| f.id == target_id) {
        rec.friends.remove(pos);
        true
    } else {
        false
    }
}

fn count_room_members_locked(online: &HashMap<i32, OnlinePlayer>, host_id: i32) -> i32 {
    online
        .values()
        .filter(|p| p.room_host_id == host_id)
        .count() as i32
}

fn to_details(p: &OnlinePlayer, count: i32) -> PlayerDetails {
    PlayerDetails {
        PlayerId: p.player_id,
        Name: p.name.as_bytes().to_vec(),
        Level: p.level,
        OriginWorldLevel: p.origin_world_level,
        CurWorldLevel: p.cur_world_level,
        HeadId: p.head_id,
        HeadFrameId: p.head_frame_id,
        Signature: p.signature.as_bytes().to_vec(),
        IsOnline: true,
        IsCanLobbyOnline: p.is_can_lobby_online,
        TeamMemberCount: count,
        Sex: p.sex,
        CrossPlayEnabled: true,
        ..Default::default()
    }
}

fn to_world_team_member(p: &OnlinePlayer, details: PlayerDetails) -> WorldTeamPlayerInfo {
    WorldTeamPlayerInfo {
        PlayerId: p.player_id,
        PlayerName: p.name.as_bytes().to_vec(),
        HeadId: p.head_id,
        Level: p.level,
        Signature: p.signature.as_bytes().to_vec(),
        FightInfo: Some(WorldTeamPlayerFightInfo {
            CurRoleId: p.cur_role_id,
            RoleInfos: vec![WorldTeamRoleInfo {
                RoleId: p.cur_role_id,
                RoleLevel: 90,
                ..Default::default()
            }],
        }),
        Item: Some(details),
        Sex: p.sex,
        WorldLevel: p.cur_world_level,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coop_service_manages_online_lobby_and_rooms() {
        let coop = CoopService::new();
        let sender: Arc<dyn CoopSender> = Arc::new(VecCoopSender::default());

        let mut host = OnlinePlayer::new(10, Arc::clone(&sender), "HostPlayer");
        host.enter_permission = WorldEnterPermission::DirectJoin;
        coop.register_online(host);
        coop.register_online(OnlinePlayer::new(20, sender, "GuestPlayer"));

        assert_eq!(coop.count_room_members(10), 1);
        let lobby = coop.build_lobby_list(20);
        assert_eq!(lobby.len(), 1);
        assert_eq!(lobby[0].PlayerId, 10);
    }
}
