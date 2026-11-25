use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyPlayData {
    pub HasRecord: bool,
}

impl PbMessage for RogueWeeklyPlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasRecord {
            crate::wire::write_tag(out, 1, WireType::Varint);
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
            1 => self.HasRecord = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroFarmPlayData {
    pub HasRecord: bool,
}

impl PbMessage for FloroFarmPlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasRecord {
            crate::wire::write_tag(out, 1, WireType::Varint);
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
            1 => self.HasRecord = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LobbyListRequest {
    pub IsFriend: bool,
}

impl LobbyListRequest {
    pub const MSG_ID: u16 = 28398;
}

impl PbMessage for LobbyListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFriend {
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
            2 => self.IsFriend = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LobbyListResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemList: Vec<PlayerDetails>,
}

impl LobbyListResponse {
    pub const MSG_ID: u16 = 18705;
}

impl PbMessage for LobbyListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ItemList {
            crate::wire::write_submessage(out, 8, item);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<PlayerDetails as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowSeqSetPlayerPosRequest {
    pub TalkId: i32,
    pub Z: f32,
    pub TalkActionId: i32,
    pub FlowIncId: i64,
    pub X: f32,
    pub Y: f32,
    pub A: f32,
}

impl FlowSeqSetPlayerPosRequest {
    pub const MSG_ID: u16 = 29398;
}

impl PbMessage for FlowSeqSetPlayerPosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalkId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalkId);
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        if self.TalkActionId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalkActionId);
        }
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
        if self.X != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.A != 0.0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.A).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TalkId = r.read_varint_i32()?,
            3 => self.Z = r.read_f32()?,
            6 => self.TalkActionId = r.read_varint_i32()?,
            7 => self.FlowIncId = r.read_varint_i64()?,
            12 => self.X = r.read_f32()?,
            13 => self.Y = r.read_f32()?,
            15 => self.A = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowSeqSetPlayerPosResponse {
    pub Code: Option<ErrorCode>,
}

impl FlowSeqSetPlayerPosResponse {
    pub const MSG_ID: u16 = 16827;
}

impl PbMessage for FlowSeqSetPlayerPosResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct FinishDownloadQuestResourceRequest {
    pub QuestIds: Vec<i32>,
}

impl FinishDownloadQuestResourceRequest {
    pub const MSG_ID: u16 = 21668;
}

impl PbMessage for FinishDownloadQuestResourceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FinishDownloadQuestResourceResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl FinishDownloadQuestResourceResponse {
    pub const MSG_ID: u16 = 19405;
}

impl PbMessage for FinishDownloadQuestResourceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct SetQuestResourceStateRequest {
    pub QuestResourceState: i32,
}

impl SetQuestResourceStateRequest {
    pub const MSG_ID: u16 = 23014;
}

impl PbMessage for SetQuestResourceStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestResourceState != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestResourceState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.QuestResourceState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetQuestResourceStateResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl SetQuestResourceStateResponse {
    pub const MSG_ID: u16 = 27184;
}

impl PbMessage for SetQuestResourceStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct DetectionTargetData {
    pub MapId: i32,
    pub TargetIds: Vec<i32>,
}

impl PbMessage for DetectionTargetData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        for item in &self.TargetIds {
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
            1 => self.MapId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetDetectionTargetRequest {
    pub DetectionId: i32,
    pub Type: Option<DetectionType>,
}

impl GetDetectionTargetRequest {
    pub const MSG_ID: u16 = 20959;
}

impl PbMessage for GetDetectionTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionId);
        }
        if let Some(v) = &self.Type {
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
            6 => self.DetectionId = r.read_varint_i32()?,
            10 => {
                self.Type = Some(<DetectionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetDetectionTargetResponse {
    pub Type: Option<DetectionType>,
    pub Code: Option<ErrorCode>,
    pub DetectionId: i32,
    pub DetectionTargetDatas: Vec<DetectionTargetData>,
}

impl GetDetectionTargetResponse {
    pub const MSG_ID: u16 = 24853;
}

impl PbMessage for GetDetectionTargetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.DetectionId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionId);
        }
        for item in &self.DetectionTargetDatas {
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
            1 => {
                self.Type = Some(<DetectionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.DetectionId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionTargetDatas
                    .push(<DetectionTargetData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OutputDebugInfoRequest {
    pub ClientInfo: Vec<u8>,
}

impl OutputDebugInfoRequest {
    pub const MSG_ID: u16 = 20551;
}

impl PbMessage for OutputDebugInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ClientInfo.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientInfo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.ClientInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OutputDebugInfoResponse {}

impl OutputDebugInfoResponse {
    pub const MSG_ID: u16 = 22307;
}

impl PbMessage for OutputDebugInfoResponse {
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
pub struct InvalidItemRequest {}

impl InvalidItemRequest {
    pub const MSG_ID: u16 = 17597;
}

impl PbMessage for InvalidItemRequest {
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
pub struct InvalidItemResponse {
    pub HasInvalidItem: bool,
    pub HasExpireConvertItem: bool,
}

impl InvalidItemResponse {
    pub const MSG_ID: u16 = 25982;
}

impl PbMessage for InvalidItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasInvalidItem {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HasExpireConvertItem {
            crate::wire::write_tag(out, 14, WireType::Varint);
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
            12 => self.HasInvalidItem = r.read_bool()?,
            14 => self.HasExpireConvertItem = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShowInvalidItemRequest {}

impl ShowInvalidItemRequest {
    pub const MSG_ID: u16 = 22706;
}

impl PbMessage for ShowInvalidItemRequest {
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
pub struct InvalidItem {
    pub ItemId: i32,
    pub Count: i32,
    pub IncrId: i32,
}

impl PbMessage for InvalidItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            3 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpireConvertTargetItemInfo {
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for ExpireConvertTargetItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
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
            2 => self.ItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExpireConvertItemInfo {
    pub ItemId: i32,
    pub ItemCount: i32,
    pub ExpireConvertTargetItemInfos: Vec<ExpireConvertTargetItemInfo>,
}

impl PbMessage for ExpireConvertItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        for item in &self.ExpireConvertTargetItemInfos {
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.ItemCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ExpireConvertTargetItemInfos
                    .push(<ExpireConvertTargetItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShowInvalidItemResponse {
    pub InvalidItemList: Vec<InvalidItem>,
    pub ExpireConvertItemInfoList: Vec<ExpireConvertItemInfo>,
}

impl ShowInvalidItemResponse {
    pub const MSG_ID: u16 = 17796;
}

impl PbMessage for ShowInvalidItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InvalidItemList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ExpireConvertItemInfoList {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.InvalidItemList
                    .push(<InvalidItem as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ExpireConvertItemInfoList
                    .push(<ExpireConvertItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GoogleGameStatsShow {
    pub TeleportNumber: i32,
    pub BoxOpenNumber: i32,
    pub ActiveRoleNumber: i32,
    pub ActiveDays: i32,
    pub FetterUnLockMonsters: i32,
}

impl PbMessage for GoogleGameStatsShow {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TeleportNumber != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeleportNumber);
        }
        if self.BoxOpenNumber != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxOpenNumber);
        }
        if self.ActiveRoleNumber != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveRoleNumber);
        }
        if self.ActiveDays != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveDays);
        }
        if self.FetterUnLockMonsters != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FetterUnLockMonsters);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TeleportNumber = r.read_varint_i32()?,
            2 => self.BoxOpenNumber = r.read_varint_i32()?,
            3 => self.ActiveRoleNumber = r.read_varint_i32()?,
            4 => self.ActiveDays = r.read_varint_i32()?,
            5 => self.FetterUnLockMonsters = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameStatsShowRequest {}

impl GameStatsShowRequest {
    pub const MSG_ID: u16 = 16656;
}

impl PbMessage for GameStatsShowRequest {
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
pub struct GameStatsShowResponse {
    pub GoogleGameStatsShow: Option<GoogleGameStatsShow>,
}

impl GameStatsShowResponse {
    pub const MSG_ID: u16 = 18943;
}

impl PbMessage for GameStatsShowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GoogleGameStatsShow {
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
                self.GoogleGameStatsShow
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThirdPartyShareRequest {
    pub SharedId: i32,
}

impl ThirdPartyShareRequest {
    pub const MSG_ID: u16 = 17006;
}

impl PbMessage for ThirdPartyShareRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SharedId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SharedId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.SharedId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThirdPartyShareResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThirdPartyShareResponse {
    pub const MSG_ID: u16 = 20634;
}

impl PbMessage for ThirdPartyShareResponse {
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
pub struct LevelPlayGainRewardInfo {
    pub RewardNodeIds: Vec<i32>,
    pub TreasureBoxIds: Vec<i32>,
}

impl PbMessage for LevelPlayGainRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardNodeIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TreasureBoxIds {
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
            1 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardNodeIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TreasureBoxIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstLevelPlayStateReq {
    pub InstId: i32,
    pub LevelPlayIds: Vec<i32>,
}

impl PbMessage for InstLevelPlayStateReq {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.LevelPlayIds {
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LevelPlayIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstLevelPlayStateRsp {
    pub InstId: i32,
    pub LevelPlayStateDict: BTreeMap<i32, i32>,
    pub CompleteNumberDict: BTreeMap<i32, i32>,
    pub HideDict: BTreeMap<i32, bool>,
    pub HideGroupInfoDict: BTreeMap<i32, Vec<u8>>,
    pub IsUnlockDict: BTreeMap<i32, bool>,
    pub GainRewardInfoDict: BTreeMap<i32, LevelPlayGainRewardInfo>,
}

impl PbMessage for InstLevelPlayStateRsp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if !self.LevelPlayStateDict.is_empty() {
            crate::wire::write_map(out, 2, &self.LevelPlayStateDict);
        }
        if !self.CompleteNumberDict.is_empty() {
            crate::wire::write_map(out, 3, &self.CompleteNumberDict);
        }
        if !self.HideDict.is_empty() {
            crate::wire::write_map(out, 4, &self.HideDict);
        }
        if !self.HideGroupInfoDict.is_empty() {
            crate::wire::write_map(out, 5, &self.HideGroupInfoDict);
        }
        if !self.IsUnlockDict.is_empty() {
            crate::wire::write_map(out, 6, &self.IsUnlockDict);
        }
        if !self.GainRewardInfoDict.is_empty() {
            crate::wire::write_map(out, 7, &self.GainRewardInfoDict);
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
            2 => crate::wire::decode_map_entry(r, &mut self.LevelPlayStateDict)?,
            3 => crate::wire::decode_map_entry(r, &mut self.CompleteNumberDict)?,
            4 => crate::wire::decode_map_entry(r, &mut self.HideDict)?,
            5 => crate::wire::decode_map_entry(r, &mut self.HideGroupInfoDict)?,
            6 => crate::wire::decode_map_entry(r, &mut self.IsUnlockDict)?,
            7 => crate::wire::decode_map_entry(r, &mut self.GainRewardInfoDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayStateListRequest {
    pub InstLevelPlayStateReqs: Vec<InstLevelPlayStateReq>,
}

impl LevelPlayStateListRequest {
    pub const MSG_ID: u16 = 17858;
}

impl PbMessage for LevelPlayStateListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstLevelPlayStateReqs {
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
                self.InstLevelPlayStateReqs
                    .push(<InstLevelPlayStateReq as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayStateListResponse {
    pub InstLevelPlayStateRsps: Vec<InstLevelPlayStateRsp>,
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlayStateListResponse {
    pub const MSG_ID: u16 = 24664;
}

impl PbMessage for LevelPlayStateListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstLevelPlayStateRsps {
            crate::wire::write_submessage(out, 2, item);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.InstLevelPlayStateRsps
                    .push(<InstLevelPlayStateRsp as PbMessage>::decode(bytes)?);
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
pub struct IllustratedUnlockRequest {
    pub Type: Option<IllustratedType>,
    pub Id: i32,
}

impl IllustratedUnlockRequest {
    pub const MSG_ID: u16 = 19831;
}

impl PbMessage for IllustratedUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => {
                self.Type = Some(
                    <IllustratedType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedUnlockResponse {
    pub Entry: Option<IllustratedEntry>,
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl IllustratedUnlockResponse {
    pub const MSG_ID: u16 = 20078;
}

impl PbMessage for IllustratedUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Entry {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Entry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisLevelInfo {
    pub Level: i32,
    pub TotalProficiency: i32,
}

impl PbMessage for SynthesisLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.TotalProficiency != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalProficiency);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Level = r.read_varint_i32()?,
            2 => self.TotalProficiency = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisInfoRequest {}

impl SynthesisInfoRequest {
    pub const MSG_ID: u16 = 25534;
}

impl PbMessage for SynthesisInfoRequest {
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
pub struct OneSynthesisConfig {
    pub Id: i32,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for OneSynthesisConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.LimitBeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitBeginTime);
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
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
            2 => self.LimitBeginTime = r.read_varint_i64()?,
            3 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneSynthesisInfo {
    pub Id: i32,
    pub Count: i32,
    pub LastRoleId: i32,
    pub LimitCount: i32,
    pub LimitSynthesisCount: i32,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for OneSynthesisInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.LastRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRoleId);
        }
        if self.LimitCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitCount);
        }
        if self.LimitSynthesisCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitSynthesisCount);
        }
        if self.LimitBeginTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitBeginTime);
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
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
            3 => self.LastRoleId = r.read_varint_i32()?,
            4 => self.LimitCount = r.read_varint_i32()?,
            5 => self.LimitSynthesisCount = r.read_varint_i32()?,
            6 => self.LimitBeginTime = r.read_varint_i64()?,
            7 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisInfoResponse {
    pub LevelInfo: Option<SynthesisLevelInfo>,
    pub LimitRefreshTime: i64,
    pub SynthesisConfigs: Vec<OneSynthesisConfig>,
    pub Code: Option<ErrorCode>,
    pub SynthesisInfoList: Vec<OneSynthesisInfo>,
}

impl SynthesisInfoResponse {
    pub const MSG_ID: u16 = 19842;
}

impl PbMessage for SynthesisInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LevelInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.LimitRefreshTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitRefreshTime);
        }
        for item in &self.SynthesisConfigs {
            crate::wire::write_submessage(out, 8, item);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SynthesisInfoList {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.LevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.LimitRefreshTime = r.read_varint_i64()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SynthesisConfigs
                    .push(<OneSynthesisConfig as PbMessage>::decode(bytes)?);
            }
            9 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SynthesisInfoList
                    .push(<OneSynthesisInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolStartRequest {
    pub Dir: bool,
    pub EntityId: i64,
}

impl EntityPatrolStartRequest {
    pub const MSG_ID: u16 = 26715;
}

impl PbMessage for EntityPatrolStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Dir {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            11 => self.Dir = r.read_bool()?,
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolStartResponse {
    pub Code: Option<ErrorCode>,
}

impl EntityPatrolStartResponse {
    pub const MSG_ID: u16 = 19013;
}

impl PbMessage for EntityPatrolStartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct LandingDamageRequest {
    pub TimeExceeding: i32,
    pub EntityId: i64,
    pub SpeedZ: f32,
}

impl LandingDamageRequest {
    pub const MSG_ID: u16 = 27510;
}

impl PbMessage for LandingDamageRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeExceeding != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeExceeding);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.SpeedZ != 0.0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedZ).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.TimeExceeding = r.read_varint_i32()?,
            8 => self.EntityId = r.read_varint_i64()?,
            11 => self.SpeedZ = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LandingDamageResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LandingDamageResponse {
    pub const MSG_ID: u16 = 27519;
}

impl PbMessage for LandingDamageResponse {
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
pub struct SceneBlockChangePush {
    pub DownloadBlockId: Vec<i32>,
    pub DeleteBlockId: Vec<i32>,
}

impl SceneBlockChangePush {
    pub const MSG_ID: u16 = 18483;
}

impl PbMessage for SceneBlockChangePush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DownloadBlockId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DeleteBlockId {
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
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DownloadBlockId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DeleteBlockId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapOpenPush {
    pub OpenType: Option<MapOpenType>,
}

impl MapOpenPush {
    pub const MSG_ID: u16 = 19992;
}

impl PbMessage for MapOpenPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OpenType {
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
                self.OpenType = Some(<MapOpenType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AceAntiDataPush {
    pub AntiData: Vec<u8>,
}

impl AceAntiDataPush {
    pub const MSG_ID: u16 = 21559;
}

impl PbMessage for AceAntiDataPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AntiData.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AntiData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.AntiData = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitClientLoginPush {
    pub DownloadBlockId: Vec<i32>,
}

impl SceneBlockSplitClientLoginPush {
    pub const MSG_ID: u16 = 24812;
}

impl PbMessage for SceneBlockSplitClientLoginPush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DownloadBlockId {
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DownloadBlockId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitSwitchStatePush {
    pub SwitchState: Option<ESceneBlockSwithState>,
}

impl SceneBlockSplitSwitchStatePush {
    pub const MSG_ID: u16 = 29772;
}

impl PbMessage for SceneBlockSplitSwitchStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SwitchState {
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
                self.SwitchState = Some(
                    <ESceneBlockSwithState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct SceneChangeDataLayerNotify {
    pub SequencePath: Vec<u8>,
    pub MaterialDataForUnloadLayers: Vec<u8>,
    pub UnloadDataLayers: Vec<i32>,
    pub MaterialDataForLoadedLayers: Vec<u8>,
    pub EntityId: i32,
    pub SeqMarkBeforeModifyMat: Vec<u8>,
    pub LoadDataLayers: Vec<i32>,
}

impl SceneChangeDataLayerNotify {
    pub const MSG_ID: u16 = 15688;
}

impl PbMessage for SceneChangeDataLayerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SequencePath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SequencePath);
        }
        if !self.MaterialDataForUnloadLayers.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MaterialDataForUnloadLayers);
        }
        for item in &self.UnloadDataLayers {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.MaterialDataForLoadedLayers.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MaterialDataForLoadedLayers);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        if !self.SeqMarkBeforeModifyMat.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SeqMarkBeforeModifyMat);
        }
        for item in &self.LoadDataLayers {
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
            1 => self.SequencePath = r.read_len_delimited()?.to_vec(),
            2 => self.MaterialDataForUnloadLayers = r.read_len_delimited()?.to_vec(),
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnloadDataLayers,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.MaterialDataForLoadedLayers = r.read_len_delimited()?.to_vec(),
            8 => self.EntityId = r.read_varint_i32()?,
            10 => self.SeqMarkBeforeModifyMat = r.read_len_delimited()?.to_vec(),
            15 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LoadDataLayers, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeDataLayerFinishRequest {
    pub InstId: i32,
}

impl ChangeDataLayerFinishRequest {
    pub const MSG_ID: u16 = 16648;
}

impl PbMessage for ChangeDataLayerFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeDataLayerFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeDataLayerFinishResponse {
    pub const MSG_ID: u16 = 20475;
}

impl PbMessage for ChangeDataLayerFinishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
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
pub struct SceneSubLevelsNotify {
    pub CurHideLoaded: Vec<Vec<u8>>,
    pub CurLoaded: Vec<Vec<u8>>,
}

impl SceneSubLevelsNotify {
    pub const MSG_ID: u16 = 21860;
}

impl PbMessage for SceneSubLevelsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CurHideLoaded {
            crate::wire::write_tag(out, 5, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.CurLoaded {
            crate::wire::write_tag(out, 12, crate::wire::WireType::LengthDelimited);
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
            5 => self.CurHideLoaded.push(r.read_len_delimited()?.to_vec()),
            12 => self.CurLoaded.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneSubLevelsChangedNotify {
    pub NewUnloaded: Vec<Vec<u8>>,
    pub HideSubLevels: Vec<Vec<u8>>,
    pub NewLoaded: Vec<Vec<u8>>,
    pub TeleportEntityId: i32,
}

impl SceneSubLevelsChangedNotify {
    pub const MSG_ID: u16 = 22107;
}

impl PbMessage for SceneSubLevelsChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NewUnloaded {
            crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.HideSubLevels {
            crate::wire::write_tag(out, 6, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.NewLoaded {
            crate::wire::write_tag(out, 9, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.TeleportEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeleportEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.NewUnloaded.push(r.read_len_delimited()?.to_vec()),
            6 => self.HideSubLevels.push(r.read_len_delimited()?.to_vec()),
            9 => self.NewLoaded.push(r.read_len_delimited()?.to_vec()),
            15 => self.TeleportEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneSubLevelsRequest {
    pub HideSubLevels: Vec<Vec<u8>>,
    pub ShowSubLevels: Vec<Vec<u8>>,
}

impl SceneSubLevelsRequest {
    pub const MSG_ID: u16 = 16545;
}

impl PbMessage for SceneSubLevelsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HideSubLevels {
            crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.ShowSubLevels {
            crate::wire::write_tag(out, 9, crate::wire::WireType::LengthDelimited);
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
            8 => self.HideSubLevels.push(r.read_len_delimited()?.to_vec()),
            9 => self.ShowSubLevels.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneSubLevelsResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneSubLevelsResponse {
    pub const MSG_ID: u16 = 22498;
}

impl PbMessage for SceneSubLevelsResponse {
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
pub struct SceneRoleInformation {
    pub RoleId: i32,
    pub EntityId: i64,
    pub OnStageOutOfControl: bool,
}

impl PbMessage for SceneRoleInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.OnStageOutOfControl {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.EntityId = r.read_varint_i64()?,
            3 => self.OnStageOutOfControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSceneGroupData {
    pub Group: i32,
    pub FightRoleInfos: Vec<SceneRoleInformation>,
    pub CurRole: i32,
    pub LivingStatus: Option<LivingStatus>,
    pub FixedLocationByCfg: bool,
}

impl PbMessage for PlayerSceneGroupData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Group != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Group);
        }
        for item in &self.FightRoleInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CurRole != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRole);
        }
        if let Some(v) = &self.LivingStatus {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FixedLocationByCfg {
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
            1 => self.Group = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfos
                    .push(<SceneRoleInformation as PbMessage>::decode(bytes)?);
            }
            3 => self.CurRole = r.read_varint_i32()?,
            4 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            5 => self.FixedLocationByCfg = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSceneGroupDataList {
    pub PlayerId: i32,
    pub PlayerSceneGroupData: Vec<PlayerSceneGroupData>,
    pub CurrentGroup: i32,
}

impl PbMessage for PlayerSceneGroupDataList {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.PlayerSceneGroupData {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CurrentGroup != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentGroup);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerSceneGroupData
                    .push(<PlayerSceneGroupData as PbMessage>::decode(bytes)?);
            }
            3 => self.CurrentGroup = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchSceneGroupNotify {
    pub PlayerId: i32,
    pub Group: i32,
}

impl SwitchSceneGroupNotify {
    pub const MSG_ID: u16 = 21909;
}

impl PbMessage for SwitchSceneGroupNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Group != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Group);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.PlayerId = r.read_varint_i32()?,
            15 => self.Group = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateSceneGroupDataNotify {
    pub PlayerSceneGroupDataList: Vec<PlayerSceneGroupDataList>,
}

impl UpdateSceneGroupDataNotify {
    pub const MSG_ID: u16 = 22332;
}

impl PbMessage for UpdateSceneGroupDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerSceneGroupDataList {
            crate::wire::write_submessage(out, 12, item);
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
                let bytes = r.read_len_delimited()?;
                self.PlayerSceneGroupDataList
                    .push(<PlayerSceneGroupDataList as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrialRoleItem {
    pub RoleId: i32,
    pub Show: bool,
}

impl PbMessage for TrialRoleItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Show {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Show = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrialOpenNotify {
    pub ShowTips: bool,
    pub RoleIds: Vec<i32>,
    pub CanEnterDungeonList: Vec<i32>,
    pub TrialRoleItems: Vec<TrialRoleItem>,
    pub EnableMapAndTeleport: bool,
}

impl RoleTrialOpenNotify {
    pub const MSG_ID: u16 = 17675;
}

impl PbMessage for RoleTrialOpenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowTips {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.CanEnterDungeonList {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TrialRoleItems {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.EnableMapAndTeleport {
            crate::wire::write_tag(out, 15, WireType::Varint);
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
            1 => self.ShowTips = r.read_bool()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CanEnterDungeonList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TrialRoleItems
                    .push(<TrialRoleItem as PbMessage>::decode(bytes)?);
            }
            15 => self.EnableMapAndTeleport = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrialCloseNotify {
    pub ShowTips: bool,
    pub EnableMapAndTeleport: bool,
    pub RoleIds: Vec<i32>,
    pub CanEnterDungeonList: Vec<i32>,
}

impl RoleTrialCloseNotify {
    pub const MSG_ID: u16 = 16731;
}

impl PbMessage for RoleTrialCloseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowTips {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EnableMapAndTeleport {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.CanEnterDungeonList {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            2 => self.ShowTips = r.read_bool()?,
            5 => self.EnableMapAndTeleport = r.read_bool()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CanEnterDungeonList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrialSettleNotify {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTrialSettleNotify {
    pub const MSG_ID: u16 = 22045;
}

impl PbMessage for RoleTrialSettleNotify {
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

#[derive(Debug, Clone, PartialEq)]
pub enum EnterInstContextEnterCtxOneof {
    RoleTrialCtx(RoleTrialEnterCtx),
    RoleTeachCtx(RoleTeachEnterCtx),
    AdventureGuideCtx(AdventureGuideCtx),
    RogueWhiteCatCtx(RogueWhiteCatCtx),
    BossRushCtx(BossRushCtx),
    MowTowerCtx(MowTowerCtx),
    RoleSkinTrialEnterCtx(RoleSkinTrialEnterCtx),
    SlashAndTowerCtx(SlashAndTowerCtx),
    BabelTowerCtx(BabelTowerCtx),
    PreOpenDetectionCtx(PreOpenDetectionCtx),
    AbyssInstCtx(AbyssInstCtx),
    DangoMonopolyCtx(DangoMonopolyCtx),
    PhantomBattleEnterCtx(PhantomBattleEnterCtx),
    TrapDefenseCtx(TrapDefenseCtx),
    FunPlayCtx(FunPlayCtx),
    HonamiStoryEnterCtx(HonamiStoryEnterCtx),
    SurvivorsCtx(SurvivorsCtx),
    PhotoFightCtx(PhotoFightCtx),
    NewTowerClimbingCtx(NewTowerClimbingCtx),
    MotorParkourCtx(MotorParkourCtx),
    GachaRoleDevelopInsCtx(GachaRoleDevelopInsCtx),
    MotorFightCtx(MotorFightCtx),
    FlagChallengeCtx(FlagChallengeCtx),
    RhythmCtx(RhythmCtx),
    PinballCtx(PinballCtx),
    BossPilingCtx(BossPilingCtx),
    KurotatoCtx(KurotatoCtx),
    EdgeRunnerLordGymCtx(EdgeRunnerLordGymCtx),
    QuestRecallInstCtx(QuestRecallInstCtx),
    AdventureSilentCtx(AdventureSilentCtx),
    RoverRogueInsCtx(RoverRogueInsCtx),
    DangoBrawlEnterCtx(DangoBrawlEnterCtx),
    DarkTideCtx(DarkTideEnterCtx),
    RecallInstMarkCtx(QuestRecallInstMarkCtx),
    FarmGoldCtx(FarmGoldCtx),
    RoleTowerCtx(RoleTowerCtx),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterInstContext {
    pub EnterCtx: Option<EnterInstContextEnterCtxOneof>,
}

impl PbMessage for EnterInstContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.EnterCtx {
            match oneof {
                EnterInstContextEnterCtxOneof::RoleTrialCtx(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                EnterInstContextEnterCtxOneof::RoleTeachCtx(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                EnterInstContextEnterCtxOneof::AdventureGuideCtx(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                EnterInstContextEnterCtxOneof::RogueWhiteCatCtx(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                EnterInstContextEnterCtxOneof::BossRushCtx(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                EnterInstContextEnterCtxOneof::MowTowerCtx(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                EnterInstContextEnterCtxOneof::RoleSkinTrialEnterCtx(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                EnterInstContextEnterCtxOneof::SlashAndTowerCtx(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                EnterInstContextEnterCtxOneof::BabelTowerCtx(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                EnterInstContextEnterCtxOneof::PreOpenDetectionCtx(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                EnterInstContextEnterCtxOneof::AbyssInstCtx(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                EnterInstContextEnterCtxOneof::DangoMonopolyCtx(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                EnterInstContextEnterCtxOneof::PhantomBattleEnterCtx(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                EnterInstContextEnterCtxOneof::TrapDefenseCtx(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                EnterInstContextEnterCtxOneof::FunPlayCtx(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                EnterInstContextEnterCtxOneof::HonamiStoryEnterCtx(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                EnterInstContextEnterCtxOneof::SurvivorsCtx(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                EnterInstContextEnterCtxOneof::PhotoFightCtx(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                EnterInstContextEnterCtxOneof::NewTowerClimbingCtx(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                EnterInstContextEnterCtxOneof::MotorParkourCtx(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                EnterInstContextEnterCtxOneof::GachaRoleDevelopInsCtx(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                EnterInstContextEnterCtxOneof::MotorFightCtx(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                EnterInstContextEnterCtxOneof::FlagChallengeCtx(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                EnterInstContextEnterCtxOneof::RhythmCtx(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                EnterInstContextEnterCtxOneof::PinballCtx(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                EnterInstContextEnterCtxOneof::BossPilingCtx(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                EnterInstContextEnterCtxOneof::KurotatoCtx(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                EnterInstContextEnterCtxOneof::EdgeRunnerLordGymCtx(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                EnterInstContextEnterCtxOneof::QuestRecallInstCtx(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                EnterInstContextEnterCtxOneof::AdventureSilentCtx(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                EnterInstContextEnterCtxOneof::RoverRogueInsCtx(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                EnterInstContextEnterCtxOneof::DangoBrawlEnterCtx(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                EnterInstContextEnterCtxOneof::DarkTideCtx(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                EnterInstContextEnterCtxOneof::RecallInstMarkCtx(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                EnterInstContextEnterCtxOneof::FarmGoldCtx(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                EnterInstContextEnterCtxOneof::RoleTowerCtx(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
            }
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
                let val = <RoleTrialEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RoleTrialCtx(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTeachEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RoleTeachCtx(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <AdventureGuideCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::AdventureGuideCtx(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <RogueWhiteCatCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RogueWhiteCatCtx(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <BossRushCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::BossRushCtx(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <MowTowerCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::MowTowerCtx(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleSkinTrialEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RoleSkinTrialEnterCtx(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <SlashAndTowerCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::SlashAndTowerCtx(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <BabelTowerCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::BabelTowerCtx(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <PreOpenDetectionCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::PreOpenDetectionCtx(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <AbyssInstCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::AbyssInstCtx(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <DangoMonopolyCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::DangoMonopolyCtx(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhantomBattleEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::PhantomBattleEnterCtx(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::TrapDefenseCtx(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <FunPlayCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::FunPlayCtx(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <HonamiStoryEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::HonamiStoryEnterCtx(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::SurvivorsCtx(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <PhotoFightCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::PhotoFightCtx(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewTowerClimbingCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::NewTowerClimbingCtx(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorParkourCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::MotorParkourCtx(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <GachaRoleDevelopInsCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::GachaRoleDevelopInsCtx(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorFightCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::MotorFightCtx(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <FlagChallengeCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::FlagChallengeCtx(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <RhythmCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RhythmCtx(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <PinballCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::PinballCtx(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <BossPilingCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::BossPilingCtx(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::KurotatoCtx(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <EdgeRunnerLordGymCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::EdgeRunnerLordGymCtx(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestRecallInstCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::QuestRecallInstCtx(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <AdventureSilentCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::AdventureSilentCtx(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoverRogueInsCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RoverRogueInsCtx(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <DangoBrawlEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::DangoBrawlEnterCtx(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <DarkTideEnterCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::DarkTideCtx(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuestRecallInstMarkCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RecallInstMarkCtx(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <FarmGoldCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::FarmGoldCtx(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTowerCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterInstContextEnterCtxOneof::RoleTowerCtx(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterInstRequest {
    pub ContinueLastInst: bool,
    pub RoleIds: Vec<i32>,
    pub EntranceId: i32,
    pub EnterInstContext: Option<EnterInstContext>,
    pub InstanceId: i32,
    pub PosEntityId: i32,
    pub TowerDefencePhantomId: Vec<i32>,
    pub TransitionOption: Option<TransitionOptionPb>,
}

impl EnterInstRequest {
    pub const MSG_ID: u16 = 18872;
}

impl PbMessage for EnterInstRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ContinueLastInst {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EntranceId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntranceId);
        }
        if let Some(v) = &self.EnterInstContext {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
        }
        if self.PosEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PosEntityId);
        }
        for item in &self.TowerDefencePhantomId {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.TransitionOption {
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
            2 => self.ContinueLastInst = r.read_bool()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => self.EntranceId = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.EnterInstContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.InstanceId = r.read_varint_i32()?,
            13 => self.PosEntityId = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TowerDefencePhantomId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.TransitionOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterInstResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnterInstResponse {
    pub const MSG_ID: u16 = 18932;
}

impl PbMessage for EnterInstResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum EnterBigWorldInstContextEnterCtxOneof {
    SpringFestivalCtx(SpringFestivalCtx),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterBigWorldInstContext {
    pub EnterCtx: Option<EnterBigWorldInstContextEnterCtxOneof>,
}

impl PbMessage for EnterBigWorldInstContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.EnterCtx {
            match oneof {
                EnterBigWorldInstContextEnterCtxOneof::SpringFestivalCtx(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
            }
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
                let val = <SpringFestivalCtx as PbMessage>::decode(bytes)?;
                self.EnterCtx = Some(EnterBigWorldInstContextEnterCtxOneof::SpringFestivalCtx(
                    val,
                ));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterBigWorldInstRequest {
    pub EnterBigWorldInstContext: Option<EnterBigWorldInstContext>,
}

impl EnterBigWorldInstRequest {
    pub const MSG_ID: u16 = 17063;
}

impl PbMessage for EnterBigWorldInstRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EnterBigWorldInstContext {
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
                self.EnterBigWorldInstContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterBigWorldInstResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnterBigWorldInstResponse {
    pub const MSG_ID: u16 = 15876;
}

impl PbMessage for EnterBigWorldInstResponse {
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
pub struct LeaveInstRequest {
    pub PosEntityId: i32,
    pub Way: Option<LeaveInstWay>,
}

impl LeaveInstRequest {
    pub const MSG_ID: u16 = 21028;
}

impl PbMessage for LeaveInstRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PosEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PosEntityId);
        }
        if let Some(v) = &self.Way {
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
            4 => self.PosEntityId = r.read_varint_i32()?,
            15 => {
                self.Way = Some(<LeaveInstWay as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LeaveInstResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LeaveInstResponse {
    pub const MSG_ID: u16 = 20271;
}

impl PbMessage for LeaveInstResponse {
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
pub struct InstSettleNotify {
    pub From: Option<DoubleDropFrom>,
    pub RewardItems: BTreeMap<i32, RewardItemInfoList>,
    pub RewardFailTips: bool,
    pub Magnification: i32,
    pub IsSuccess: bool,
}

impl InstSettleNotify {
    pub const MSG_ID: u16 = 21939;
}

impl PbMessage for InstSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.From {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 5, &self.RewardItems);
        }
        if self.RewardFailTips {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Magnification != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Magnification);
        }
        if self.IsSuccess {
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
            2 => {
                self.From = Some(
                    <DoubleDropFrom as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            6 => self.RewardFailTips = r.read_bool()?,
            11 => self.Magnification = r.read_varint_i32()?,
            12 => self.IsSuccess = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstResultNotify {
    pub Reason: i32,
    pub Succ: bool,
    pub IsRecReward: bool,
    pub Id: i32,
}

impl InstResultNotify {
    pub const MSG_ID: u16 = 26696;
}

impl PbMessage for InstResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Reason != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.Succ {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRecReward {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
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
            5 => self.Reason = r.read_varint_i32()?,
            8 => self.Succ = r.read_bool()?,
            9 => self.IsRecReward = r.read_bool()?,
            14 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportDungeonActionNotify {
    pub HostPlayerId: i32,
    pub IsNeedSecondaryConfirmation: bool,
    pub IsRegroup: bool,
    pub IncId: i32,
    pub DungeonId: i32,
    pub TransitionOption: Option<TransitionOptionPb>,
    pub ContinueLastInst: bool,
}

impl TeleportDungeonActionNotify {
    pub const MSG_ID: u16 = 25362;
}

impl PbMessage for TeleportDungeonActionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.IsNeedSecondaryConfirmation {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRegroup {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.DungeonId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DungeonId);
        }
        if let Some(v) = &self.TransitionOption {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.ContinueLastInst {
            crate::wire::write_tag(out, 15, WireType::Varint);
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
            1 => self.HostPlayerId = r.read_varint_i32()?,
            6 => self.IsNeedSecondaryConfirmation = r.read_bool()?,
            10 => self.IsRegroup = r.read_bool()?,
            11 => self.IncId = r.read_varint_i32()?,
            12 => self.DungeonId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.TransitionOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.ContinueLastInst = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportDungeonActionRequest {
    pub ContinueLastInst: bool,
    pub IncId: i32,
    pub HostPlayerId: i32,
    pub IsConfirm: bool,
    pub RoleIds: Vec<i32>,
}

impl TeleportDungeonActionRequest {
    pub const MSG_ID: u16 = 17415;
}

impl PbMessage for TeleportDungeonActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ContinueLastInst {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.IsConfirm {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleIds {
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
            2 => self.ContinueLastInst = r.read_bool()?,
            6 => self.IncId = r.read_varint_i32()?,
            8 => self.HostPlayerId = r.read_varint_i32()?,
            9 => self.IsConfirm = r.read_bool()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportDungeonActionResponse {
    pub Code: Option<ErrorCode>,
}

impl TeleportDungeonActionResponse {
    pub const MSG_ID: u16 = 18689;
}

impl PbMessage for TeleportDungeonActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct TeleportFlowNotify {
    pub FlowName: Vec<u8>,
}

impl TeleportFlowNotify {
    pub const MSG_ID: u16 = 27892;
}

impl PbMessage for TeleportFlowNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FlowName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowName);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.FlowName = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportFlowEndRequest {
    pub FlowName: Vec<u8>,
}

impl TeleportFlowEndRequest {
    pub const MSG_ID: u16 = 16124;
}

impl PbMessage for TeleportFlowEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FlowName.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowName);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.FlowName = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportFlowEndResponse {
    pub Code: Option<ErrorCode>,
}

impl TeleportFlowEndResponse {
    pub const MSG_ID: u16 = 15441;
}

impl PbMessage for TeleportFlowEndResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct FlowEndNotify {
    pub FlowIncId: i64,
    pub Async: bool,
    pub IsFinished: bool,
}

impl FlowEndNotify {
    pub const MSG_ID: u16 = 29697;
}

impl PbMessage for FlowEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
        if self.Async {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsFinished {
            crate::wire::write_tag(out, 14, WireType::Varint);
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
            7 => self.FlowIncId = r.read_varint_i64()?,
            8 => self.Async = r.read_bool()?,
            14 => self.IsFinished = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowShowEntityNotify {
    pub EntityIds: Vec<i32>,
    pub IsHide: bool,
}

impl FlowShowEntityNotify {
    pub const MSG_ID: u16 = 28233;
}

impl PbMessage for FlowShowEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsHide {
            crate::wire::write_tag(out, 7, WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => self.IsHide = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowRestartRequest {
    pub FlowIncId: i64,
}

impl FlowRestartRequest {
    pub const MSG_ID: u16 = 24285;
}

impl PbMessage for FlowRestartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.FlowIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowRestartResponse {
    pub Code: Option<ErrorCode>,
}

impl FlowRestartResponse {
    pub const MSG_ID: u16 = 23746;
}

impl PbMessage for FlowRestartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct SubmitPhotoNodeRequest {
    pub TreeIncId: i64,
    pub NodeId: i32,
    pub TreeOwnerId: i32,
    pub EntityIds: Vec<i32>,
}

impl SubmitPhotoNodeRequest {
    pub const MSG_ID: u16 = 24093;
}

impl PbMessage for SubmitPhotoNodeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        for item in &self.EntityIds {
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
            1 => self.TreeIncId = r.read_varint_i64()?,
            4 => self.NodeId = r.read_varint_i32()?,
            7 => self.TreeOwnerId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubmitPhotoNodeResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl SubmitPhotoNodeResponse {
    pub const MSG_ID: u16 = 23527;
}

impl PbMessage for SubmitPhotoNodeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct TimerEndRequest {
    pub TreeOwnerId: i32,
    pub TimerType: Vec<u8>,
    pub TreeIncId: i64,
}

impl TimerEndRequest {
    pub const MSG_ID: u16 = 28680;
}

impl PbMessage for TimerEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if !self.TimerType.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TimerType);
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
            2 => self.TreeOwnerId = r.read_varint_i32()?,
            10 => self.TimerType = r.read_len_delimited()?.to_vec(),
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimerEndResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl TimerEndResponse {
    pub const MSG_ID: u16 = 29668;
}

impl PbMessage for TimerEndResponse {
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
pub struct BtSetTimerInfoRequest {
    pub TreeOwnerId: i32,
    pub TimerType: Vec<u8>,
    pub Second: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
    pub SetType: Option<TimerSetType>,
}

impl BtSetTimerInfoRequest {
    pub const MSG_ID: u16 = 19327;
}

impl PbMessage for BtSetTimerInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if !self.TimerType.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TimerType);
        }
        if self.Second != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Second);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if let Some(v) = &self.SetType {
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
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            2 => self.TimerType = r.read_len_delimited()?.to_vec(),
            6 => self.Second = r.read_varint_i32()?,
            10 => self.NodeId = r.read_varint_i32()?,
            11 => self.TreeIncId = r.read_varint_i64()?,
            12 => {
                self.SetType = Some(<TimerSetType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtSetTimerInfoResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl BtSetTimerInfoResponse {
    pub const MSG_ID: u16 = 20250;
}

impl PbMessage for BtSetTimerInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct UpdateTimerInfoNotify {
    pub TreeOwnerId: i32,
    pub TimerInfo: Option<TimerInfoPb>,
    pub TreeIncId: i64,
}

impl UpdateTimerInfoNotify {
    pub const MSG_ID: u16 = 27863;
}

impl PbMessage for UpdateTimerInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if let Some(v) = &self.TimerInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            4 => self.TreeOwnerId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.TimerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionGroupFinishInfo {
    pub ConditionGroupId: i32,
    pub FinishConditions: Vec<i32>,
}

impl PbMessage for ConditionGroupFinishInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConditionGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionGroupId);
        }
        for item in &self.FinishConditions {
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
            1 => self.ConditionGroupId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishConditions,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionGroupStateRequest {
    pub ConditionModule: i32,
    pub ConditionGroupIds: Vec<i32>,
}

impl ConditionGroupStateRequest {
    pub const MSG_ID: u16 = 25810;
}

impl PbMessage for ConditionGroupStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConditionModule != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionModule);
        }
        for item in &self.ConditionGroupIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            7 => self.ConditionModule = r.read_varint_i32()?,
            11 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ConditionGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionGroupStateResponse {
    pub ConditionGroupFinishInfos: Vec<ConditionGroupFinishInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ConditionGroupStateResponse {
    pub const MSG_ID: u16 = 16252;
}

impl PbMessage for ConditionGroupStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionGroupFinishInfos {
            crate::wire::write_submessage(out, 5, item);
        }
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionGroupFinishInfos
                    .push(<ConditionGroupFinishInfo as PbMessage>::decode(bytes)?);
            }
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
pub struct DailyQuestReward {
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for DailyQuestReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
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
            2 => self.ItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyQuestStateNotify {
    pub AllQuest: BTreeMap<i32, i32>,
    pub GetReward: Vec<i32>,
}

impl DailyQuestStateNotify {
    pub const MSG_ID: u16 = 26923;
}

impl PbMessage for DailyQuestStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AllQuest.is_empty() {
            crate::wire::write_map(out, 6, &self.AllQuest);
        }
        for item in &self.GetReward {
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
            6 => crate::wire::decode_map_entry(r, &mut self.AllQuest)?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GetReward, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyQuestCancelNotify {
    pub QuestIds: Vec<i32>,
}

impl DailyQuestCancelNotify {
    pub const MSG_ID: u16 = 29129;
}

impl PbMessage for DailyQuestCancelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyQuestGetRewardRequest {
    pub QuestId: Vec<i32>,
}

impl DailyQuestGetRewardRequest {
    pub const MSG_ID: u16 = 15012;
}

impl PbMessage for DailyQuestGetRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyQuestGetRewardResponse {
    pub Rewards: Vec<DailyQuestReward>,
    pub ErrorId: Option<ErrorCode>,
}

impl DailyQuestGetRewardResponse {
    pub const MSG_ID: u16 = 21806;
}

impl PbMessage for DailyQuestGetRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Rewards {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.ErrorId {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rewards
                    .push(<DailyQuestReward as PbMessage>::decode(bytes)?);
            }
            6 => {
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
pub struct DailyQuestChangePreferRequest {
    pub TargetArea: i32,
    pub TargetInfluence: i32,
}

impl DailyQuestChangePreferRequest {
    pub const MSG_ID: u16 = 23357;
}

impl PbMessage for DailyQuestChangePreferRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetArea != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetArea);
        }
        if self.TargetInfluence != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetInfluence);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.TargetArea = r.read_varint_i32()?,
            12 => self.TargetInfluence = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyQuestChangePreferResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl DailyQuestChangePreferResponse {
    pub const MSG_ID: u16 = 15733;
}

impl PbMessage for DailyQuestChangePreferResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct DailyQuestUnlockAreaNotify {
    pub CurrChoosedArea: i32,
    pub UnlockAreaIds: Vec<i32>,
    pub IsInit: bool,
    pub UnlockInfluenceIds: Vec<i32>,
    pub CurrChoosedInfluence: i32,
}

impl DailyQuestUnlockAreaNotify {
    pub const MSG_ID: u16 = 16310;
}

impl PbMessage for DailyQuestUnlockAreaNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrChoosedArea != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrChoosedArea);
        }
        for item in &self.UnlockAreaIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsInit {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.UnlockInfluenceIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CurrChoosedInfluence != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrChoosedInfluence);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.CurrChoosedArea = r.read_varint_i32()?,
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockAreaIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            9 => self.IsInit = r.read_bool()?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockInfluenceIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            13 => self.CurrChoosedInfluence = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DestroyQuestsNotify {
    pub QuestIds: Vec<i32>,
}

impl DestroyQuestsNotify {
    pub const MSG_ID: u16 = 20252;
}

impl PbMessage for DestroyQuestsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AutoQuestNotify {
    pub IsAuto: bool,
}

impl AutoQuestNotify {
    pub const MSG_ID: u16 = 20792;
}

impl PbMessage for AutoQuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAuto {
            crate::wire::write_tag(out, 6, WireType::Varint);
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
            6 => self.IsAuto = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InteractQuestSuspendEntityNotify {
    pub EntityId: i64,
    pub QuestId: i32,
}

impl InteractQuestSuspendEntityNotify {
    pub const MSG_ID: u16 = 22595;
}

impl PbMessage for InteractQuestSuspendEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            2 => self.EntityId = r.read_varint_i64()?,
            9 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfirmCompleteRefMp4QuestNotify {
    pub QuestIds: Vec<i32>,
}

impl ConfirmCompleteRefMp4QuestNotify {
    pub const MSG_ID: u16 = 20002;
}

impl PbMessage for ConfirmCompleteRefMp4QuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConfirmResourceSceneInfo {
    pub InstanceId: i32,
    pub Location: Option<Vector>,
}

impl PbMessage for ConfirmResourceSceneInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
        }
        if let Some(v) = &self.Location {
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
            1 => self.InstanceId = r.read_varint_i32()?,
            2 => {
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
pub struct ConfirmResourceQuestNotify {
    pub QuestIds: Vec<i32>,
    pub SceneInfo: Vec<ConfirmResourceSceneInfo>,
}

impl ConfirmResourceQuestNotify {
    pub const MSG_ID: u16 = 18602;
}

impl PbMessage for ConfirmResourceQuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SceneInfo {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SceneInfo
                    .push(<ConfirmResourceSceneInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WaitQuestConfirmResourceNotify {
    pub LackResourceQuestIds: Vec<i32>,
}

impl WaitQuestConfirmResourceNotify {
    pub const MSG_ID: u16 = 29463;
}

impl PbMessage for WaitQuestConfirmResourceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LackResourceQuestIds {
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LackResourceQuestIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherNotify {
    pub WeatherId: i32,
    pub IsClient: bool,
}

impl WeatherNotify {
    pub const MSG_ID: u16 = 21865;
}

impl PbMessage for WeatherNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeatherId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeatherId);
        }
        if self.IsClient {
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
            9 => self.WeatherId = r.read_varint_i32()?,
            13 => self.IsClient = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherControlUnlockNotify {
    pub WeatherSwitchId: i32,
}

impl WeatherControlUnlockNotify {
    pub const MSG_ID: u16 = 22697;
}

impl PbMessage for WeatherControlUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeatherSwitchId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeatherSwitchId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.WeatherSwitchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeWeatherRequest {
    pub Reason: Option<ChangeWeatherReason>,
    pub WeatherId: i32,
}

impl ChangeWeatherRequest {
    pub const MSG_ID: u16 = 29890;
}

impl PbMessage for ChangeWeatherRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.WeatherId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeatherId);
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
                self.Reason = Some(
                    <ChangeWeatherReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => self.WeatherId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeWeatherResponse {
    pub WeatherId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeWeatherResponse {
    pub const MSG_ID: u16 = 19397;
}

impl PbMessage for ChangeWeatherResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeatherId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeatherId);
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
            6 => self.WeatherId = r.read_varint_i32()?,
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
pub struct SyncSceneTimeNotify {
    pub TimeInfo: Option<SceneTimeInfo>,
    pub ShowUi: bool,
}

impl SyncSceneTimeNotify {
    pub const MSG_ID: u16 = 28973;
}

impl PbMessage for SyncSceneTimeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TimeInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.ShowUi {
            crate::wire::write_tag(out, 10, WireType::Varint);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TimeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.ShowUi = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeLockInfoNotify {
    pub IsLock: bool,
    pub Hour: i64,
    pub Minute: i64,
}

impl TimeLockInfoNotify {
    pub const MSG_ID: u16 = 16370;
}

impl PbMessage for TimeLockInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsLock {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Hour != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Hour);
        }
        if self.Minute != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Minute);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsLock = r.read_bool()?,
            2 => self.Hour = r.read_varint_i64()?,
            14 => self.Minute = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneAreaStateNotify {
    pub AreaState: Option<SceneAreaState>,
}

impl SceneAreaStateNotify {
    pub const MSG_ID: u16 = 15646;
}

impl PbMessage for SceneAreaStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AreaState {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AreaState
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkShowIdInfoUpdateNotify {
    pub ShowFlag: u32,
    pub IsShow: bool,
    pub MarkId: i32,
    pub NeedFocus: bool,
}

impl MapMarkShowIdInfoUpdateNotify {
    pub const MSG_ID: u16 = 16700;
}

impl PbMessage for MapMarkShowIdInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowFlag != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.ShowFlag));
        }
        if self.IsShow {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if self.NeedFocus {
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
            1 => self.ShowFlag = r.read_varint_u32()?,
            3 => self.IsShow = r.read_bool()?,
            5 => self.MarkId = r.read_varint_i32()?,
            11 => self.NeedFocus = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemUseRequest {
    pub ItemId: i32,
    pub Count: i32,
}

impl ItemUseRequest {
    pub const MSG_ID: u16 = 15211;
}

impl PbMessage for ItemUseRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            11 => self.ItemId = r.read_varint_i32()?,
            14 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemUseResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ItemUseResponse {
    pub const MSG_ID: u16 = 21049;
}

impl PbMessage for ItemUseResponse {
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
pub struct MailReadRequest {
    pub Id: Vec<u8>,
}

impl MailReadRequest {
    pub const MSG_ID: u16 = 20227;
}

impl PbMessage for MailReadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Id = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailReadResponse {
    pub ExpiryTime: i64,
    pub Id: Vec<u8>,
    pub State: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ReadTime: i64,
}

impl MailReadResponse {
    pub const MSG_ID: u16 = 23666;
}

impl PbMessage for MailReadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExpiryTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExpiryTime);
        }
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Id);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ReadTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReadTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.ExpiryTime = r.read_varint_i64()?,
            5 => self.Id = r.read_len_delimited()?.to_vec(),
            13 => self.State = r.read_varint_i32()?,
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.ReadTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFocusModeDeterCondRequest {
    pub DeterCond: bool,
}

impl SetFocusModeDeterCondRequest {
    pub const MSG_ID: u16 = 23749;
}

impl PbMessage for SetFocusModeDeterCondRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DeterCond {
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
            4 => self.DeterCond = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFocusModeDeterCondResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetFocusModeDeterCondResponse {
    pub const MSG_ID: u16 = 15892;
}

impl PbMessage for SetFocusModeDeterCondResponse {
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
pub struct QuestBranchSelectRequest {
    pub BranchId: i32,
    pub BranchPageId: i32,
}

impl QuestBranchSelectRequest {
    pub const MSG_ID: u16 = 19447;
}

impl PbMessage for QuestBranchSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BranchId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchId);
        }
        if self.BranchPageId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchPageId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.BranchId = r.read_varint_i32()?,
            13 => self.BranchPageId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestBranchSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub OneQuestBranchPageInfo: Option<OneQuestBranchPageInfo>,
}

impl QuestBranchSelectResponse {
    pub const MSG_ID: u16 = 29618;
}

impl PbMessage for QuestBranchSelectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.OneQuestBranchPageInfo {
            crate::wire::write_submessage(out, 7, v);
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.OneQuestBranchPageInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuestBranchUpdateNotify {
    pub QuestBranchInfo: Option<QuestBranchInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl QuestBranchUpdateNotify {
    pub const MSG_ID: u16 = 26971;
}

impl PbMessage for QuestBranchUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.QuestBranchInfo {
            crate::wire::write_submessage(out, 1, v);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.QuestBranchInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct SubmitNodeRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl SubmitNodeRequest {
    pub const MSG_ID: u16 = 21759;
}

impl PbMessage for SubmitNodeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            3 => self.TreeOwnerId = r.read_varint_i32()?,
            4 => self.NodeId = r.read_varint_i32()?,
            12 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubmitNodeResponse {
    pub TreeIncId: i64,
    pub ErrorId: Option<ErrorCode>,
    pub NodeId: i32,
    pub TreeOwnerId: i32,
}

impl SubmitNodeResponse {
    pub const MSG_ID: u16 = 21283;
}

impl PbMessage for SubmitNodeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.TreeIncId = r.read_varint_i64()?,
            7 => {
                self.ErrorId = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.NodeId = r.read_varint_i32()?,
            15 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedRedDotRequest {}

impl IllustratedRedDotRequest {
    pub const MSG_ID: u16 = 17930;
}

impl PbMessage for IllustratedRedDotRequest {
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
pub struct IllustratedRedDotResponse {
    pub RedDotList: Vec<IllustratedType>,
}

impl IllustratedRedDotResponse {
    pub const MSG_ID: u16 = 26946;
}

impl PbMessage for IllustratedRedDotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RedDotList {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RedDotList, |sr| {
                Ok(
                    <IllustratedType as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ),
                )
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedReadRequest {
    pub Id: i32,
    pub Type: Option<IllustratedType>,
}

impl IllustratedReadRequest {
    pub const MSG_ID: u16 = 19077;
}

impl PbMessage for IllustratedReadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
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
            2 => self.Id = r.read_varint_i32()?,
            14 => {
                self.Type = Some(
                    <IllustratedType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct IllustratedReadResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl IllustratedReadResponse {
    pub const MSG_ID: u16 = 27163;
}

impl PbMessage for IllustratedReadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 12, crate::wire::WireType::LengthDelimited);
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionFinishRequest {
    pub ErrorMsg: Vec<u8>,
    pub PlayerId: i32,
    pub IncId: i32,
    pub StartIndex: i32,
}

impl ActionFinishRequest {
    pub const MSG_ID: u16 = 27577;
}

impl PbMessage for ActionFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ErrorMsg.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ErrorMsg);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.StartIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ErrorMsg = r.read_len_delimited()?.to_vec(),
            3 => self.PlayerId = r.read_varint_i32()?,
            5 => self.IncId = r.read_varint_i32()?,
            14 => self.StartIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionFinishResponse {
    pub Code: Option<ErrorCode>,
}

impl ActionFinishResponse {
    pub const MSG_ID: u16 = 26069;
}

impl PbMessage for ActionFinishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct ActionFinishImmediatelyNotify {
    pub IncId: i32,
    pub PlayerId: i32,
}

impl ActionFinishImmediatelyNotify {
    pub const MSG_ID: u16 = 24609;
}

impl PbMessage for ActionFinishImmediatelyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.IncId = r.read_varint_i32()?,
            11 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtGiveUpRequest {
    pub NodeId: i32,
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
}

impl BtGiveUpRequest {
    pub const MSG_ID: u16 = 18051;
}

impl PbMessage for BtGiveUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.NodeId = r.read_varint_i32()?,
            11 => self.TreeIncId = r.read_varint_i64()?,
            13 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtGiveUpResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl BtGiveUpResponse {
    pub const MSG_ID: u16 = 22791;
}

impl PbMessage for BtGiveUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct BtRollbackRequest {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub Select: i32,
}

impl BtRollbackRequest {
    pub const MSG_ID: u16 = 23225;
}

impl PbMessage for BtRollbackRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.Select != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Select);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            3 => self.TreeIncId = r.read_varint_i64()?,
            4 => self.Select = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRollbackResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl BtRollbackResponse {
    pub const MSG_ID: u16 = 22916;
}

impl PbMessage for BtRollbackResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct TemporaryTeleportInfo {
    pub MarkId: i32,
    pub TemporaryTeleportId: i64,
    pub MapId: i32,
    pub InstId: i32,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub PlayerTeleportPos: Option<Vector>,
    pub Area: i32,
}

impl PbMessage for TemporaryTeleportInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if self.TemporaryTeleportId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TemporaryTeleportId);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.PlayerTeleportPos {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.Area != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Area);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MarkId = r.read_varint_i32()?,
            2 => self.TemporaryTeleportId = r.read_varint_i64()?,
            3 => self.MapId = r.read_varint_i32()?,
            4 => self.InstId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTeleportPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.Area = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddTemporaryTeleportRequest {
    pub Rot: Option<Rotator>,
    pub Pos: Option<Vector>,
    pub Area: i32,
}

impl AddTemporaryTeleportRequest {
    pub const MSG_ID: u16 = 25491;
}

impl PbMessage for AddTemporaryTeleportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.Area != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Area);
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
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.Area = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddTemporaryTeleportResponse {
    pub TemporaryTeleportId: i64,
    pub Code: Option<ErrorCode>,
}

impl AddTemporaryTeleportResponse {
    pub const MSG_ID: u16 = 20601;
}

impl PbMessage for AddTemporaryTeleportResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TemporaryTeleportId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TemporaryTeleportId);
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
            11 => self.TemporaryTeleportId = r.read_varint_i64()?,
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
pub struct RemoveTemporaryTeleportRequest {
    pub TemporaryTeleportId: i64,
}

impl RemoveTemporaryTeleportRequest {
    pub const MSG_ID: u16 = 22106;
}

impl PbMessage for RemoveTemporaryTeleportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TemporaryTeleportId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TemporaryTeleportId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.TemporaryTeleportId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveTemporaryTeleportResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl RemoveTemporaryTeleportResponse {
    pub const MSG_ID: u16 = 28735;
}

impl PbMessage for RemoveTemporaryTeleportResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct AddTemporaryTeleportInfoNotify {
    pub TemporaryTeleportInfo: Option<TemporaryTeleportInfo>,
}

impl AddTemporaryTeleportInfoNotify {
    pub const MSG_ID: u16 = 22362;
}

impl PbMessage for AddTemporaryTeleportInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TemporaryTeleportInfo {
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
                self.TemporaryTeleportInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemporaryTeleportAllInfoNotify {
    pub AllTemporaryTeleportInfo: Vec<TemporaryTeleportInfo>,
}

impl TemporaryTeleportAllInfoNotify {
    pub const MSG_ID: u16 = 27094;
}

impl PbMessage for TemporaryTeleportAllInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AllTemporaryTeleportInfo {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AllTemporaryTeleportInfo
                    .push(<TemporaryTeleportInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemporaryTeleportChangeNotify {
    pub TemporaryTeleportInfo: Option<TemporaryTeleportInfo>,
}

impl TemporaryTeleportChangeNotify {
    pub const MSG_ID: u16 = 28744;
}

impl PbMessage for TemporaryTeleportChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TemporaryTeleportInfo {
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.TemporaryTeleportInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveTemporaryTeleportNotify {
    pub TemporaryTeleportId: i64,
}

impl RemoveTemporaryTeleportNotify {
    pub const MSG_ID: u16 = 21656;
}

impl PbMessage for RemoveTemporaryTeleportNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TemporaryTeleportId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TemporaryTeleportId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.TemporaryTeleportId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportToTargetRequest {
    pub Rot: Option<Rotator>,
    pub TemporaryTeleportId: i64,
}

impl TeleportToTargetRequest {
    pub const MSG_ID: u16 = 28616;
}

impl PbMessage for TeleportToTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TemporaryTeleportId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TemporaryTeleportId);
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
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.TemporaryTeleportId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportToTargetResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl TeleportToTargetResponse {
    pub const MSG_ID: u16 = 17547;
}

impl PbMessage for TeleportToTargetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct EntityPosAbnormalRequest {
    pub EntityId: i64,
    pub IsAbnormal: bool,
}

impl EntityPosAbnormalRequest {
    pub const MSG_ID: u16 = 26177;
}

impl PbMessage for EntityPosAbnormalRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsAbnormal {
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
            3 => self.EntityId = r.read_varint_i64()?,
            11 => self.IsAbnormal = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPosAbnormalResponse {
    pub Code: Option<ErrorCode>,
}

impl EntityPosAbnormalResponse {
    pub const MSG_ID: u16 = 24226;
}

impl PbMessage for EntityPosAbnormalResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct EntityPosResetNotify {
    pub EntityId: i32,
}

impl EntityPosResetNotify {
    pub const MSG_ID: u16 = 19731;
}

impl PbMessage for EntityPosResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioStatePb {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub Group: Vec<u8>,
    pub State: Vec<u8>,
}

impl PbMessage for AudioStatePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if !self.Group.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Group);
        }
        if !self.State.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.State);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            2 => self.TreeIncId = r.read_varint_i64()?,
            3 => self.Group = r.read_len_delimited()?.to_vec(),
            4 => self.State = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AudioStateUpdateNotify {
    pub AudioStates: Vec<AudioStatePb>,
}

impl AudioStateUpdateNotify {
    pub const MSG_ID: u16 = 17667;
}

impl PbMessage for AudioStateUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AudioStates {
            crate::wire::write_submessage(out, 12, item);
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
                let bytes = r.read_len_delimited()?;
                self.AudioStates
                    .push(<AudioStatePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAddBubbleNotify {
    pub EntityId: i64,
    pub ActionGuid: Vec<u8>,
}

impl EntityAddBubbleNotify {
    pub const MSG_ID: u16 = 22334;
}

impl PbMessage for EntityAddBubbleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.ActionGuid.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActionGuid);
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
            8 => self.ActionGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRemoveBubbleNotify {
    pub ActionGuid: Vec<u8>,
    pub EntityId: i64,
}

impl EntityRemoveBubbleNotify {
    pub const MSG_ID: u16 = 21182;
}

impl PbMessage for EntityRemoveBubbleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActionGuid.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActionGuid);
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
            9 => self.ActionGuid = r.read_len_delimited()?.to_vec(),
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAddDynamicInteractNotify {
    pub GameCtx: Option<GameCtxPb>,
    pub EntityId: i64,
    pub DelayRemove: bool,
    pub Text: Vec<u8>,
    pub OptionGuid: Vec<u8>,
}

impl EntityAddDynamicInteractNotify {
    pub const MSG_ID: u16 = 17186;
}

impl PbMessage for EntityAddDynamicInteractNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.DelayRemove {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Text.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Text);
        }
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OptionGuid);
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
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.EntityId = r.read_varint_i64()?,
            5 => self.DelayRemove = r.read_bool()?,
            6 => self.Text = r.read_len_delimited()?.to_vec(),
            14 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRemoveDynamicInteractNotify {
    pub OptionGuid: Vec<u8>,
    pub EntityId: i64,
}

impl EntityRemoveDynamicInteractNotify {
    pub const MSG_ID: u16 = 18312;
}

impl PbMessage for EntityRemoveDynamicInteractNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OptionGuid);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            11 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityChangeDynamicInteractTextNotify {
    pub EntityId: i64,
    pub Text: Vec<u8>,
    pub OptionGuid: Vec<u8>,
}

impl EntityChangeDynamicInteractTextNotify {
    pub const MSG_ID: u16 = 25073;
}

impl PbMessage for EntityChangeDynamicInteractTextNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.Text.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Text);
        }
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.OptionGuid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.EntityId = r.read_varint_i64()?,
            11 => self.Text = r.read_len_delimited()?.to_vec(),
            14 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddGuestNotify {
    pub GuestId: i32,
}

impl AddGuestNotify {
    pub const MSG_ID: u16 = 15984;
}

impl PbMessage for AddGuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GuestId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.GuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveGuestNotify {
    pub GuestId: i32,
}

impl RemoveGuestNotify {
    pub const MSG_ID: u16 = 18978;
}

impl PbMessage for RemoveGuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GuestId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GuestId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.GuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemData {
    pub ItemId: i32,
    pub ItemIncId: i32,
    pub Count: i32,
}

impl PbMessage for ItemData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemIncId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            2 => self.ItemIncId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleItemInfo {
    pub ItemId: i32,
    pub ItemNum: i32,
}

impl PbMessage for SingleItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemNum);
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
            2 => self.ItemNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopUpdateRequest {
    pub Id: i32,
}

impl PayShopUpdateRequest {
    pub const MSG_ID: u16 = 29578;
}

impl PbMessage for PayShopUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopUpdateResponse {
    pub Info: Option<PayShopInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PayShopUpdateResponse {
    pub const MSG_ID: u16 = 26133;
}

impl PbMessage for PayShopUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 6, v);
        }
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct PayShopItemUpdateRequest {
    pub ShopItemIds: Vec<i32>,
}

impl PayShopItemUpdateRequest {
    pub const MSG_ID: u16 = 21015;
}

impl PbMessage for PayShopItemUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShopItemIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ShopItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopItemUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Items: Vec<PayShopItem>,
}

impl PayShopItemUpdateResponse {
    pub const MSG_ID: u16 = 16503;
}

impl PbMessage for PayShopItemUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Items {
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Items.push(<PayShopItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopBuyRequest {
    pub Count: i32,
    pub Version: Vec<u8>,
    pub Id: i32,
}

impl PayShopBuyRequest {
    pub const MSG_ID: u16 = 28780;
}

impl PbMessage for PayShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Count != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => self.Count = r.read_varint_i32()?,
            3 => self.Version = r.read_len_delimited()?.to_vec(),
            7 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopBuyResponse {
    pub Count: i32,
    pub Id: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PayShopBuyResponse {
    pub const MSG_ID: u16 = 19539;
}

impl PbMessage for PayShopBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Count != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
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
            4 => self.Count = r.read_varint_i32()?,
            9 => self.Id = r.read_varint_i32()?,
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
pub struct PayShopInfoNotify {
    pub PayShopRecommendData: Vec<ShopRecommend>,
    pub PayGiftShopInfo: Option<PayGiftShopInfo>,
    pub Infos: Vec<PayShopInfo>,
    pub PayShopTabData: Vec<ShopTab>,
    pub Version: Vec<u8>,
}

impl PayShopInfoNotify {
    pub const MSG_ID: u16 = 29139;
}

impl PbMessage for PayShopInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PayShopRecommendData {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.PayGiftShopInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.Infos {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.PayShopTabData {
            crate::wire::write_submessage(out, 12, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
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
                self.PayShopRecommendData
                    .push(<ShopRecommend as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PayGiftShopInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Infos.push(<PayShopInfo as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PayShopTabData
                    .push(<ShopTab as PbMessage>::decode(bytes)?);
            }
            13 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopDirectBuyNotify {
    pub ItemId: i32,
    pub ReceiptId: Vec<u8>,
    pub ShopItemId: i32,
    pub ItemCount: i32,
}

impl PayShopDirectBuyNotify {
    pub const MSG_ID: u16 = 18553;
}

impl PbMessage for PayShopDirectBuyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
        }
        if self.ShopItemId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ItemId = r.read_varint_i32()?,
            6 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            12 => self.ShopItemId = r.read_varint_i32()?,
            13 => self.ItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShoppMoneyInfo {
    pub MoneyId: i32,
    pub MoneyNum: i32,
}

impl PbMessage for ShoppMoneyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoneyId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoneyId);
        }
        if self.MoneyNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoneyNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MoneyId = r.read_varint_i32()?,
            2 => self.MoneyNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopItemInfoNew {
    pub Id: i32,
    pub BoughtCount: i32,
    pub Lock: bool,
    pub ItemId: i32,
    pub ItemNum: i32,
    pub CondText: Vec<u8>,
    pub MoneyList: Vec<ShoppMoneyInfo>,
    pub BeginTime: u32,
    pub EndTime: u32,
    pub LimitNum: i32,
    pub OriginalMoneyList: Vec<ShoppMoneyInfo>,
    pub Label: Vec<u8>,
    pub SwitchText: Vec<u8>,
    pub PurchaseText: Vec<u8>,
    pub UnlockConditionId: i32,
    pub SortIndex: i32,
}

impl PbMessage for ShopItemInfoNew {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.BoughtCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoughtCount);
        }
        if self.Lock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemNum != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemNum);
        }
        if !self.CondText.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CondText);
        }
        for item in &self.MoneyList {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.BeginTime));
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.EndTime));
        }
        if self.LimitNum != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitNum);
        }
        for item in &self.OriginalMoneyList {
            crate::wire::write_submessage(out, 11, item);
        }
        if !self.Label.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Label);
        }
        if !self.SwitchText.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SwitchText);
        }
        if !self.PurchaseText.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PurchaseText);
        }
        if self.UnlockConditionId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockConditionId);
        }
        if self.SortIndex != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SortIndex);
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
            2 => self.BoughtCount = r.read_varint_i32()?,
            3 => self.Lock = r.read_bool()?,
            4 => self.ItemId = r.read_varint_i32()?,
            5 => self.ItemNum = r.read_varint_i32()?,
            6 => self.CondText = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.MoneyList
                    .push(<ShoppMoneyInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.BeginTime = r.read_varint_u32()?,
            9 => self.EndTime = r.read_varint_u32()?,
            10 => self.LimitNum = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.OriginalMoneyList
                    .push(<ShoppMoneyInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.Label = r.read_len_delimited()?.to_vec(),
            13 => self.SwitchText = r.read_len_delimited()?.to_vec(),
            14 => self.PurchaseText = r.read_len_delimited()?.to_vec(),
            15 => self.UnlockConditionId = r.read_varint_i32()?,
            16 => self.SortIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopInfo {
    pub ShopId: i32,
    pub UpdateTime: u32,
    pub ItemInfoList: Vec<ShopItemInfoNew>,
}

impl PbMessage for ShopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.UpdateTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.UpdateTime));
        }
        for item in &self.ItemInfoList {
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
            1 => self.ShopId = r.read_varint_i32()?,
            2 => self.UpdateTime = r.read_varint_u32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ItemInfoList
                    .push(<ShopItemInfoNew as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopInfoRequest {
    pub VersionStr: Vec<u8>,
}

impl ShopInfoRequest {
    pub const MSG_ID: u16 = 29540;
}

impl PbMessage for ShopInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.VersionStr.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VersionStr);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.VersionStr = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopInfoResponse {
    pub ShopList: Vec<ShopInfo>,
    pub VersionStr: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ShopInfoResponse {
    pub const MSG_ID: u16 = 28926;
}

impl PbMessage for ShopInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShopList {
            crate::wire::write_submessage(out, 8, item);
        }
        if !self.VersionStr.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VersionStr);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ShopList.push(<ShopInfo as PbMessage>::decode(bytes)?);
            }
            9 => self.VersionStr = r.read_len_delimited()?.to_vec(),
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
pub struct ShopBuyRequest {
    pub Num: i32,
    pub MoneyId: i32,
    pub Id: i32,
    pub VersionStr: Vec<u8>,
    pub ShopId: i32,
    pub InteractEntityId: i64,
}

impl ShopBuyRequest {
    pub const MSG_ID: u16 = 28078;
}

impl PbMessage for ShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Num != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.MoneyId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoneyId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.VersionStr.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VersionStr);
        }
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Num = r.read_varint_i32()?,
            6 => self.MoneyId = r.read_varint_i32()?,
            7 => self.Id = r.read_varint_i32()?,
            10 => self.VersionStr = r.read_len_delimited()?.to_vec(),
            11 => self.ShopId = r.read_varint_i32()?,
            12 => self.InteractEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopBuyResponse {
    pub Id: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ShopId: i32,
    pub BoughtCount: i32,
}

impl ShopBuyResponse {
    pub const MSG_ID: u16 = 27294;
}

impl PbMessage for ShopBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.BoughtCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoughtCount);
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.ShopId = r.read_varint_i32()?,
            14 => self.BoughtCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopInfoUpdateNotify {
    pub ShopInfo: Option<ShopInfo>,
}

impl ShopInfoUpdateNotify {
    pub const MSG_ID: u16 = 28209;
}

impl PbMessage for ShopInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ShopInfo {
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
                self.ShopInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemExchangeRequest {
    pub ItemId: i32,
    pub ExchangeTimes: i32,
}

impl ItemExchangeRequest {
    pub const MSG_ID: u16 = 16929;
}

impl PbMessage for ItemExchangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ExchangeTimes != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExchangeTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.ItemId = r.read_varint_i32()?,
            14 => self.ExchangeTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemExchangeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl ItemExchangeResponse {
    pub const MSG_ID: u16 = 24645;
}

impl PbMessage for ItemExchangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
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
            10 => self.ItemId = r.read_varint_i32()?,
            13 => self.ItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeFormulaUnlockRequest {
    pub Id: i32,
}

impl ForgeFormulaUnlockRequest {
    pub const MSG_ID: u16 = 23395;
}

impl PbMessage for ForgeFormulaUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeFormulaUnlockResponse {
    pub Code: Option<ErrorCode>,
    pub Id: i32,
}

impl ForgeFormulaUnlockResponse {
    pub const MSG_ID: u16 = 26326;
}

impl PbMessage for ForgeFormulaUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            4 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeItemRequest {
    pub RoleId: i32,
    pub Count: i32,
    pub Id: i32,
    pub InteractEntityId: i64,
}

impl ForgeItemRequest {
    pub const MSG_ID: u16 = 21849;
}

impl PbMessage for ForgeItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.RoleId = r.read_varint_i32()?,
            13 => self.Count = r.read_varint_i32()?,
            14 => self.Id = r.read_varint_i32()?,
            15 => self.InteractEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeItemResponse {
    pub ExtraItemInfos: Vec<SingleItemInfo>,
    pub ActiveSkillType: i32,
    pub ItemInfos: Vec<SingleItemInfo>,
    pub Code: Option<ErrorCode>,
    pub RoleId: i32,
    pub Id: i32,
}

impl ForgeItemResponse {
    pub const MSG_ID: u16 = 24992;
}

impl PbMessage for ForgeItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ExtraItemInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.ActiveSkillType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveSkillType);
        }
        for item in &self.ItemInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.ActiveSkillType = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.RoleId = r.read_varint_i32()?,
            14 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeItemInfoUpdateNotify {
    pub ForgeInfoList: Vec<OneForgeInfo>,
}

impl ForgeItemInfoUpdateNotify {
    pub const MSG_ID: u16 = 28572;
}

impl PbMessage for ForgeItemInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ForgeInfoList {
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
                self.ForgeInfoList
                    .push(<OneForgeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisFormulaUnlockRequest {
    pub Id: i32,
}

impl SynthesisFormulaUnlockRequest {
    pub const MSG_ID: u16 = 23511;
}

impl PbMessage for SynthesisFormulaUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisFormulaUnlockResponse {
    pub Code: Option<ErrorCode>,
    pub Id: i32,
}

impl SynthesisFormulaUnlockResponse {
    pub const MSG_ID: u16 = 23763;
}

impl PbMessage for SynthesisFormulaUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            7 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisItemRequest {
    pub RoleId: i32,
    pub Count: i32,
    pub InteractEntityId: i64,
    pub Id: i32,
}

impl SynthesisItemRequest {
    pub const MSG_ID: u16 = 24085;
}

impl PbMessage for SynthesisItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
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
            11 => self.RoleId = r.read_varint_i32()?,
            12 => self.Count = r.read_varint_i32()?,
            13 => self.InteractEntityId = r.read_varint_i64()?,
            14 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisItemResponse {
    pub ActiveSkillType: i32,
    pub RoleId: i32,
    pub ExtraItemInfos: Vec<SingleItemInfo>,
    pub ItemInfos: Vec<SingleItemInfo>,
    pub Id: i32,
    pub Code: Option<ErrorCode>,
}

impl SynthesisItemResponse {
    pub const MSG_ID: u16 = 24774;
}

impl PbMessage for SynthesisItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActiveSkillType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveSkillType);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.ExtraItemInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.ItemInfos {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Code {
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
            1 => self.ActiveSkillType = r.read_varint_i32()?,
            5 => self.RoleId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.Id = r.read_varint_i32()?,
            14 => {
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
pub struct BatchItem {
    pub Id: i32,
    pub Count: i32,
    pub CostItems: Vec<SingleItemInfo>,
}

impl PbMessage for BatchItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        for item in &self.CostItems {
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
            2 => self.Count = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CostItems
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchSynthesisItemRequest {
    pub BatchItems: Vec<BatchItem>,
}

impl BatchSynthesisItemRequest {
    pub const MSG_ID: u16 = 19088;
}

impl PbMessage for BatchSynthesisItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BatchItems {
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
                self.BatchItems
                    .push(<BatchItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchSynthesisItemResponse {
    pub Items: BTreeMap<i32, i32>,
    pub Code: Option<ErrorCode>,
}

impl BatchSynthesisItemResponse {
    pub const MSG_ID: u16 = 26720;
}

impl PbMessage for BatchSynthesisItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Items.is_empty() {
            crate::wire::write_map(out, 1, &self.Items);
        }
        if let Some(v) = &self.Code {
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
            1 => crate::wire::decode_map_entry(r, &mut self.Items)?,
            13 => {
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
pub struct SynthesisLevelRewardRequest {}

impl SynthesisLevelRewardRequest {
    pub const MSG_ID: u16 = 15075;
}

impl PbMessage for SynthesisLevelRewardRequest {
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
pub struct SynthesisLevelRewardResponse {
    pub Code: Option<ErrorCode>,
    pub LevelInfo: Option<SynthesisLevelInfo>,
}

impl SynthesisLevelRewardResponse {
    pub const MSG_ID: u16 = 21274;
}

impl PbMessage for SynthesisLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.LevelInfo {
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
            4 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.LevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisLevelUpdateNotify {
    pub LevelInfo: Option<SynthesisLevelInfo>,
}

impl SynthesisLevelUpdateNotify {
    pub const MSG_ID: u16 = 18205;
}

impl PbMessage for SynthesisLevelUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LevelInfo {
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
                self.LevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookingInfo {
    pub CookingLevel: i32,
    pub TotalProficiencys: i32,
}

impl PbMessage for CookingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CookingLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookingLevel);
        }
        if self.TotalProficiencys != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalProficiencys);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CookingLevel = r.read_varint_i32()?,
            2 => self.TotalProficiencys = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneFormulaConfig {
    pub Id: i32,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for OneFormulaConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.LimitBeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitBeginTime);
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
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
            2 => self.LimitBeginTime = r.read_varint_i64()?,
            3 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleFoodFormulaInfo {
    pub Id: i32,
    pub CookCount: i32,
    pub LastRoleId: i32,
    pub LimitCount: i32,
    pub LimitCookCount: i32,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for SingleFoodFormulaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CookCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookCount);
        }
        if self.LastRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRoleId);
        }
        if self.LimitCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitCount);
        }
        if self.LimitCookCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitCookCount);
        }
        if self.LimitBeginTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitBeginTime);
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
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
            2 => self.CookCount = r.read_varint_i32()?,
            3 => self.LastRoleId = r.read_varint_i32()?,
            4 => self.LimitCount = r.read_varint_i32()?,
            5 => self.LimitCookCount = r.read_varint_i32()?,
            6 => self.LimitBeginTime = r.read_varint_i64()?,
            7 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleProcessedFoodFormulaInfo {
    pub Id: i32,
    pub LockState: bool,
    pub Interations: Vec<i32>,
    pub UnlockParams: Vec<i32>,
}

impl PbMessage for SingleProcessedFoodFormulaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.LockState {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Interations {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockParams {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.LockState = r.read_bool()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Interations, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockParams, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookingDataRequest {}

impl CookingDataRequest {
    pub const MSG_ID: u16 = 24585;
}

impl PbMessage for CookingDataRequest {
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
pub struct CookingDataResponse {
    pub ProcessedFoodFormulaInfos: Vec<SingleProcessedFoodFormulaInfo>,
    pub FormulaConfigs: Vec<OneFormulaConfig>,
    pub FoodFormulaInfos: Vec<SingleFoodFormulaInfo>,
    pub Code: Option<ErrorCode>,
    pub LimitRefreshTime: i64,
    pub CookingInfo: Option<CookingInfo>,
}

impl CookingDataResponse {
    pub const MSG_ID: u16 = 26168;
}

impl PbMessage for CookingDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ProcessedFoodFormulaInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.FormulaConfigs {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.FoodFormulaInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LimitRefreshTime != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitRefreshTime);
        }
        if let Some(v) = &self.CookingInfo {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ProcessedFoodFormulaInfos.push(
                    <SingleProcessedFoodFormulaInfo as PbMessage>::decode(bytes)?,
                );
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FormulaConfigs
                    .push(<OneFormulaConfig as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FoodFormulaInfos
                    .push(<SingleFoodFormulaInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.LimitRefreshTime = r.read_varint_i64()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CookingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookFoodRequest {
    pub InteractEntityId: i64,
    pub CookCount: i32,
    pub Id: i32,
    pub RoleId: i32,
}

impl CookFoodRequest {
    pub const MSG_ID: u16 = 29582;
}

impl PbMessage for CookFoodRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
        if self.CookCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookCount);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            3 => self.InteractEntityId = r.read_varint_i64()?,
            10 => self.CookCount = r.read_varint_i32()?,
            13 => self.Id = r.read_varint_i32()?,
            14 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookFoodResponse {
    pub ItemInfos: Vec<SingleItemInfo>,
    pub ExtraItemInfos: Vec<SingleItemInfo>,
    pub Code: Option<ErrorCode>,
    pub ActiveSkillType: i32,
    pub Id: i32,
    pub RoleId: i32,
}

impl CookFoodResponse {
    pub const MSG_ID: u16 = 23883;
}

impl PbMessage for CookFoodResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ExtraItemInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActiveSkillType != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveSkillType);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
                self.ItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItemInfos
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.ActiveSkillType = r.read_varint_i32()?,
            13 => self.Id = r.read_varint_i32()?,
            15 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookFormulaRequest {
    pub CookFormulaId: i32,
}

impl CookFormulaRequest {
    pub const MSG_ID: u16 = 27218;
}

impl PbMessage for CookFormulaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CookFormulaId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookFormulaId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.CookFormulaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookFormulaResponse {
    pub Code: Option<ErrorCode>,
    pub CookFormulaId: i32,
}

impl CookFormulaResponse {
    pub const MSG_ID: u16 = 23024;
}

impl PbMessage for CookFormulaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CookFormulaId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookFormulaId);
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
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.CookFormulaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookingInfoUpdateNotify {
    pub CookingInfo: Option<CookingInfo>,
}

impl CookingInfoUpdateNotify {
    pub const MSG_ID: u16 = 24324;
}

impl PbMessage for CookingInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CookingInfo {
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
                self.CookingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CookingFormulaUpdateNotify {
    pub ProcessedFoodFormulaInfos: Vec<SingleProcessedFoodFormulaInfo>,
    pub FoodFormulaInfos: Vec<SingleFoodFormulaInfo>,
}

impl CookingFormulaUpdateNotify {
    pub const MSG_ID: u16 = 28308;
}

impl PbMessage for CookingFormulaUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ProcessedFoodFormulaInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.FoodFormulaInfos {
            crate::wire::write_submessage(out, 8, item);
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
                self.ProcessedFoodFormulaInfos.push(
                    <SingleProcessedFoodFormulaInfo as PbMessage>::decode(bytes)?,
                );
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.FoodFormulaInfos
                    .push(<SingleFoodFormulaInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementFinishRequest {
    pub Id: i32,
}

impl AchievementFinishRequest {
    pub const MSG_ID: u16 = 27912;
}

impl PbMessage for AchievementFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AchievementFinishResponse {
    pub const MSG_ID: u16 = 19182;
}

impl PbMessage for AchievementFinishResponse {
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
pub struct AchievementReceiveRequest {
    pub Id: i32,
    pub IsGroupId: bool,
}

impl AchievementReceiveRequest {
    pub const MSG_ID: u16 = 16062;
}

impl PbMessage for AchievementReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsGroupId {
            crate::wire::write_tag(out, 15, WireType::Varint);
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
            9 => self.Id = r.read_varint_i32()?,
            15 => self.IsGroupId = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl AchievementReceiveResponse {
    pub const MSG_ID: u16 = 25408;
}

impl PbMessage for AchievementReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 5, &self.ItemMap);
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 7, crate::wire::WireType::LengthDelimited);
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            7 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulAchievementReceiveRequest {
    pub AchievementGroupIds: Vec<i32>,
    pub AchievementIds: Vec<i32>,
}

impl MulAchievementReceiveRequest {
    pub const MSG_ID: u16 = 28143;
}

impl PbMessage for MulAchievementReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AchievementGroupIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AchievementIds {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AchievementGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AchievementIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulAchievementReceiveResponse {
    pub AchievementIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
    pub AchievementGroupIds: Vec<i32>,
}

impl MulAchievementReceiveResponse {
    pub const MSG_ID: u16 = 19770;
}

impl PbMessage for MulAchievementReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AchievementIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AchievementGroupIds {
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
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AchievementIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AchievementGroupIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementProgressNotify {
    pub AchievementEntry: Option<AchievementEntry>,
}

impl AchievementProgressNotify {
    pub const MSG_ID: u16 = 29757;
}

impl PbMessage for AchievementProgressNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AchievementEntry {
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
                self.AchievementEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementListProgressNotify {
    pub AchievementEntryList: Vec<AchievementEntry>,
}

impl AchievementListProgressNotify {
    pub const MSG_ID: u16 = 29111;
}

impl PbMessage for AchievementListProgressNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AchievementEntryList {
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
                self.AchievementEntryList
                    .push(<AchievementEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementGroupProgressNotify {
    pub AchievementGroupEntry: Option<AchievementGroupEntry>,
}

impl AchievementGroupProgressNotify {
    pub const MSG_ID: u16 = 18514;
}

impl PbMessage for AchievementGroupProgressNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AchievementGroupEntry {
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
                self.AchievementGroupEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementCountChangeNotify {
    pub StarCount: i32,
    pub FinishCount: i32,
}

impl AchievementCountChangeNotify {
    pub const MSG_ID: u16 = 20230;
}

impl PbMessage for AchievementCountChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StarCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StarCount);
        }
        if self.FinishCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinishCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.StarCount = r.read_varint_i32()?,
            8 => self.FinishCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessTaskTakeRequest {
    pub TaskIds: Vec<i32>,
}

impl LivenessTaskTakeRequest {
    pub const MSG_ID: u16 = 29864;
}

impl PbMessage for LivenessTaskTakeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessTaskTakeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskIds: Vec<i32>,
}

impl LivenessTaskTakeResponse {
    pub const MSG_ID: u16 = 23181;
}

impl PbMessage for LivenessTaskTakeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskIds {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessCountUpdateNotify {
    pub LivenessCount: i32,
}

impl LivenessCountUpdateNotify {
    pub const MSG_ID: u16 = 22746;
}

impl PbMessage for LivenessCountUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LivenessCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LivenessCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.LivenessCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessUpdateNotify {
    pub LivenessInfo: Option<LivenessInfo>,
}

impl LivenessUpdateNotify {
    pub const MSG_ID: u16 = 20096;
}

impl PbMessage for LivenessUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LivenessInfo {
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.LivenessInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymBeginRequest {
    pub LoadGymId: i32,
}

impl LordGymBeginRequest {
    pub const MSG_ID: u16 = 16902;
}

impl PbMessage for LordGymBeginRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadGymId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadGymId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LoadGymId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymBeginResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LordGymBeginResponse {
    pub const MSG_ID: u16 = 25090;
}

impl PbMessage for LordGymBeginResponse {
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
pub struct LordGymReadRequest {
    pub LoadGymId: i32,
}

impl LordGymReadRequest {
    pub const MSG_ID: u16 = 22064;
}

impl PbMessage for LordGymReadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadGymId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadGymId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.LoadGymId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymReadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LordGymReadResponse {
    pub const MSG_ID: u16 = 21070;
}

impl PbMessage for LordGymReadResponse {
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
pub struct LordGymUnlockNotify {
    pub UnlockLoadGymIds: Vec<i32>,
}

impl LordGymUnlockNotify {
    pub const MSG_ID: u16 = 25755;
}

impl PbMessage for LordGymUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockLoadGymIds {
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
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockLoadGymIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymGroupInfoUpdateNotify {
    pub LordGymGroupInfos: Vec<LordGymGroupInfo>,
}

impl LordGymGroupInfoUpdateNotify {
    pub const MSG_ID: u16 = 22195;
}

impl PbMessage for LordGymGroupInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LordGymGroupInfos {
            crate::wire::write_submessage(out, 12, item);
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
                let bytes = r.read_len_delimited()?;
                self.LordGymGroupInfos
                    .push(<LordGymGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LordGymLevelPlayResultNotify {
    pub FailLoadGymId: i32,
    pub IsSuccess: bool,
    pub PassTime: i32,
    pub LordGymPassRecord: Option<LordGymPassRecord>,
    pub IsNewRecord: bool,
    pub IsPkgFull: bool,
    pub ItemDatas: Vec<ItemData>,
}

impl LordGymLevelPlayResultNotify {
    pub const MSG_ID: u16 = 17494;
}

impl PbMessage for LordGymLevelPlayResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FailLoadGymId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailLoadGymId);
        }
        if self.IsSuccess {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if let Some(v) = &self.LordGymPassRecord {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.IsNewRecord {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsPkgFull {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ItemDatas {
            crate::wire::write_submessage(out, 12, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FailLoadGymId = r.read_varint_i32()?,
            4 => self.IsSuccess = r.read_bool()?,
            7 => self.PassTime = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.LordGymPassRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.IsNewRecord = r.read_bool()?,
            11 => self.IsPkgFull = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ItemDatas.push(<ItemData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerRewardRequest {
    pub RewardIndexList: Vec<i32>,
    pub RewardIndex: i32,
    pub Difficulty: i32,
}

impl TowerRewardRequest {
    pub const MSG_ID: u16 = 27282;
}

impl PbMessage for TowerRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIndexList {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if self.Difficulty != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Difficulty);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardIndexList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => self.RewardIndex = r.read_varint_i32()?,
            8 => self.Difficulty = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerRewardResponse {
    pub NeedUpdateSeason: bool,
    pub ErrorCode: Option<ErrorCode>,
    pub RewardIndexList: Vec<i32>,
}

impl TowerRewardResponse {
    pub const MSG_ID: u16 = 16113;
}

impl PbMessage for TowerRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NeedUpdateSeason {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RewardIndexList {
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
            3 => self.NeedUpdateSeason = r.read_bool()?,
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardIndexList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerStartRequest {
    pub TowerConfigId: i32,
    pub Formation: Vec<TowerRolePb>,
}

impl TowerStartRequest {
    pub const MSG_ID: u16 = 19933;
}

impl PbMessage for TowerStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerConfigId);
        }
        for item in &self.Formation {
            crate::wire::write_submessage(out, 12, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.TowerConfigId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .push(<TowerRolePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerStartResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub NeedUpdateSeason: bool,
}

impl TowerStartResponse {
    pub const MSG_ID: u16 = 27533;
}

impl PbMessage for TowerStartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NeedUpdateSeason {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.NeedUpdateSeason = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerResetRequest {
    pub TowerConfigId: i32,
}

impl TowerResetRequest {
    pub const MSG_ID: u16 = 27650;
}

impl PbMessage for TowerResetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.TowerConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerResetResponse {
    pub NeedUpdateSeason: bool,
    pub TowerConfigId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerResetResponse {
    pub const MSG_ID: u16 = 26793;
}

impl PbMessage for TowerResetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NeedUpdateSeason {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerConfigId);
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
            9 => self.NeedUpdateSeason = r.read_bool()?,
            11 => self.TowerConfigId = r.read_varint_i32()?,
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
pub struct TowerEndNotify {
    pub Success: bool,
    pub NeedUpdateSeason: bool,
    pub CurrentFloorData: Option<TowerFloorPb>,
}

impl TowerEndNotify {
    pub const MSG_ID: u16 = 28183;
}

impl PbMessage for TowerEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Success {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NeedUpdateSeason {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CurrentFloorData {
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
            7 => self.Success = r.read_bool()?,
            8 => self.NeedUpdateSeason = r.read_bool()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CurrentFloorData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerInfoUpdateNotify {
    pub TowerInfo: Option<TowerInfo>,
}

impl TowerInfoUpdateNotify {
    pub const MSG_ID: u16 = 27219;
}

impl PbMessage for TowerInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TowerInfo {
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
                self.TowerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDifficultyUpdateNotify {
    pub TowerDifficulties: Vec<TowerDifficultyPb>,
}

impl TowerDifficultyUpdateNotify {
    pub const MSG_ID: u16 = 24752;
}

impl PbMessage for TowerDifficultyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TowerDifficulties {
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
                self.TowerDifficulties
                    .push(<TowerDifficultyPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerFloorUpdateNotify {
    pub TowerFloors: Vec<TowerFloorPb>,
}

impl TowerFloorUpdateNotify {
    pub const MSG_ID: u16 = 17128;
}

impl PbMessage for TowerFloorUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TowerFloors {
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
                self.TowerFloors
                    .push(<TowerFloorPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CdKeyVerifyRequest {
    pub CdKey: Vec<u8>,
}

impl CdKeyVerifyRequest {
    pub const MSG_ID: u16 = 17701;
}

impl PbMessage for CdKeyVerifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CdKey.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CdKey);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CdKey = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CdKeyVerifyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CdKeyVerifyResponse {
    pub const MSG_ID: u16 = 19513;
}

impl PbMessage for CdKeyVerifyResponse {
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
pub struct ChatRequest {
    pub ChannelId: i32,
    pub ChatParams: Vec<Vec<u8>>,
    pub Content: Vec<u8>,
}

impl ChatRequest {
    pub const MSG_ID: u16 = 28634;
}

impl PbMessage for ChatRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChannelId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChannelId);
        }
        for item in &self.ChatParams {
            crate::wire::write_tag(out, 13, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.ChannelId = r.read_varint_i32()?,
            13 => self.ChatParams.push(r.read_len_delimited()?.to_vec()),
            15 => self.Content = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatResponse {
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
    pub Data: BTreeMap<i32, Vec<u8>>,
}

impl ChatResponse {
    pub const MSG_ID: u16 = 26663;
}

impl PbMessage for ChatResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 7, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Data.is_empty() {
            crate::wire::write_map(out, 14, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => crate::wire::decode_map_entry(r, &mut self.Data)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelChatRequest {
    pub SubChannelType: Option<SubChatChannelType>,
    pub ChannelType: Option<ChatChannelType>,
    pub ChatContentType: Option<ChatContentType>,
    pub Content: Vec<u8>,
    pub XboxBlockedPlayerIds: Vec<i32>,
}

impl ChannelChatRequest {
    pub const MSG_ID: u16 = 18878;
}

impl PbMessage for ChannelChatRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SubChannelType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChannelType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChatContentType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
        for item in &self.XboxBlockedPlayerIds {
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
            2 => {
                self.SubChannelType = Some(
                    <SubChatChannelType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                self.ChannelType = Some(
                    <ChatChannelType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => {
                self.ChatContentType = Some(
                    <ChatContentType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.Content = r.read_len_delimited()?.to_vec(),
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.XboxBlockedPlayerIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelChatResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub BanEndTime: i64,
}

impl ChannelChatResponse {
    pub const MSG_ID: u16 = 20052;
}

impl PbMessage for ChannelChatResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BanEndTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BanEndTime);
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.BanEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingCabinPutRequest {
    pub CabinType: Option<CabinType>,
    pub Id: i32,
    pub RemoveIncrId: i32,
    pub RightFishingItems: Vec<FishingItemInfo>,
    pub LeftFishingItems: Vec<FishingItemInfo>,
}

impl FishingCabinPutRequest {
    pub const MSG_ID: u16 = 27290;
}

impl PbMessage for FishingCabinPutRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CabinType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RemoveIncrId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemoveIncrId);
        }
        for item in &self.RightFishingItems {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.LeftFishingItems {
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
            3 => {
                self.CabinType = Some(<CabinType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.Id = r.read_varint_i32()?,
            8 => self.RemoveIncrId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RightFishingItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.LeftFishingItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingCabinPutResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingCabinPutResponse {
    pub const MSG_ID: u16 = 27464;
}

impl PbMessage for FishingCabinPutResponse {
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
pub struct FishingSellRequest {
    pub IncrIds: Vec<i32>,
}

impl FishingSellRequest {
    pub const MSG_ID: u16 = 18289;
}

impl PbMessage for FishingSellRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.IncrIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncrIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSellResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingSellResponse {
    pub const MSG_ID: u16 = 23025;
}

impl PbMessage for FishingSellResponse {
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
pub struct FishingQuickSellRequest {}

impl FishingQuickSellRequest {
    pub const MSG_ID: u16 = 19986;
}

impl PbMessage for FishingQuickSellRequest {
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
pub struct FishingQuickSellResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingQuickSellResponse {
    pub const MSG_ID: u16 = 24478;
}

impl PbMessage for FishingQuickSellResponse {
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
pub struct FishingTechLevelUpRequest {
    pub NodeId: i32,
}

impl FishingTechLevelUpRequest {
    pub const MSG_ID: u16 = 18947;
}

impl PbMessage for FishingTechLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.NodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingTechLevelUpResponse {
    pub FishingTech: Option<FishingTechInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingTechLevelUpResponse {
    pub const MSG_ID: u16 = 24459;
}

impl PbMessage for FishingTechLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishingTech {
            crate::wire::write_submessage(out, 6, v);
        }
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FishingTech
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct AbyssChallenge {
    pub ChallengeId: i32,
    pub Unlock: bool,
    pub CanChallenge: bool,
    pub UnlockTime: i64,
    pub IsConditionUnlock: bool,
    pub MaxProgress: i32,
    pub MinPassTime: i32,
    pub Passed: bool,
}

impl PbMessage for AbyssChallenge {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanChallenge {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.IsConditionUnlock {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxProgress != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxProgress);
        }
        if self.MinPassTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinPassTime);
        }
        if self.Passed {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.Unlock = r.read_bool()?,
            3 => self.CanChallenge = r.read_bool()?,
            4 => self.UnlockTime = r.read_varint_i64()?,
            5 => self.IsConditionUnlock = r.read_bool()?,
            6 => self.MaxProgress = r.read_varint_i32()?,
            7 => self.MinPassTime = r.read_varint_i32()?,
            8 => self.Passed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeGotoNextRoomRequest {
    pub RoomId: i32,
}

impl AbyssChallengeGotoNextRoomRequest {
    pub const MSG_ID: u16 = 25389;
}

impl PbMessage for AbyssChallengeGotoNextRoomRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.RoomId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeGotoNextRoomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssChallengeGotoNextRoomResponse {
    pub const MSG_ID: u16 = 27237;
}

impl PbMessage for AbyssChallengeGotoNextRoomResponse {
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
pub struct AbyssChallengeInfo {
    pub PlayerInfos: Vec<AbyssChallengePassPlayerInfo>,
    pub PassTime: i32,
    pub ChallengeId: i32,
    pub IsSingle: bool,
    pub ShowName: bool,
    pub OwnerId: i32,
    pub PassWave: i32,
}

impl PbMessage for AbyssChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.IsSingle {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ShowName {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerId);
        }
        if self.PassWave != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassWave);
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
                self.PlayerInfos
                    .push(<AbyssChallengePassPlayerInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.PassTime = r.read_varint_i32()?,
            3 => self.ChallengeId = r.read_varint_i32()?,
            4 => self.IsSingle = r.read_bool()?,
            5 => self.ShowName = r.read_bool()?,
            6 => self.OwnerId = r.read_varint_i32()?,
            7 => self.PassWave = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengePassPlayerInfo {
    pub Name: Vec<u8>,
    pub PlayerId: i32,
    pub RoleInfos: Vec<AbyssPassRoleInfo>,
}

impl PbMessage for AbyssChallengePassPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.RoleInfos {
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
            1 => self.Name = r.read_len_delimited()?.to_vec(),
            2 => self.PlayerId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfos
                    .push(<AbyssPassRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeQuitRequest {}

impl AbyssChallengeQuitRequest {
    pub const MSG_ID: u16 = 20789;
}

impl PbMessage for AbyssChallengeQuitRequest {
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
pub struct AbyssChallengeQuitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssChallengeQuitResponse {
    pub const MSG_ID: u16 = 25360;
}

impl PbMessage for AbyssChallengeQuitResponse {
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
pub struct AbyssChallengeResultInfo {
    pub PassTime: i32,
    pub MinPassTime: i32,
    pub PlayerInfos: Vec<AbyssChallengeResultPlayerInfo>,
    pub Success: bool,
    pub RewardItems: Vec<RewardItem>,
    pub LikeRecord: Vec<AbyssLikeRecord>,
}

impl PbMessage for AbyssChallengeResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if self.MinPassTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinPassTime);
        }
        for item in &self.PlayerInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.Success {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RewardItems {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.LikeRecord {
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
            1 => self.PassTime = r.read_varint_i32()?,
            2 => self.MinPassTime = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfos
                    .push(<AbyssChallengeResultPlayerInfo as PbMessage>::decode(
                        bytes,
                    )?);
            }
            4 => self.Success = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItems
                    .push(<RewardItem as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.LikeRecord
                    .push(<AbyssLikeRecord as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeResultNotify {
    pub Result: Option<AbyssChallengeResultInfo>,
}

impl AbyssChallengeResultNotify {
    pub const MSG_ID: u16 = 27258;
}

impl PbMessage for AbyssChallengeResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
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
pub struct AbyssChallengeResultPlayerInfo {
    pub PlayerId: i32,
    pub LikeCount: i32,
    pub RoleInfos: Vec<AbyssChallengeResultRoleInfo>,
}

impl PbMessage for AbyssChallengeResultPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.LikeCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LikeCount);
        }
        for item in &self.RoleInfos {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.LikeCount = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfos
                    .push(<AbyssChallengeResultRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeResultRoleInfo {
    pub RoleSkinId: i32,
    pub RoleLevel: i32,
    pub LittleRoleId: i32,
    pub MainHonor: Option<AbyssChallengeRoleHonor>,
    pub OtherHonor: Vec<AbyssChallengeRoleHonor>,
}

impl PbMessage for AbyssChallengeResultRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.LittleRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LittleRoleId);
        }
        if let Some(v) = &self.MainHonor {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.OtherHonor {
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
            1 => self.RoleSkinId = r.read_varint_i32()?,
            2 => self.RoleLevel = r.read_varint_i32()?,
            3 => self.LittleRoleId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MainHonor
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.OtherHonor
                    .push(<AbyssChallengeRoleHonor as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeRoleHonor {
    pub Type: i32,
    pub Value: i32,
    pub RankValue: i32,
}

impl PbMessage for AbyssChallengeRoleHonor {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Type != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Value);
        }
        if self.RankValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankValue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Type = r.read_varint_i32()?,
            2 => self.Value = r.read_varint_i32()?,
            3 => self.RankValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeRoomInfoNotify {
    pub MaxScore: i32,
    pub GainBox: i32,
    pub MaxFloor: i32,
    pub RoomId: i32,
    pub CurScore: i32,
    pub ChallengeId: i32,
    pub CurFloor: i32,
    pub LikeRecord: Vec<AbyssLikeRecord>,
    pub RoleEquipMap: BTreeMap<i32, i32>,
    pub RouteId: i32,
    pub MaxBox: i32,
}

impl AbyssChallengeRoomInfoNotify {
    pub const MSG_ID: u16 = 18217;
}

impl PbMessage for AbyssChallengeRoomInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.GainBox != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GainBox);
        }
        if self.MaxFloor != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFloor);
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
        if self.CurScore != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurScore);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CurFloor != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurFloor);
        }
        for item in &self.LikeRecord {
            crate::wire::write_submessage(out, 9, item);
        }
        if !self.RoleEquipMap.is_empty() {
            crate::wire::write_map(out, 11, &self.RoleEquipMap);
        }
        if self.RouteId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RouteId);
        }
        if self.MaxBox != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxBox);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.MaxScore = r.read_varint_i32()?,
            3 => self.GainBox = r.read_varint_i32()?,
            4 => self.MaxFloor = r.read_varint_i32()?,
            5 => self.RoomId = r.read_varint_i32()?,
            6 => self.CurScore = r.read_varint_i32()?,
            7 => self.ChallengeId = r.read_varint_i32()?,
            8 => self.CurFloor = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.LikeRecord
                    .push(<AbyssLikeRecord as PbMessage>::decode(bytes)?);
            }
            11 => crate::wire::decode_map_entry(r, &mut self.RoleEquipMap)?,
            13 => self.RouteId = r.read_varint_i32()?,
            15 => self.MaxBox = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeSubLevelNotify {
    pub X: f32,
    pub A: f32,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
    pub Z: f32,
    pub RoomId: i32,
    pub Y: f32,
    pub LoadSubLevels: Vec<Vec<u8>>,
}

impl AbyssChallengeSubLevelNotify {
    pub const MSG_ID: u16 = 18059;
}

impl PbMessage for AbyssChallengeSubLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.A != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.A).to_bits().to_le_bytes());
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        for item in &self.LoadSubLevels {
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
            1 => self.X = r.read_f32()?,
            2 => self.A = r.read_f32()?,
            3 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            7 => self.Z = r.read_f32()?,
            10 => self.RoomId = r.read_varint_i32()?,
            11 => self.Y = r.read_f32()?,
            14 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeUnlockRequest {
    pub ChallengeId: i32,
}

impl AbyssChallengeUnlockRequest {
    pub const MSG_ID: u16 = 23937;
}

impl PbMessage for AbyssChallengeUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssChallengeUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssChallengeUnlockResponse {
    pub const MSG_ID: u16 = 21147;
}

impl PbMessage for AbyssChallengeUnlockResponse {
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
pub struct AbyssChallengeUpdateNotify {
    pub ChallengeInfos: Vec<AbyssChallenge>,
    pub UnlockChallengeIds: Vec<i32>,
}

impl AbyssChallengeUpdateNotify {
    pub const MSG_ID: u16 = 20812;
}

impl PbMessage for AbyssChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.UnlockChallengeIds {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfos
                    .push(<AbyssChallenge as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockChallengeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssEntityReviveTime {
    pub EntityId: i64,
    pub ReviveTime: i64,
}

impl PbMessage for AbyssEntityReviveTime {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ReviveTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReviveTime);
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
            2 => self.ReviveTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssInfo {
    pub RoleList: Vec<AbyssRoleInfo>,
    pub PluginItems: Vec<AbyssPluginItemInfo>,
    pub RewardInfo: Vec<AbyssRewardInfo>,
    pub UnlockChallengeIds: Vec<i32>,
    pub LikeCount: i32,
    pub ChallengeInfos: Vec<AbyssChallenge>,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for AbyssInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.PluginItems {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RewardInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.UnlockChallengeIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LikeCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LikeCount);
        }
        for item in &self.ChallengeInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.LimitBeginTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitBeginTime);
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
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
                self.RoleList
                    .push(<AbyssRoleInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PluginItems
                    .push(<AbyssPluginItemInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RewardInfo
                    .push(<AbyssRewardInfo as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockChallengeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.LikeCount = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfos
                    .push(<AbyssChallenge as PbMessage>::decode(bytes)?);
            }
            7 => self.LimitBeginTime = r.read_varint_i64()?,
            8 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssInstCtx {
    pub LittleRoleMap: BTreeMap<i32, i32>,
}

impl PbMessage for AbyssInstCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.LittleRoleMap.is_empty() {
            crate::wire::write_map(out, 1, &self.LittleRoleMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.LittleRoleMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssLikeNotify {
    pub LikeRecord: Vec<AbyssLikeRecord>,
    pub LikeCount: i32,
    pub PlayerId: i32,
}

impl AbyssLikeNotify {
    pub const MSG_ID: u16 = 22047;
}

impl PbMessage for AbyssLikeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LikeRecord {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.LikeCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LikeCount);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
                self.LikeRecord
                    .push(<AbyssLikeRecord as PbMessage>::decode(bytes)?);
            }
            9 => self.LikeCount = r.read_varint_i32()?,
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssLikeRecord {
    pub PlayerId: i32,
    pub LikedPlayerId: Vec<i32>,
}

impl PbMessage for AbyssLikeRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.LikedPlayerId {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LikedPlayerId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssLikeRequest {
    pub PlayerId: i32,
}

impl AbyssLikeRequest {
    pub const MSG_ID: u16 = 15542;
}

impl PbMessage for AbyssLikeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssLikeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssLikeResponse {
    pub const MSG_ID: u16 = 21792;
}

impl PbMessage for AbyssLikeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
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
pub struct AbyssLittleRoleLevelUpRequest {
    pub Level: i32,
    pub Id: i32,
}

impl AbyssLittleRoleLevelUpRequest {
    pub const MSG_ID: u16 = 21041;
}

impl PbMessage for AbyssLittleRoleLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
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
            3 => self.Level = r.read_varint_i32()?,
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssLittleRoleLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssLittleRoleLevelUpResponse {
    pub const MSG_ID: u16 = 22337;
}

impl PbMessage for AbyssLittleRoleLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
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
pub struct AbyssMatchChangeRoleContext {
    pub RoleEquipMap: BTreeMap<i32, i32>,
}

impl PbMessage for AbyssMatchChangeRoleContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RoleEquipMap.is_empty() {
            crate::wire::write_map(out, 1, &self.RoleEquipMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.RoleEquipMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssMatchRoleContext {
    pub RoleInfo: Option<AbyssRoleInfo>,
}

impl PbMessage for AbyssMatchRoleContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleInfo {
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
                self.RoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssOneRoleSelectInfo {
    pub RoleId: i32,
    pub LittleRoleId: i32,
    pub PluginItemId: Vec<i32>,
}

impl PbMessage for AbyssOneRoleSelectInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.LittleRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LittleRoleId);
        }
        for item in &self.PluginItemId {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.LittleRoleId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PluginItemId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPassRoleInfo {
    pub RoleSkinId: i32,
    pub RoleLevel: i32,
    pub LittleRoleId: i32,
    pub PluginItemIds: Vec<i32>,
}

impl PbMessage for AbyssPassRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.LittleRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LittleRoleId);
        }
        for item in &self.PluginItemIds {
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
            1 => self.RoleSkinId = r.read_varint_i32()?,
            2 => self.RoleLevel = r.read_varint_i32()?,
            3 => self.LittleRoleId = r.read_varint_i32()?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PluginItemIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginItemAddNotify {
    pub PluginItems: Vec<AbyssPluginItemInfo>,
}

impl AbyssPluginItemAddNotify {
    pub const MSG_ID: u16 = 15136;
}

impl PbMessage for AbyssPluginItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PluginItems {
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
                self.PluginItems
                    .push(<AbyssPluginItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginItemInfoUpdateNotify {
    pub PluginItems: Vec<AbyssPluginItemInfo>,
}

impl AbyssPluginItemInfoUpdateNotify {
    pub const MSG_ID: u16 = 23091;
}

impl PbMessage for AbyssPluginItemInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PluginItems {
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
                self.PluginItems
                    .push(<AbyssPluginItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginItemRemoveNotify {
    pub IncrIds: Vec<i32>,
}

impl AbyssPluginItemRemoveNotify {
    pub const MSG_ID: u16 = 25037;
}

impl PbMessage for AbyssPluginItemRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.IncrIds {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncrIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginPutRequest {
    pub LittleRoleId: i32,
    pub EquipItems: Vec<i32>,
}

impl AbyssPluginPutRequest {
    pub const MSG_ID: u16 = 27978;
}

impl PbMessage for AbyssPluginPutRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LittleRoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LittleRoleId);
        }
        for item in &self.EquipItems {
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
            6 => self.LittleRoleId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginPutResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssPluginPutResponse {
    pub const MSG_ID: u16 = 23547;
}

impl PbMessage for AbyssPluginPutResponse {
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
pub struct AbyssPluginSynthesisRequest {
    pub PluginItemIncrId: Vec<i32>,
}

impl AbyssPluginSynthesisRequest {
    pub const MSG_ID: u16 = 15330;
}

impl PbMessage for AbyssPluginSynthesisRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PluginItemIncrId {
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
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PluginItemIncrId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssPluginSynthesisResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub OutputItems: Vec<AddCountItemInfo>,
}

impl AbyssPluginSynthesisResponse {
    pub const MSG_ID: u16 = 20432;
}

impl PbMessage for AbyssPluginSynthesisResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.OutputItems {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.OutputItems
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRankListRequest {
    pub Id: i32,
}

impl AbyssRankListRequest {
    pub const MSG_ID: u16 = 21276;
}

impl PbMessage for AbyssRankListRequest {
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
pub struct AbyssRankListResponse {
    pub FriendPassData: Vec<AbyssChallengeInfo>,
    pub SelfPassData: Vec<AbyssChallengeInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssRankListResponse {
    pub const MSG_ID: u16 = 25849;
}

impl PbMessage for AbyssRankListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FriendPassData {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.SelfPassData {
            crate::wire::write_submessage(out, 8, item);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.FriendPassData
                    .push(<AbyssChallengeInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SelfPassData
                    .push(<AbyssChallengeInfo as PbMessage>::decode(bytes)?);
            }
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
pub struct AbyssReviveCharacterRequest {
    pub ReviveAll: bool,
    pub RoleId: i32,
}

impl AbyssReviveCharacterRequest {
    pub const MSG_ID: u16 = 19506;
}

impl PbMessage for AbyssReviveCharacterRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveAll {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            2 => self.ReviveAll = r.read_bool()?,
            13 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssReviveCharacterResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssReviveCharacterResponse {
    pub const MSG_ID: u16 = 17377;
}

impl PbMessage for AbyssReviveCharacterResponse {
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
pub struct AbyssReviveNotify {
    pub MaxReviveTimes: i32,
    pub CurRevieTimes: i32,
    pub CurReviveTimes: i32,
}

impl AbyssReviveNotify {
    pub const MSG_ID: u16 = 24492;
}

impl PbMessage for AbyssReviveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxReviveTimes != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReviveTimes);
        }
        if self.CurRevieTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRevieTimes);
        }
        if self.CurReviveTimes != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurReviveTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MaxReviveTimes = r.read_varint_i32()?,
            2 => self.CurRevieTimes = r.read_varint_i32()?,
            14 => self.CurReviveTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssReviveTimeNotify {
    pub ReviveTimeList: Vec<AbyssEntityReviveTime>,
}

impl AbyssReviveTimeNotify {
    pub const MSG_ID: u16 = 26847;
}

impl PbMessage for AbyssReviveTimeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReviveTimeList {
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ReviveTimeList
                    .push(<AbyssEntityReviveTime as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRewardRequest {
    pub Id: i32,
    pub Ids: Vec<i32>,
}

impl AbyssRewardRequest {
    pub const MSG_ID: u16 = 27914;
}

impl PbMessage for AbyssRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Ids {
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
            14 => self.Id = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssRewardResponse {
    pub const MSG_ID: u16 = 26593;
}

impl PbMessage for AbyssRewardResponse {
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
pub struct AbyssRewardsUpdateNotify {
    pub RewardInfos: Vec<AbyssRewardInfo>,
}

impl AbyssRewardsUpdateNotify {
    pub const MSG_ID: u16 = 21881;
}

impl PbMessage for AbyssRewardsUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardInfos {
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
                self.RewardInfos
                    .push(<AbyssRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRoleAddNotify {
    pub RoleList: Vec<AbyssRoleInfo>,
}

impl AbyssRoleAddNotify {
    pub const MSG_ID: u16 = 23255;
}

impl PbMessage for AbyssRoleAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
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
                self.RoleList
                    .push(<AbyssRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRoleInfo {
    pub Id: i32,
    pub Level: i32,
    pub EquipItems: Vec<i32>,
}

impl PbMessage for AbyssRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.EquipItems {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRoleInfoUpdateNotify {
    pub RoleList: Vec<AbyssRoleInfo>,
}

impl AbyssRoleInfoUpdateNotify {
    pub const MSG_ID: u16 = 17390;
}

impl PbMessage for AbyssRoleInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
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
                self.RoleList
                    .push(<AbyssRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRoleSelectInfo {
    pub PlayerId: i32,
    pub RoleInfos: Vec<AbyssOneRoleSelectInfo>,
}

impl PbMessage for AbyssRoleSelectInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.RoleInfos {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfos
                    .push(<AbyssOneRoleSelectInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssRoleSelectUpdateNotify {
    pub PlayerInfos: Vec<AbyssRoleSelectInfo>,
}

impl AbyssRoleSelectUpdateNotify {
    pub const MSG_ID: u16 = 19763;
}

impl PbMessage for AbyssRoleSelectUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerInfos {
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
                self.PlayerInfos
                    .push(<AbyssRoleSelectInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssSelfPassRequest {
    pub Id: i32,
}

impl AbyssSelfPassRequest {
    pub const MSG_ID: u16 = 25588;
}

impl PbMessage for AbyssSelfPassRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssSelfPassResponse {
    pub SelfPassData: Vec<AbyssChallengeInfo>,
}

impl AbyssSelfPassResponse {
    pub const MSG_ID: u16 = 15039;
}

impl PbMessage for AbyssSelfPassResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelfPassData {
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
                self.SelfPassData
                    .push(<AbyssChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssShowNameRequest {
    pub ShowName: bool,
    pub ChallengeId: i32,
}

impl AbyssShowNameRequest {
    pub const MSG_ID: u16 = 25707;
}

impl PbMessage for AbyssShowNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowName {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ShowName = r.read_bool()?,
            14 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AbyssShowNameResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AbyssShowNameResponse {
    pub const MSG_ID: u16 = 18840;
}

impl PbMessage for AbyssShowNameResponse {
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
pub struct AcceptEntrustRequest {
    pub RoleIds: Vec<i32>,
    pub EntrustId: i32,
}

impl AcceptEntrustRequest {
    pub const MSG_ID: u16 = 29331;
}

impl PbMessage for AcceptEntrustRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AcceptEntrustResponse {
    pub AddEntrust: Option<EntrustInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub RoleProperSettle: Vec<RoleProperSettle>,
    pub TriggerIdeaRole: i32,
    pub Wish: i32,
    pub Gold: i32,
    pub Heat: i32,
    pub TriggerIdea: bool,
    pub RoleGoods: Vec<RoleGood>,
    pub EvaluationLevel: i32,
}

impl AcceptEntrustResponse {
    pub const MSG_ID: u16 = 21620;
}

impl PbMessage for AcceptEntrustResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AddEntrust {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RoleProperSettle {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.TriggerIdeaRole != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TriggerIdeaRole);
        }
        if self.Wish != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Wish);
        }
        if self.Gold != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Gold);
        }
        if self.Heat != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Heat);
        }
        if self.TriggerIdea {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleGoods {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.EvaluationLevel != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EvaluationLevel);
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
                self.AddEntrust
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoleProperSettle
                    .push(<RoleProperSettle as PbMessage>::decode(bytes)?);
            }
            5 => self.TriggerIdeaRole = r.read_varint_i32()?,
            7 => self.Wish = r.read_varint_i32()?,
            8 => self.Gold = r.read_varint_i32()?,
            12 => self.Heat = r.read_varint_i32()?,
            13 => self.TriggerIdea = r.read_bool()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RoleGoods.push(<RoleGood as PbMessage>::decode(bytes)?);
            }
            15 => self.EvaluationLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessForceClosedAreaNotify {
    pub TreeOwnerId: i32,
    pub IsEnter: bool,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl AccessForceClosedAreaNotify {
    pub const MSG_ID: u16 = 27267;
}

impl PbMessage for AccessForceClosedAreaNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.IsEnter {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.TreeOwnerId = r.read_varint_i32()?,
            2 => self.IsEnter = r.read_bool()?,
            4 => self.NodeId = r.read_varint_i32()?,
            5 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessPasserbyNpcSpawnerRequest {
    pub EntityId: i64,
    pub IsSplneNotValid: bool,
    pub SplineConfigId: i32,
}

impl AccessPasserbyNpcSpawnerRequest {
    pub const MSG_ID: u16 = 21857;
}

impl PbMessage for AccessPasserbyNpcSpawnerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsSplneNotValid {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SplineConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.EntityId = r.read_varint_i64()?,
            8 => self.IsSplneNotValid = r.read_bool()?,
            11 => self.SplineConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessPasserbyNpcSpawnerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AccessPasserbyNpcSpawnerResponse {
    pub const MSG_ID: u16 = 17585;
}

impl PbMessage for AccessPasserbyNpcSpawnerResponse {
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
pub struct AccessPathConfig {
    pub Id: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for AccessPathConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.BeginTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessPathConfigRequest {}

impl AccessPathConfigRequest {
    pub const MSG_ID: u16 = 28506;
}

impl PbMessage for AccessPathConfigRequest {
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
pub struct AccessPathConfigResponse {
    pub Configs: Vec<AccessPathConfig>,
}

impl AccessPathConfigResponse {
    pub const MSG_ID: u16 = 22626;
}

impl PbMessage for AccessPathConfigResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Configs {
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
                self.Configs
                    .push(<AccessPathConfig as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AccessRangeResultMessage {
    pub AccessEntityId: i64,
    pub AccessRangeType: Option<AccessRangeType>,
    pub AccessRangeResults: BTreeMap<i32, ErrorCode>,
}

impl PbMessage for AccessRangeResultMessage {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AccessEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccessEntityId);
        }
        if let Some(v) = &self.AccessRangeType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.AccessRangeResults.is_empty() {
            crate::wire::write_map(out, 3, &self.AccessRangeResults);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AccessEntityId = r.read_varint_i64()?,
            2 => {
                self.AccessRangeType = Some(
                    <AccessRangeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => crate::wire::decode_map_entry(r, &mut self.AccessRangeResults)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AceAntiDataNotify {
    pub AntiData: Vec<u8>,
}

impl AceAntiDataNotify {
    pub const MSG_ID: u16 = 24754;
}

impl PbMessage for AceAntiDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AntiData.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AntiData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.AntiData = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AchievementUpdateRequest {}

impl AchievementUpdateRequest {
    pub const MSG_ID: u16 = 28975;
}

impl PbMessage for AchievementUpdateRequest {
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
pub struct AchievementUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AchievementEntryList: Vec<AchievementEntry>,
}

impl AchievementUpdateResponse {
    pub const MSG_ID: u16 = 22115;
}

impl PbMessage for AchievementUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AchievementEntryList {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AchievementEntryList
                    .push(<AchievementEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionErrorCodeNotify {
    pub GameCtx: Option<GameCtxPb>,
    pub Code: Option<ErrorCode>,
    pub IncId: i32,
    pub Index: i32,
}

impl ActionErrorCodeNotify {
    pub const MSG_ID: u16 = 21270;
}

impl PbMessage for ActionErrorCodeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.IncId = r.read_varint_i32()?,
            14 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionGroupCompleteNotify {
    pub PlayerId: i32,
    pub IncId: i32,
}

impl ActionGroupCompleteNotify {
    pub const MSG_ID: u16 = 17755;
}

impl PbMessage for ActionGroupCompleteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
            1 => self.PlayerId = r.read_varint_i32()?,
            13 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionGroupCreateNotify {
    pub GameCtx: Option<GameCtxPb>,
    pub PlayerId: i32,
    pub IncId: i32,
}

impl ActionGroupCreateNotify {
    pub const MSG_ID: u16 = 25896;
}

impl PbMessage for ActionGroupCreateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.PlayerId = r.read_varint_i32()?,
            14 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionNotify {
    pub GameCtx: Option<GameCtxPb>,
    pub TotalCount: i32,
    pub PlayerId: i32,
    pub StartIndex: i32,
    pub EndIndex: i32,
    pub NeedFinishReq: bool,
    pub IncId: i32,
}

impl ActionNotify {
    pub const MSG_ID: u16 = 27631;
}

impl PbMessage for ActionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.StartIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartIndex);
        }
        if self.EndIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndIndex);
        }
        if self.NeedFinishReq {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.TotalCount = r.read_varint_i32()?,
            6 => self.PlayerId = r.read_varint_i32()?,
            8 => self.StartIndex = r.read_varint_i32()?,
            10 => self.EndIndex = r.read_varint_i32()?,
            11 => self.NeedFinishReq = r.read_bool()?,
            12 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionOpenSystemBoardNotify {
    pub Param: Option<ActionOpenSystemParam>,
    pub PlayerId: i32,
}

impl ActionOpenSystemBoardNotify {
    pub const MSG_ID: u16 = 21125;
}

impl PbMessage for ActionOpenSystemBoardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Param {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
                self.Param
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionOpenSystemParam {
    pub ActionOpenType: Option<ActionOpenSystemType>,
    pub IncId: i32,
    pub OpenSystemBoardData: Option<OpenSystemBoardData>,
    pub SoaringChallengeData: Option<SoaringChallengeData>,
    pub FishingHandInData: Option<FishingHandInData>,
    pub GreatSwordChallengeData: Option<GreatSwordChallengeData>,
    pub InfrHandInData: Option<InfrHandInData>,
    pub MotorRaceChallengeData: Option<MotorRaceChallengeData>,
    pub InfrV2HandInData: Option<InfrV2HandInData>,
    pub GiantSystemBoardData: Option<GiantSystemBoardData>,
}

impl PbMessage for ActionOpenSystemParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActionOpenType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.OpenSystemBoardData {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.SoaringChallengeData {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.FishingHandInData {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.GreatSwordChallengeData {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.InfrHandInData {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.MotorRaceChallengeData {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.InfrV2HandInData {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.GiantSystemBoardData {
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
                self.ActionOpenType = Some(
                    <ActionOpenSystemType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.IncId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.OpenSystemBoardData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SoaringChallengeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FishingHandInData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.GreatSwordChallengeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.InfrHandInData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MotorRaceChallengeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.InfrV2HandInData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.GiantSystemBoardData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionOperationScreenNotify {
    pub GameCtx: Option<GameCtxPb>,
    pub IncId: i32,
    pub Type: Option<ActionScreenOperationType>,
    pub Params: Vec<u8>,
    pub PlayerId: i32,
}

impl ActionOperationScreenNotify {
    pub const MSG_ID: u16 = 16381;
}

impl PbMessage for ActionOperationScreenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Params.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Params);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.IncId = r.read_varint_i32()?,
            6 => {
                self.Type = Some(
                    <ActionScreenOperationType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.Params = r.read_len_delimited()?.to_vec(),
            13 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionOperationScreenSuccessRequest {
    pub IncId: i32,
    pub PlayerId: i32,
    pub Type: Option<ActionScreenOperationType>,
}

impl ActionOperationScreenSuccessRequest {
    pub const MSG_ID: u16 = 15213;
}

impl PbMessage for ActionOperationScreenSuccessRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Type {
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
            8 => self.IncId = r.read_varint_i32()?,
            10 => self.PlayerId = r.read_varint_i32()?,
            14 => {
                self.Type = Some(
                    <ActionScreenOperationType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ActionOperationScreenSuccessResponse {
    pub Code: Option<ErrorCode>,
}

impl ActionOperationScreenSuccessResponse {
    pub const MSG_ID: u16 = 28397;
}

impl PbMessage for ActionOperationScreenSuccessResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct ActionRelatedEntityInfoNotify {
    pub EntityStateChangeResult: Vec<EntityStateChangeResult>,
    pub EntityDestroyResult: Vec<EntityDestroyResult>,
}

impl ActionRelatedEntityInfoNotify {
    pub const MSG_ID: u16 = 24779;
}

impl PbMessage for ActionRelatedEntityInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityStateChangeResult {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.EntityDestroyResult {
            crate::wire::write_submessage(out, 8, item);
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
                self.EntityStateChangeResult
                    .push(<EntityStateChangeResult as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.EntityDestroyResult
                    .push(<EntityDestroyResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionResetPosPrePerformanceEndRequest {
    pub PlayerId: i32,
    pub Index: i32,
    pub ContinueExecute: bool,
    pub IncId: i32,
}

impl ActionResetPosPrePerformanceEndRequest {
    pub const MSG_ID: u16 = 19745;
}

impl PbMessage for ActionResetPosPrePerformanceEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.ContinueExecute {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            1 => self.PlayerId = r.read_varint_i32()?,
            5 => self.Index = r.read_varint_i32()?,
            8 => self.ContinueExecute = r.read_bool()?,
            14 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActionResetPosPrePerformanceEndResponse {
    pub Code: Option<ErrorCode>,
}

impl ActionResetPosPrePerformanceEndResponse {
    pub const MSG_ID: u16 = 29352;
}

impl PbMessage for ActionResetPosPrePerformanceEndResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct ActionResetPosPrePerformanceNotify {
    pub Params: Vec<u8>,
    pub GameCtx: Option<GameCtxPb>,
    pub IncId: i32,
    pub Index: i32,
    pub PlayerId: i32,
}

impl ActionResetPosPrePerformanceNotify {
    pub const MSG_ID: u16 = 27087;
}

impl PbMessage for ActionResetPosPrePerformanceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Params.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Params);
        }
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Params = r.read_len_delimited()?.to_vec(),
            5 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.IncId = r.read_varint_i32()?,
            9 => self.Index = r.read_varint_i32()?,
            14 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivateBuffPush {
    pub Handle: i32,
    pub On: bool,
}

impl ActivateBuffPush {
    pub const MSG_ID: u16 = 16028;
}

impl PbMessage for ActivateBuffPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.On {
            crate::wire::write_tag(out, 15, WireType::Varint);
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
            13 => self.Handle = r.read_varint_i32()?,
            15 => self.On = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActiveDragonPoolRequest {
    pub DragonPoolId: i32,
}

impl ActiveDragonPoolRequest {
    pub const MSG_ID: u16 = 27171;
}

impl PbMessage for ActiveDragonPoolRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.DragonPoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActiveDragonPoolResponse {
    pub DragonPoolId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ActiveStatus: i32,
}

impl ActiveDragonPoolResponse {
    pub const MSG_ID: u16 = 25761;
}

impl PbMessage for ActiveDragonPoolResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActiveStatus != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveStatus);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.DragonPoolId = r.read_varint_i32()?,
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.ActiveStatus = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityBuyItem {
    pub Id: i32,
    pub Count: i32,
}

impl PbMessage for ActivityBuyItem {
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
pub struct ActivityDisableNotify {
    pub ActivityIds: Vec<i32>,
}

impl ActivityDisableNotify {
    pub const MSG_ID: u16 = 18605;
}

impl PbMessage for ActivityDisableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityIds {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActivityIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityFirstReadRequest {
    pub ActivityId: i32,
}

impl ActivityFirstReadRequest {
    pub const MSG_ID: u16 = 19271;
}

impl PbMessage for ActivityFirstReadRequest {
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
pub struct ActivityFirstReadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ActivityFirstReadResponse {
    pub const MSG_ID: u16 = 19193;
}

impl PbMessage for ActivityFirstReadResponse {
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
pub struct ActivityLinkagePageData {
    pub PageActivityId: i32,
    pub EndTime: i64,
    pub Rewards: Vec<ActivityLinkageReward>,
    pub Receive: bool,
    pub StartTime: i64,
}

impl PbMessage for ActivityLinkagePageData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PageActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PageActivityId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.Rewards {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.Receive {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PageActivityId = r.read_varint_i32()?,
            2 => self.EndTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Rewards
                    .push(<ActivityLinkageReward as PbMessage>::decode(bytes)?);
            }
            4 => self.Receive = r.read_bool()?,
            5 => self.StartTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityLinkageReward {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for ActivityLinkageReward {
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
pub struct ActivityPayShopBuyRequest {
    pub BuyItems: Vec<ActivityBuyItem>,
    pub Version: Vec<u8>,
}

impl ActivityPayShopBuyRequest {
    pub const MSG_ID: u16 = 23899;
}

impl PbMessage for ActivityPayShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BuyItems {
            crate::wire::write_submessage(out, 3, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
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
                self.BuyItems
                    .push(<ActivityBuyItem as PbMessage>::decode(bytes)?);
            }
            14 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityPayShopBuyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub BuyItems: Vec<ActivityBuyItem>,
}

impl ActivityPayShopBuyResponse {
    pub const MSG_ID: u16 = 29072;
}

impl PbMessage for ActivityPayShopBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.BuyItems {
            crate::wire::write_submessage(out, 12, item);
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.BuyItems
                    .push(<ActivityBuyItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityPreOpenRequest {
    pub ActivityId: i32,
}

impl ActivityPreOpenRequest {
    pub const MSG_ID: u16 = 18144;
}

impl PbMessage for ActivityPreOpenRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityPreOpenResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ActivityPreOpenResponse {
    pub const MSG_ID: u16 = 26181;
}

impl PbMessage for ActivityPreOpenResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
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
pub struct ActivityRecommendNotify {
    pub RedDotStartTimeLatest: i64,
    pub Recommends: Vec<ActivityRecommendOnePb>,
}

impl ActivityRecommendNotify {
    pub const MSG_ID: u16 = 24619;
}

impl PbMessage for ActivityRecommendNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RedDotStartTimeLatest != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RedDotStartTimeLatest);
        }
        for item in &self.Recommends {
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
            6 => self.RedDotStartTimeLatest = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Recommends
                    .push(<ActivityRecommendOnePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityRecommendOnePb {
    pub ActivityId: i32,
    pub BeginShowTime: i64,
    pub BeginOpenTime: i64,
}

impl PbMessage for ActivityRecommendOnePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.BeginShowTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginShowTime);
        }
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
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
            2 => self.BeginShowTime = r.read_varint_i64()?,
            3 => self.BeginOpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityResultRewardRequest {
    pub ResultId: i32,
    pub ActivityId: i32,
}

impl ActivityResultRewardRequest {
    pub const MSG_ID: u16 = 19569;
}

impl PbMessage for ActivityResultRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResultId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResultId);
        }
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
            8 => self.ResultId = r.read_varint_i32()?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityResultRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TakeRewardResultIds: Vec<i32>,
}

impl ActivityResultRewardResponse {
    pub const MSG_ID: u16 = 18003;
}

impl PbMessage for ActivityResultRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TakeRewardResultIds {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TakeRewardResultIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityResultUpdateNotify {
    pub ActivityResultDatas: Vec<CiacconaActivityResultPbData>,
    pub ActivityId: i32,
}

impl ActivityResultUpdateNotify {
    pub const MSG_ID: u16 = 22938;
}

impl PbMessage for ActivityResultUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityResultDatas {
            crate::wire::write_submessage(out, 2, item);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityResultDatas
                    .push(<CiacconaActivityResultPbData as PbMessage>::decode(bytes)?);
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityUpdateNotify {
    pub Activities: Vec<ActivityData>,
}

impl ActivityUpdateNotify {
    pub const MSG_ID: u16 = 17864;
}

impl PbMessage for ActivityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Activities {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Activities
                    .push(<ActivityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddChapterResultRequest {
    pub ActivityId: i32,
    pub ChapterId: i32,
    pub ResultId: i32,
}

impl AddChapterResultRequest {
    pub const MSG_ID: u16 = 28807;
}

impl PbMessage for AddChapterResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterId);
        }
        if self.ResultId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResultId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.ActivityId = r.read_varint_i32()?,
            10 => self.ChapterId = r.read_varint_i32()?,
            11 => self.ResultId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddChapterResultResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AddChapterResultResponse {
    pub const MSG_ID: u16 = 19681;
}

impl PbMessage for AddChapterResultResponse {
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
pub struct AddDecalRequest {
    pub EffectId: i32,
    pub Rotation: Option<Vector>,
    pub SceneId: i32,
    pub Position: Option<Vector>,
}

impl AddDecalRequest {
    pub const MSG_ID: u16 = 29268;
}

impl PbMessage for AddDecalRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.SceneId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneId);
        }
        if let Some(v) = &self.Position {
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
            2 => self.EffectId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.SceneId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Position
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddDecalResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl AddDecalResponse {
    pub const MSG_ID: u16 = 19752;
}

impl PbMessage for AddDecalResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct AddEntityAoiNotify {
    pub EntityInfos: Vec<DynamicEntityInformation>,
}

impl AddEntityAoiNotify {
    pub const MSG_ID: u16 = 21567;
}

impl PbMessage for AddEntityAoiNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityInfos {
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
                self.EntityInfos
                    .push(<DynamicEntityInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddForbidControlNotify {
    pub ForbidList: Vec<ControlParam>,
}

impl AddForbidControlNotify {
    pub const MSG_ID: u16 = 19356;
}

impl PbMessage for AddForbidControlNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ForbidList {
            crate::wire::write_submessage(out, 12, item);
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
pub struct AddHackingEntityNotify {
    pub HackingEntityId: i64,
    pub AuxiliaryMachineEntityId: i64,
}

impl AddHackingEntityNotify {
    pub const MSG_ID: u16 = 28303;
}

impl PbMessage for AddHackingEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HackingEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HackingEntityId);
        }
        if self.AuxiliaryMachineEntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AuxiliaryMachineEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.HackingEntityId = r.read_varint_i64()?,
            11 => self.AuxiliaryMachineEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddMarkInfoNotify {
    pub Info: Option<MarkPointInfo>,
    pub TreasureBoxMarkInfo: Option<MarkTreasureBoxInfo>,
    pub AttachMark: Option<AttachMarkInfo>,
}

impl AddMarkInfoNotify {
    pub const MSG_ID: u16 = 19623;
}

impl PbMessage for AddMarkInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.TreasureBoxMarkInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.AttachMark {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TreasureBoxMarkInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.AttachMark
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddOccupationInfoNotify {
    pub OccupationInfo: Vec<OccupationPbInfo>,
}

impl AddOccupationInfoNotify {
    pub const MSG_ID: u16 = 19074;
}

impl PbMessage for AddOccupationInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OccupationInfo {
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
                self.OccupationInfo
                    .push(<OccupationPbInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddPhantomEquipGroupRequest {
    pub Name: Vec<u8>,
    pub RoleId: i32,
}

impl AddPhantomEquipGroupRequest {
    pub const MSG_ID: u16 = 18869;
}

impl PbMessage for AddPhantomEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            7 => self.Name = r.read_len_delimited()?.to_vec(),
            15 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddPhantomEquipGroupResponse {
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
    pub IsOldData: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl AddPhantomEquipGroupResponse {
    pub const MSG_ID: u16 = 17340;
}

impl PbMessage for AddPhantomEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomEquipGroupInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsOldData {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
            }
            6 => self.IsOldData = r.read_bool()?,
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
pub struct AddPlacementNotify {
    pub GridInfo: Option<OccupiedBoardGridInfo>,
    pub BoardEntityConfigId: i32,
}

impl AddPlacementNotify {
    pub const MSG_ID: u16 = 19989;
}

impl PbMessage for AddPlacementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GridInfo {
            crate::wire::write_submessage(out, 6, v);
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
            6 => {
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
pub struct AddReviveRegionRequest {
    pub ReviveId: i32,
}

impl AddReviveRegionRequest {
    pub const MSG_ID: u16 = 21181;
}

impl PbMessage for AddReviveRegionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.ReviveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddReviveRegionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AddReviveRegionResponse {
    pub const MSG_ID: u16 = 19029;
}

impl PbMessage for AddReviveRegionResponse {
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
pub struct AddRoleView {
    pub OldRogueResRole: Option<RogueResRole>,
    pub NewRogueResRole: Option<RogueResRole>,
}

impl PbMessage for AddRoleView {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OldRogueResRole {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.NewRogueResRole {
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
                self.OldRogueResRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NewRogueResRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddSysBuffNotify {
    pub BuffInfos: Vec<SysBuffInformation>,
    pub EntityId: i64,
}

impl AddSysBuffNotify {
    pub const MSG_ID: u16 = 19225;
}

impl PbMessage for AddSysBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BuffInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BuffInfos
                    .push(<SysBuffInformation as PbMessage>::decode(bytes)?);
            }
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddTokenView {
    pub Type: i32,
    pub RogueResGainDatas: Vec<RogueResGainData>,
}

impl PbMessage for AddTokenView {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Type != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        for item in &self.RogueResGainDatas {
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
            1 => self.Type = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGainDatas
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
