use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PublicBabelTowerChallengeInfo {
    pub ChallengeId: i32,
    pub Star: i32,
    pub UseTime: i32,
    pub BabelPassRoleInfos: Vec<BabelPassRoleInfo>,
}

impl PbMessage for PublicBabelTowerChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.Star != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Star);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        for item in &self.BabelPassRoleInfos {
            crate::wire::write_submessage(out, 4, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.Star = r.read_varint_i32()?,
            3 => self.UseTime = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BabelPassRoleInfos
                    .push(<BabelPassRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PublicBabelTowerInfo {
    pub ChallengeInfo: Vec<PublicBabelTowerChallengeInfo>,
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub HeadId: i32,
    pub TitleId: i32,
    pub TitleExtraParam: i32,
    pub ShowName: bool,
}

impl PbMessage for PublicBabelTowerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
        }
        if self.TitleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TitleId);
        }
        if self.TitleExtraParam != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TitleExtraParam);
        }
        if self.ShowName {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfo
                    .push(<PublicBabelTowerChallengeInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.PlayerId = r.read_varint_i32()?,
            4 => self.Name = r.read_len_delimited()?.to_vec(),
            5 => self.HeadId = r.read_varint_i32()?,
            6 => self.TitleId = r.read_varint_i32()?,
            7 => self.TitleExtraParam = r.read_varint_i32()?,
            9 => self.ShowName = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PullingFoundationRequest {
    pub Rot: Option<Rotator>,
    pub Pos: Option<Vector>,
    pub AccusedEntityId: i64,
    pub EntityId: i64,
}

impl PullingFoundationRequest {
    pub const MSG_ID: u16 = 19966;
}

impl PbMessage for PullingFoundationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.AccusedEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccusedEntityId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.AccusedEntityId = r.read_varint_i64()?,
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PullingFoundationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PullingFoundationResponse {
    pub const MSG_ID: u16 = 21229;
}

impl PbMessage for PullingFoundationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QingXiaoTaskRewardRequest {
    pub TaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl QingXiaoTaskRewardRequest {
    pub const MSG_ID: u16 = 29336;
}

impl PbMessage for QingXiaoTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QingXiaoTaskRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl QingXiaoTaskRewardResponse {
    pub const MSG_ID: u16 = 21118;
}

impl PbMessage for QingXiaoTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QingXiaoTaskUpdateNotify {
    pub Task: Option<ConditionTask>,
}

impl QingXiaoTaskUpdateNotify {
    pub const MSG_ID: u16 = 22617;
}

impl PbMessage for QingXiaoTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
            crate::wire::write_submessage(out, 15, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryOnlinePlayersAreaRequest {}

impl QueryOnlinePlayersAreaRequest {
    pub const MSG_ID: u16 = 15827;
}

impl PbMessage for QueryOnlinePlayersAreaRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryOnlinePlayersAreaResponse {
    pub AreaDict: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl QueryOnlinePlayersAreaResponse {
    pub const MSG_ID: u16 = 21413;
}

impl PbMessage for QueryOnlinePlayersAreaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AreaDict.is_empty() {
            crate::wire::write_map(out, 13, &self.AreaDict);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.AreaDict)?,
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryRecallItemBagRequest {
    pub RecallId: i32,
}

impl QueryRecallItemBagRequest {
    pub const MSG_ID: u16 = 20446;
}

impl PbMessage for QueryRecallItemBagRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.RecallId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QueryRecallItemBagResponse {
    pub Items: Vec<RecallItemBagItem>,
    pub ErrorCode: Option<ErrorCode>,
}

impl QueryRecallItemBagResponse {
    pub const MSG_ID: u16 = 25460;
}

impl PbMessage for QueryRecallItemBagResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<RecallItemBagItem as PbMessage>::decode(bytes)?);
            }
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestActionRequest {
    pub NodeId: i32,
    pub ActionId: i32,
    pub ActTime: Option<ActionTime>,
    pub QuestId: i32,
}

impl QuestActionRequest {
    pub const MSG_ID: u16 = 18164;
}

impl PbMessage for QuestActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionId);
        }
        if let Some(v) = &self.ActTime {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.NodeId = r.read_varint_i32()?,
            3 => self.ActionId = r.read_varint_i32()?,
            13 => {
                self.ActTime = Some(<ActionTime as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestActionResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl QuestActionResponse {
    pub const MSG_ID: u16 = 15105;
}

impl PbMessage for QuestActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestChangeTraceNotify {
    pub QuestIds: Vec<i32>,
}

impl QuestChangeTraceNotify {
    pub const MSG_ID: u16 = 22862;
}

impl PbMessage for QuestChangeTraceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFastReturnPosRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl QuestFastReturnPosRequest {
    pub const MSG_ID: u16 = 23769;
}

impl PbMessage for QuestFastReturnPosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.TreeOwnerId = r.read_varint_i32()?,
            6 => self.NodeId = r.read_varint_i32()?,
            14 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFastReturnPosResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestFastReturnPosResponse {
    pub const MSG_ID: u16 = 28798;
}

impl PbMessage for QuestFastReturnPosResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFinishActionEndRequest {
    pub QuestId: Vec<i32>,
}

impl QuestFinishActionEndRequest {
    pub const MSG_ID: u16 = 29054;
}

impl PbMessage for QuestFinishActionEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFinishActionEndResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl QuestFinishActionEndResponse {
    pub const MSG_ID: u16 = 18310;
}

impl PbMessage for QuestFinishActionEndResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFinishActionRequest {
    pub NodeId: i32,
    pub ActTime: Option<ActionTime>,
    pub ActionId: i32,
    pub QuestId: i32,
}

impl QuestFinishActionRequest {
    pub const MSG_ID: u16 = 15063;
}

impl PbMessage for QuestFinishActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if let Some(v) = &self.ActTime {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionId);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.NodeId = r.read_varint_i32()?,
            8 => {
                self.ActTime = Some(<ActionTime as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.ActionId = r.read_varint_i32()?,
            14 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFinishActionResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl QuestFinishActionResponse {
    pub const MSG_ID: u16 = 17278;
}

impl PbMessage for QuestFinishActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestFocusInfoNotify {
    pub FocusQuestId: i32,
}

impl QuestFocusInfoNotify {
    pub const MSG_ID: u16 = 16030;
}

impl PbMessage for QuestFocusInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FocusQuestId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FocusQuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.FocusQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestGiveUpNotify {
    pub FlowIncIds: Vec<i64>,
    pub QuestId: i32,
    pub TreeIncId: i64,
}

impl QuestGiveUpNotify {
    pub const MSG_ID: u16 = 26304;
}

impl PbMessage for QuestGiveUpNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlowIncIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FlowIncIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            12 => self.QuestId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestLockInfo {
    pub QuestId: i32,
    pub MultiModeLock: bool,
    pub LockType: Option<QuestLockType>,
}

impl PbMessage for QuestLockInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
        if self.MultiModeLock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.LockType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.QuestId = r.read_varint_i32()?,
            2 => self.MultiModeLock = r.read_bool()?,
            3 => {
                self.LockType = Some(<QuestLockType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestLockInfoNotify {
    pub QuestLockInfos: Vec<QuestLockInfo>,
}

impl QuestLockInfoNotify {
    pub const MSG_ID: u16 = 17725;
}

impl PbMessage for QuestLockInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestLockInfos {
            crate::wire::write_submessage(out, 14, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => {
                let bytes = r.read_len_delimited()?;
                self.QuestLockInfos
                    .push(<QuestLockInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestNpcMoveOverRequest {
    pub EntityId: i64,
}

impl QuestNpcMoveOverRequest {
    pub const MSG_ID: u16 = 19428;
}

impl PbMessage for QuestNpcMoveOverRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestNpcMoveOverResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl QuestNpcMoveOverResponse {
    pub const MSG_ID: u16 = 21885;
}

impl PbMessage for QuestNpcMoveOverResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReadyForGiveUpRequest {
    pub QuestId: i32,
}

impl QuestReadyForGiveUpRequest {
    pub const MSG_ID: u16 = 27030;
}

impl PbMessage for QuestReadyForGiveUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReadyForGiveUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestReadyForGiveUpResponse {
    pub const MSG_ID: u16 = 19178;
}

impl PbMessage for QuestReadyForGiveUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestRecallInstCtx {
    pub Continue: bool,
    pub RecallId: i32,
}

impl PbMessage for QuestRecallInstCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Continue {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Continue = r.read_bool()?,
            2 => self.RecallId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestRecallInstMarkCtx {
    pub RecallId: i32,
    pub MarkId: i32,
}

impl PbMessage for QuestRecallInstMarkCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RecallId = r.read_varint_i32()?,
            2 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewEntryInfo {
    pub EntryId: i32,
    pub ShowCondition: bool,
    pub HideCondition: bool,
}

impl PbMessage for QuestReviewEntryInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntryId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntryId);
        }
        if self.ShowCondition {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HideCondition {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EntryId = r.read_varint_i32()?,
            2 => self.ShowCondition = r.read_bool()?,
            3 => self.HideCondition = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewInfoNotify {
    pub EntryInfos: Vec<QuestReviewEntryInfo>,
    pub TabInfos: Vec<QuestReviewTabInfo>,
    pub NodeInfos: Vec<QuestReviewNodeInfo>,
    pub LineInfos: Vec<QuestReviewLineInfo>,
}

impl QuestReviewInfoNotify {
    pub const MSG_ID: u16 = 25208;
}

impl PbMessage for QuestReviewInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntryInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.TabInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.NodeInfos {
            crate::wire::write_submessage(out, 14, item);
        }
        for item in &self.LineInfos {
            crate::wire::write_submessage(out, 15, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.EntryInfos
                    .push(<QuestReviewEntryInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TabInfos
                    .push(<QuestReviewTabInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.NodeInfos
                    .push(<QuestReviewNodeInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.LineInfos
                    .push(<QuestReviewLineInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewInfoRequest {}

impl QuestReviewInfoRequest {
    pub const MSG_ID: u16 = 23616;
}

impl PbMessage for QuestReviewInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestReviewInfoResponse {
    pub const MSG_ID: u16 = 25121;
}

impl PbMessage for QuestReviewInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewLineInfo {
    pub LineId: i32,
    pub ShowCondition: bool,
    pub DestroyCondition: bool,
}

impl PbMessage for QuestReviewLineInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LineId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LineId);
        }
        if self.ShowCondition {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DestroyCondition {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LineId = r.read_varint_i32()?,
            2 => self.ShowCondition = r.read_bool()?,
            3 => self.DestroyCondition = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewNodeInfo {
    pub NodeId: i32,
    pub UnlockCondition: bool,
    pub InProgressCondition: bool,
    pub FinishCondition: bool,
}

impl PbMessage for QuestReviewNodeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.UnlockCondition {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InProgressCondition {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FinishCondition {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.NodeId = r.read_varint_i32()?,
            2 => self.UnlockCondition = r.read_bool()?,
            3 => self.InProgressCondition = r.read_bool()?,
            4 => self.FinishCondition = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestReviewTabInfo {
    pub TabId: i32,
    pub UnlockCondition: bool,
}

impl PbMessage for QuestReviewTabInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TabId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabId);
        }
        if self.UnlockCondition {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TabId = r.read_varint_i32()?,
            2 => self.UnlockCondition = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestRollBackEndNotify {
    pub TreeIncId: i64,
    pub QuestId: i32,
}

impl QuestRollBackEndNotify {
    pub const MSG_ID: u16 = 27925;
}

impl PbMessage for QuestRollBackEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeIncId = r.read_varint_i64()?,
            5 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestStateUpdateNotify {
    pub State: Option<QuestState>,
    pub QuestId: i32,
}

impl QuestStateUpdateNotify {
    pub const MSG_ID: u16 = 25205;
}

impl PbMessage for QuestStateUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.State = Some(<QuestState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestTreeInfoNotify {
    pub UnlockChapterNodeIds: Vec<i32>,
    pub UnlockChapterIds: Vec<i32>,
}

impl QuestTreeInfoNotify {
    pub const MSG_ID: u16 = 24869;
}

impl PbMessage for QuestTreeInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockChapterNodeIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockChapterIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockChapterNodeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockChapterIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestTreeInfoRequest {}

impl QuestTreeInfoRequest {
    pub const MSG_ID: u16 = 15519;
}

impl PbMessage for QuestTreeInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestTreeInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestTreeInfoResponse {
    pub const MSG_ID: u16 = 15945;
}

impl PbMessage for QuestTreeInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestionTask {
    pub Id: i32,
    pub Status: Option<ActivityTaskState>,
}

impl PbMessage for QuestionTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <ActivityTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuickHackSkillRequest {
    pub SkillId: i32,
    pub EntityId: i64,
}

impl QuickHackSkillRequest {
    pub const MSG_ID: u16 = 23390;
}

impl PbMessage for QuickHackSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.SkillId = r.read_varint_i32()?,
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuickHackSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl QuickHackSkillResponse {
    pub const MSG_ID: u16 = 17263;
}

impl PbMessage for QuickHackSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopAnimPush {
    pub Duration: i32,
    pub Dilation: i32,
}

impl RTimeStopAnimPush {
    pub const MSG_ID: u16 = 28729;
}

impl PbMessage for RTimeStopAnimPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.Dilation != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Dilation);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Duration = r.read_varint_i32()?,
            14 => self.Dilation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopAnimRequest {
    pub Duration: i32,
    pub Dilation: i32,
}

impl RTimeStopAnimRequest {
    pub const MSG_ID: u16 = 28674;
}

impl PbMessage for RTimeStopAnimRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.Dilation != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Dilation);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Duration = r.read_varint_i32()?,
            7 => self.Dilation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopAnimResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RTimeStopAnimResponse {
    pub const MSG_ID: u16 = 29834;
}

impl PbMessage for RTimeStopAnimResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetLegMatchResultNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub ActivityId: i32,
    pub PromoteInfo: Option<RacingBetsPromoteInfo>,
    pub BetDangoRank: i32,
    pub Reward: i32,
    pub Odds: i32,
    pub WinnerDangoId: i32,
    pub BetCount: i32,
    pub LegMatchId: i32,
    pub BetDangoId: i32,
}

impl RacingBetLegMatchResultNotify {
    pub const MSG_ID: u16 = 29552;
}

impl PbMessage for RacingBetLegMatchResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.PromoteInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.BetDangoRank != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BetDangoRank);
        }
        if self.Reward != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reward);
        }
        if self.Odds != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Odds);
        }
        if self.WinnerDangoId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WinnerDangoId);
        }
        if self.BetCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BetCount);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.BetDangoId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BetDangoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.ActivityId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PromoteInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.BetDangoRank = r.read_varint_i32()?,
            9 => self.Reward = r.read_varint_i32()?,
            11 => self.Odds = r.read_varint_i32()?,
            12 => self.WinnerDangoId = r.read_varint_i32()?,
            13 => self.BetCount = r.read_varint_i32()?,
            14 => self.LegMatchId = r.read_varint_i32()?,
            15 => self.BetDangoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetLegMatchSettleNotify {
    pub Result: Option<MatchResult>,
}

impl RacingBetLegMatchSettleNotify {
    pub const MSG_ID: u16 = 27991;
}

impl PbMessage for RacingBetLegMatchSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 1, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchActionRequest {
    pub ActivityId: i32,
    pub LegMatchId: i32,
}

impl RacingBetMatchActionRequest {
    pub const MSG_ID: u16 = 19065;
}

impl PbMessage for RacingBetMatchActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.ActivityId = r.read_varint_i32()?,
            14 => self.LegMatchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchActionResponse {
    pub LegMatchId: i32,
    pub ActivityId: i32,
    pub Rounds: Vec<RacingBetsMatchRoundInfo>,
    pub Replay: bool,
    pub ErrorCode: Option<ErrorCode>,
    pub DangoInfo: Vec<RacingBetsInstanceDangoInfo>,
}

impl RacingBetMatchActionResponse {
    pub const MSG_ID: u16 = 23348;
}

impl PbMessage for RacingBetMatchActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Rounds {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.Replay {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 15, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.LegMatchId = r.read_varint_i32()?,
            3 => self.ActivityId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rounds
                    .push(<RacingBetsMatchRoundInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.Replay = r.read_bool()?,
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsInstanceDangoInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchInfoRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
}

impl RacingBetMatchInfoRequest {
    pub const MSG_ID: u16 = 23216;
}

impl PbMessage for RacingBetMatchInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.LegMatchId = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchInfoResponse {
    pub DangoInfo: Vec<RacingBetsInstanceDangoInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetMatchInfoResponse {
    pub const MSG_ID: u16 = 23023;
}

impl PbMessage for RacingBetMatchInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 10, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsInstanceDangoInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchRequest {
    pub ActivityId: i32,
    pub LegMatchId: i32,
}

impl RacingBetMatchRequest {
    pub const MSG_ID: u16 = 27478;
}

impl PbMessage for RacingBetMatchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.ActivityId = r.read_varint_i32()?,
            11 => self.LegMatchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchResponse {
    pub ActivityId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub DangoInfo: Vec<RacingBetsInstanceDangoInfo>,
    pub Rounds: Vec<RacingBetsMatchRound>,
    pub LegMatchId: i32,
}

impl RacingBetMatchResponse {
    pub const MSG_ID: u16 = 18394;
}

impl PbMessage for RacingBetMatchResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.Rounds {
            crate::wire::write_submessage(out, 13, item);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.ActivityId = r.read_varint_i32()?,
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsInstanceDangoInfo as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Rounds
                    .push(<RacingBetsMatchRound as PbMessage>::decode(bytes)?);
            }
            14 => self.LegMatchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchRoundActionRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
    pub Order: i32,
}

impl RacingBetMatchRoundActionRequest {
    pub const MSG_ID: u16 = 16403;
}

impl PbMessage for RacingBetMatchRoundActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Order != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Order);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.LegMatchId = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            15 => self.Order = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchRoundActionResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Rounds: Option<RacingBetsMatchRoundInfo>,
    pub IsResultRound: bool,
    pub DangoInfo: Vec<RacingBetsInstanceDangoInfo>,
}

impl RacingBetMatchRoundActionResponse {
    pub const MSG_ID: u16 = 24685;
}

impl PbMessage for RacingBetMatchRoundActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Rounds {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.IsResultRound {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Rounds
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.IsResultRound = r.read_bool()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsInstanceDangoInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchRoundRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
    pub Order: i32,
}

impl RacingBetMatchRoundRequest {
    pub const MSG_ID: u16 = 17145;
}

impl PbMessage for RacingBetMatchRoundRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Order != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Order);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.LegMatchId = r.read_varint_i32()?,
            9 => self.ActivityId = r.read_varint_i32()?,
            12 => self.Order = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetMatchRoundResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub DangoInfo: Vec<RacingBetsInstanceDangoInfo>,
    pub IsResultRound: bool,
    pub Round: Option<RacingBetsMatchRound>,
}

impl RacingBetMatchRoundResponse {
    pub const MSG_ID: u16 = 19125;
}

impl PbMessage for RacingBetMatchRoundResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.IsResultRound {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Round {
            crate::wire::write_submessage(out, 9, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsInstanceDangoInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.IsResultRound = r.read_bool()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Round
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetPlayerInfo {
    pub CurCash: i32,
    pub TotalCash: i32,
    pub GearInfo: Vec<RacingBetsGearInfo>,
    pub HitNum: i32,
}

impl PbMessage for RacingBetPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCash != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCash);
        }
        if self.TotalCash != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCash);
        }
        for item in &self.GearInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.HitNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HitNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurCash = r.read_varint_i32()?,
            2 => self.TotalCash = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.GearInfo
                    .push(<RacingBetsGearInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.HitNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetRankPlayerInfo {
    pub PlayerId: i32,
    pub HeadIcon: i32,
    pub Name: Vec<u8>,
    pub Cash: i32,
    pub HitNum: i32,
    pub Rank: i32,
    pub RankNameless: bool,
}

impl PbMessage for RacingBetRankPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.HeadIcon != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadIcon);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Cash != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cash);
        }
        if self.HitNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HitNum);
        }
        if self.Rank != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rank);
        }
        if self.RankNameless {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.HeadIcon = r.read_varint_i32()?,
            3 => self.Name = r.read_len_delimited()?.to_vec(),
            4 => self.Cash = r.read_varint_i32()?,
            6 => self.HitNum = r.read_varint_i32()?,
            7 => self.Rank = r.read_varint_i32()?,
            8 => self.RankNameless = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetRankSelfData {
    pub HeadIcon: i32,
    pub Name: Vec<u8>,
    pub Cash: i32,
    pub HitNum: i32,
}

impl PbMessage for RacingBetRankSelfData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HeadIcon != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadIcon);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Cash != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cash);
        }
        if self.HitNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HitNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HeadIcon = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => self.Cash = r.read_varint_i32()?,
            4 => self.HitNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsActivityData {
    pub ActivityId: i32,
    pub ActivityStarAndEndTime: Option<RacingBetsTimeTuple>,
    pub MatchInfos: Vec<RacingBetsGroupMatchesInfo>,
    pub PlayerInfo: Option<RacingBetPlayerInfo>,
    pub TaskDatas: Vec<ConditionTaskData>,
    pub RankListUpdateTime: Vec<i64>,
    pub CloseSettleMenuLegMatchList: Vec<i32>,
}

impl PbMessage for RacingBetsActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.ActivityStarAndEndTime {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.MatchInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.PlayerInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.TaskDatas {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RankListUpdateTime {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.CloseSettleMenuLegMatchList {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityStarAndEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MatchInfos
                    .push(<RacingBetsGroupMatchesInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDatas
                    .push(<ConditionTaskData as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RankListUpdateTime,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CloseSettleMenuLegMatchList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsBulletScreenRequest {
    pub BulletScreenId: i32,
    pub LegMatchId: i32,
    pub ActionIndex: i32,
    pub ActivityId: i32,
}

impl RacingBetsBulletScreenRequest {
    pub const MSG_ID: u16 = 21780;
}

impl PbMessage for RacingBetsBulletScreenRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletScreenId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletScreenId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActionIndex != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionIndex);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BulletScreenId = r.read_varint_i32()?,
            2 => self.LegMatchId = r.read_varint_i32()?,
            7 => self.ActionIndex = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsBulletScreenResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsBulletScreenResponse {
    pub const MSG_ID: u16 = 19809;
}

impl PbMessage for RacingBetsBulletScreenResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsChangeHighData {
    pub DangoId: i32,
    pub High: i32,
}

impl PbMessage for RacingBetsChangeHighData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.High != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.High);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => self.High = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsCloseSettleMenuRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
}

impl RacingBetsCloseSettleMenuRequest {
    pub const MSG_ID: u16 = 15685;
}

impl PbMessage for RacingBetsCloseSettleMenuRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.LegMatchId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsCloseSettleMenuResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsCloseSettleMenuResponse {
    pub const MSG_ID: u16 = 23836;
}

impl PbMessage for RacingBetsCloseSettleMenuResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionBlackHoleTransmit {
    pub Data: Vec<RacingBetsTransmitData>,
}

impl PbMessage for RacingBetsDangoActionBlackHoleTransmit {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
            crate::wire::write_submessage(out, 1, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<RacingBetsTransmitData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionChangeHigh {
    pub Data: Vec<RacingBetsChangeHighData>,
}

impl PbMessage for RacingBetsDangoActionChangeHigh {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
            crate::wire::write_submessage(out, 1, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<RacingBetsChangeHighData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionDice {
    pub DiceNums: Vec<i32>,
    pub TotalDice: i32,
}

impl PbMessage for RacingBetsDangoActionDice {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DiceNums {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TotalDice != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDice);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DiceNums, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.TotalDice = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionMove {
    pub DangoId: i32,
    pub StartPoint: i32,
    pub EndPoint: i32,
    pub CarryDango: Vec<i32>,
    pub Direction: Option<RacingBetsDangoMoveDirection>,
}

impl PbMessage for RacingBetsDangoActionMove {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.StartPoint != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartPoint);
        }
        if self.EndPoint != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndPoint);
        }
        for item in &self.CarryDango {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Direction {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => self.StartPoint = r.read_varint_i32()?,
            3 => self.EndPoint = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CarryDango, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => {
                self.Direction = Some(
                    <RacingBetsDangoMoveDirection as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionSkill {
    pub DangoId: i32,
    pub SkillId: i32,
    pub IsSuccess: bool,
}

impl PbMessage for RacingBetsDangoActionSkill {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.IsSuccess {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => self.SkillId = r.read_varint_i32()?,
            3 => self.IsSuccess = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoActionTransmit {
    pub Data: Vec<RacingBetsTransmitData>,
    pub TriggerDangoId: i32,
}

impl PbMessage for RacingBetsDangoActionTransmit {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.TriggerDangoId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TriggerDangoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<RacingBetsTransmitData as PbMessage>::decode(bytes)?);
            }
            4 => self.TriggerDangoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoCameraMove {
    pub DangoId: i32,
}

impl PbMessage for RacingBetsDangoCameraMove {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoInfo {
    pub Id: i32,
    pub CurPoint: i32,
    pub Odds: i32,
}

impl PbMessage for RacingBetsDangoInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CurPoint != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPoint);
        }
        if self.Odds != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Odds);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => self.CurPoint = r.read_varint_i32()?,
            3 => self.Odds = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDangoRoundStart {
    pub DangoId: i32,
}

impl PbMessage for RacingBetsDangoRoundStart {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDestination {
    pub DangoId: i32,
}

impl PbMessage for RacingBetsDestination {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDiceInfo {
    pub DiceInfo: Vec<DangoIdToDiceNum>,
}

impl PbMessage for RacingBetsDiceInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DiceInfo {
            crate::wire::write_submessage(out, 1, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.DiceInfo
                    .push(<DangoIdToDiceNum as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsDungeonStart {
    pub LegMatchId: i32,
}

impl PbMessage for RacingBetsDungeonStart {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LegMatchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGearInfo {
    pub LegMatchesId: i32,
    pub DangoId: i32,
    pub BettingGearId: i32,
    pub BettingGearCash: i32,
    pub Odds: i32,
    pub OddsVersion: Vec<u8>,
    pub LeaveCancelNum: i32,
    pub OddsReward: i32,
}

impl PbMessage for RacingBetsGearInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchesId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchesId);
        }
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.BettingGearId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BettingGearId);
        }
        if self.BettingGearCash != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BettingGearCash);
        }
        if self.Odds != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Odds);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if self.LeaveCancelNum != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeaveCancelNum);
        }
        if self.OddsReward != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OddsReward);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LegMatchesId = r.read_varint_i32()?,
            2 => self.DangoId = r.read_varint_i32()?,
            3 => self.BettingGearId = r.read_varint_i32()?,
            4 => self.BettingGearCash = r.read_varint_i32()?,
            5 => self.Odds = r.read_varint_i32()?,
            6 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            7 => self.LeaveCancelNum = r.read_varint_i32()?,
            8 => self.OddsReward = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGearRefundRequest {
    pub DangoId: i32,
    pub ActivityId: i32,
    pub LegMatchId: i32,
}

impl RacingBetsGearRefundRequest {
    pub const MSG_ID: u16 = 22909;
}

impl PbMessage for RacingBetsGearRefundRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.DangoId = r.read_varint_i32()?,
            7 => self.ActivityId = r.read_varint_i32()?,
            14 => self.LegMatchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGearRefundResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerInfo: Option<RacingBetPlayerInfo>,
}

impl RacingBetsGearRefundResponse {
    pub const MSG_ID: u16 = 17259;
}

impl PbMessage for RacingBetsGearRefundResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PlayerInfo {
            crate::wire::write_submessage(out, 10, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGearRequest {
    pub DangoId: i32,
    pub OddsVersion: Vec<u8>,
    pub ActivityId: i32,
    pub Cash: i32,
    pub LegMatchId: i32,
    pub GearId: i32,
}

impl RacingBetsGearRequest {
    pub const MSG_ID: u16 = 16270;
}

impl PbMessage for RacingBetsGearRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Cash != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cash);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.GearId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GearId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.DangoId = r.read_varint_i32()?,
            9 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            12 => self.ActivityId = r.read_varint_i32()?,
            13 => self.Cash = r.read_varint_i32()?,
            14 => self.LegMatchId = r.read_varint_i32()?,
            15 => self.GearId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGearResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerInfo: Option<RacingBetPlayerInfo>,
}

impl RacingBetsGearResponse {
    pub const MSG_ID: u16 = 29611;
}

impl PbMessage for RacingBetsGearResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PlayerInfo {
            crate::wire::write_submessage(out, 12, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsGroupMatchesInfo {
    pub GroupMatchId: i32,
    pub GroupMatchTime: Option<RacingBetsTimeTuple>,
    pub LegMatchesInfo: Vec<RacingBetsLegMatchesInfo>,
    pub PromoteDangos: Vec<i32>,
    pub Dangos: Vec<i32>,
}

impl PbMessage for RacingBetsGroupMatchesInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupMatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupMatchId);
        }
        if let Some(v) = &self.GroupMatchTime {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.LegMatchesInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.PromoteDangos {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Dangos {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GroupMatchId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GroupMatchTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LegMatchesInfo
                    .push(<RacingBetsLegMatchesInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PromoteDangos, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Dangos, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsInstanceDangoAction {
    pub ActionType: Option<RacingBetsDangoActionType>,
    pub Move: Option<RacingBetsDangoActionMove>,
    pub Dice: Option<RacingBetsDangoActionDice>,
    pub Skill: Option<RacingBetsDangoActionSkill>,
    pub ChangeHigh: Option<RacingBetsDangoActionChangeHigh>,
    pub Index: i32,
    pub BulletScreenTimes: Vec<BulletScreenTimes>,
    pub Transmit: Option<RacingBetsDangoActionTransmit>,
    pub OrganEffect: Option<RacingBetsOrganEffect>,
    pub BlackHoleTransmit: Option<RacingBetsDangoActionBlackHoleTransmit>,
}

impl PbMessage for RacingBetsInstanceDangoAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActionType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Move {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Dice {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Skill {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ChangeHigh {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        for item in &self.BulletScreenTimes {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.Transmit {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.OrganEffect {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.BlackHoleTransmit {
            crate::wire::write_submessage(out, 10, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ActionType = Some(
                    <RacingBetsDangoActionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Move
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Dice
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Skill
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ChangeHigh
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.Index = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.BulletScreenTimes
                    .push(<BulletScreenTimes as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Transmit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.OrganEffect
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.BlackHoleTransmit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsInstanceDangoInfo {
    pub DangoId: i32,
    pub CurPoint: i32,
    pub DiceId: i32,
    pub High: i32,
    pub EntityId: i64,
    pub BpId: i32,
}

impl PbMessage for RacingBetsInstanceDangoInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.CurPoint != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPoint);
        }
        if self.DiceId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiceId);
        }
        if self.High != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.High);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.BpId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BpId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => self.CurPoint = r.read_varint_i32()?,
            3 => self.DiceId = r.read_varint_i32()?,
            4 => self.High = r.read_varint_i32()?,
            5 => self.EntityId = r.read_varint_i64()?,
            6 => self.BpId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsInstanceDangoRound {
    pub DangoId: i32,
    pub ActionList: Vec<RacingBetsInstanceDangoAction>,
}

impl PbMessage for RacingBetsInstanceDangoRound {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        for item in &self.ActionList {
            crate::wire::write_submessage(out, 2, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActionList
                    .push(<RacingBetsInstanceDangoAction as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsInstanceRoundAction {
    pub ActionIndex: i32,
    pub ActionType: Option<RacingBetsRoundActionType>,
    pub DungeonStart: Option<RacingBetsDungeonStart>,
    pub RoundStart: Option<RacingBetsRoundStart>,
    pub DangoStart: Option<RacingBetsDangoRoundStart>,
    pub DiceInfo: Option<RacingBetsDiceInfo>,
    pub Move: Option<RacingBetsDangoActionMove>,
    pub Skill: Option<RacingBetsDangoActionSkill>,
    pub ChangeHigh: Option<RacingBetsDangoActionChangeHigh>,
    pub CameraMove: Option<RacingBetsDangoCameraMove>,
    pub Destination: Option<RacingBetsDestination>,
    pub UpdateRank: Option<RacingBetsUpdateRank>,
    pub BulletScreenTimes: Vec<BulletScreenTimes>,
    pub Transmit: Option<RacingBetsDangoActionTransmit>,
    pub OrganEffect: Option<RacingBetsOrganEffect>,
    pub BlackHoleTransmit: Option<RacingBetsDangoActionBlackHoleTransmit>,
}

impl PbMessage for RacingBetsInstanceRoundAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActionIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionIndex);
        }
        if let Some(v) = &self.ActionType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.DungeonStart {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.RoundStart {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.DangoStart {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.DiceInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Move {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.Skill {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.ChangeHigh {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.CameraMove {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.Destination {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.UpdateRank {
            crate::wire::write_submessage(out, 12, v);
        }
        for item in &self.BulletScreenTimes {
            crate::wire::write_submessage(out, 13, item);
        }
        if let Some(v) = &self.Transmit {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.OrganEffect {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.BlackHoleTransmit {
            crate::wire::write_submessage(out, 16, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ActionIndex = r.read_varint_i32()?,
            2 => {
                self.ActionType = Some(
                    <RacingBetsRoundActionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DungeonStart
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoundStart
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DangoStart
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.DiceInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Move
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Skill
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ChangeHigh
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.CameraMove
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Destination
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateRank
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.BulletScreenTimes
                    .push(<BulletScreenTimes as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Transmit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.OrganEffect
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                self.BlackHoleTransmit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsLegMatchInfoRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
}

impl RacingBetsLegMatchInfoRequest {
    pub const MSG_ID: u16 = 26734;
}

impl PbMessage for RacingBetsLegMatchInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.LegMatchId = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsLegMatchInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LegInfo: Option<RacingBetsLegMatchesInfo>,
}

impl RacingBetsLegMatchInfoResponse {
    pub const MSG_ID: u16 = 29075;
}

impl PbMessage for RacingBetsLegMatchInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.LegInfo {
            crate::wire::write_submessage(out, 12, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.LegInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsLegMatchesInfo {
    pub Id: i32,
    pub DangoInfo: Vec<RacingBetsDangoInfo>,
    pub MatchStartEndTime: Option<RacingBetsTimeTuple>,
    pub GearStartEndTime: Option<RacingBetsTimeTuple>,
    pub DangoRank: Vec<i32>,
    pub NextOddsTime: i64,
    pub OddsVersion: Vec<u8>,
    pub LegMatchWholeTime: Option<RacingBetsTimeTuple>,
    pub OrganInfo: Vec<RacingBetsOrganInfo>,
}

impl PbMessage for RacingBetsLegMatchesInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.MatchStartEndTime {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.GearStartEndTime {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.DangoRank {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.NextOddsTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextOddsTime);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if let Some(v) = &self.LegMatchWholeTime {
            crate::wire::write_submessage(out, 8, v);
        }
        for item in &self.OrganInfo {
            crate::wire::write_submessage(out, 9, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsDangoInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MatchStartEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GearStartEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoRank, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => self.NextOddsTime = r.read_varint_i64()?,
            7 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            8 => {
                let bytes = r.read_len_delimited()?;
                self.LegMatchWholeTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.OrganInfo
                    .push(<RacingBetsOrganInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsMatchInfoRequest {
    pub ActivityId: i32,
}

impl RacingBetsMatchInfoRequest {
    pub const MSG_ID: u16 = 27743;
}

impl PbMessage for RacingBetsMatchInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsMatchInfoResponse {
    pub MatchInfos: Vec<RacingBetsGroupMatchesInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub ActivityStarAndEndTime: Option<RacingBetsTimeTuple>,
    pub ActivityId: i32,
}

impl RacingBetsMatchInfoResponse {
    pub const MSG_ID: u16 = 27440;
}

impl PbMessage for RacingBetsMatchInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MatchInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ActivityStarAndEndTime {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MatchInfos
                    .push(<RacingBetsGroupMatchesInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityStarAndEndTime
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsMatchRound {
    pub RoundRank: i32,
    pub DangoSequence: Vec<i32>,
    pub DangoRound: Vec<RacingBetsInstanceDangoRound>,
    pub IsFinalRound: bool,
    pub IdToNum: Vec<DangoIdToDiceNum>,
    pub Result: Option<MatchResult>,
}

impl PbMessage for RacingBetsMatchRound {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundRank != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundRank);
        }
        for item in &self.DangoSequence {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DangoRound {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsFinalRound {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.IdToNum {
            crate::wire::write_submessage(out, 5, item);
        }
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 6, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoundRank = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoSequence, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DangoRound
                    .push(<RacingBetsInstanceDangoRound as PbMessage>::decode(bytes)?);
            }
            4 => self.IsFinalRound = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.IdToNum
                    .push(<DangoIdToDiceNum as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsMatchRoundInfo {
    pub RoundRank: i32,
    pub RoundAction: Vec<RacingBetsInstanceRoundAction>,
    pub IsFinalRound: bool,
    pub Result: Option<MatchResult>,
}

impl PbMessage for RacingBetsMatchRoundInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundRank != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundRank);
        }
        for item in &self.RoundAction {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.IsFinalRound {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 4, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoundRank = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoundAction
                    .push(<RacingBetsInstanceRoundAction as PbMessage>::decode(bytes)?);
            }
            3 => self.IsFinalRound = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsOrganEffect {
    pub OrganId: i32,
    pub SkillId: i32,
    pub DangoId: i32,
    pub OrganPoint: i32,
}

impl PbMessage for RacingBetsOrganEffect {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OrganId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OrganId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.OrganPoint != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OrganPoint);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.OrganId = r.read_varint_i32()?,
            2 => self.SkillId = r.read_varint_i32()?,
            3 => self.DangoId = r.read_varint_i32()?,
            4 => self.OrganPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsPlayerInfoRequest {
    pub ActivityId: i32,
}

impl RacingBetsPlayerInfoRequest {
    pub const MSG_ID: u16 = 22312;
}

impl PbMessage for RacingBetsPlayerInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsPlayerInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerInfo: Option<RacingBetPlayerInfo>,
}

impl RacingBetsPlayerInfoResponse {
    pub const MSG_ID: u16 = 21541;
}

impl PbMessage for RacingBetsPlayerInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PlayerInfo {
            crate::wire::write_submessage(out, 11, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsPlayerInfoUpdateNotify {
    pub PlayerInfo: Option<RacingBetPlayerInfo>,
}

impl RacingBetsPlayerInfoUpdateNotify {
    pub const MSG_ID: u16 = 24358;
}

impl PbMessage for RacingBetsPlayerInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlayerInfo {
            crate::wire::write_submessage(out, 6, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsPromoteInfo {
    pub GroupMatchId: i32,
    pub DangoId: Vec<i32>,
}

impl PbMessage for RacingBetsPromoteInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupMatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupMatchId);
        }
        for item in &self.DangoId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GroupMatchId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRankRequest {
    pub ActivityId: i32,
}

impl RacingBetsRankRequest {
    pub const MSG_ID: u16 = 28072;
}

impl PbMessage for RacingBetsRankRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRankResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Status: Option<RacingBetsRankStatus>,
    pub PercentScore: i32,
    pub PlayerInfo: Vec<RacingBetRankPlayerInfo>,
    pub SelfData: Option<RacingBetRankSelfData>,
}

impl RacingBetsRankResponse {
    pub const MSG_ID: u16 = 21901;
}

impl PbMessage for RacingBetsRankResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PercentScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PercentScore);
        }
        for item in &self.PlayerInfo {
            crate::wire::write_submessage(out, 12, item);
        }
        if let Some(v) = &self.SelfData {
            crate::wire::write_submessage(out, 14, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                self.Status = Some(
                    <RacingBetsRankStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => self.PercentScore = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfo
                    .push(<RacingBetRankPlayerInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.SelfData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRepeatedTaskRewardRequest {
    pub TaskId: Vec<i32>,
}

impl RacingBetsRepeatedTaskRewardRequest {
    pub const MSG_ID: u16 = 25268;
}

impl PbMessage for RacingBetsRepeatedTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskId {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRepeatedTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsRepeatedTaskRewardResponse {
    pub const MSG_ID: u16 = 22994;
}

impl PbMessage for RacingBetsRepeatedTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsRoundStart {
    pub LegMatchId: i32,
    pub RoundRank: i32,
}

impl PbMessage for RacingBetsRoundStart {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.RoundRank != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundRank);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LegMatchId = r.read_varint_i32()?,
            2 => self.RoundRank = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsSettleNotify {
    pub ActivityId: i32,
    pub LegMatchId: i32,
    pub SelfOddsDangoId: i32,
    pub RewardFunds: i32,
    pub DangoRank: Vec<i32>,
}

impl RacingBetsSettleNotify {
    pub const MSG_ID: u16 = 28131;
}

impl PbMessage for RacingBetsSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.SelfOddsDangoId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelfOddsDangoId);
        }
        if self.RewardFunds != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardFunds);
        }
        for item in &self.DangoRank {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ActivityId = r.read_varint_i32()?,
            9 => self.LegMatchId = r.read_varint_i32()?,
            10 => self.SelfOddsDangoId = r.read_varint_i32()?,
            13 => self.RewardFunds = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoRank, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTaskNotify {
    pub TaskDatas: Vec<ConditionTaskData>,
}

impl RacingBetsTaskNotify {
    pub const MSG_ID: u16 = 15994;
}

impl PbMessage for RacingBetsTaskNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskDatas {
            crate::wire::write_submessage(out, 14, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDatas
                    .push(<ConditionTaskData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTaskRequest {
    pub ActivityId: i32,
}

impl RacingBetsTaskRequest {
    pub const MSG_ID: u16 = 26366;
}

impl PbMessage for RacingBetsTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsTaskResponse {
    pub const MSG_ID: u16 = 28070;
}

impl PbMessage for RacingBetsTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTaskRewardRequest {
    pub TaskId: i32,
}

impl RacingBetsTaskRewardRequest {
    pub const MSG_ID: u16 = 17938;
}

impl PbMessage for RacingBetsTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsTaskRewardResponse {
    pub const MSG_ID: u16 = 20940;
}

impl PbMessage for RacingBetsTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsTransmitData {
    pub DangoId: i32,
    pub Point: i32,
    pub High: i32,
}

impl PbMessage for RacingBetsTransmitData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Point);
        }
        if self.High != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.High);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangoId = r.read_varint_i32()?,
            2 => self.Point = r.read_varint_i32()?,
            3 => self.High = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsUpdateOddsRequest {
    pub LegMatchId: i32,
    pub ActivityId: i32,
}

impl RacingBetsUpdateOddsRequest {
    pub const MSG_ID: u16 = 17175;
}

impl PbMessage for RacingBetsUpdateOddsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.LegMatchId = r.read_varint_i32()?,
            7 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsUpdateOddsResponse {
    pub DangoInfo: Vec<RacingBetsDangoInfo>,
    pub OddsVersion: Vec<u8>,
    pub ActivityId: i32,
    pub LegMatchId: i32,
    pub NextOddsTime: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl RacingBetsUpdateOddsResponse {
    pub const MSG_ID: u16 = 19078;
}

impl PbMessage for RacingBetsUpdateOddsResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DangoInfo {
            crate::wire::write_submessage(out, 4, item);
        }
        if !self.OddsVersion.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OddsVersion);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        if self.NextOddsTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextOddsTime);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.DangoInfo
                    .push(<RacingBetsDangoInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.OddsVersion = r.read_len_delimited()?.to_vec(),
            8 => self.ActivityId = r.read_varint_i32()?,
            10 => self.LegMatchId = r.read_varint_i32()?,
            11 => self.NextOddsTime = r.read_varint_i64()?,
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RacingBetsUpdateRank {
    pub DangoId: Vec<i32>,
}

impl PbMessage for RacingBetsUpdateRank {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DangoId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RandomSummonRequestInfo {
    pub SkillId: i32,
    pub SummonIndex: i32,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub IsVisible: bool,
}

impl PbMessage for RandomSummonRequestInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.SummonIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonIndex);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SkillId = r.read_varint_i32()?,
            2 => self.SummonIndex = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.IsVisible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockStateChangeNotify {
    pub EntityId: i64,
    pub IncId: i32,
    pub State: Option<RbBlockPbState>,
}

impl RbBlockStateChangeNotify {
    pub const MSG_ID: u16 = 27504;
}

impl PbMessage for RbBlockStateChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_submessage(out, 11, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.EntityId = r.read_varint_i64()?,
            4 => self.IncId = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.State
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbInput {
    pub Roll: Option<RbRollInput>,
}

impl PbMessage for RbInput {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Roll {
            crate::wire::write_submessage(out, 1, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Roll
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbRollInput {
    pub Direction: Option<RbGridDirection>,
}

impl PbMessage for RbRollInput {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Direction {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Direction = Some(
                    <RbGridDirection as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReadDisplayInfoRequest {
    pub DisplayId: i32,
}

impl ReadDisplayInfoRequest {
    pub const MSG_ID: u16 = 22933;
}

impl PbMessage for ReadDisplayInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DisplayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DisplayId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DisplayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReadDisplayInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ReadDisplayInfoResponse {
    pub const MSG_ID: u16 = 20372;
}

impl PbMessage for ReadDisplayInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenFullRewardRequest {}

impl RealmBetweenFullRewardRequest {
    pub const MSG_ID: u16 = 15385;
}

impl PbMessage for RealmBetweenFullRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenFullRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RealmBetweenFullRewardResponse {
    pub const MSG_ID: u16 = 18737;
}

impl PbMessage for RealmBetweenFullRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenInfoUpdateNotify {
    pub UnLockMonsterId: i32,
    pub MotorcycleLevel: Option<RealmBetweenMotorcycleInfo>,
    pub UnlockTask: i32,
    pub UnLockArea: i32,
    pub ActivityTask: Option<ConditionTask>,
}

impl RealmBetweenInfoUpdateNotify {
    pub const MSG_ID: u16 = 25055;
}

impl PbMessage for RealmBetweenInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnLockMonsterId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockMonsterId);
        }
        if let Some(v) = &self.MotorcycleLevel {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.UnlockTask != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockTask);
        }
        if self.UnLockArea != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockArea);
        }
        if let Some(v) = &self.ActivityTask {
            crate::wire::write_submessage(out, 15, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.UnLockMonsterId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.MotorcycleLevel
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.UnlockTask = r.read_varint_i32()?,
            14 => self.UnLockArea = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenLevelUpRequest {}

impl RealmBetweenLevelUpRequest {
    pub const MSG_ID: u16 = 27479;
}

impl PbMessage for RealmBetweenLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RealmBetweenLevelUpResponse {
    pub const MSG_ID: u16 = 27232;
}

impl PbMessage for RealmBetweenLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenMotorRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl RealmBetweenMotorRewardRequest {
    pub const MSG_ID: u16 = 27266;
}

impl PbMessage for RealmBetweenMotorRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenMotorRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskIds: Vec<i32>,
}

impl RealmBetweenMotorRewardResponse {
    pub const MSG_ID: u16 = 20454;
}

impl PbMessage for RealmBetweenMotorRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenTaskRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl RealmBetweenTaskRewardRequest {
    pub const MSG_ID: u16 = 19830;
}

impl PbMessage for RealmBetweenTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RealmBetweenTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskIds: Vec<i32>,
}

impl RealmBetweenTaskRewardResponse {
    pub const MSG_ID: u16 = 26868;
}

impl PbMessage for RealmBetweenTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallExploreSkillRouletteNotify {
    pub ExtraItemId: i32,
}

impl RecallExploreSkillRouletteNotify {
    pub const MSG_ID: u16 = 27174;
}

impl PbMessage for RecallExploreSkillRouletteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExtraItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraItemId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.ExtraItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallExploreSkillRouletteSetRequest {
    pub ExtraItemId: i32,
}

impl RecallExploreSkillRouletteSetRequest {
    pub const MSG_ID: u16 = 29727;
}

impl PbMessage for RecallExploreSkillRouletteSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExtraItemId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraItemId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.ExtraItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallExploreSkillRouletteSetResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallExploreSkillRouletteSetResponse {
    pub const MSG_ID: u16 = 25464;
}

impl PbMessage for RecallExploreSkillRouletteSetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallInfo {
    pub RecallId: i32,
    pub State: Option<RecallState>,
    pub Status: i32,
}

impl PbMessage for RecallInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RecallId = r.read_varint_i32()?,
            2 => {
                self.State = Some(<RecallState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallInfoNotify {
    pub RecallIds: Vec<i32>,
    pub RecallInfos: Vec<RecallInfo>,
}

impl RecallInfoNotify {
    pub const MSG_ID: u16 = 25321;
}

impl PbMessage for RecallInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RecallInfos {
            crate::wire::write_submessage(out, 11, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RecallIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.RecallInfos
                    .push(<RecallInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallInstSaveDataNotify {
    pub Ids: Vec<i64>,
    pub RecallId: i32,
}

impl RecallInstSaveDataNotify {
    pub const MSG_ID: u16 = 22471;
}

impl PbMessage for RecallInstSaveDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            14 => self.RecallId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemBagDeltaNotify {
    pub RecallId: i32,
    pub Reason: i32,
    pub Items: Vec<RecallItemBagItem>,
}

impl RecallItemBagDeltaNotify {
    pub const MSG_ID: u16 = 27494;
}

impl PbMessage for RecallItemBagDeltaNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 10, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.RecallId = r.read_varint_i32()?,
            6 => self.Reason = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<RecallItemBagItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemBagItem {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for RecallItemBagItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemBagObtainNotify {
    pub Items: Vec<RecallItemBagItem>,
    pub RecallId: i32,
    pub Reason: i32,
}

impl RecallItemBagObtainNotify {
    pub const MSG_ID: u16 = 15236;
}

impl PbMessage for RecallItemBagObtainNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<RecallItemBagItem as PbMessage>::decode(bytes)?);
            }
            9 => self.RecallId = r.read_varint_i32()?,
            15 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemBagRemoveNotify {
    pub RecallId: i32,
    pub ItemIds: Vec<i32>,
}

impl RecallItemBagRemoveNotify {
    pub const MSG_ID: u16 = 16591;
}

impl PbMessage for RecallItemBagRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        for item in &self.ItemIds {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.RecallId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemRewardNotify {
    pub RecallId: i32,
    pub Reason: i32,
    pub DropId: i32,
    pub RewardItems: Vec<RewardItemInfo>,
}

impl RecallItemRewardNotify {
    pub const MSG_ID: u16 = 27085;
}

impl PbMessage for RecallItemRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.DropId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropId);
        }
        for item in &self.RewardItems {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.RecallId = r.read_varint_i32()?,
            5 => self.Reason = r.read_varint_i32()?,
            7 => self.DropId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItems
                    .push(<RewardItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemUseRequest {
    pub ItemId: i32,
    pub Count: i32,
}

impl RecallItemUseRequest {
    pub const MSG_ID: u16 = 18849;
}

impl PbMessage for RecallItemUseRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.ItemId = r.read_varint_i32()?,
            15 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallItemUseResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RecallItemUseResponse {
    pub const MSG_ID: u16 = 19067;
}

impl PbMessage for RecallItemUseResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallMapMarkInfoNotify {
    pub MapMarkShowInfos: Vec<MapMarkShowInfo>,
}

impl RecallMapMarkInfoNotify {
    pub const MSG_ID: u16 = 22163;
}

impl PbMessage for RecallMapMarkInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MapMarkShowInfos {
            crate::wire::write_submessage(out, 9, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                let bytes = r.read_len_delimited()?;
                self.MapMarkShowInfos
                    .push(<MapMarkShowInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallMapMarkUpdateNotify {
    pub MarkInfo: Option<MapMarkShowInfo>,
}

impl RecallMapMarkUpdateNotify {
    pub const MSG_ID: u16 = 15791;
}

impl PbMessage for RecallMapMarkUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MarkInfo {
            crate::wire::write_submessage(out, 4, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MarkInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestActionRequest {
    pub ActTime: Option<ActionTime>,
    pub NodeId: i32,
    pub RecallQuestId: i32,
    pub ActionId: i32,
}

impl RecallQuestActionRequest {
    pub const MSG_ID: u16 = 28770;
}

impl PbMessage for RecallQuestActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActTime {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ActTime = Some(<ActionTime as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.NodeId = r.read_varint_i32()?,
            8 => self.RecallQuestId = r.read_varint_i32()?,
            13 => self.ActionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestActionResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallQuestActionResponse {
    pub const MSG_ID: u16 = 22619;
}

impl PbMessage for RecallQuestActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestChangeTraceNotify {
    pub RecallQuestIds: Vec<i32>,
}

impl RecallQuestChangeTraceNotify {
    pub const MSG_ID: u16 = 28015;
}

impl PbMessage for RecallQuestChangeTraceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallQuestIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RecallQuestIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishActionEndRequest {
    pub RecallQuestId: Vec<i32>,
}

impl RecallQuestFinishActionEndRequest {
    pub const MSG_ID: u16 = 21179;
}

impl PbMessage for RecallQuestFinishActionEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallQuestId {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RecallQuestId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishActionEndResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallQuestFinishActionEndResponse {
    pub const MSG_ID: u16 = 18345;
}

impl PbMessage for RecallQuestFinishActionEndResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishActionRequest {
    pub RecallQuestId: i32,
    pub ActionId: i32,
    pub NodeId: i32,
    pub ActTime: Option<ActionTime>,
}

impl RecallQuestFinishActionRequest {
    pub const MSG_ID: u16 = 28969;
}

impl PbMessage for RecallQuestFinishActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if let Some(v) = &self.ActTime {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.RecallQuestId = r.read_varint_i32()?,
            3 => self.ActionId = r.read_varint_i32()?,
            7 => self.NodeId = r.read_varint_i32()?,
            9 => {
                self.ActTime = Some(<ActionTime as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishActionResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallQuestFinishActionResponse {
    pub const MSG_ID: u16 = 15780;
}

impl PbMessage for RecallQuestFinishActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestFinishListNotify {
    pub RecallQuestIds: Vec<i32>,
}

impl RecallQuestFinishListNotify {
    pub const MSG_ID: u16 = 26450;
}

impl PbMessage for RecallQuestFinishListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallQuestIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RecallQuestIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestGiveUpNotify {
    pub FlowIncIds: Vec<i64>,
    pub TreeIncId: i64,
    pub RecallQuestId: i32,
}

impl RecallQuestGiveUpNotify {
    pub const MSG_ID: u16 = 22535;
}

impl PbMessage for RecallQuestGiveUpNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlowIncIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FlowIncIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            12 => self.TreeIncId = r.read_varint_i64()?,
            14 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestInfo {
    pub RecallQuestId: i32,
    pub Status: Option<RecallQuestStatus>,
}

impl PbMessage for RecallQuestInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RecallQuestId = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <RecallQuestStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestListNotify {
    pub RecallQuestInfos: Vec<RecallQuestInfo>,
}

impl RecallQuestListNotify {
    pub const MSG_ID: u16 = 19235;
}

impl PbMessage for RecallQuestListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallQuestInfos {
            crate::wire::write_submessage(out, 15, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RecallQuestInfos
                    .push(<RecallQuestInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestNpcMoveOverRequest {
    pub EntityId: i64,
}

impl RecallQuestNpcMoveOverRequest {
    pub const MSG_ID: u16 = 26604;
}

impl PbMessage for RecallQuestNpcMoveOverRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestNpcMoveOverResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallQuestNpcMoveOverResponse {
    pub const MSG_ID: u16 = 19333;
}

impl PbMessage for RecallQuestNpcMoveOverResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestReadyForGiveUpRequest {
    pub RecallQuestId: i32,
}

impl RecallQuestReadyForGiveUpRequest {
    pub const MSG_ID: u16 = 17217;
}

impl PbMessage for RecallQuestReadyForGiveUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestReadyForGiveUpResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallQuestReadyForGiveUpResponse {
    pub const MSG_ID: u16 = 21194;
}

impl PbMessage for RecallQuestReadyForGiveUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestRollBackEndNotify {
    pub RecallQuestId: i32,
    pub TreeIncId: i64,
}

impl RecallQuestRollBackEndNotify {
    pub const MSG_ID: u16 = 27936;
}

impl PbMessage for RecallQuestRollBackEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.RecallQuestId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallQuestStatusUpdateNotify {
    pub Status: Option<RecallQuestStatus>,
    pub RecallQuestId: i32,
}

impl RecallQuestStatusUpdateNotify {
    pub const MSG_ID: u16 = 16968;
}

impl PbMessage for RecallQuestStatusUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                self.Status = Some(
                    <RecallQuestStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            13 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallTeleportToMarkIdRequest {
    pub MarkId: i32,
}

impl RecallTeleportToMarkIdRequest {
    pub const MSG_ID: u16 = 23922;
}

impl PbMessage for RecallTeleportToMarkIdRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecallTeleportToMarkIdResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RecallTeleportToMarkIdResponse {
    pub const MSG_ID: u16 = 19697;
}

impl PbMessage for RecallTeleportToMarkIdResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveRechallengeNotify {
    pub PlayerId: i32,
    pub Result: Option<ERechallengeResultResult>,
}

impl ReceiveRechallengeNotify {
    pub const MSG_ID: u16 = 29770;
}

impl PbMessage for ReceiveRechallengeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Result {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.PlayerId = r.read_varint_i32()?,
            13 => {
                self.Result = Some(
                    <ERechallengeResultResult as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveRechallengePlayerIdsNotify {
    pub PlayerIds: Vec<i32>,
}

impl ReceiveRechallengePlayerIdsNotify {
    pub const MSG_ID: u16 = 29063;
}

impl PbMessage for ReceiveRechallengePlayerIdsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PlayerIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveRechallengeRequest {
    pub Result: Option<ERechallengeResultResult>,
}

impl ReceiveRechallengeRequest {
    pub const MSG_ID: u16 = 25942;
}

impl PbMessage for ReceiveRechallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                self.Result = Some(
                    <ERechallengeResultResult as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveRechallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ReceiveRechallengeResponse {
    pub const MSG_ID: u16 = 21241;
}

impl PbMessage for ReceiveRechallengeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceivedSilentFirstAwardRequest {
    pub Id: i32,
}

impl ReceivedSilentFirstAwardRequest {
    pub const MSG_ID: u16 = 17342;
}

impl PbMessage for ReceivedSilentFirstAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceivedSilentFirstAwardResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl ReceivedSilentFirstAwardResponse {
    pub const MSG_ID: u16 = 28437;
}

impl PbMessage for ReceivedSilentFirstAwardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 4, &self.ItemMap);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 14, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecentlyTeamInfo {
    pub Info: Option<PlayerDetails>,
    pub TeamTime: i64,
}

impl PbMessage for RecentlyTeamInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TeamTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TeamTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.TeamTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecordRoleEnergy {
    pub LevelId: i32,
    pub RoleEnergyDict: BTreeMap<i32, i32>,
}

impl PbMessage for RecordRoleEnergy {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if !self.RoleEnergyDict.is_empty() {
            crate::wire::write_map(out, 2, &self.RoleEnergyDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LevelId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.RoleEnergyDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecoverAttr {
    pub AttrId: i32,
    pub Ratio: i32,
    pub MaxValue: i32,
    pub CurrentValue: i32,
}

impl PbMessage for RecoverAttr {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrId);
        }
        if self.Ratio != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ratio);
        }
        if self.MaxValue != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxValue);
        }
        if self.CurrentValue != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentValue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AttrId = r.read_varint_i32()?,
            2 => self.Ratio = r.read_varint_i32()?,
            4 => self.MaxValue = r.read_varint_i32()?,
            5 => self.CurrentValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressActivityTrialRoleUpdateNotify {
    pub CurUseRoleInfo: Option<roleInfo>,
    pub TrialRoleId: Vec<i32>,
}

impl RegressActivityTrialRoleUpdateNotify {
    pub const MSG_ID: u16 = 19657;
}

impl PbMessage for RegressActivityTrialRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        for item in &self.TrialRoleId {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TrialRoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressBonusRequest {
    pub ActivityId: i32,
    pub Ids: Vec<i32>,
}

impl RegressBonusRequest {
    pub const MSG_ID: u16 = 27943;
}

impl PbMessage for RegressBonusRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.ActivityId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressBonusResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Ids: Vec<i32>,
    pub Info: Vec<RegressBonusRewardInfo>,
}

impl RegressBonusResponse {
    pub const MSG_ID: u16 = 23806;
}

impl PbMessage for RegressBonusResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Info {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .push(<RegressBonusRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressBonusRewardInfo {
    pub Id: i32,
    pub Type: Option<RegressBonusRewardType>,
}

impl PbMessage for RegressBonusRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <RegressBonusRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressBuyBonusLvRequest {
    pub BuyLv: i32,
}

impl RegressBuyBonusLvRequest {
    pub const MSG_ID: u16 = 25368;
}

impl PbMessage for RegressBuyBonusLvRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuyLv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyLv);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.BuyLv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressBuyBonusLvResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub BonusNum: i32,
}

impl RegressBuyBonusLvResponse {
    pub const MSG_ID: u16 = 25492;
}

impl PbMessage for RegressBuyBonusLvResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BonusNum != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BonusNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.BonusNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressData {
    pub ActivityTasks: Vec<ActivityTask>,
    pub SignRewards: Vec<ActivityTask>,
    pub ScoreRewards: Vec<i32>,
    pub GradeId: i32,
    pub EndTime: i64,
    pub NextRefreshTime: i64,
    pub BossDoubleCount: i32,
    pub WeekDoubleCount: i32,
    pub Questionnaire: Vec<i32>,
    pub BossDoubleUnLock: bool,
    pub WeekDoubleUnLock: bool,
    pub Question: Vec<QuestionTask>,
    pub PayScoreRewards: Vec<i32>,
    pub DisposableReward: bool,
    pub RoleInfo: Vec<NewTrialRoleInfo>,
    pub PayRewardUnlock: bool,
    pub CurUseTrialRoleId: i32,
    pub CurUseRoleInfo: Option<roleInfo>,
}

impl PbMessage for RegressData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.SignRewards {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.ScoreRewards {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.GradeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GradeId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextRefreshTime);
        }
        if self.BossDoubleCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossDoubleCount);
        }
        if self.WeekDoubleCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekDoubleCount);
        }
        for item in &self.Questionnaire {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.BossDoubleUnLock {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.WeekDoubleUnLock {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Question {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.PayScoreRewards {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.DisposableReward {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleInfo {
            crate::wire::write_submessage(out, 16, item);
        }
        if self.PayRewardUnlock {
            crate::wire::write_tag(out, 17, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurUseTrialRoleId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurUseTrialRoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 19, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SignRewards
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreRewards, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.GradeId = r.read_varint_i32()?,
            6 => self.EndTime = r.read_varint_i64()?,
            7 => self.NextRefreshTime = r.read_varint_i64()?,
            8 => self.BossDoubleCount = r.read_varint_i32()?,
            9 => self.WeekDoubleCount = r.read_varint_i32()?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Questionnaire, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.BossDoubleUnLock = r.read_bool()?,
            12 => self.WeekDoubleUnLock = r.read_bool()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Question
                    .push(<QuestionTask as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PayScoreRewards,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => self.DisposableReward = r.read_bool()?,
            16 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .push(<NewTrialRoleInfo as PbMessage>::decode(bytes)?);
            }
            17 => self.PayRewardUnlock = r.read_bool()?,
            18 => self.CurUseTrialRoleId = r.read_varint_i32()?,
            19 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressDisposableRewardRequest {}

impl RegressDisposableRewardRequest {
    pub const MSG_ID: u16 = 29911;
}

impl PbMessage for RegressDisposableRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressDisposableRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RegressDisposableRewardResponse {
    pub const MSG_ID: u16 = 23423;
}

impl PbMessage for RegressDisposableRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressQuestionOpenRequest {
    pub QuestionId: i32,
}

impl RegressQuestionOpenRequest {
    pub const MSG_ID: u16 = 21964;
}

impl PbMessage for RegressQuestionOpenRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestionId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.QuestionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressQuestionOpenResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RegressQuestionOpenResponse {
    pub const MSG_ID: u16 = 22169;
}

impl PbMessage for RegressQuestionOpenResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressRepeatedTaskRequest {
    pub ActivityId: i32,
    pub TaskId: Vec<i32>,
}

impl RegressRepeatedTaskRequest {
    pub const MSG_ID: u16 = 26659;
}

impl PbMessage for RegressRepeatedTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TaskId {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.ActivityId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressRepeatedTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RegressRepeatedTaskResponse {
    pub const MSG_ID: u16 = 26837;
}

impl PbMessage for RegressRepeatedTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressSetCurUseTrialRoleRequest {
    pub TrialRoleId: i32,
}

impl RegressSetCurUseTrialRoleRequest {
    pub const MSG_ID: u16 = 27245;
}

impl PbMessage for RegressSetCurUseTrialRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrialRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressSetCurUseTrialRoleResponse {
    pub Code: Option<ErrorCode>,
    pub ReplacedRoleId: i32,
    pub RoleId: i32,
    pub CurUseRoleInfo: Option<roleInfo>,
    pub ReplacedCurRole: bool,
}

impl RegressSetCurUseTrialRoleResponse {
    pub const MSG_ID: u16 = 26566;
}

impl PbMessage for RegressSetCurUseTrialRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ReplacedRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReplacedRoleId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.ReplacedCurRole {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.ReplacedRoleId = r.read_varint_i32()?,
            8 => self.RoleId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.ReplacedCurRole = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressTaskRequest {
    pub RewardType: Option<RegressTaskTypeEnum>,
    pub TaskId: i32,
    pub ActivityId: i32,
}

impl RegressTaskRequest {
    pub const MSG_ID: u16 = 17178;
}

impl PbMessage for RegressTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.RewardType = Some(
                    <RegressTaskTypeEnum as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.TaskId = r.read_varint_i32()?,
            8 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RegressTaskResponse {
    pub const MSG_ID: u16 = 29284;
}

impl PbMessage for RegressTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressTrialRoleLvUpRequest {
    pub TrialRoleId: i32,
}

impl RegressTrialRoleLvUpRequest {
    pub const MSG_ID: u16 = 16654;
}

impl PbMessage for RegressTrialRoleLvUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrialRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressTrialRoleLvUpResponse {
    pub CurUseRoleInfo: Option<roleInfo>,
    pub RoleId: i32,
    pub Code: Option<ErrorCode>,
}

impl RegressTrialRoleLvUpResponse {
    pub const MSG_ID: u16 = 29813;
}

impl PbMessage for RegressTrialRoleLvUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.RoleId = r.read_varint_i32()?,
            15 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RegressTrialRoleUnlockNotify {
    pub RoleId: Vec<i32>,
}

impl RegressTrialRoleUnlockNotify {
    pub const MSG_ID: u16 = 21529;
}

impl PbMessage for RegressTrialRoleUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RelationIdNotify {
    pub EntityConfigId: i32,
    pub RelationId: i32,
    pub MatchIndex: i32,
    pub IstId: i32,
}

impl RelationIdNotify {
    pub const MSG_ID: u16 = 26419;
}

impl PbMessage for RelationIdNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.RelationId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RelationId);
        }
        if self.MatchIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchIndex);
        }
        if self.IstId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IstId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EntityConfigId = r.read_varint_i32()?,
            9 => self.RelationId = r.read_varint_i32()?,
            13 => self.MatchIndex = r.read_varint_i32()?,
            14 => self.IstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoteBatchCollectRequest {
    pub EntityIds: Vec<i64>,
    pub HostPlayerId: i32,
}

impl RemoteBatchCollectRequest {
    pub const MSG_ID: u16 = 20492;
}

impl PbMessage for RemoteBatchCollectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            11 => self.HostPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoteBatchCollectResponse {
    pub EntityIds: Vec<i64>,
    pub ErrCode: Option<ErrorCode>,
}

impl RemoteBatchCollectResponse {
    pub const MSG_ID: u16 = 20188;
}

impl PbMessage for RemoteBatchCollectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            14 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveEntityAoiNotify {
    pub RemoveInfos: Vec<EntityRemoveInfo>,
}

impl RemoveEntityAoiNotify {
    pub const MSG_ID: u16 = 17978;
}

impl PbMessage for RemoveEntityAoiNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RemoveInfos {
            crate::wire::write_submessage(out, 2, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RemoveInfos
                    .push(<EntityRemoveInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveForbidControlNotify {
    pub ForbidList: Vec<ControlParam>,
}

impl RemoveForbidControlNotify {
    pub const MSG_ID: u16 = 25975;
}

impl PbMessage for RemoveForbidControlNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ForbidList {
            crate::wire::write_submessage(out, 5, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ForbidList
                    .push(<ControlParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveHackingEntityNotify {
    pub AuxiliaryMachineEntityId: i64,
    pub HackingEntityId: i64,
}

impl RemoveHackingEntityNotify {
    pub const MSG_ID: u16 = 20772;
}

impl PbMessage for RemoveHackingEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AuxiliaryMachineEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AuxiliaryMachineEntityId);
        }
        if self.HackingEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HackingEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.AuxiliaryMachineEntityId = r.read_varint_i64()?,
            12 => self.HackingEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveInstanceTimerNotify {
    pub TimerId: i32,
}

impl RemoveInstanceTimerNotify {
    pub const MSG_ID: u16 = 27135;
}

impl PbMessage for RemoveInstanceTimerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.TimerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveInstanceTimerRequest {
    pub TimerId: i32,
}

impl RemoveInstanceTimerRequest {
    pub const MSG_ID: u16 = 24804;
}

impl PbMessage for RemoveInstanceTimerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.TimerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveInstanceTimerResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TimerId: i32,
}

impl RemoveInstanceTimerResponse {
    pub const MSG_ID: u16 = 22241;
}

impl PbMessage for RemoveInstanceTimerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TimerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.TimerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveMarkInfoNotify {
    pub MarkIds: Vec<i32>,
}

impl RemoveMarkInfoNotify {
    pub const MSG_ID: u16 = 26990;
}

impl PbMessage for RemoveMarkInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveOccupationInfoNotify {
    pub Resource: Vec<Vec<u8>>,
}

impl RemoveOccupationInfoNotify {
    pub const MSG_ID: u16 = 23251;
}

impl PbMessage for RemoveOccupationInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Resource {
            crate::wire::write_tag(out, 15, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Resource.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemovePlacementFromBoardNotify {
    pub GridInfo: Option<OccupiedBoardGridInfo>,
    pub BoardEntityConfigId: i32,
}

impl RemovePlacementFromBoardNotify {
    pub const MSG_ID: u16 = 22490;
}

impl PbMessage for RemovePlacementFromBoardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GridInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.BoardEntityConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoardEntityConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.GridInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.BoardEntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemovePreloadFlowsNotify {
    pub FlowListName: Vec<u8>,
    pub StateId: i32,
    pub FlowGuid: Vec<u8>,
    pub FlowId: i32,
}

impl RemovePreloadFlowsNotify {
    pub const MSG_ID: u16 = 24692;
}

impl PbMessage for RemovePreloadFlowsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FlowListName.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowListName);
        }
        if self.StateId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateId);
        }
        if !self.FlowGuid.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowGuid);
        }
        if self.FlowId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlowId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.FlowListName = r.read_len_delimited()?.to_vec(),
            7 => self.StateId = r.read_varint_i32()?,
            11 => self.FlowGuid = r.read_len_delimited()?.to_vec(),
            13 => self.FlowId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveReviveRegionRequest {
    pub ReviveId: i32,
}

impl RemoveReviveRegionRequest {
    pub const MSG_ID: u16 = 15252;
}

impl PbMessage for RemoveReviveRegionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.ReviveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveReviveRegionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RemoveReviveRegionResponse {
    pub const MSG_ID: u16 = 19862;
}

impl PbMessage for RemoveReviveRegionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveSysBuffNotify {
    pub EntityId: i64,
    pub Ids: Vec<i32>,
}

impl RemoveSysBuffNotify {
    pub const MSG_ID: u16 = 23366;
}

impl PbMessage for RemoveSysBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.EntityId = r.read_varint_i64()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveTreasureBoxSlotRequest {
    pub DetectionSlotId: i64,
}

impl RemoveTreasureBoxSlotRequest {
    pub const MSG_ID: u16 = 21336;
}

impl PbMessage for RemoveTreasureBoxSlotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionSlotId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DetectionSlotId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.DetectionSlotId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveTreasureBoxSlotResponse {
    pub Code: Option<ErrorCode>,
}

impl RemoveTreasureBoxSlotResponse {
    pub const MSG_ID: u16 = 24211;
}

impl PbMessage for RemoveTreasureBoxSlotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveValidTimeItem {
    pub ItemId: i32,
    pub IncrId: i32,
}

impl PbMessage for RemoveValidTimeItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenjuDataPb {
    pub MoveCount: i32,
    pub LastMoveEntityConfigId: i32,
    pub Winner: i32,
}

impl PbMessage for RenjuDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoveCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveCount);
        }
        if self.LastMoveEntityConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastMoveEntityConfigId);
        }
        if self.Winner != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Winner);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MoveCount = r.read_varint_i32()?,
            2 => self.LastMoveEntityConfigId = r.read_varint_i32()?,
            3 => self.Winner = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenjuExitMatchedActionRequest {
    pub EntityId: i64,
}

impl RenjuExitMatchedActionRequest {
    pub const MSG_ID: u16 = 25656;
}

impl PbMessage for RenjuExitMatchedActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenjuExitMatchedActionResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RenjuExitMatchedActionResponse {
    pub const MSG_ID: u16 = 26468;
}

impl PbMessage for RenjuExitMatchedActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenjuResetRequest {
    pub EntityId: i64,
}

impl RenjuResetRequest {
    pub const MSG_ID: u16 = 16999;
}

impl PbMessage for RenjuResetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenjuResetResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RenjuResetResponse {
    pub const MSG_ID: u16 = 24055;
}

impl PbMessage for RenjuResetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReorderFormationRequest {
    pub FormationId: i32,
    pub TargetId: i32,
}

impl ReorderFormationRequest {
    pub const MSG_ID: u16 = 27566;
}

impl PbMessage for ReorderFormationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.FormationId = r.read_varint_i32()?,
            12 => self.TargetId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReorderFormationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ReorderFormationResponse {
    pub const MSG_ID: u16 = 28600;
}

impl PbMessage for ReorderFormationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReplaceVehicleEntityNotify {
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub EntityId: i64,
    pub IsReplace: bool,
    pub GravityDirection: Option<Vector>,
}

impl ReplaceVehicleEntityNotify {
    pub const MSG_ID: u16 = 28957;
}

impl PbMessage for ReplaceVehicleEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsReplace {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.GravityDirection {
            crate::wire::write_submessage(out, 15, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.EntityId = r.read_varint_i64()?,
            12 => self.IsReplace = r.read_bool()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.GravityDirection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReportChatInfo {
    pub ChatMessage: Vec<u8>,
}

impl PbMessage for ReportChatInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ChatMessage.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ChatMessage);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ChatMessage = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReportPlayerRequest {
    pub TargetInfo: Option<ReportTargetInfo>,
    pub ChatInfo: Option<ReportChatInfo>,
    pub ReportMessage: Vec<u8>,
    pub TargetPlayerId: i32,
    pub ReportSource: i32,
    pub ReportReason: i32,
}

impl ReportPlayerRequest {
    pub const MSG_ID: u16 = 24953;
}

impl PbMessage for ReportPlayerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TargetInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.ChatInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if !self.ReportMessage.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReportMessage);
        }
        if self.TargetPlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetPlayerId);
        }
        if self.ReportSource != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReportSource);
        }
        if self.ReportReason != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReportReason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TargetInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ChatInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.ReportMessage = r.read_len_delimited()?.to_vec(),
            9 => self.TargetPlayerId = r.read_varint_i32()?,
            12 => self.ReportSource = r.read_varint_i32()?,
            15 => self.ReportReason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReportPlayerResponse {
    pub ReportCount: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl ReportPlayerResponse {
    pub const MSG_ID: u16 = 22547;
}

impl PbMessage for ReportPlayerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReportCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReportCount);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.ReportCount = r.read_varint_i32()?,
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReportTargetInfo {
    pub Name: Vec<u8>,
    pub Signature: Vec<u8>,
}

impl PbMessage for ReportTargetInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Signature);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Name = r.read_len_delimited()?.to_vec(),
            2 => self.Signature = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetBabelTowerLevelRequest {
    pub BabelTowerLevelId: i32,
}

impl ResetBabelTowerLevelRequest {
    pub const MSG_ID: u16 = 22844;
}

impl PbMessage for ResetBabelTowerLevelRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BabelTowerLevelId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BabelTowerLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.BabelTowerLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetBabelTowerLevelResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ResetBabelTowerLevelResponse {
    pub const MSG_ID: u16 = 27850;
}

impl PbMessage for ResetBabelTowerLevelResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetLocationForZRangeNotify {
    pub Rotation: Option<Rotator>,
    pub ResetLocation: Option<Vector>,
    pub EntityId: i64,
    pub NotifyRot: bool,
    pub ChangeInitPos: bool,
}

impl ResetLocationForZRangeNotify {
    pub const MSG_ID: u16 = 21097;
}

impl PbMessage for ResetLocationForZRangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ResetLocation {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.NotifyRot {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ChangeInitPos {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ResetLocation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.EntityId = r.read_varint_i64()?,
            11 => self.NotifyRot = r.read_bool()?,
            13 => self.ChangeInitPos = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetPointEntityNotify {
    pub ResetPointEntityId: i32,
}

impl ResetPointEntityNotify {
    pub const MSG_ID: u16 = 29014;
}

impl PbMessage for ResetPointEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResetPointEntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResetPointEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.ResetPointEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetQuestFocusModeNotify {
    pub Reason: Option<ResetQuestFocusModeReason>,
}

impl ResetQuestFocusModeNotify {
    pub const MSG_ID: u16 = 25104;
}

impl PbMessage for ResetQuestFocusModeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.Reason = Some(
                    <ResetQuestFocusModeReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResetSpecialBonusNotify {
    pub Ids: Vec<i32>,
}

impl ResetSpecialBonusNotify {
    pub const MSG_ID: u16 = 21478;
}

impl PbMessage for ResetSpecialBonusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResonantChainOptionalLimitInfo {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for ResonantChainOptionalLimitInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResonantChainOptionalLimitRequest {}

impl ResonantChainOptionalLimitRequest {
    pub const MSG_ID: u16 = 17766;
}

impl PbMessage for ResonantChainOptionalLimitRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResonantChainOptionalLimitResponse {
    pub Limit: i32,
    pub Infos: Vec<ResonantChainOptionalLimitInfo>,
}

impl ResonantChainOptionalLimitResponse {
    pub const MSG_ID: u16 = 27438;
}

impl PbMessage for ResonantChainOptionalLimitResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Limit != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Limit);
        }
        for item in &self.Infos {
            crate::wire::write_submessage(out, 14, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Limit = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<ResonantChainOptionalLimitInfo as PbMessage>::decode(
                        bytes,
                    )?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResultView {
    pub SurvivorsGainDatas: Vec<SurvivorsGainData>,
    pub CostTime: i64,
    pub WeaponMaxCount: i32,
    pub WaveId: i32,
    pub LevelId: i32,
    pub InfiniteMode: bool,
    pub PointSkill: i32,
    pub IsFinishCond: bool,
    pub KillMonsterCount: i32,
    pub Reason: i32,
}

impl PbMessage for ResultView {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SurvivorsGainDatas {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTime);
        }
        if self.WeaponMaxCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponMaxCount);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.InfiniteMode {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PointSkill != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PointSkill);
        }
        if self.IsFinishCond {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.KillMonsterCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillMonsterCount);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsGainDatas
                    .push(<SurvivorsGainData as PbMessage>::decode(bytes)?);
            }
            2 => self.CostTime = r.read_varint_i64()?,
            3 => self.WeaponMaxCount = r.read_varint_i32()?,
            4 => self.WaveId = r.read_varint_i32()?,
            5 => self.LevelId = r.read_varint_i32()?,
            6 => self.InfiniteMode = r.read_bool()?,
            7 => self.PointSkill = r.read_varint_i32()?,
            8 => self.IsFinishCond = r.read_bool()?,
            9 => self.KillMonsterCount = r.read_varint_i32()?,
            10 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReviveRequest {
    pub CustomParam: i32,
    pub UseItem: bool,
}

impl ReviveRequest {
    pub const MSG_ID: u16 = 15137;
}

impl PbMessage for ReviveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CustomParam != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CustomParam);
        }
        if self.UseItem {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.CustomParam = r.read_varint_i32()?,
            12 => self.UseItem = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReviveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ReviveResponse {
    pub const MSG_ID: u16 = 24465;
}

impl PbMessage for ReviveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReviveRoleInformation {
    pub EntityId: i64,
    pub CurHpAttribute: Option<GameplayAttributeData>,
}

impl PbMessage for ReviveRoleInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.CurHpAttribute {
            crate::wire::write_submessage(out, 2, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CurHpAttribute
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RewardGainingPanelInfo {
    pub GainingGroupIds: Vec<i32>,
    pub RefreshCost: i32,
    pub RemainRefreshTimes: i32,
    pub TotalRefreshTimes: i32,
}

impl PbMessage for RewardGainingPanelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GainingGroupIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCost);
        }
        if self.RemainRefreshTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainRefreshTimes);
        }
        if self.TotalRefreshTimes != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalRefreshTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.GainingGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => self.RefreshCost = r.read_varint_i32()?,
            3 => self.RemainRefreshTimes = r.read_varint_i32()?,
            4 => self.TotalRefreshTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RewardItem {
    pub Id: i32,
    pub Count: i32,
    pub IncrId: i32,
}

impl PbMessage for RewardItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            3 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RewardItemPackage {
    pub RewardItem: Option<RewardItem>,
    pub RewardItems: Vec<RewardItem>,
}

impl PbMessage for RewardItemPackage {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardItem {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.RewardItems {
            crate::wire::write_submessage(out, 2, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItem
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItems
                    .push(<RewardItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmChangePartnerRoleRequest {
    pub RhythmRoleId: i32,
}

impl RhythmChangePartnerRoleRequest {
    pub const MSG_ID: u16 = 23553;
}

impl PbMessage for RhythmChangePartnerRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RhythmRoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RhythmRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.RhythmRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmChangePartnerRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RhythmChangePartnerRoleResponse {
    pub const MSG_ID: u16 = 23561;
}

impl PbMessage for RhythmChangePartnerRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmCtx {
    pub SubLevelId: i32,
}

impl PbMessage for RhythmCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SubLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmMainLineInfoRequest {}

impl RhythmMainLineInfoRequest {
    pub const MSG_ID: u16 = 17348;
}

impl PbMessage for RhythmMainLineInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmMainLineInfoResponse {
    pub RhythmMainLinePb: Option<RhythmActivityPb>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RhythmMainLineInfoResponse {
    pub const MSG_ID: u16 = 21999;
}

impl PbMessage for RhythmMainLineInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RhythmMainLinePb {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RhythmMainLinePb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmPlayerRankingInfo {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub SubLevelInfo: Vec<RhythmPlayerRankingSubLevelInfo>,
}

impl PbMessage for RhythmPlayerRankingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        for item in &self.SubLevelInfo {
            crate::wire::write_submessage(out, 3, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SubLevelInfo
                    .push(<RhythmPlayerRankingSubLevelInfo as PbMessage>::decode(
                        bytes,
                    )?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmPlayerRankingSubLevelInfo {
    pub SubLevelId: i32,
    pub RhythmRoleId: i32,
    pub Score: i32,
    pub Accuracy: i32,
}

impl PbMessage for RhythmPlayerRankingSubLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelId);
        }
        if self.RhythmRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RhythmRoleId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Accuracy != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Accuracy);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SubLevelId = r.read_varint_i32()?,
            2 => self.RhythmRoleId = r.read_varint_i32()?,
            3 => self.Score = r.read_varint_i32()?,
            4 => self.Accuracy = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRankListRequest {
    pub ActivityId: i32,
}

impl RhythmRankListRequest {
    pub const MSG_ID: u16 = 26399;
}

impl PbMessage for RhythmRankListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRankListResponse {
    pub SelfData: Option<RhythmPlayerRankingInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub FriendData: Vec<RhythmPlayerRankingInfo>,
}

impl RhythmRankListResponse {
    pub const MSG_ID: u16 = 26553;
}

impl PbMessage for RhythmRankListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SelfData {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FriendData {
            crate::wire::write_submessage(out, 9, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.SelfData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.FriendData
                    .push(<RhythmPlayerRankingInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRankListSelfDataRequest {
    pub ActivityId: i32,
}

impl RhythmRankListSelfDataRequest {
    pub const MSG_ID: u16 = 25501;
}

impl PbMessage for RhythmRankListSelfDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRankListSelfDataResponse {
    pub SelfData: Option<RhythmPlayerRankingInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RhythmRankListSelfDataResponse {
    pub const MSG_ID: u16 = 24201;
}

impl PbMessage for RhythmRankListSelfDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SelfData {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SelfData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmResultPayload {
    pub Score: i32,
    pub Accuracy: i32,
    pub JudgementCount: Vec<i32>,
    pub MaxCombo: i32,
    pub Completion: i32,
    pub Rank: Option<RhythmShipRank>,
    pub CostSecond: i32,
    pub FeverCount: i32,
}

impl PbMessage for RhythmResultPayload {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Accuracy != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Accuracy);
        }
        for item in &self.JudgementCount {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MaxCombo != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCombo);
        }
        if self.Completion != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Completion);
        }
        if let Some(v) = &self.Rank {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CostSecond != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostSecond);
        }
        if self.FeverCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FeverCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Score = r.read_varint_i32()?,
            3 => self.Accuracy = r.read_varint_i32()?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.JudgementCount, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => self.MaxCombo = r.read_varint_i32()?,
            6 => self.Completion = r.read_varint_i32()?,
            7 => {
                self.Rank = Some(
                    <RhythmShipRank as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => self.CostSecond = r.read_varint_i32()?,
            9 => self.FeverCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmRoleUpdateNotify {
    pub ActivityId: i32,
    pub UnlockedRole: Vec<i32>,
}

impl RhythmRoleUpdateNotify {
    pub const MSG_ID: u16 = 27775;
}

impl PbMessage for RhythmRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.UnlockedRole {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ActivityId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedRole, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSetRedDotRequest {
    pub ActivityId: i32,
    pub RedDot: Option<RhythmRedDotPb>,
}

impl RhythmSetRedDotRequest {
    pub const MSG_ID: u16 = 24911;
}

impl PbMessage for RhythmSetRedDotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.RedDot {
            crate::wire::write_submessage(out, 12, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.ActivityId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RedDot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSetRedDotResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RedDot: Option<RhythmRedDotPb>,
}

impl RhythmSetRedDotResponse {
    pub const MSG_ID: u16 = 21670;
}

impl PbMessage for RhythmSetRedDotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RedDot {
            crate::wire::write_submessage(out, 12, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RedDot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSettleRequest {
    pub SettleReason: Option<RhythmSettleReasonPb>,
    pub SubLevelId: i32,
    pub Payload: Option<RhythmResultPayload>,
}

impl RhythmSettleRequest {
    pub const MSG_ID: u16 = 18268;
}

impl PbMessage for RhythmSettleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SettleReason {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SubLevelId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelId);
        }
        if let Some(v) = &self.Payload {
            crate::wire::write_submessage(out, 15, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.SettleReason = Some(
                    <RhythmSettleReasonPb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => self.SubLevelId = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSettleResponse {
    pub BestScore: i32,
    pub BestScoreFlag: bool,
    pub ErrCode: Option<ErrorCode>,
    pub Payload: Option<RhythmResultPayload>,
}

impl RhythmSettleResponse {
    pub const MSG_ID: u16 = 18537;
}

impl PbMessage for RhythmSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BestScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestScore);
        }
        if self.BestScoreFlag {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Payload {
            crate::wire::write_submessage(out, 15, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.BestScore = r.read_varint_i32()?,
            7 => self.BestScoreFlag = r.read_bool()?,
            10 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmShipStartNotify {
    pub RhythmRoleId: i32,
    pub SubLevelId: i32,
}

impl RhythmShipStartNotify {
    pub const MSG_ID: u16 = 24795;
}

impl PbMessage for RhythmShipStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RhythmRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RhythmRoleId);
        }
        if self.SubLevelId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.RhythmRoleId = r.read_varint_i32()?,
            6 => self.SubLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmSubLevelUpdateNotify {
    pub UpdateSubLevelPb: Vec<RhythmSubLevelPb>,
}

impl RhythmSubLevelUpdateNotify {
    pub const MSG_ID: u16 = 18647;
}

impl PbMessage for RhythmSubLevelUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UpdateSubLevelPb {
            crate::wire::write_submessage(out, 14, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateSubLevelPb
                    .push(<RhythmSubLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmTaskOneKeyRewardRequest {
    pub TaskType: Option<RhythmTaskTypePb>,
    pub TaskId: Vec<i32>,
}

impl RhythmTaskOneKeyRewardRequest {
    pub const MSG_ID: u16 = 27736;
}

impl PbMessage for RhythmTaskOneKeyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TaskType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskId {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                self.TaskType = Some(
                    <RhythmTaskTypePb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmTaskOneKeyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RhythmTaskOneKeyRewardResponse {
    pub const MSG_ID: u16 = 28347;
}

impl PbMessage for RhythmTaskOneKeyRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RhythmTaskUpdateNotify {
    pub RhythmTask: Vec<RhythmTaskPb>,
}

impl RhythmTaskUpdateNotify {
    pub const MSG_ID: u16 = 15309;
}

impl PbMessage for RhythmTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RhythmTask {
            crate::wire::write_submessage(out, 6, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RhythmTask
                    .push(<RhythmTaskPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestActivityInfo {
    pub InstInfos: Vec<RiskHarvestInstInfo>,
    pub RewardedScores: Vec<i32>,
    pub RewardedBuffGroups: Vec<i32>,
    pub UnlockBuffGroups: Vec<i32>,
    pub RewardedBuffTypeIds: Vec<i32>,
}

impl PbMessage for RiskHarvestActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardedScores {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardedBuffGroups {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBuffGroups {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardedBuffTypeIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.InstInfos
                    .push(<RiskHarvestInstInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedScores, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedBuffGroups,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBuffGroups,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedBuffTypeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestActivityUpdateNotify {
    pub ActivityInfo: Option<RiskHarvestActivityInfo>,
}

impl RiskHarvestActivityUpdateNotify {
    pub const MSG_ID: u16 = 18015;
}

impl PbMessage for RiskHarvestActivityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityInfo {
            crate::wire::write_submessage(out, 10, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestArtifactNotify {
    pub ArtifactInfo: Option<RiskHarvestInstArtifactInfo>,
}

impl RiskHarvestArtifactNotify {
    pub const MSG_ID: u16 = 21422;
}

impl PbMessage for RiskHarvestArtifactNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ArtifactInfo {
            crate::wire::write_submessage(out, 11, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ArtifactInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffCountRewardRequest {
    pub Id: i32,
}

impl RiskHarvestBuffCountRewardRequest {
    pub const MSG_ID: u16 = 16445;
}

impl PbMessage for RiskHarvestBuffCountRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffCountRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestBuffCountRewardResponse {
    pub const MSG_ID: u16 = 25669;
}

impl PbMessage for RiskHarvestBuffCountRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffUnlockNotify {
    pub UnlockBuffGroup: Vec<i32>,
}

impl RiskHarvestBuffUnlockNotify {
    pub const MSG_ID: u16 = 16190;
}

impl PbMessage for RiskHarvestBuffUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockBuffGroup {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockBuffGroup,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffUnlockRewardRequest {
    pub Id: i32,
}

impl RiskHarvestBuffUnlockRewardRequest {
    pub const MSG_ID: u16 = 29264;
}

impl PbMessage for RiskHarvestBuffUnlockRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffUnlockRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestBuffUnlockRewardResponse {
    pub const MSG_ID: u16 = 27680;
}

impl PbMessage for RiskHarvestBuffUnlockRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestBuffUpdateNotify {
    pub RemoveBuffGroups: Vec<i32>,
    pub AddBuffGroups: Vec<i32>,
    pub ArtifactInfo: Option<RiskHarvestInstArtifactInfo>,
}

impl RiskHarvestBuffUpdateNotify {
    pub const MSG_ID: u16 = 28221;
}

impl PbMessage for RiskHarvestBuffUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RemoveBuffGroups {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AddBuffGroups {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ArtifactInfo {
            crate::wire::write_submessage(out, 14, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RemoveBuffGroups,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AddBuffGroups, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.ArtifactInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestEndNotify {
    pub MonsterScore: i32,
    pub Pass: bool,
    pub TimeScore: i32,
    pub TotalScore: i32,
    pub Id: i32,
    pub InstScore: i32,
}

impl RiskHarvestEndNotify {
    pub const MSG_ID: u16 = 25888;
}

impl PbMessage for RiskHarvestEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterScore);
        }
        if self.Pass {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TimeScore != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeScore);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.InstScore != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.MonsterScore = r.read_varint_i32()?,
            5 => self.Pass = r.read_bool()?,
            7 => self.TimeScore = r.read_varint_i32()?,
            10 => self.TotalScore = r.read_varint_i32()?,
            12 => self.Id = r.read_varint_i32()?,
            15 => self.InstScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestInstArtifactInfo {
    pub Id: i32,
    pub Progress: i32,
    pub Buffs: Vec<RiskHarvestInstBuffInfo>,
}

impl PbMessage for RiskHarvestInstArtifactInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        for item in &self.Buffs {
            crate::wire::write_submessage(out, 3, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Progress = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Buffs
                    .push(<RiskHarvestInstBuffInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestInstBuffInfo {
    pub Id: i32,
    pub Count: i32,
}

impl PbMessage for RiskHarvestInstBuffInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestInstRewardRequest {
    pub Id: i32,
}

impl RiskHarvestInstRewardRequest {
    pub const MSG_ID: u16 = 16068;
}

impl PbMessage for RiskHarvestInstRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestInstRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestInstRewardResponse {
    pub const MSG_ID: u16 = 25323;
}

impl PbMessage for RiskHarvestInstRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestInstUpdateNotify {
    pub InstInfos: Vec<RiskHarvestInstInfo>,
}

impl RiskHarvestInstUpdateNotify {
    pub const MSG_ID: u16 = 18648;
}

impl PbMessage for RiskHarvestInstUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstInfos {
            crate::wire::write_submessage(out, 4, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InstInfos
                    .push(<RiskHarvestInstInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestScoreRewardRequest {
    pub Id: i32,
}

impl RiskHarvestScoreRewardRequest {
    pub const MSG_ID: u16 = 26581;
}

impl PbMessage for RiskHarvestScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestScoreRewardResponse {
    pub const MSG_ID: u16 = 27437;
}

impl PbMessage for RiskHarvestScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestSettleRequest {}

impl RiskHarvestSettleRequest {
    pub const MSG_ID: u16 = 29019;
}

impl PbMessage for RiskHarvestSettleRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestSettleResponse {
    pub const MSG_ID: u16 = 25343;
}

impl PbMessage for RiskHarvestSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestStarRewardRequest {
    pub Index: i32,
    pub Id: i32,
}

impl RiskHarvestStarRewardRequest {
    pub const MSG_ID: u16 = 22005;
}

impl PbMessage for RiskHarvestStarRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Index = r.read_varint_i32()?,
            11 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RiskHarvestStarRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RiskHarvestStarRewardResponse {
    pub const MSG_ID: u16 = 17366;
}

impl PbMessage for RiskHarvestStarRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookFullRewardRequest {}

impl RoadBookFullRewardRequest {
    pub const MSG_ID: u16 = 25619;
}

impl PbMessage for RoadBookFullRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookFullRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoadBookFullRewardResponse {
    pub const MSG_ID: u16 = 26687;
}

impl PbMessage for RoadBookFullRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookInfoUpdateNotify {
    pub UnLockArea: i32,
    pub ActivityTask: Option<ConditionTask>,
    pub MotorcycleLevel: Option<RoadBookMotorcycleInfo>,
    pub UnlockTask: i32,
    pub UnLockMonsterId: i32,
}

impl RoadBookInfoUpdateNotify {
    pub const MSG_ID: u16 = 20058;
}

impl PbMessage for RoadBookInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnLockArea != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockArea);
        }
        if let Some(v) = &self.ActivityTask {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.MotorcycleLevel {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.UnlockTask != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockTask);
        }
        if self.UnLockMonsterId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockMonsterId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.UnLockArea = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.MotorcycleLevel
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.UnlockTask = r.read_varint_i32()?,
            14 => self.UnLockMonsterId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookLevelUpRequest {}

impl RoadBookLevelUpRequest {
    pub const MSG_ID: u16 = 21349;
}

impl PbMessage for RoadBookLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoadBookLevelUpResponse {
    pub const MSG_ID: u16 = 19714;
}

impl PbMessage for RoadBookLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookMotorRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl RoadBookMotorRewardRequest {
    pub const MSG_ID: u16 = 19974;
}

impl PbMessage for RoadBookMotorRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookMotorRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskIds: Vec<i32>,
}

impl RoadBookMotorRewardResponse {
    pub const MSG_ID: u16 = 23361;
}

impl PbMessage for RoadBookMotorRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookTaskRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl RoadBookTaskRewardRequest {
    pub const MSG_ID: u16 = 28058;
}

impl PbMessage for RoadBookTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadBookTaskRewardResponse {
    pub TaskIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoadBookTaskRewardResponse {
    pub const MSG_ID: u16 = 24523;
}

impl PbMessage for RoadBookTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadNetworkEntityStartNavNotify {
    pub EntityId: i64,
    pub DestRoadId: i32,
    pub StartRoadIndex: i32,
    pub StartRoadId: i32,
    pub DestIndex: i32,
}

impl RoadNetworkEntityStartNavNotify {
    pub const MSG_ID: u16 = 28943;
}

impl PbMessage for RoadNetworkEntityStartNavNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.DestRoadId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestRoadId);
        }
        if self.StartRoadIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartRoadIndex);
        }
        if self.StartRoadId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartRoadId);
        }
        if self.DestIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.EntityId = r.read_varint_i64()?,
            9 => self.DestRoadId = r.read_varint_i32()?,
            10 => self.StartRoadIndex = r.read_varint_i32()?,
            12 => self.StartRoadId = r.read_varint_i32()?,
            13 => self.DestIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadNetworkEntityUpdateCurRoadPush {
    pub CurRoadId: i32,
    pub CurRoadIndex: i32,
    pub EntityId: i64,
}

impl RoadNetworkEntityUpdateCurRoadPush {
    pub const MSG_ID: u16 = 17914;
}

impl PbMessage for RoadNetworkEntityUpdateCurRoadPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurRoadId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRoadId);
        }
        if self.CurRoadIndex != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRoadIndex);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurRoadId = r.read_varint_i32()?,
            7 => self.CurRoadIndex = r.read_varint_i32()?,
            11 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RobotDestinationNotify {
    pub Location: Option<Vector>,
}

impl RobotDestinationNotify {
    pub const MSG_ID: u16 = 21753;
}

impl PbMessage for RobotDestinationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 5, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RobotDestinationPush {
    pub Location: Option<Vector>,
}

impl RobotDestinationPush {
    pub const MSG_ID: u16 = 24844;
}

impl PbMessage for RobotDestinationPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 3, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueArchiveInfo {
    pub SlotId: i32,
    pub HasData: bool,
    pub SaveTime: i64,
    pub InstId: i32,
    pub SeasonId: i32,
    pub RoleIds: Vec<i32>,
    pub Attributes: BTreeMap<i32, i32>,
    pub RoguelikeInstInfo: Option<RoguelikeInstInfo>,
}

impl PbMessage for RogueArchiveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.HasData {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SaveTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SaveTime);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.Attributes.is_empty() {
            crate::wire::write_map(out, 7, &self.Attributes);
        }
        if let Some(v) = &self.RoguelikeInstInfo {
            crate::wire::write_submessage(out, 8, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SlotId = r.read_varint_i32()?,
            2 => self.HasData = r.read_bool()?,
            3 => self.SaveTime = r.read_varint_i64()?,
            4 => self.InstId = r.read_varint_i32()?,
            5 => self.SeasonId = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => crate::wire::decode_map_entry(r, &mut self.Attributes)?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeInstInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueArchiveListRequest {
    pub SeasonId: i32,
}

impl RogueArchiveListRequest {
    pub const MSG_ID: u16 = 28324;
}

impl PbMessage for RogueArchiveListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueArchiveListResponse {
    pub ArchiveInfos: Vec<RogueArchiveInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueArchiveListResponse {
    pub const MSG_ID: u16 = 26762;
}

impl PbMessage for RogueArchiveListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ArchiveInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ArchiveInfos
                    .push(<RogueArchiveInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueArchiveSaveRequest {
    pub SlotId: i32,
}

impl RogueArchiveSaveRequest {
    pub const MSG_ID: u16 = 20091;
}

impl PbMessage for RogueArchiveSaveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueArchiveSaveResponse {
    pub SaveTime: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueArchiveSaveResponse {
    pub const MSG_ID: u16 = 29633;
}

impl PbMessage for RogueArchiveSaveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SaveTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SaveTime);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.SaveTime = r.read_varint_i64()?,
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueBlackFlowerNotify {
    pub EntityConfigId: i32,
}

impl RogueBlackFlowerNotify {
    pub const MSG_ID: u16 = 24206;
}

impl PbMessage for RogueBlackFlowerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueBlackFlowerRequest {
    pub Ret: bool,
    pub EntityConfigId: i32,
}

impl RogueBlackFlowerRequest {
    pub const MSG_ID: u16 = 24506;
}

impl PbMessage for RogueBlackFlowerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Ret {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.Ret = r.read_bool()?,
            14 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueBlackFlowerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueBlackFlowerResponse {
    pub const MSG_ID: u16 = 19376;
}

impl PbMessage for RogueBlackFlowerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueBossLinkSettleNotify {
    pub TimeScore: i32,
    pub HistoryTotalScore: i32,
    pub TotalScore: i32,
    pub IsNewRecord: bool,
    pub IsSucc: bool,
    pub LifeScore: i32,
    pub StepScore: i32,
    pub CostTime: i32,
}

impl RogueBossLinkSettleNotify {
    pub const MSG_ID: u16 = 19729;
}

impl PbMessage for RogueBossLinkSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeScore);
        }
        if self.HistoryTotalScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryTotalScore);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
        if self.IsNewRecord {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSucc {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LifeScore != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LifeScore);
        }
        if self.StepScore != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StepScore);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TimeScore = r.read_varint_i32()?,
            3 => self.HistoryTotalScore = r.read_varint_i32()?,
            4 => self.TotalScore = r.read_varint_i32()?,
            5 => self.IsNewRecord = r.read_bool()?,
            9 => self.IsSucc = r.read_bool()?,
            10 => self.LifeScore = r.read_varint_i32()?,
            11 => self.StepScore = r.read_varint_i32()?,
            15 => self.CostTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueGainEntry {
    pub Type: Option<RoguelikeGainDataType>,
    pub Index: i32,
    pub ConfigId: i32,
    pub ElementDict: BTreeMap<i32, i32>,
    pub AffixEntryList: Vec<AffixEntry>,
    pub DiscountInfo: Option<DiscountInfo>,
    pub IsSell: bool,
    pub IsSelect: bool,
    pub IsNew: bool,
    pub Cost: i32,
    pub RestCount: i32,
    pub IsValid: bool,
    pub IncId: i32,
}

impl PbMessage for RogueGainEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if !self.ElementDict.is_empty() {
            crate::wire::write_map(out, 4, &self.ElementDict);
        }
        for item in &self.AffixEntryList {
            crate::wire::write_submessage(out, 5, item);
        }
        if let Some(v) = &self.DiscountInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.IsSell {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Cost != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cost);
        }
        if self.RestCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RestCount);
        }
        if self.IsValid {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Type = Some(
                    <RoguelikeGainDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Index = r.read_varint_i32()?,
            3 => self.ConfigId = r.read_varint_i32()?,
            4 => crate::wire::decode_map_entry(r, &mut self.ElementDict)?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AffixEntryList
                    .push(<AffixEntry as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.DiscountInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.IsSell = r.read_bool()?,
            8 => self.IsSelect = r.read_bool()?,
            9 => self.IsNew = r.read_bool()?,
            10 => self.Cost = r.read_varint_i32()?,
            11 => self.RestCount = r.read_varint_i32()?,
            12 => self.IsValid = r.read_bool()?,
            13 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueGotoLevelPlayOp {
    pub IsFormation: bool,
    pub RoomId: i32,
    pub RommType: i32,
    pub CanSkipBattle: bool,
    pub RogueResSubLevelData: Option<RogueResSubLevelData>,
}

impl PbMessage for RogueGotoLevelPlayOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFormation {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
        if self.RommType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RommType);
        }
        if self.CanSkipBattle {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.RogueResSubLevelData {
            crate::wire::write_submessage(out, 6, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsFormation = r.read_bool()?,
            3 => self.RoomId = r.read_varint_i32()?,
            4 => self.RommType = r.read_varint_i32()?,
            5 => self.CanSkipBattle = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResSubLevelData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueHotEntryInfo {
    pub InstId: i32,
    pub VisibleHotEntryGroupIds: Vec<i32>,
    pub SelectableHotEntryGroupIds: Vec<i32>,
    pub UnlockHotEntryGroupIds: Vec<i32>,
    pub VisibleRoleIds: Vec<i32>,
    pub VisiblePhantomIds: Vec<i32>,
    pub RoleShowLevel: i32,
}

impl PbMessage for RogueHotEntryInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.VisibleHotEntryGroupIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SelectableHotEntryGroupIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockHotEntryGroupIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.VisibleRoleIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.VisiblePhantomIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RoleShowLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleShowLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.InstId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.VisibleHotEntryGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectableHotEntryGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockHotEntryGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.VisibleRoleIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.VisiblePhantomIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.RoleShowLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueInstExtraInfo {
    pub SeasonId: i32,
    pub RogueHotEntryInfos: Vec<RogueHotEntryInfo>,
    pub UnlockRoleIds: Vec<i32>,
    pub UnlockedPhantomIds: Vec<i32>,
}

impl PbMessage for RogueInstExtraInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        for item in &self.RogueHotEntryInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnlockRoleIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedPhantomIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SeasonId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueHotEntryInfos
                    .push(<RogueHotEntryInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockRoleIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedPhantomIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueInstExtraInfoRequest {
    pub SeasonId: i32,
}

impl RogueInstExtraInfoRequest {
    pub const MSG_ID: u16 = 28775;
}

impl PbMessage for RogueInstExtraInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueInstExtraInfoResponse {
    pub RogueInstExtraInfo: Option<RogueInstExtraInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueInstExtraInfoResponse {
    pub const MSG_ID: u16 = 19878;
}

impl PbMessage for RogueInstExtraInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueInstExtraInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueInstExtraInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueInstExtraInfoUpdateNotify {
    pub RogueInstExtraInfo: Option<RogueInstExtraInfo>,
}

impl RogueInstExtraInfoUpdateNotify {
    pub const MSG_ID: u16 = 18838;
}

impl PbMessage for RogueInstExtraInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueInstExtraInfo {
            crate::wire::write_submessage(out, 10, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RogueInstExtraInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueInstRoleData {
    pub InstId: i32,
    pub ValidTrialRoleIds: Vec<i32>,
    pub RoleIds: Vec<i32>,
    pub LockRoleIds: Vec<i32>,
}

impl PbMessage for RogueInstRoleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.ValidTrialRoleIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LockRoleIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.InstId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ValidTrialRoleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockRoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueLevelPlayData {
    pub Index: i32,
    pub LevelPlayId: i32,
    pub Reward: Option<SignState>,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub Record: i32,
}

impl PbMessage for RogueLevelPlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if let Some(v) = &self.Reward {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.Record != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Record);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Index = r.read_varint_i32()?,
            2 => self.LevelPlayId = r.read_varint_i32()?,
            3 => {
                self.Reward = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.IsUnlock = r.read_bool()?,
            5 => self.UnlockTime = r.read_varint_i64()?,
            6 => self.Record = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueLevelPlayNotify {
    pub IsSuccess: bool,
    pub UseTime: i32,
    pub ConfigId: i32,
    pub IsNewRecord: bool,
}

impl RogueLevelPlayNotify {
    pub const MSG_ID: u16 = 29192;
}

impl PbMessage for RogueLevelPlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsSuccess {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNewRecord {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.IsSuccess = r.read_bool()?,
            11 => self.UseTime = r.read_varint_i32()?,
            12 => self.ConfigId = r.read_varint_i32()?,
            13 => self.IsNewRecord = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueLimitedAward {
    pub SignState: Option<SignState>,
    pub CurProgress: i32,
    pub MaxProgress: i32,
    pub ConfigId: i32,
}

impl PbMessage for RogueLimitedAward {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SignState {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurProgress);
        }
        if self.MaxProgress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxProgress);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.SignState = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.CurProgress = r.read_varint_i32()?,
            3 => self.MaxProgress = r.read_varint_i32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResActivityData {
    pub RogueResSeasonData: Vec<RogueResSeasonData>,
    pub RogueResGlobalInfo: Option<RogueResGlobalInfo>,
}

impl PbMessage for RogueResActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueResSeasonData {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.RogueResGlobalInfo {
            crate::wire::write_submessage(out, 2, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResSeasonData
                    .push(<RogueResSeasonData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGlobalInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResBranchTaskTheme {
    pub BranchActivityTasks: Vec<ActivityTask>,
    pub RogueResTaskThemeId: i32,
    pub EndTime: i64,
}

impl PbMessage for RogueResBranchTaskTheme {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BranchActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RogueResTaskThemeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RogueResTaskThemeId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.BranchActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => self.RogueResTaskThemeId = r.read_varint_i32()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResChangeFormationRequest {
    pub Index: i32,
    pub RogueResFormation: Option<RogueResFormation>,
}

impl RogueResChangeFormationRequest {
    pub const MSG_ID: u16 = 24365;
}

impl PbMessage for RogueResChangeFormationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.RogueResFormation {
            crate::wire::write_submessage(out, 4, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Index = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResChangeFormationResponse {
    pub RogueResFormation: Option<RogueResFormation>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResChangeFormationResponse {
    pub const MSG_ID: u16 = 23844;
}

impl PbMessage for RogueResChangeFormationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResFormation {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResComplex {
    pub ConfigId: i32,
    pub ItemId: i32,
    pub Count: i32,
    pub IsSelect: bool,
    pub IsRole: bool,
}

impl PbMessage for RogueResComplex {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRole {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            4 => self.IsSelect = r.read_bool()?,
            5 => self.IsRole = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResCurrencyNotify {
    pub CurrencyDict: BTreeMap<i32, i32>,
}

impl RogueResCurrencyNotify {
    pub const MSG_ID: u16 = 29934;
}

impl PbMessage for RogueResCurrencyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CurrencyDict.is_empty() {
            crate::wire::write_map(out, 14, &self.CurrencyDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => crate::wire::decode_map_entry(r, &mut self.CurrencyDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResCurrencyUpdateNotify {
    pub EventType: i32,
    pub CurrencyUpdateDict: BTreeMap<i32, i32>,
}

impl RogueResCurrencyUpdateNotify {
    pub const MSG_ID: u16 = 20861;
}

impl PbMessage for RogueResCurrencyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventType);
        }
        if !self.CurrencyUpdateDict.is_empty() {
            crate::wire::write_map(out, 12, &self.CurrencyUpdateDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.EventType = r.read_varint_i32()?,
            12 => crate::wire::decode_map_entry(r, &mut self.CurrencyUpdateDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResElementUpdateNotify {
    pub ElementUnits: Vec<ElementUnit>,
}

impl RogueResElementUpdateNotify {
    pub const MSG_ID: u16 = 27121;
}

impl PbMessage for RogueResElementUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ElementUnits {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ElementUnits
                    .push(<ElementUnit as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResEndingRewardRequest {
    pub SeasonId: i32,
    pub Index: i32,
}

impl RogueResEndingRewardRequest {
    pub const MSG_ID: u16 = 28009;
}

impl PbMessage for RogueResEndingRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SeasonId = r.read_varint_i32()?,
            4 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResEndingRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResEndingRewardResponse {
    pub const MSG_ID: u16 = 17493;
}

impl PbMessage for RogueResEndingRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResEndingUpdateNotify {
    pub Endings: Vec<i32>,
    pub EndingRewardIds: Vec<ActivityTask>,
}

impl RogueResEndingUpdateNotify {
    pub const MSG_ID: u16 = 16750;
}

impl PbMessage for RogueResEndingUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Endings {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.EndingRewardIds {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Endings, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.EndingRewardIds
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResEvent {
    pub ConfigId: i32,
    pub CurOption: i32,
    pub EventOptions: Vec<EventOption>,
    pub SelectOptions: Vec<i32>,
}

impl PbMessage for RogueResEvent {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.CurOption != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurOption);
        }
        for item in &self.EventOptions {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.SelectOptions {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.CurOption = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.EventOptions
                    .push(<EventOption as PbMessage>::decode(bytes)?);
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectOptions, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResFormation {
    pub RoleId: Vec<i32>,
    pub LinkId: i32,
}

impl PbMessage for RogueResFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LinkId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LinkId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.LinkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResFormationUpdateNotify {
    pub RogueResFormations: Vec<RogueResFormation>,
}

impl RogueResFormationUpdateNotify {
    pub const MSG_ID: u16 = 28455;
}

impl PbMessage for RogueResFormationUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueResFormations {
            crate::wire::write_submessage(out, 9, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResFormations
                    .push(<RogueResFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResFovUpdateNotify {
    pub Fov: i32,
}

impl RogueResFovUpdateNotify {
    pub const MSG_ID: u16 = 17242;
}

impl PbMessage for RogueResFovUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Fov != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Fov);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Fov = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGainData {
    pub RogueResToken: Option<RogueResToken>,
    pub RogueResShopToken: Option<RogueResShopToken>,
    pub RogueResRoleBuff: Option<RogueResRoleBuff>,
    pub RogueResPhantom: Option<RogueResPhantom>,
    pub RogueResMiracleCreation: Option<RogueResMiracleCreation>,
    pub RogueResRole: Option<RogueResRole>,
    pub RogueResEvent: Option<RogueResEvent>,
    pub RogueResComplex: Option<RogueResComplex>,
    pub IncId: i32,
    pub RogueResDataType: Option<RogueResDataType>,
}

impl PbMessage for RogueResGainData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResToken {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.RogueResShopToken {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.RogueResRoleBuff {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.RogueResPhantom {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.RogueResMiracleCreation {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.RogueResRole {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.RogueResEvent {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.RogueResComplex {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.RogueResDataType {
            crate::wire::write_tag(out, 1001, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResToken
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResShopToken
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResRoleBuff
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResPhantom
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResMiracleCreation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResEvent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResComplex
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1000 => self.IncId = r.read_varint_i32()?,
            1001 => {
                self.RogueResDataType = Some(
                    <RogueResDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGainDataTotalNotify {
    pub ElementUnits: Vec<ElementUnit>,
    pub RoleBondInfos: Vec<RoleBondInfo>,
    pub RogueResGainDatas: Vec<RogueResGainData>,
}

impl RogueResGainDataTotalNotify {
    pub const MSG_ID: u16 = 19437;
}

impl PbMessage for RogueResGainDataTotalNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ElementUnits {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RoleBondInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.RogueResGainDatas {
            crate::wire::write_submessage(out, 15, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ElementUnits
                    .push(<ElementUnit as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RoleBondInfos
                    .push(<RoleBondInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGainDatas
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGainDataUpdateNotify {
    pub Adds: Vec<RogueResGainData>,
    pub Removes: Vec<i32>,
    pub Updates: Vec<RogueResGainData>,
}

impl RogueResGainDataUpdateNotify {
    pub const MSG_ID: u16 = 29404;
}

impl PbMessage for RogueResGainDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Adds {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Removes {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Updates {
            crate::wire::write_submessage(out, 13, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGlobalInfo {
    pub RogueResTaskUpdateNotify: Option<RogueResTaskUpdateNotify>,
    pub Dict: BTreeMap<i32, SignState>,
}

impl PbMessage for RogueResGlobalInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResTaskUpdateNotify {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.Dict.is_empty() {
            crate::wire::write_map(out, 2, &self.Dict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResTaskUpdateNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_map_entry(r, &mut self.Dict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGotoNextRoomRequest {}

impl RogueResGotoNextRoomRequest {
    pub const MSG_ID: u16 = 22010;
}

impl PbMessage for RogueResGotoNextRoomRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGotoNextRoomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResGotoNextRoomResponse {
    pub const MSG_ID: u16 = 16110;
}

impl PbMessage for RogueResGotoNextRoomResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGridData {
    pub ConfigId: i32,
    pub IsExplore: bool,
    pub HasVision: bool,
    pub IsBlock: bool,
    pub MapType: i32,
    pub Cost: i32,
    pub EventId: i32,
    pub RecommondLevel: i32,
    pub CaptureEffects: Vec<i32>,
    pub EventCost: i32,
    pub Pass: bool,
    pub PreAward: Vec<i32>,
    pub CondInfo: Option<CondInfo>,
    pub RecommondLevelRange: i32,
    pub CanSkipBattle: bool,
    pub SkipBattleLv: i32,
}

impl PbMessage for RogueResGridData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsExplore {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HasVision {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsBlock {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MapType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapType);
        }
        if self.Cost != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cost);
        }
        if self.EventId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
        }
        if self.RecommondLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommondLevel);
        }
        for item in &self.CaptureEffects {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EventCost != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventCost);
        }
        if self.Pass {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.PreAward {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.CondInfo {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.RecommondLevelRange != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommondLevelRange);
        }
        if self.CanSkipBattle {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SkipBattleLv != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkipBattleLv);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IsExplore = r.read_bool()?,
            3 => self.HasVision = r.read_bool()?,
            4 => self.IsBlock = r.read_bool()?,
            5 => self.MapType = r.read_varint_i32()?,
            6 => self.Cost = r.read_varint_i32()?,
            7 => self.EventId = r.read_varint_i32()?,
            8 => self.RecommondLevel = r.read_varint_i32()?,
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CaptureEffects, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            10 => self.EventCost = r.read_varint_i32()?,
            11 => self.Pass = r.read_bool()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreAward, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.CondInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.RecommondLevelRange = r.read_varint_i32()?,
            15 => self.CanSkipBattle = r.read_bool()?,
            16 => self.SkipBattleLv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGridDataUpdateNotify {
    pub RogueResGridDataDict: BTreeMap<i32, RogueResGridData>,
    pub Reason: Option<GridDataUpdateReason>,
    pub LightBlockByLocation: Option<LightBlockByLocationParam>,
    pub PlayerIndex: i32,
}

impl RogueResGridDataUpdateNotify {
    pub const MSG_ID: u16 = 28389;
}

impl PbMessage for RogueResGridDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RogueResGridDataDict.is_empty() {
            crate::wire::write_map(out, 1, &self.RogueResGridDataDict);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.LightBlockByLocation {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.PlayerIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.RogueResGridDataDict)?,
            3 => {
                self.Reason = Some(
                    <GridDataUpdateReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.LightBlockByLocation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.PlayerIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGridEventRequest {
    pub OptionIndex: i32,
    pub GridIndex: i32,
}

impl RogueResGridEventRequest {
    pub const MSG_ID: u16 = 16464;
}

impl PbMessage for RogueResGridEventRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OptionIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OptionIndex);
        }
        if self.GridIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.OptionIndex = r.read_varint_i32()?,
            14 => self.GridIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResGridEventResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RogueResGridEventResponse {
    pub const MSG_ID: u16 = 28230;
}

impl PbMessage for RogueResGridEventResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResIllustrationAwardRequest {
    pub ConfigIds: Vec<i32>,
}

impl RogueResIllustrationAwardRequest {
    pub const MSG_ID: u16 = 19081;
}

impl PbMessage for RogueResIllustrationAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConfigIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ConfigIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResIllustrationAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IsFull: bool,
}

impl RogueResIllustrationAwardResponse {
    pub const MSG_ID: u16 = 24417;
}

impl PbMessage for RogueResIllustrationAwardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsFull {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.IsFull = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResIllustrationUpdateNotify {
    pub Dict: BTreeMap<i32, SignState>,
}

impl RogueResIllustrationUpdateNotify {
    pub const MSG_ID: u16 = 24683;
}

impl PbMessage for RogueResIllustrationUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Dict.is_empty() {
            crate::wire::write_map(out, 9, &self.Dict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => crate::wire::decode_map_entry(r, &mut self.Dict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstGlobalDataNotify {
    pub Fov: i32,
    pub MinMood: i32,
    pub Rows: i32,
    pub RogueResGridDatas: Vec<RogueResGridData>,
    pub InBattle: bool,
    pub GoldItem: i32,
    pub MaxMood: i32,
    pub ShowType: i32,
    pub Index: i32,
    pub RoleLevel: i32,
    pub Columns: i32,
    pub RandomSeedId: i32,
    pub Mood: i32,
    pub RogueResGainDataTotalNotify: Option<RogueResGainDataTotalNotify>,
    pub MoodRuleId: i32,
    pub RoleMaxStar: i32,
    pub TeamLevel: i32,
    pub InstId: i32,
    pub RogueResInstOptionsNotify: Option<RogueResInstOptionsNotify>,
    pub RogueResOpDataNotify: Option<RogueResOpDataNotify>,
    pub RogueResFormations: Vec<RogueResFormation>,
}

impl RogueResInstGlobalDataNotify {
    pub const MSG_ID: u16 = 25331;
}

impl PbMessage for RogueResInstGlobalDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Fov != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Fov);
        }
        if self.MinMood != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinMood);
        }
        if self.Rows != 0 {
            crate::wire::write_tag(out, 51, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rows);
        }
        for item in &self.RogueResGridDatas {
            crate::wire::write_submessage(out, 53, item);
        }
        if self.InBattle {
            crate::wire::write_tag(out, 56, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.GoldItem != 0 {
            crate::wire::write_tag(out, 59, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoldItem);
        }
        if self.MaxMood != 0 {
            crate::wire::write_tag(out, 90, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxMood);
        }
        if self.ShowType != 0 {
            crate::wire::write_tag(out, 93, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowType);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 95, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 96, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.Columns != 0 {
            crate::wire::write_tag(out, 108, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Columns);
        }
        if self.RandomSeedId != 0 {
            crate::wire::write_tag(out, 123, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RandomSeedId);
        }
        if self.Mood != 0 {
            crate::wire::write_tag(out, 126, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Mood);
        }
        if let Some(v) = &self.RogueResGainDataTotalNotify {
            crate::wire::write_submessage(out, 129, v);
        }
        if self.MoodRuleId != 0 {
            crate::wire::write_tag(out, 149, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoodRuleId);
        }
        if self.RoleMaxStar != 0 {
            crate::wire::write_tag(out, 168, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleMaxStar);
        }
        if self.TeamLevel != 0 {
            crate::wire::write_tag(out, 185, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeamLevel);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 197, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if let Some(v) = &self.RogueResInstOptionsNotify {
            crate::wire::write_submessage(out, 204, v);
        }
        if let Some(v) = &self.RogueResOpDataNotify {
            crate::wire::write_submessage(out, 209, v);
        }
        for item in &self.RogueResFormations {
            crate::wire::write_submessage(out, 236, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Fov = r.read_varint_i32()?,
            10 => self.MinMood = r.read_varint_i32()?,
            51 => self.Rows = r.read_varint_i32()?,
            53 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGridDatas
                    .push(<RogueResGridData as PbMessage>::decode(bytes)?);
            }
            56 => self.InBattle = r.read_bool()?,
            59 => self.GoldItem = r.read_varint_i32()?,
            90 => self.MaxMood = r.read_varint_i32()?,
            93 => self.ShowType = r.read_varint_i32()?,
            95 => self.Index = r.read_varint_i32()?,
            96 => self.RoleLevel = r.read_varint_i32()?,
            108 => self.Columns = r.read_varint_i32()?,
            123 => self.RandomSeedId = r.read_varint_i32()?,
            126 => self.Mood = r.read_varint_i32()?,
            129 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGainDataTotalNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            149 => self.MoodRuleId = r.read_varint_i32()?,
            168 => self.RoleMaxStar = r.read_varint_i32()?,
            185 => self.TeamLevel = r.read_varint_i32()?,
            197 => self.InstId = r.read_varint_i32()?,
            204 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResInstOptionsNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            209 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResOpDataNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            236 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResFormations
                    .push(<RogueResFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstOptionsNotify {
    pub RogueResOptionDict: BTreeMap<i32, RogueResOption>,
}

impl RogueResInstOptionsNotify {
    pub const MSG_ID: u16 = 16744;
}

impl PbMessage for RogueResInstOptionsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RogueResOptionDict.is_empty() {
            crate::wire::write_map(out, 10, &self.RogueResOptionDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_map_entry(r, &mut self.RogueResOptionDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstOptionsUpdateNotify {
    pub Removes: Vec<i32>,
    pub Updates: BTreeMap<i32, RogueResOption>,
    pub Adds: BTreeMap<i32, RogueResOption>,
}

impl RogueResInstOptionsUpdateNotify {
    pub const MSG_ID: u16 = 28509;
}

impl PbMessage for RogueResInstOptionsUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Removes {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.Updates.is_empty() {
            crate::wire::write_map(out, 6, &self.Updates);
        }
        if !self.Adds.is_empty() {
            crate::wire::write_map(out, 9, &self.Adds);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => crate::wire::decode_map_entry(r, &mut self.Updates)?,
            9 => crate::wire::decode_map_entry(r, &mut self.Adds)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstResultRequest {}

impl RogueResInstResultRequest {
    pub const MSG_ID: u16 = 22518;
}

impl PbMessage for RogueResInstResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstResultResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub InstResultView: Option<InstResultView>,
}

impl RogueResInstResultResponse {
    pub const MSG_ID: u16 = 28357;
}

impl PbMessage for RogueResInstResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InstResultView {
            crate::wire::write_submessage(out, 13, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.InstResultView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstStartRequest {
    pub InstId: i32,
}

impl RogueResInstStartRequest {
    pub const MSG_ID: u16 = 21018;
}

impl PbMessage for RogueResInstStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResInstStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResInstStartResponse {
    pub const MSG_ID: u16 = 29103;
}

impl PbMessage for RogueResInstStartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResLastInstInfoRequest {}

impl RogueResLastInstInfoRequest {
    pub const MSG_ID: u16 = 29790;
}

impl PbMessage for RogueResLastInstInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {}

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResLastInstInfoResponse {
    pub InstId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResLastInstInfoResponse {
    pub const MSG_ID: u16 = 18335;
}

impl PbMessage for RogueResLastInstInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.InstId = r.read_varint_i32()?,
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResMiracleCreation {
    pub ConfigId: i32,
    pub IsNew: bool,
}

impl PbMessage for RogueResMiracleCreation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IsNew = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResMoodRangeUpdateNotify {
    pub MaxMood: i32,
    pub MinMood: i32,
}

impl RogueResMoodRangeUpdateNotify {
    pub const MSG_ID: u16 = 19251;
}

impl PbMessage for RogueResMoodRangeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxMood != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxMood);
        }
        if self.MinMood != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinMood);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.MaxMood = r.read_varint_i32()?,
            9 => self.MinMood = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResMoodUpdateNotify {
    pub IgnorePerformance: bool,
    pub Mood: i32,
    pub MoodRuleId: i32,
    pub EventType: i32,
}

impl RogueResMoodUpdateNotify {
    pub const MSG_ID: u16 = 26318;
}

impl PbMessage for RogueResMoodUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IgnorePerformance {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Mood != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Mood);
        }
        if self.MoodRuleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoodRuleId);
        }
        if self.EventType != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IgnorePerformance = r.read_bool()?,
            4 => self.Mood = r.read_varint_i32()?,
            6 => self.MoodRuleId = r.read_varint_i32()?,
            13 => self.EventType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResMoveRequest {
    pub MoveOp: Option<MoveOp>,
}

impl RogueResMoveRequest {
    pub const MSG_ID: u16 = 26615;
}

impl PbMessage for RogueResMoveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MoveOp {
            crate::wire::write_submessage(out, 2, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MoveOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResMoveResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RogueResMoveResponse {
    pub const MSG_ID: u16 = 15952;
}

impl PbMessage for RogueResMoveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOpData {
    pub MoveOp: Option<MoveOp>,
    pub GridEventOp: Option<GridEventOp>,
    pub SelectViewOp: Option<SelectViewOp>,
    pub ShowViewOp: Option<ShowViewOp>,
    pub RogueGotoLevelPlayOp: Option<RogueGotoLevelPlayOp>,
    pub RollBuffBondLinkIdOp: Option<RollBuffBondLinkIdOp>,
    pub FallbackView: Option<FallbackViewOp>,
    pub LightBlockByLocationOp: Option<LightBlockByLocationOp>,
    pub MapTeleportByLocationOp: Option<MapTeleportByLocationOp>,
    pub ChangeEventByPosOp: Option<ChangeEventByPosOp>,
    pub IncId: i32,
    pub RogueResOpType: Option<RogueResOpType>,
}

impl PbMessage for RogueResOpData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MoveOp {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.GridEventOp {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.SelectViewOp {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.ShowViewOp {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.RogueGotoLevelPlayOp {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.RollBuffBondLinkIdOp {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.FallbackView {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.LightBlockByLocationOp {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.MapTeleportByLocationOp {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.ChangeEventByPosOp {
            crate::wire::write_submessage(out, 12, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.RogueResOpType {
            crate::wire::write_tag(out, 1001, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.MoveOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GridEventOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SelectViewOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ShowViewOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RogueGotoLevelPlayOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RollBuffBondLinkIdOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.FallbackView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.LightBlockByLocationOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.MapTeleportByLocationOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ChangeEventByPosOp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1000 => self.IncId = r.read_varint_i32()?,
            1001 => {
                self.RogueResOpType = Some(
                    <RogueResOpType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOpDataNotify {
    pub RogueResOpDatas: Vec<RogueResOpData>,
}

impl RogueResOpDataNotify {
    pub const MSG_ID: u16 = 27709;
}

impl PbMessage for RogueResOpDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueResOpDatas {
            crate::wire::write_submessage(out, 6, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResOpDatas
                    .push(<RogueResOpData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOpDataUpdateNotify {
    pub Updates: Vec<RogueResOpData>,
    pub Adds: Vec<RogueResOpData>,
    pub Removes: Vec<i32>,
}

impl RogueResOpDataUpdateNotify {
    pub const MSG_ID: u16 = 29300;
}

impl PbMessage for RogueResOpDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Updates {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.Adds {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.Removes {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<RogueResOpData as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<RogueResOpData as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOpDoneRequest {
    pub IncId: i32,
    pub ClientSelectId: i32,
    pub ClientSelectIds: Vec<i32>,
}

impl RogueResOpDoneRequest {
    pub const MSG_ID: u16 = 27894;
}

impl PbMessage for RogueResOpDoneRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ClientSelectId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientSelectId);
        }
        for item in &self.ClientSelectIds {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.IncId = r.read_varint_i32()?,
            12 => self.ClientSelectId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ClientSelectIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOpDoneResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RogueResOpDoneResponse {
    pub const MSG_ID: u16 = 16584;
}

impl PbMessage for RogueResOpDoneResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResOption {
    pub RogueResGainDatas: Vec<RogueResGainData>,
    pub RogueResDataType: Option<RogueResDataType>,
    pub IsSelect: bool,
    pub CanGiveUp: bool,
    pub CanMultiSelect: bool,
    pub MaxSelectCount: i32,
    pub UseSelectCount: i32,
    pub MaxRefreshCount: i32,
    pub UseRefreshCount: i32,
    pub RefreshItem: i32,
    pub RefreshCost: i32,
    pub RoleId: i32,
}

impl PbMessage for RogueResOption {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueResGainDatas {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.RogueResDataType {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 1001, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanGiveUp {
            crate::wire::write_tag(out, 1002, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanMultiSelect {
            crate::wire::write_tag(out, 1003, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxSelectCount != 0 {
            crate::wire::write_tag(out, 1004, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxSelectCount);
        }
        if self.UseSelectCount != 0 {
            crate::wire::write_tag(out, 1005, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseSelectCount);
        }
        if self.MaxRefreshCount != 0 {
            crate::wire::write_tag(out, 1006, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxRefreshCount);
        }
        if self.UseRefreshCount != 0 {
            crate::wire::write_tag(out, 1007, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseRefreshCount);
        }
        if self.RefreshItem != 0 {
            crate::wire::write_tag(out, 1008, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshItem);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 1009, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCost);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1010, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGainDatas
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            1000 => {
                self.RogueResDataType = Some(
                    <RogueResDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            1001 => self.IsSelect = r.read_bool()?,
            1002 => self.CanGiveUp = r.read_bool()?,
            1003 => self.CanMultiSelect = r.read_bool()?,
            1004 => self.MaxSelectCount = r.read_varint_i32()?,
            1005 => self.UseSelectCount = r.read_varint_i32()?,
            1006 => self.MaxRefreshCount = r.read_varint_i32()?,
            1007 => self.UseRefreshCount = r.read_varint_i32()?,
            1008 => self.RefreshItem = r.read_varint_i32()?,
            1009 => self.RefreshCost = r.read_varint_i32()?,
            1010 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResPhantom {
    pub ConfigId: i32,
    pub IsNew: bool,
    pub PhantomAffixInfos: Vec<PhantomAffixInfo>,
}

impl PbMessage for RogueResPhantom {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.PhantomAffixInfos {
            crate::wire::write_submessage(out, 3, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IsNew = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomAffixInfos
                    .push(<PhantomAffixInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResPlayerStatusUpdateNotify {
    pub InBattle: bool,
}

impl RogueResPlayerStatusUpdateNotify {
    pub const MSG_ID: u16 = 23230;
}

impl PbMessage for RogueResPlayerStatusUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InBattle {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResRole {
    pub IsNew: bool,
    pub Level: i32,
    pub RoleAffixs: Vec<i32>,
    pub SourcePrice: i32,
    pub CurPrice: i32,
    pub ItemId: i32,
    pub IsSell: bool,
    pub Index: i32,
    pub RoleIdOrTrialRoleId: i32,
}

impl PbMessage for RogueResRole {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNew {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.RoleAffixs {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SourcePrice != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourcePrice);
        }
        if self.CurPrice != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPrice);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IsSell {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIdOrTrialRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.IsNew = r.read_bool()?,
            3 => self.Level = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleAffixs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => self.SourcePrice = r.read_varint_i32()?,
            6 => self.CurPrice = r.read_varint_i32()?,
            7 => self.ItemId = r.read_varint_i32()?,
            8 => self.IsSell = r.read_bool()?,
            9 => self.Index = r.read_varint_i32()?,
            10 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
