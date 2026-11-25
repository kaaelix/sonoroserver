use serde::{Deserialize, Serialize};
use sonoro_proto::enums::MailLevel;
use sonoro_proto::pb::{MailInfosNotify, PbMailAttachment, PbMailInfo};
use sonoro_proto::wire::PbEnum;

pub mod state {
    pub const UNREAD: i32 = 0;
    pub const READ: i32 = 1;
    pub const CLAIMED: i32 = 2;
    pub const FAVORITE: i32 = 4;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Attachment {
    pub item_id: i32,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MailEntry {
    pub id: String,
    pub config_id: i32,
    pub title: String,
    pub content: String,
    pub sender: String,
    pub level: i32,
    pub received_time: i64,
    pub read_time: i64,
    pub state: i32,
    pub valid_time: i32,
    pub read_valid_time: i32,
    pub expiry_time: i64,
    pub is_favorite: bool,
    pub attachments: Vec<Attachment>,
}

impl Default for MailEntry {
    fn default() -> Self {
        Self {
            id: String::new(),
            config_id: 0,
            title: String::new(),
            content: String::new(),
            sender: String::new(),
            level: 1,
            received_time: 0,
            read_time: 0,
            state: 0,
            valid_time: 2_592_000,
            read_valid_time: 2_592_000,
            expiry_time: 0,
            is_favorite: false,
            attachments: Vec::new(),
        }
    }
}

impl MailEntry {
    pub fn is_read(&self) -> bool {
        (self.state & state::READ) != 0
    }

    pub fn is_claimed(&self) -> bool {
        (self.state & state::CLAIMED) != 0
    }

    pub fn is_favorite(&self) -> bool {
        self.is_favorite || (self.state & state::FAVORITE) != 0
    }

    pub fn wire_state(&self) -> i32 {
        if self.is_favorite {
            self.state | state::FAVORITE
        } else {
            self.state & !state::FAVORITE
        }
    }

    pub fn to_proto(&self) -> PbMailInfo {
        let attachments = self
            .attachments
            .iter()
            .filter(|a| a.item_id > 0 && a.count > 0)
            .map(|a| PbMailAttachment {
                Id: a.item_id,
                Count: a.count,
            })
            .collect();
        PbMailInfo {
            Id: self.id.as_bytes().to_vec(),
            ReceivedTime: self.received_time,
            ReadTime: self.read_time,
            State: self.wire_state(),
            Level: Some(MailLevel::from_i32_or_default(self.level)),
            Title: self.title.as_bytes().to_vec(),
            Content: self.content.as_bytes().to_vec(),
            Sender: self.sender.as_bytes().to_vec(),
            ValidTime: self.valid_time,
            ReadValidTime: self.read_valid_time,
            Attachments: attachments,
            ConfigId: self.config_id,
            ExpiryTime: self.expiry_time,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MailInfo {
    pub mails: Vec<MailEntry>,
    pub bind_rewarded: bool,
}

impl MailInfo {
    pub const DATA_PATH: &'static str = "mail_info";

    pub fn find_mut(&mut self, id: &str) -> Option<&mut MailEntry> {
        self.mails.iter_mut().find(|m| m.id == id)
    }

    pub fn upsert_npc_mail(&mut self, mail_id: i32, now_sec: i64) -> &mut MailEntry {
        let id_str = mail_id.to_string();
        if let Some(pos) = self
            .mails
            .iter()
            .position(|m| m.config_id == mail_id || m.id == id_str)
        {
            return &mut self.mails[pos];
        }
        self.mails.push(MailEntry {
            id: id_str,
            config_id: mail_id,
            received_time: now_sec,
            state: state::UNREAD,
            valid_time: 2_592_000,
            read_valid_time: 2_592_000,
            expiry_time: now_sec + 2_592_000,
            ..Default::default()
        });
        self.mails.last_mut().expect("just pushed")
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_mail(
        &mut self,
        id: &str,
        config_id: i32,
        title: &str,
        content: &str,
        sender: &str,
        attachments: &[Attachment],
        now_sec: i64,
        valid_seconds: i32,
    ) -> &mut MailEntry {
        if let Some(pos) = self.mails.iter().position(|m| m.id == id) {
            return &mut self.mails[pos];
        }
        let ttl = if valid_seconds > 0 {
            valid_seconds
        } else {
            2_592_000
        };
        self.mails.push(MailEntry {
            id: id.to_string(),
            config_id,
            title: title.to_string(),
            content: content.to_string(),
            sender: sender.to_string(),
            level: 1,
            received_time: now_sec,
            read_time: 0,
            state: state::UNREAD,
            valid_time: ttl,
            read_valid_time: ttl,
            expiry_time: now_sec + i64::from(ttl),
            is_favorite: false,
            attachments: attachments.to_vec(),
        });
        self.mails.last_mut().expect("just pushed")
    }

    pub fn prune_expired(&mut self, now_sec: i64) -> bool {
        let before = self.mails.len();
        self.mails
            .retain(|m| !(m.expiry_time > 0 && m.expiry_time <= now_sec));
        self.mails.len() != before
    }

    pub fn remove_mail(&mut self, id: &str) -> bool {
        let before = self.mails.len();
        self.mails.retain(|m| m.id != id);
        self.mails.len() != before
    }

    pub fn to_proto_notify(&self) -> MailInfosNotify {
        MailInfosNotify {
            MailInfos: self.mails.iter().map(MailEntry::to_proto).collect(),
        }
    }
}
