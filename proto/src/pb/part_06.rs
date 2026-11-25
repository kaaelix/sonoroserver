use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddTreasureBoxSlotRequest {}

impl AddTreasureBoxSlotRequest {
    pub const MSG_ID: u16 = 23884;
}

impl PbMessage for AddTreasureBoxSlotRequest {
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
pub struct AddTreasureBoxSlotResponse {
    pub Code: Option<ErrorCode>,
    pub TreasureBoxSlotInfo: Option<TreasureBoxSlotInfo>,
}

impl AddTreasureBoxSlotResponse {
    pub const MSG_ID: u16 = 16859;
}

impl PbMessage for AddTreasureBoxSlotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.TreasureBoxSlotInfo {
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
            5 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.TreasureBoxSlotInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddUnlockedDetectionTextNotify {
    pub UnlockedDetectionTextIds: Vec<i32>,
}

impl AddUnlockedDetectionTextNotify {
    pub const MSG_ID: u16 = 17144;
}

impl PbMessage for AddUnlockedDetectionTextNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedDetectionTextIds {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, *item as i32);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedDetectionTextIds,
                |sr| Ok(sr.read_sint32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddUnlockedGuideNotify {
    pub UnlockedGuideIds: Vec<i32>,
}

impl AddUnlockedGuideNotify {
    pub const MSG_ID: u16 = 28351;
}

impl PbMessage for AddUnlockedGuideNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedGuideIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, *item as i32);
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
                &mut self.UnlockedGuideIds,
                |sr| Ok(sr.read_sint32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureGuideCtx {
    pub DungeonDetectionId: i32,
}

impl PbMessage for AdventureGuideCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DungeonDetectionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DungeonDetectionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DungeonDetectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdventureSilentCtx {
    pub AdventureSilentId: i32,
}

impl PbMessage for AdventureSilentCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AdventureSilentId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AdventureSilentId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AdventureSilentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceContentUpdateNotify {
    pub Contents: Vec<PbAdviceContent>,
    pub Id: i64,
}

impl AdviceContentUpdateNotify {
    pub const MSG_ID: u16 = 16880;
}

impl PbMessage for AdviceContentUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Contents {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
                self.Contents
                    .push(<PbAdviceContent as PbMessage>::decode(bytes)?);
            }
            11 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceCreateRequest {
    pub Contents: Vec<PbAdviceContent>,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
}

impl AdviceCreateRequest {
    pub const MSG_ID: u16 = 21703;
}

impl PbMessage for AdviceCreateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Contents {
            crate::wire::write_submessage(out, 11, item);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.Rot {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Contents
                    .push(<PbAdviceContent as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceCreateResponse {
    pub Advice: Option<PbAdvice>,
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceCreateResponse {
    pub const MSG_ID: u16 = 19764;
}

impl PbMessage for AdviceCreateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Advice {
            crate::wire::write_submessage(out, 2, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Advice
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct AdviceDeleteRequest {
    pub Id: i64,
}

impl AdviceDeleteRequest {
    pub const MSG_ID: u16 = 18057;
}

impl PbMessage for AdviceDeleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceDeleteResponse {
    pub const MSG_ID: u16 = 28966;
}

impl PbMessage for AdviceDeleteResponse {
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
pub struct AdviceModifyRequest {
    pub Contents: Vec<PbAdviceContent>,
    pub Id: i64,
}

impl AdviceModifyRequest {
    pub const MSG_ID: u16 = 28074;
}

impl PbMessage for AdviceModifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Contents {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
                self.Contents
                    .push(<PbAdviceContent as PbMessage>::decode(bytes)?);
            }
            13 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceModifyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceModifyResponse {
    pub const MSG_ID: u16 = 18162;
}

impl PbMessage for AdviceModifyResponse {
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
pub struct AdviceUpdateNotify {
    pub UpVoteIds: Vec<i64>,
}

impl AdviceUpdateNotify {
    pub const MSG_ID: u16 = 21702;
}

impl PbMessage for AdviceUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UpVoteIds {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UpVoteIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AdviceVoteRequest {
    pub Id: i64,
    pub Type: Option<PbAdviceVoteType>,
}

impl AdviceVoteRequest {
    pub const MSG_ID: u16 = 23765;
}

impl PbMessage for AdviceVoteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.Type {
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
            5 => self.Id = r.read_varint_i64()?,
            8 => {
                self.Type = Some(
                    <PbAdviceVoteType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct AdviceVoteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AdviceVoteResponse {
    pub const MSG_ID: u16 = 18076;
}

impl PbMessage for AdviceVoteResponse {
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
pub struct AdviceVoteUpdateNotify {
    pub UpVote: i32,
    pub Id: i64,
}

impl AdviceVoteUpdateNotify {
    pub const MSG_ID: u16 = 15493;
}

impl PbMessage for AdviceVoteUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UpVote != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UpVote);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.UpVote = r.read_varint_i32()?,
            13 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AffixEntry {
    pub Id: i32,
    pub IsUnlock: bool,
    pub ElementDict: BTreeMap<i32, i32>,
}

impl PbMessage for AffixEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.ElementDict.is_empty() {
            crate::wire::write_map(out, 3, &self.ElementDict);
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
            2 => self.IsUnlock = r.read_bool()?,
            3 => crate::wire::decode_map_entry(r, &mut self.ElementDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AgreeJoinResultNotify {
    pub PlayerName: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl AgreeJoinResultNotify {
    pub const MSG_ID: u16 = 21974;
}

impl PbMessage for AgreeJoinResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
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
            1 => self.PlayerName = r.read_len_delimited()?.to_vec(),
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
pub struct AgreeJoinResultRequest {
    pub Way: Option<AgreeJoinWay>,
    pub Result: bool,
    pub PlayerId: i32,
}

impl AgreeJoinResultRequest {
    pub const MSG_ID: u16 = 16876;
}

impl PbMessage for AgreeJoinResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Way {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Result {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            6 => {
                self.Way = Some(<AgreeJoinWay as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.Result = r.read_bool()?,
            14 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AgreeJoinResultResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerName: Vec<u8>,
}

impl AgreeJoinResultResponse {
    pub const MSG_ID: u16 = 24721;
}

impl PbMessage for AgreeJoinResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
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
            11 => self.PlayerName = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ai_AiHateEntity {
    pub EntityId: i64,
    pub HatredValue: i32,
}

impl PbMessage for Ai_AiHateEntity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HatredValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HatredValue);
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
            2 => self.HatredValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiRoleComponentPb {
    pub IsAiRole: bool,
    pub ContextId: i64,
}

impl PbMessage for AiRoleComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAiRole {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ContextId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ContextId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsAiRole = r.read_bool()?,
            2 => self.ContextId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AlertArea {
    pub AreaId: i32,
    pub AlertValue: f32,
    pub ChangeSpeed: f32,
    pub EnableUi: bool,
    pub VisibleUi: bool,
}

impl PbMessage for AlertArea {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        if self.AlertValue != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.AlertValue).to_bits().to_le_bytes());
        }
        if self.ChangeSpeed != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ChangeSpeed).to_bits().to_le_bytes());
        }
        if self.EnableUi {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.VisibleUi {
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
            1 => self.AreaId = r.read_varint_i32()?,
            2 => self.AlertValue = r.read_f32()?,
            3 => self.ChangeSpeed = r.read_f32()?,
            4 => self.EnableUi = r.read_bool()?,
            5 => self.VisibleUi = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AlertAreaNotify {
    pub AlertAreas: Vec<AlertArea>,
    pub RemoveAreaIds: Vec<i32>,
}

impl AlertAreaNotify {
    pub const MSG_ID: u16 = 25643;
}

impl PbMessage for AlertAreaNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AlertAreas {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.RemoveAreaIds {
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.AlertAreas
                    .push(<AlertArea as PbMessage>::decode(bytes)?);
            }
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RemoveAreaIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AllApplyJoinNotify {
    pub ApplyList: Vec<ApplyJoinWorldNotify>,
}

impl AllApplyJoinNotify {
    pub const MSG_ID: u16 = 16708;
}

impl PbMessage for AllApplyJoinNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ApplyList {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ApplyList
                    .push(<ApplyJoinWorldNotify as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimalCollectComponentPb {
    pub PartIds: Vec<i32>,
}

impl PbMessage for AnimalCollectComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PartIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PartIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimalCollectStateNotify {
    pub PartId: i32,
    pub EntityId: i64,
    pub StateType: Option<AnimalCollectStateType>,
}

impl AnimalCollectStateNotify {
    pub const MSG_ID: u16 = 28587;
}

impl PbMessage for AnimalCollectStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PartId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.StateType {
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
            6 => self.PartId = r.read_varint_i32()?,
            12 => self.EntityId = r.read_varint_i64()?,
            14 => {
                self.StateType = Some(
                    <AnimalCollectStateType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct AnimalDieNotify {
    pub EntityId: i64,
    pub PlayerId: i32,
}

impl AnimalDieNotify {
    pub const MSG_ID: u16 = 24686;
}

impl PbMessage for AnimalDieNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            5 => self.EntityId = r.read_varint_i64()?,
            9 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AppGradeNotify {
    pub Type: i32,
    pub Id: i32,
    pub Delay: i32,
}

impl AppGradeNotify {
    pub const MSG_ID: u16 = 26027;
}

impl PbMessage for AppGradeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Type != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Delay != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Delay);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Type = r.read_varint_i32()?,
            12 => self.Id = r.read_varint_i32()?,
            15 => self.Delay = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyBuffFailedNotify {
    pub BuffId: i64,
    pub InstigatorId: i64,
    pub TransferContextId: Option<BattleDefine_TransferContextId>,
    pub StackCount: i32,
}

impl ApplyBuffFailedNotify {
    pub const MSG_ID: u16 = 19969;
}

impl PbMessage for ApplyBuffFailedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.TransferContextId {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BuffId = r.read_varint_i64()?,
            3 => self.InstigatorId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TransferContextId
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.StackCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyJoinWorldNotify {
    pub XboxOnlineId: Vec<u8>,
    pub HeadId: i32,
    pub PlayerId: i32,
    pub Level: i32,
    pub PsnAccountId: Vec<u8>,
    pub PsnOnlineId: Vec<u8>,
    pub PlayerName: Vec<u8>,
    pub PsnUserId: Vec<u8>,
    pub XboxAccountId: Vec<u8>,
    pub RefuseTimestamp: i64,
    pub XboxUserId: Vec<u8>,
}

impl ApplyJoinWorldNotify {
    pub const MSG_ID: u16 = 27800;
}

impl PbMessage for ApplyJoinWorldNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.XboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxOnlineId);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if !self.PsnAccountId.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnAccountId);
        }
        if !self.PsnOnlineId.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnOnlineId);
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
        if !self.PsnUserId.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnUserId);
        }
        if !self.XboxAccountId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxAccountId);
        }
        if self.RefuseTimestamp != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RefuseTimestamp);
        }
        if !self.XboxUserId.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxUserId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.XboxOnlineId = r.read_len_delimited()?.to_vec(),
            2 => self.HeadId = r.read_varint_i32()?,
            4 => self.PlayerId = r.read_varint_i32()?,
            5 => self.Level = r.read_varint_i32()?,
            6 => self.PsnAccountId = r.read_len_delimited()?.to_vec(),
            7 => self.PsnOnlineId = r.read_len_delimited()?.to_vec(),
            8 => self.PlayerName = r.read_len_delimited()?.to_vec(),
            9 => self.PsnUserId = r.read_len_delimited()?.to_vec(),
            10 => self.XboxAccountId = r.read_len_delimited()?.to_vec(),
            14 => self.RefuseTimestamp = r.read_varint_i64()?,
            15 => self.XboxUserId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyJoinWorldRequest {
    pub PlayerId: i32,
    pub Ways: Option<WorldEnterWay>,
}

impl ApplyJoinWorldRequest {
    pub const MSG_ID: u16 = 17687;
}

impl PbMessage for ApplyJoinWorldRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Ways {
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
            5 => self.PlayerId = r.read_varint_i32()?,
            6 => {
                self.Ways = Some(<WorldEnterWay as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyJoinWorldResponse {
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ApplyJoinWorldResponse {
    pub const MSG_ID: u16 = 15396;
}

impl PbMessage for ApplyJoinWorldResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 3, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            3 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
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
pub struct ApplyRechallengeRequest {
    pub Reason: Option<ApplyRechallengeReason>,
}

impl ApplyRechallengeRequest {
    pub const MSG_ID: u16 = 24437;
}

impl PbMessage for ApplyRechallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
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
                self.Reason = Some(
                    <ApplyRechallengeReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ApplyRechallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ApplyRechallengeResponse {
    pub const MSG_ID: u16 = 29801;
}

impl PbMessage for ApplyRechallengeResponse {
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
pub struct ApplyerEnterSceneNotify {
    pub PlayerId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl ApplyerEnterSceneNotify {
    pub const MSG_ID: u16 = 24262;
}

impl PbMessage for ApplyerEnterSceneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
            7 => self.PlayerId = r.read_varint_i32()?,
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
pub struct AreaSealInfo {
    pub UId: i32,
    pub AreaType: Option<BattleAreaType>,
    pub IsSeal: bool,
    pub UnLockRound: i32,
}

impl PbMessage for AreaSealInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UId);
        }
        if let Some(v) = &self.AreaType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsSeal {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnLockRound != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockRound);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.UId = r.read_varint_i32()?,
            2 => {
                self.AreaType = Some(
                    <BattleAreaType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.IsSeal = r.read_bool()?,
            4 => self.UnLockRound = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaTerminalFuncConditionInfo {
    pub FuncId: i32,
    pub FinishConditionIds: Vec<i32>,
}

impl PbMessage for AreaTerminalFuncConditionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FuncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncId);
        }
        for item in &self.FinishConditionIds {
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
            1 => self.FuncId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishConditionIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaTerminalInfoNotify {
    pub AreaTerminalFuncConditionInfos: Vec<AreaTerminalFuncConditionInfo>,
    pub PinIds: Vec<i32>,
    pub AreaMapGroup: Vec<i32>,
}

impl AreaTerminalInfoNotify {
    pub const MSG_ID: u16 = 19616;
}

impl PbMessage for AreaTerminalInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaTerminalFuncConditionInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.PinIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AreaMapGroup {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AreaTerminalFuncConditionInfos
                    .push(<AreaTerminalFuncConditionInfo as PbMessage>::decode(bytes)?);
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AreaMapGroup, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaTerminalPinOperationRequest {
    pub Id: i32,
    pub OpType: bool,
}

impl AreaTerminalPinOperationRequest {
    pub const MSG_ID: u16 = 21848;
}

impl PbMessage for AreaTerminalPinOperationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.OpType {
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
            2 => self.Id = r.read_varint_i32()?,
            9 => self.OpType = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaTerminalPinOperationResponse {
    pub Code: Option<ErrorCode>,
}

impl AreaTerminalPinOperationResponse {
    pub const MSG_ID: u16 = 27615;
}

impl PbMessage for AreaTerminalPinOperationResponse {
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
pub struct ArtemisActivityInfo {
    pub ActivityId: i32,
    pub UnlockIndex: i32,
    pub RewardedIndex: i32,
}

impl PbMessage for ArtemisActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.UnlockIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockIndex);
        }
        if self.RewardedIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedIndex);
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
            2 => self.UnlockIndex = r.read_varint_i32()?,
            3 => self.RewardedIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArtemisNodeUpdateNotify {
    pub ActivityId: i32,
    pub RewardedIndex: i32,
    pub UnlockIndex: i32,
}

impl ArtemisNodeUpdateNotify {
    pub const MSG_ID: u16 = 21651;
}

impl PbMessage for ArtemisNodeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.RewardedIndex != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedIndex);
        }
        if self.UnlockIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockIndex);
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
            12 => self.RewardedIndex = r.read_varint_i32()?,
            14 => self.UnlockIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArtemisNodeUpdateRequest {
    pub Index: i32,
    pub ActivityId: i32,
}

impl ArtemisNodeUpdateRequest {
    pub const MSG_ID: u16 = 26355;
}

impl PbMessage for ArtemisNodeUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
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
            4 => self.Index = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ArtemisNodeUpdateResponse {
    pub Error: Option<ErrorCode>,
}

impl ArtemisNodeUpdateResponse {
    pub const MSG_ID: u16 = 18649;
}

impl PbMessage for ArtemisNodeUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtomicProcessAddBuffRequest {
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub EntityId: i32,
    pub BuffIds: Vec<i64>,
}

impl AtomicProcessAddBuffRequest {
    pub const MSG_ID: u16 = 27678;
}

impl PbMessage for AtomicProcessAddBuffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.TreeIncId = r.read_varint_i64()?,
            11 => self.TreeOwnerId = r.read_varint_i32()?,
            12 => self.NodeId = r.read_varint_i32()?,
            13 => self.EntityId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtomicProcessAddBuffResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl AtomicProcessAddBuffResponse {
    pub const MSG_ID: u16 = 15504;
}

impl PbMessage for AtomicProcessAddBuffResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct AtomicProcessRemoveBuffRequest {
    pub TreeIncId: i64,
    pub EntityId: i32,
    pub NodeId: i32,
    pub BuffIds: Vec<i64>,
    pub TreeOwnerId: i32,
}

impl AtomicProcessRemoveBuffRequest {
    pub const MSG_ID: u16 = 19269;
}

impl PbMessage for AtomicProcessRemoveBuffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            1 => self.TreeIncId = r.read_varint_i64()?,
            2 => self.EntityId = r.read_varint_i32()?,
            6 => self.NodeId = r.read_varint_i32()?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            12 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtomicProcessRemoveBuffResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl AtomicProcessRemoveBuffResponse {
    pub const MSG_ID: u16 = 24883;
}

impl PbMessage for AtomicProcessRemoveBuffResponse {
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
pub struct AtomicProcessTeleportRequest {
    pub NodeId: i32,
    pub TreeOwnerId: i32,
    pub SplineEntityId: i32,
    pub TreeIncId: i64,
}

impl AtomicProcessTeleportRequest {
    pub const MSG_ID: u16 = 24800;
}

impl PbMessage for AtomicProcessTeleportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
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
            1 => self.NodeId = r.read_varint_i32()?,
            9 => self.TreeOwnerId = r.read_varint_i32()?,
            12 => self.SplineEntityId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtomicProcessTeleportResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl AtomicProcessTeleportResponse {
    pub const MSG_ID: u16 = 27008;
}

impl PbMessage for AtomicProcessTeleportResponse {
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
pub struct AttachEntityRequest {
    pub RelatePos: Option<Vector>,
    pub EntityId: i64,
    pub TargetType: Option<AttachTargetType>,
    pub AttachPath: Vec<u8>,
    pub AttachEntity: Option<AttachTargetEntityInfo>,
    pub RelateRot: Option<Rotator>,
}

impl AttachEntityRequest {
    pub const MSG_ID: u16 = 25142;
}

impl PbMessage for AttachEntityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RelatePos {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.TargetType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.AttachPath.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AttachPath);
        }
        if let Some(v) = &self.AttachEntity {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.RelateRot {
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
                let bytes = r.read_len_delimited()?;
                self.RelatePos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.EntityId = r.read_varint_i64()?,
            5 => {
                self.TargetType = Some(
                    <AttachTargetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => self.AttachPath = r.read_len_delimited()?.to_vec(),
            14 => {
                let bytes = r.read_len_delimited()?;
                self.AttachEntity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RelateRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttachEntityResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AttachEntityResponse {
    pub const MSG_ID: u16 = 18874;
}

impl PbMessage for AttachEntityResponse {
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
pub struct AttachTargetEntityInfo {
    pub TargetConfigId: i32,
    pub Actor: Vec<u8>,
}

impl PbMessage for AttachTargetEntityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetConfigId);
        }
        if !self.Actor.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Actor);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetConfigId = r.read_varint_i32()?,
            2 => self.Actor = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttrChangeSyncDataPb {
    pub AttrType: Option<SolarSchoolAttrType>,
    pub Delta: i32,
    pub AttrCost: i32,
    pub AttrAdd: i32,
}

impl PbMessage for AttrChangeSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AttrType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Delta != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Delta);
        }
        if self.AttrCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrCost);
        }
        if self.AttrAdd != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrAdd);
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
                self.AttrType = Some(
                    <SolarSchoolAttrType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Delta = r.read_varint_i32()?,
            3 => self.AttrCost = r.read_varint_i32()?,
            4 => self.AttrAdd = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttributeInitNotify {
    pub Attributes: Vec<GameplayAttributeData>,
    pub Id: i64,
}

impl AttributeInitNotify {
    pub const MSG_ID: u16 = 21138;
}

impl PbMessage for AttributeInitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attributes {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
                self.Attributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            10 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttributeInitRequest {
    pub Id: i64,
    pub Attributes: Vec<GameplayAttributeData>,
}

impl AttributeInitRequest {
    pub const MSG_ID: u16 = 18143;
}

impl PbMessage for AttributeInitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.Attributes {
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
            3 => self.Id = r.read_varint_i64()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Attributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttributeInitResponse {
    pub Success: bool,
    pub Id: i64,
}

impl AttributeInitResponse {
    pub const MSG_ID: u16 = 18934;
}

impl PbMessage for AttributeInitResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Success {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.Success = r.read_bool()?,
            11 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvignonActivityInfo {
    pub ActivityTaskDatas: Option<ActivityTaskData>,
    pub UnLockStepId: Vec<i32>,
}

impl PbMessage for AvignonActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTaskDatas {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.UnLockStepId {
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
                let bytes = r.read_len_delimited()?;
                self.ActivityTaskDatas
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnLockStepId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvignonRewardRequest {
    pub TaskId: i32,
    pub ActivityId: i32,
}

impl AvignonRewardRequest {
    pub const MSG_ID: u16 = 16650;
}

impl PbMessage for AvignonRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            7 => self.TaskId = r.read_varint_i32()?,
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvignonRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AvignonRewardResponse {
    pub const MSG_ID: u16 = 26226;
}

impl PbMessage for AvignonRewardResponse {
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
pub struct AvignonTaskNotify {
    pub UnLockStepId: i32,
    pub ActivityTask: Option<ActivityTask>,
}

impl AvignonTaskNotify {
    pub const MSG_ID: u16 = 21154;
}

impl PbMessage for AvignonTaskNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnLockStepId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockStepId);
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
            12 => self.UnLockStepId = r.read_varint_i32()?,
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
pub struct AvoidBlockClientInitDoneRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl AvoidBlockClientInitDoneRequest {
    pub const MSG_ID: u16 = 15216;
}

impl PbMessage for AvoidBlockClientInitDoneRequest {
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
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvoidBlockClientInitDoneResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl AvoidBlockClientInitDoneResponse {
    pub const MSG_ID: u16 = 23288;
}

impl PbMessage for AvoidBlockClientInitDoneResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct AvoidBlockRoundInfoNotify {
    pub RewardId: Vec<i32>,
    pub InstId: i32,
    pub HistoryPass: bool,
    pub HistoryScore: i32,
}

impl AvoidBlockRoundInfoNotify {
    pub const MSG_ID: u16 = 21465;
}

impl PbMessage for AvoidBlockRoundInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.HistoryPass {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HistoryScore != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => self.InstId = r.read_varint_i32()?,
            13 => self.HistoryPass = r.read_bool()?,
            15 => self.HistoryScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvoidBlockSettlementNotify {
    pub Win: bool,
    pub RewardItemMap: BTreeMap<i32, i32>,
    pub HistoryScore: i32,
    pub InstId: i32,
    pub IsFirstPass: bool,
}

impl AvoidBlockSettlementNotify {
    pub const MSG_ID: u16 = 23761;
}

impl PbMessage for AvoidBlockSettlementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Win {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.RewardItemMap.is_empty() {
            crate::wire::write_map(out, 8, &self.RewardItemMap);
        }
        if self.HistoryScore != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryScore);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.IsFirstPass {
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
            2 => self.Win = r.read_bool()?,
            8 => crate::wire::decode_map_entry(r, &mut self.RewardItemMap)?,
            9 => self.HistoryScore = r.read_varint_i32()?,
            12 => self.InstId = r.read_varint_i32()?,
            14 => self.IsFirstPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvoidBlockSubmitOverNodeRequest {
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
    pub Win: bool,
    pub NodeId: i32,
    pub Score: i32,
}

impl AvoidBlockSubmitOverNodeRequest {
    pub const MSG_ID: u16 = 28527;
}

impl PbMessage for AvoidBlockSubmitOverNodeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.Win {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.TreeIncId = r.read_varint_i64()?,
            4 => self.TreeOwnerId = r.read_varint_i32()?,
            9 => self.Win = r.read_bool()?,
            10 => self.NodeId = r.read_varint_i32()?,
            15 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvoidBlockSubmitOverNodeResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl AvoidBlockSubmitOverNodeResponse {
    pub const MSG_ID: u16 = 27806;
}

impl PbMessage for AvoidBlockSubmitOverNodeResponse {
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
pub struct AwakeDynamicPortalRequest {
    pub BulletId: i32,
    pub IsAwake: bool,
    pub EntityId: i64,
}

impl AwakeDynamicPortalRequest {
    pub const MSG_ID: u16 = 24473;
}

impl PbMessage for AwakeDynamicPortalRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletId);
        }
        if self.IsAwake {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => self.BulletId = r.read_varint_i32()?,
            5 => self.IsAwake = r.read_bool()?,
            7 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AwakeDynamicPortalResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AwakeDynamicPortalResponse {
    pub const MSG_ID: u16 = 17133;
}

impl PbMessage for AwakeDynamicPortalResponse {
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
pub struct AwakeItemVehicleRequest {
    pub PointKey: Vec<u8>,
    pub ItemID: i32,
    pub PosEntityId: i32,
}

impl AwakeItemVehicleRequest {
    pub const MSG_ID: u16 = 27784;
}

impl PbMessage for AwakeItemVehicleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PointKey.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PointKey);
        }
        if self.ItemID != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemID);
        }
        if self.PosEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PosEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.PointKey = r.read_len_delimited()?.to_vec(),
            14 => self.ItemID = r.read_varint_i32()?,
            15 => self.PosEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AwakeItemVehicleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AwakeItemVehicleResponse {
    pub const MSG_ID: u16 = 16446;
}

impl PbMessage for AwakeItemVehicleResponse {
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
pub struct BabelActivityBuffInfo {
    pub BabelBuffId: i32,
    pub DifficultUnlock: bool,
}

impl PbMessage for BabelActivityBuffInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BabelBuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BabelBuffId);
        }
        if self.DifficultUnlock {
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
            1 => self.BabelBuffId = r.read_varint_i32()?,
            2 => self.DifficultUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityBuffInfoNotify {
    pub BabelActivityBuffInfos: Vec<BabelActivityBuffInfo>,
}

impl BabelActivityBuffInfoNotify {
    pub const MSG_ID: u16 = 21577;
}

impl PbMessage for BabelActivityBuffInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BabelActivityBuffInfos {
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
                self.BabelActivityBuffInfos
                    .push(<BabelActivityBuffInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityDailyTaskInfoNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl BabelActivityDailyTaskInfoNotify {
    pub const MSG_ID: u16 = 23786;
}

impl PbMessage for BabelActivityDailyTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityDeTermInfo {
    pub DeTermId: i32,
    pub DifficultUnlock: bool,
}

impl PbMessage for BabelActivityDeTermInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DeTermId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DeTermId);
        }
        if self.DifficultUnlock {
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
            1 => self.DeTermId = r.read_varint_i32()?,
            2 => self.DifficultUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityDeTermInfoNotify {
    pub BabelActivityDeTermInfos: Vec<BabelActivityDeTermInfo>,
}

impl BabelActivityDeTermInfoNotify {
    pub const MSG_ID: u16 = 24618;
}

impl PbMessage for BabelActivityDeTermInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BabelActivityDeTermInfos {
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
                self.BabelActivityDeTermInfos
                    .push(<BabelActivityDeTermInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityInfo {
    pub BabelActivityLevelInfos: Vec<BabelActivityLevelInfo>,
    pub BabelActivityDeTermInfos: Vec<BabelActivityDeTermInfo>,
    pub BabelActivityBuffInfos: Vec<BabelActivityBuffInfo>,
    pub ActivityTasks: Vec<ActivityTask>,
    pub DailyActivityTask: Vec<ActivityTask>,
    pub ActivityItemNum: i32,
    pub ShowName: bool,
}

impl PbMessage for BabelActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BabelActivityLevelInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BabelActivityDeTermInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.BabelActivityBuffInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.DailyActivityTask {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.ActivityItemNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityItemNum);
        }
        if self.ShowName {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.BabelActivityLevelInfos
                    .push(<BabelActivityLevelInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BabelActivityDeTermInfos
                    .push(<BabelActivityDeTermInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BabelActivityBuffInfos
                    .push(<BabelActivityBuffInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DailyActivityTask
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            6 => self.ActivityItemNum = r.read_varint_i32()?,
            7 => self.ShowName = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityInstInfoNotify {
    pub RoleCD: i32,
    pub UseReviveCount: i32,
    pub CurStar: i32,
    pub BuffSelection: Vec<i32>,
    pub LevelId: i32,
    pub BabelDeTermId: Vec<i32>,
}

impl BabelActivityInstInfoNotify {
    pub const MSG_ID: u16 = 17488;
}

impl PbMessage for BabelActivityInstInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleCD != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleCD);
        }
        if self.UseReviveCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseReviveCount);
        }
        if self.CurStar != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStar);
        }
        for item in &self.BuffSelection {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.BabelDeTermId {
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
            4 => self.RoleCD = r.read_varint_i32()?,
            6 => self.UseReviveCount = r.read_varint_i32()?,
            7 => self.CurStar = r.read_varint_i32()?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffSelection, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.LevelId = r.read_varint_i32()?,
            15 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BabelDeTermId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityItemNumUpdateNotify {
    pub ActivityItemNum: i32,
}

impl BabelActivityItemNumUpdateNotify {
    pub const MSG_ID: u16 = 28082;
}

impl PbMessage for BabelActivityItemNumUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityItemNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityItemNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.ActivityItemNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityLevelInfo {
    pub LevelsId: i32,
    pub UnlockTime: i64,
    pub BabelDeTermId: Vec<i32>,
    pub LimitWeaponTypes: Vec<i32>,
    pub BuffSelection: Vec<i32>,
    pub RoleSelection: Vec<i32>,
    pub PassStar: i32,
    pub IsFinished: bool,
    pub MaxPassRoleSelection: Vec<i32>,
    pub MaxPassBuffSelection: Vec<i32>,
    pub MaxPassStar: i32,
    pub SkillBranchId: Vec<i32>,
    pub MaxPassUseTime: i32,
}

impl PbMessage for BabelActivityLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        for item in &self.BabelDeTermId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LimitWeaponTypes {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BuffSelection {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RoleSelection {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PassStar != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassStar);
        }
        if self.IsFinished {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.MaxPassRoleSelection {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MaxPassBuffSelection {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MaxPassStar != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxPassStar);
        }
        for item in &self.SkillBranchId {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MaxPassUseTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxPassUseTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LevelsId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BabelDeTermId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LimitWeaponTypes,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffSelection, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleSelection, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            7 => self.PassStar = r.read_varint_i32()?,
            8 => self.IsFinished = r.read_bool()?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MaxPassRoleSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MaxPassBuffSelection,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => self.MaxPassStar = r.read_varint_i32()?,
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => self.MaxPassUseTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityLevelInfoNotify {
    pub BabelActivityLevelInfos: Vec<BabelActivityLevelInfo>,
}

impl BabelActivityLevelInfoNotify {
    pub const MSG_ID: u16 = 21440;
}

impl PbMessage for BabelActivityLevelInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BabelActivityLevelInfos {
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
                self.BabelActivityLevelInfos
                    .push(<BabelActivityLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivitySettleInfoNotify {
    pub Star: i32,
    pub NewBabelBuffIds: Vec<i32>,
    pub UseTime: i32,
    pub NewBabelDeTermId: Vec<i32>,
    pub IsActivityClose: bool,
    pub PassTime: i64,
    pub IsPass: bool,
}

impl BabelActivitySettleInfoNotify {
    pub const MSG_ID: u16 = 16355;
}

impl PbMessage for BabelActivitySettleInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Star != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Star);
        }
        for item in &self.NewBabelBuffIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        for item in &self.NewBabelDeTermId {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsActivityClose {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassTime);
        }
        if self.IsPass {
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
            3 => self.Star = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.NewBabelBuffIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.UseTime = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.NewBabelDeTermId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => self.IsActivityClose = r.read_bool()?,
            13 => self.PassTime = r.read_varint_i64()?,
            14 => self.IsPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityTaskInfoNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl BabelActivityTaskInfoNotify {
    pub const MSG_ID: u16 = 21420;
}

impl PbMessage for BabelActivityTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelActivityUseReviveCountNotify {
    pub UseReviveCount: i32,
}

impl BabelActivityUseReviveCountNotify {
    pub const MSG_ID: u16 = 18819;
}

impl PbMessage for BabelActivityUseReviveCountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UseReviveCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseReviveCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.UseReviveCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelPassRoleInfo {
    pub RoleId: i32,
    pub RoleLevel: i32,
}

impl PbMessage for BabelPassRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
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
            2 => self.RoleLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerCtx {
    pub LevelsId: i32,
    pub SelectBuff: Vec<i32>,
    pub SkillBranchId: Vec<i32>,
}

impl PbMessage for BabelTowerCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        for item in &self.SelectBuff {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkillBranchId {
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
            1 => self.LevelsId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectBuff, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerDailyTaskRewardRequest {
    pub TaskId: i32,
}

impl BabelTowerDailyTaskRewardRequest {
    pub const MSG_ID: u16 = 25570;
}

impl PbMessage for BabelTowerDailyTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerDailyTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BabelTowerDailyTaskRewardResponse {
    pub const MSG_ID: u16 = 23540;
}

impl PbMessage for BabelTowerDailyTaskRewardResponse {
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
pub struct BabelTowerRankListRequest {
    pub ActivityId: i32,
}

impl BabelTowerRankListRequest {
    pub const MSG_ID: u16 = 29408;
}

impl PbMessage for BabelTowerRankListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerRankListResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfPassData: Option<PublicBabelTowerInfo>,
    pub FriendPassData: Vec<PublicBabelTowerInfo>,
}

impl BabelTowerRankListResponse {
    pub const MSG_ID: u16 = 16572;
}

impl PbMessage for BabelTowerRankListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.SelfPassData {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.FriendPassData {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SelfPassData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.FriendPassData
                    .push(<PublicBabelTowerInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerSelfPassRequest {
    pub ActivityId: i32,
}

impl BabelTowerSelfPassRequest {
    pub const MSG_ID: u16 = 27544;
}

impl PbMessage for BabelTowerSelfPassRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerSelfPassResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfPassData: Option<PublicBabelTowerInfo>,
}

impl BabelTowerSelfPassResponse {
    pub const MSG_ID: u16 = 15993;
}

impl PbMessage for BabelTowerSelfPassResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.SelfPassData {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SelfPassData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerSettlementRequest {}

impl BabelTowerSettlementRequest {
    pub const MSG_ID: u16 = 26063;
}

impl PbMessage for BabelTowerSettlementRequest {
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
pub struct BabelTowerSettlementResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BabelTowerSettlementResponse {
    pub const MSG_ID: u16 = 22271;
}

impl PbMessage for BabelTowerSettlementResponse {
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
pub struct BabelTowerShowNameRequest {
    pub ShowName: bool,
    pub ActivityId: i32,
}

impl BabelTowerShowNameRequest {
    pub const MSG_ID: u16 = 16067;
}

impl PbMessage for BabelTowerShowNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowName {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            1 => self.ShowName = r.read_bool()?,
            9 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerShowNameResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BabelTowerShowNameResponse {
    pub const MSG_ID: u16 = 28419;
}

impl PbMessage for BabelTowerShowNameResponse {
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
pub struct BabelTowerTaskRewardRequest {
    pub TaskIds: Vec<i32>,
    pub TaskId: i32,
    pub ActivityId: i32,
}

impl BabelTowerTaskRewardRequest {
    pub const MSG_ID: u16 = 24223;
}

impl PbMessage for BabelTowerTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.TaskId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BabelTowerTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BabelTowerTaskRewardResponse {
    pub const MSG_ID: u16 = 16607;
}

impl PbMessage for BabelTowerTaskRewardResponse {
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
pub struct BanChatNotify {
    pub BanEndTime: i64,
    pub BanChatType: Option<BanChatType>,
}

impl BanChatNotify {
    pub const MSG_ID: u16 = 23415;
}

impl PbMessage for BanChatNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BanEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BanEndTime);
        }
        if let Some(v) = &self.BanChatType {
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
            3 => self.BanEndTime = r.read_varint_i64()?,
            15 => {
                self.BanChatType = Some(<BanChatType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BanItemUseNotify {
    pub BanEndTime: i64,
    pub BanItemUseType: Option<BanItemUseType>,
}

impl BanItemUseNotify {
    pub const MSG_ID: u16 = 22869;
}

impl PbMessage for BanItemUseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BanEndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BanEndTime);
        }
        if let Some(v) = &self.BanItemUseType {
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
            2 => self.BanEndTime = r.read_varint_i64()?,
            8 => {
                self.BanItemUseType = Some(
                    <BanItemUseType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct BatchMaterialReplaceRequest {
    pub ReplaceItems: Vec<BatchReplaceItem>,
}

impl BatchMaterialReplaceRequest {
    pub const MSG_ID: u16 = 25690;
}

impl PbMessage for BatchMaterialReplaceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReplaceItems {
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
                self.ReplaceItems
                    .push(<BatchReplaceItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchMaterialReplaceResponse {
    pub Code: Option<ErrorCode>,
}

impl BatchMaterialReplaceResponse {
    pub const MSG_ID: u16 = 21431;
}

impl PbMessage for BatchMaterialReplaceResponse {
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
pub struct BatchReplaceItem {
    pub TargetReplaceItemId: i32,
    pub TargetNum: i32,
    pub ConsumeItems: BTreeMap<i32, i32>,
}

impl PbMessage for BatchReplaceItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetReplaceItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetReplaceItemId);
        }
        if self.TargetNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetNum);
        }
        if !self.ConsumeItems.is_empty() {
            crate::wire::write_map(out, 3, &self.ConsumeItems);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetReplaceItemId = r.read_varint_i32()?,
            2 => self.TargetNum = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.ConsumeItems)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BathChangeCameraRequest {
    pub Camera: Option<BathCameraType>,
}

impl BathChangeCameraRequest {
    pub const MSG_ID: u16 = 23750;
}

impl PbMessage for BathChangeCameraRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Camera {
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
                self.Camera = Some(
                    <BathCameraType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct BathChangeCameraResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BathChangeCameraResponse {
    pub const MSG_ID: u16 = 21598;
}

impl PbMessage for BathChangeCameraResponse {
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
pub struct BathPhotoRequest {}

impl BathPhotoRequest {
    pub const MSG_ID: u16 = 23123;
}

impl PbMessage for BathPhotoRequest {
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
pub struct BathPhotoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BathPhotoResponse {
    pub const MSG_ID: u16 = 16666;
}

impl PbMessage for BathPhotoResponse {
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
pub struct BattleAntiCheatingLogNotify {
    pub LogType: i32,
    pub RoomId: i64,
    pub LogId: i64,
    pub BattleId: i64,
    pub TimoutSpan: i32,
    pub InstId: i32,
}

impl BattleAntiCheatingLogNotify {
    pub const MSG_ID: u16 = 27447;
}

impl PbMessage for BattleAntiCheatingLogNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LogType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LogType);
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RoomId);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if self.BattleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BattleId);
        }
        if self.TimoutSpan != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimoutSpan);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            4 => self.LogType = r.read_varint_i32()?,
            6 => self.RoomId = r.read_varint_i64()?,
            7 => self.LogId = r.read_varint_i64()?,
            11 => self.BattleId = r.read_varint_i64()?,
            13 => self.TimoutSpan = r.read_varint_i32()?,
            15 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleDefine_TransferContextId {
    pub BulletContextId: i64,
}

impl PbMessage for BattleDefine_TransferContextId {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletContextId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletContextId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BulletContextId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleLogNotify {
    pub Log: Vec<u8>,
}

impl BattleLogNotify {
    pub const MSG_ID: u16 = 19877;
}

impl PbMessage for BattleLogNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Log.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Log);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.Log = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassEnterPush {}

impl BattlePassEnterPush {
    pub const MSG_ID: u16 = 20512;
}

impl PbMessage for BattlePassEnterPush {
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
pub struct BattlePassExpUpdateNotify {
    pub Exp: i32,
    pub RecurringRewards: Vec<PbBattlePassRecurringReward>,
    pub Level: i32,
    pub WeeklyTotalExp: i32,
}

impl BattlePassExpUpdateNotify {
    pub const MSG_ID: u16 = 20892;
}

impl PbMessage for BattlePassExpUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Exp != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        for item in &self.RecurringRewards {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.WeeklyTotalExp != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeeklyTotalExp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.Exp = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RecurringRewards
                    .push(<PbBattlePassRecurringReward as PbMessage>::decode(bytes)?);
            }
            11 => self.Level = r.read_varint_i32()?,
            13 => self.WeeklyTotalExp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassLevelUpRequest {
    pub Level: i32,
}

impl BattlePassLevelUpRequest {
    pub const MSG_ID: u16 = 26786;
}

impl PbMessage for BattlePassLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BattlePassLevelUpResponse {
    pub const MSG_ID: u16 = 19912;
}

impl PbMessage for BattlePassLevelUpResponse {
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
pub struct BattlePassPaidNotify {
    pub PayStatus: Option<BattlePassPayStatus>,
}

impl BattlePassPaidNotify {
    pub const MSG_ID: u16 = 17219;
}

impl PbMessage for BattlePassPaidNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PayStatus {
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
                self.PayStatus = Some(
                    <BattlePassPayStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct BattlePassRecurringTakeRequest {
    pub ItemId: i32,
    pub Type: Option<BattlePassType>,
}

impl BattlePassRecurringTakeRequest {
    pub const MSG_ID: u16 = 29679;
}

impl PbMessage for BattlePassRecurringTakeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
            6 => self.ItemId = r.read_varint_i32()?,
            10 => {
                self.Type = Some(
                    <BattlePassType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct BattlePassRecurringTakeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BattlePassRecurringTakeResponse {
    pub const MSG_ID: u16 = 27272;
}

impl PbMessage for BattlePassRecurringTakeResponse {
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
pub struct BattlePassTakeAllRewardRequest {}

impl BattlePassTakeAllRewardRequest {
    pub const MSG_ID: u16 = 21032;
}

impl PbMessage for BattlePassTakeAllRewardRequest {
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
pub struct BattlePassTakeAllRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RecurringRewards: Vec<PbBattlePassRecurringReward>,
    pub TakenRewards: Vec<PbBattlePassReward>,
}

impl BattlePassTakeAllRewardResponse {
    pub const MSG_ID: u16 = 27259;
}

impl PbMessage for BattlePassTakeAllRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RecurringRewards {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.TakenRewards {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RecurringRewards
                    .push(<PbBattlePassRecurringReward as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.TakenRewards
                    .push(<PbBattlePassReward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassTakeRewardRequest {
    pub ItemId: i32,
    pub Type: Option<BattlePassType>,
    pub Level: i32,
}

impl BattlePassTakeRewardRequest {
    pub const MSG_ID: u16 = 27378;
}

impl PbMessage for BattlePassTakeRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.ItemId = r.read_varint_i32()?,
            10 => {
                self.Type = Some(
                    <BattlePassType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassTakeRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BattlePassTakeRewardResponse {
    pub const MSG_ID: u16 = 29764;
}

impl PbMessage for BattlePassTakeRewardResponse {
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
pub struct BattlePassTaskRequest {}

impl BattlePassTaskRequest {
    pub const MSG_ID: u16 = 22950;
}

impl PbMessage for BattlePassTaskRequest {
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
pub struct BattlePassTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Tasks: Vec<PbBattlePassTask>,
    pub WeekEnd: i64,
    pub DayEnd: i64,
}

impl BattlePassTaskResponse {
    pub const MSG_ID: u16 = 27662;
}

impl PbMessage for BattlePassTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.WeekEnd != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.WeekEnd);
        }
        if self.DayEnd != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DayEnd);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks
                    .push(<PbBattlePassTask as PbMessage>::decode(bytes)?);
            }
            6 => self.WeekEnd = r.read_varint_i64()?,
            15 => self.DayEnd = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassTaskTakeRequest {
    pub Ids: Vec<i32>,
}

impl BattlePassTaskTakeRequest {
    pub const MSG_ID: u16 = 29651;
}

impl PbMessage for BattlePassTaskTakeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattlePassTaskTakeResponse {
    pub Ids: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl BattlePassTaskTakeResponse {
    pub const MSG_ID: u16 = 17148;
}

impl PbMessage for BattlePassTaskTakeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct BattlePassTaskUpdateNotify {
    pub Tasks: Vec<PbBattlePassTask>,
}

impl BattlePassTaskUpdateNotify {
    pub const MSG_ID: u16 = 17567;
}

impl PbMessage for BattlePassTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Tasks {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks
                    .push(<PbBattlePassTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleScoreEnableNotify {
    pub BattleScoreConfId: i32,
    pub Enable: bool,
}

impl BattleScoreEnableNotify {
    pub const MSG_ID: u16 = 24023;
}

impl PbMessage for BattleScoreEnableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BattleScoreConfId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BattleScoreConfId);
        }
        if self.Enable {
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
            6 => self.BattleScoreConfId = r.read_varint_i32()?,
            14 => self.Enable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleScoreNotify {
    pub ScoreTotal: i32,
    pub BattleScoreConfId: i32,
}

impl BattleScoreNotify {
    pub const MSG_ID: u16 = 25993;
}

impl PbMessage for BattleScoreNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ScoreTotal != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScoreTotal);
        }
        if self.BattleScoreConfId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BattleScoreConfId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ScoreTotal = r.read_varint_i32()?,
            9 => self.BattleScoreConfId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledDestroyRequest {
    pub EntityId: i64,
}

impl BeControlledDestroyRequest {
    pub const MSG_ID: u16 = 26102;
}

impl PbMessage for BeControlledDestroyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledDestroyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BeControlledDestroyResponse {
    pub const MSG_ID: u16 = 21659;
}

impl PbMessage for BeControlledDestroyResponse {
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
pub struct BeControlledEntityHoverRequest {
    pub EntityId: i64,
}

impl BeControlledEntityHoverRequest {
    pub const MSG_ID: u16 = 15955;
}

impl PbMessage for BeControlledEntityHoverRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledEntityHoverResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BeControlledEntityHoverResponse {
    pub const MSG_ID: u16 = 27351;
}

impl PbMessage for BeControlledEntityHoverResponse {
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
pub struct BeControlledNotify {
    pub NeedReset: bool,
    pub EntityId: i64,
    pub RoleEntityId: i64,
}

impl BeControlledNotify {
    pub const MSG_ID: u16 = 17537;
}

impl PbMessage for BeControlledNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NeedReset {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.RoleEntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RoleEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.NeedReset = r.read_bool()?,
            5 => self.EntityId = r.read_varint_i64()?,
            9 => self.RoleEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledPosInFoundationNotify {
    pub FoundationEntityId: i64,
    pub BeControlledEntityId: i64,
    pub InstId: i32,
    pub MatchIndex: i32,
}

impl BeControlledPosInFoundationNotify {
    pub const MSG_ID: u16 = 23363;
}

impl PbMessage for BeControlledPosInFoundationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FoundationEntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FoundationEntityId);
        }
        if self.BeControlledEntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeControlledEntityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.MatchIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.FoundationEntityId = r.read_varint_i64()?,
            7 => self.BeControlledEntityId = r.read_varint_i64()?,
            9 => self.InstId = r.read_varint_i32()?,
            14 => self.MatchIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledRequest {
    pub EntityId: i64,
    pub IsControl: bool,
}

impl BeControlledRequest {
    pub const MSG_ID: u16 = 16785;
}

impl PbMessage for BeControlledRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsControl {
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
            14 => self.EntityId = r.read_varint_i64()?,
            15 => self.IsControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BeControlledResponse {
    pub const MSG_ID: u16 = 24551;
}

impl PbMessage for BeControlledResponse {
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
pub struct BeControlledShowNotify {
    pub RoleEntityId: i64,
    pub EntityId: i64,
    pub IsShow: bool,
}

impl BeControlledShowNotify {
    pub const MSG_ID: u16 = 23631;
}

impl PbMessage for BeControlledShowNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RoleEntityId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsShow {
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
            3 => self.RoleEntityId = r.read_varint_i64()?,
            8 => self.EntityId = r.read_varint_i64()?,
            15 => self.IsShow = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledShowRequest {
    pub IsShow: bool,
    pub EntityId: i64,
}

impl BeControlledShowRequest {
    pub const MSG_ID: u16 = 20029;
}

impl PbMessage for BeControlledShowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            12 => self.IsShow = r.read_bool()?,
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledShowResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BeControlledShowResponse {
    pub const MSG_ID: u16 = 26477;
}

impl PbMessage for BeControlledShowResponse {
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
pub struct BeControlledThrowRequest {
    pub ControlState: Option<EControlState>,
    pub EntityId: i64,
    pub ThrowType: i32,
}

impl BeControlledThrowRequest {
    pub const MSG_ID: u16 = 25094;
}

impl PbMessage for BeControlledThrowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ControlState {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ThrowType != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ThrowType);
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
                self.ControlState =
                    Some(<EControlState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            5 => self.EntityId = r.read_varint_i64()?,
            9 => self.ThrowType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeControlledThrowResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BeControlledThrowResponse {
    pub const MSG_ID: u16 = 16431;
}

impl PbMessage for BeControlledThrowResponse {
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
pub struct BeamCastHitPlayerRequest {
    pub EntityId: i64,
}

impl BeamCastHitPlayerRequest {
    pub const MSG_ID: u16 = 24426;
}

impl PbMessage for BeamCastHitPlayerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeamCastHitPlayerResponse {
    pub Code: Option<ErrorCode>,
}

impl BeamCastHitPlayerResponse {
    pub const MSG_ID: u16 = 21555;
}

impl PbMessage for BeamCastHitPlayerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct BeamReceiveNotify {
    pub IsSatisfied: bool,
    pub EntityId: i64,
}

impl BeamReceiveNotify {
    pub const MSG_ID: u16 = 17895;
}

impl PbMessage for BeamReceiveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsSatisfied {
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
            11 => self.IsSatisfied = r.read_bool()?,
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeforeHitInformation {
    pub Id: i64,
    pub FightState: i32,
    pub TriggerCounterType: i32,
    pub CounterAnIndex: i32,
}

impl PbMessage for BeforeHitInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.FightState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightState);
        }
        if self.TriggerCounterType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TriggerCounterType);
        }
        if self.CounterAnIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CounterAnIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Id = r.read_varint_i64()?,
            2 => self.FightState = r.read_varint_i32()?,
            3 => self.TriggerCounterType = r.read_varint_i32()?,
            4 => self.CounterAnIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BeforeHitPush {
    pub BeforeHitInformation: Option<BeforeHitInformation>,
}

impl BeforeHitPush {
    pub const MSG_ID: u16 = 21161;
}

impl PbMessage for BeforeHitPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BeforeHitInformation {
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
                self.BeforeHitInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BehaveSyncDataPb {
    pub BehaviorId: i32,
    pub IsSuccess: bool,
    pub SpecialState: i32,
}

impl PbMessage for BehaveSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BehaviorId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BehaviorId);
        }
        if self.IsSuccess {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SpecialState != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpecialState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BehaviorId = r.read_varint_i32()?,
            2 => self.IsSuccess = r.read_bool()?,
            4 => self.SpecialState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthDayCardRoleRequest {
    pub RoleID: i32,
    pub YearId: i32,
}

impl BirthDayCardRoleRequest {
    pub const MSG_ID: u16 = 24371;
}

impl PbMessage for BirthDayCardRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleID != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleID);
        }
        if self.YearId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.YearId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.RoleID = r.read_varint_i32()?,
            13 => self.YearId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthDayCardRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BirthDayCardRoleResponse {
    pub const MSG_ID: u16 = 22249;
}

impl PbMessage for BirthDayCardRoleResponse {
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
pub struct BirthDayInfoNotify {
    pub RecentRewardTime: i32,
    pub BirthDayReset: bool,
    pub Roles: Vec<BirthRoleSelect>,
}

impl BirthDayInfoNotify {
    pub const MSG_ID: u16 = 20525;
}

impl PbMessage for BirthDayInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecentRewardTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecentRewardTime);
        }
        if self.BirthDayReset {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Roles {
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
            1 => self.RecentRewardTime = r.read_varint_i32()?,
            2 => self.BirthDayReset = r.read_bool()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .push(<BirthRoleSelect as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthDayRewardRequest {
    pub YearId: i32,
}

impl BirthDayRewardRequest {
    pub const MSG_ID: u16 = 17381;
}

impl PbMessage for BirthDayRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.YearId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.YearId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.YearId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthDayRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BirthDayRewardResponse {
    pub const MSG_ID: u16 = 22398;
}

impl PbMessage for BirthDayRewardResponse {
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
pub struct BlackCoastThemeActiveRewardRequest {
    pub ActiveRewardIds: Vec<i32>,
    pub ActivityId: i32,
}

impl BlackCoastThemeActiveRewardRequest {
    pub const MSG_ID: u16 = 26746;
}

impl PbMessage for BlackCoastThemeActiveRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActiveRewardIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ActiveRewardIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeActiveRewardResponse {
    pub Error: Option<ErrorCode>,
    pub ActiveRewardedIds: Vec<i32>,
}

impl BlackCoastThemeActiveRewardResponse {
    pub const MSG_ID: u16 = 28050;
}

impl PbMessage for BlackCoastThemeActiveRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ActiveRewardedIds {
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
            13 => {
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ActiveRewardedIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeActivity {
    pub Stages: Vec<BlackCoastThemeStageInfo>,
    pub ActiveRewardedIds: Vec<i32>,
}

impl PbMessage for BlackCoastThemeActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Stages {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ActiveRewardedIds {
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
                let bytes = r.read_len_delimited()?;
                self.Stages
                    .push(<BlackCoastThemeStageInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ActiveRewardedIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeStageUpdateNotify {
    pub Stages: Vec<BlackCoastThemeStageInfo>,
}

impl BlackCoastThemeStageUpdateNotify {
    pub const MSG_ID: u16 = 25531;
}

impl PbMessage for BlackCoastThemeStageUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Stages {
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
                self.Stages
                    .push(<BlackCoastThemeStageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeTaskRewardRequest {
    pub TaskId: i32,
}

impl BlackCoastThemeTaskRewardRequest {
    pub const MSG_ID: u16 = 23757;
}

impl PbMessage for BlackCoastThemeTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackCoastThemeTaskRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl BlackCoastThemeTaskRewardResponse {
    pub const MSG_ID: u16 = 20483;
}

impl PbMessage for BlackCoastThemeTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackSwordChallengeInfo {
    pub LevelPlayId: i32,
    pub IsUnLocked: bool,
    pub CompleteNumber: i32,
}

impl PbMessage for BlackSwordChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if self.IsUnLocked {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CompleteNumber != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CompleteNumber);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LevelPlayId = r.read_varint_i32()?,
            2 => self.IsUnLocked = r.read_bool()?,
            3 => self.CompleteNumber = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackSwordChallengeInfoRequest {
    pub ChallengeId: i32,
}

impl BlackSwordChallengeInfoRequest {
    pub const MSG_ID: u16 = 22114;
}

impl PbMessage for BlackSwordChallengeInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            14 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackSwordChallengeInfoResponse {
    pub LevelPlayUiInfoList: Vec<BlackSwordChallengeInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub ChallengeId: i32,
}

impl BlackSwordChallengeInfoResponse {
    pub const MSG_ID: u16 = 20453;
}

impl PbMessage for BlackSwordChallengeInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelPlayUiInfoList {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.LevelPlayUiInfoList
                    .push(<BlackSwordChallengeInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackSwordChallengeMarkItemPanelInfoRequest {
    pub ChallengeId: i32,
}

impl BlackSwordChallengeMarkItemPanelInfoRequest {
    pub const MSG_ID: u16 = 29544;
}

impl PbMessage for BlackSwordChallengeMarkItemPanelInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackSwordChallengeMarkItemPanelInfoResponse {
    pub LevelPlayUiInfoList: Vec<BlackSwordChallengeInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub ChallengeId: i32,
}

impl BlackSwordChallengeMarkItemPanelInfoResponse {
    pub const MSG_ID: u16 = 19063;
}

impl PbMessage for BlackSwordChallengeMarkItemPanelInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelPlayUiInfoList {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.LevelPlayUiInfoList
                    .push(<BlackSwordChallengeInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlockListRequest {}

impl BlockListRequest {
    pub const MSG_ID: u16 = 22469;
}

impl PbMessage for BlockListRequest {
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
pub struct BlockListResponse {
    pub BlockList: Vec<PlayerDetails>,
    pub ErrorCode: Option<ErrorCode>,
}

impl BlockListResponse {
    pub const MSG_ID: u16 = 23512;
}

impl PbMessage for BlockListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BlockList {
            crate::wire::write_submessage(out, 2, item);
        }
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BlockList
                    .push(<PlayerDetails as PbMessage>::decode(bytes)?);
            }
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
pub struct BlockPlayerRequest {
    pub Id: i32,
}

impl BlockPlayerRequest {
    pub const MSG_ID: u16 = 26186;
}

impl PbMessage for BlockPlayerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            11 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlockPlayerResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Info: Option<PlayerDetails>,
}

impl BlockPlayerResponse {
    pub const MSG_ID: u16 = 25057;
}

impl PbMessage for BlockPlayerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Info {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardCanMovePlacementNotify {
    pub BoardEntityId: i64,
    pub CanMove: bool,
}

impl BoardCanMovePlacementNotify {
    pub const MSG_ID: u16 = 27769;
}

impl PbMessage for BoardCanMovePlacementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoardEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardEntityId);
        }
        if self.CanMove {
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
            12 => self.BoardEntityId = r.read_varint_i64()?,
            14 => self.CanMove = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardGridDynamicConfigChangeNotify {
    pub ChangedConfigs: Vec<BoardGridDynamicConfig>,
    pub BoardEntityId: i64,
}

impl BoardGridDynamicConfigChangeNotify {
    pub const MSG_ID: u16 = 23350;
}

impl PbMessage for BoardGridDynamicConfigChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChangedConfigs {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.BoardEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardEntityId);
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
                self.ChangedConfigs
                    .push(<BoardGridDynamicConfig as PbMessage>::decode(bytes)?);
            }
            10 => self.BoardEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BookItemUpdateNotify {
    pub BookItems: Vec<BookItemInfo>,
    pub ActivityId: i32,
    pub BrochureId: i32,
}

impl BookItemUpdateNotify {
    pub const MSG_ID: u16 = 27626;
}

impl PbMessage for BookItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BookItems {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.BrochureId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BrochureId);
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
                self.BookItems
                    .push(<BookItemInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.ActivityId = r.read_varint_i32()?,
            14 => self.BrochureId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingBuffUpdateNotify {
    pub BossPilingBuffs: BTreeMap<i32, i32>,
}

impl BossPilingBuffUpdateNotify {
    pub const MSG_ID: u16 = 19869;
}

impl PbMessage for BossPilingBuffUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BossPilingBuffs.is_empty() {
            crate::wire::write_map(out, 13, &self.BossPilingBuffs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.BossPilingBuffs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingCtx {
    pub BossPilingLevelId: i32,
    pub SkillBranchId: Vec<i32>,
}

impl PbMessage for BossPilingCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BossPilingLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossPilingLevelId);
        }
        for item in &self.SkillBranchId {
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
            1 => self.BossPilingLevelId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingInstNotify {
    pub BossPilingBuffs: BTreeMap<i32, i32>,
    pub BossPilingLevelId: i32,
}

impl BossPilingInstNotify {
    pub const MSG_ID: u16 = 15347;
}

impl PbMessage for BossPilingInstNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BossPilingBuffs.is_empty() {
            crate::wire::write_map(out, 1, &self.BossPilingBuffs);
        }
        if self.BossPilingLevelId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossPilingLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.BossPilingBuffs)?,
            12 => self.BossPilingLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingLevelInfoUpdateNotify {
    pub BossPilingLevelInfo: Vec<BossPilingLevelInfo>,
}

impl BossPilingLevelInfoUpdateNotify {
    pub const MSG_ID: u16 = 29397;
}

impl PbMessage for BossPilingLevelInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BossPilingLevelInfo {
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
                self.BossPilingLevelInfo
                    .push(<BossPilingLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingRewardRequest {
    pub BossPilingTaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl BossPilingRewardRequest {
    pub const MSG_ID: u16 = 21613;
}

impl PbMessage for BossPilingRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BossPilingTaskIds {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.BossPilingTaskIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossPilingRewardResponse {
    pub const MSG_ID: u16 = 26058;
}

impl PbMessage for BossPilingRewardResponse {
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
pub struct BossPilingSettleNotify {
    pub ChallengeLevelId: i32,
    pub IsSuc: bool,
    pub BossHpNum: i32,
    pub IsUpdateBossHpNum: bool,
}

impl BossPilingSettleNotify {
    pub const MSG_ID: u16 = 25811;
}

impl PbMessage for BossPilingSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeLevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeLevelId);
        }
        if self.IsSuc {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BossHpNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossHpNum);
        }
        if self.IsUpdateBossHpNum {
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
            2 => self.ChallengeLevelId = r.read_varint_i32()?,
            3 => self.IsSuc = r.read_bool()?,
            6 => self.BossHpNum = r.read_varint_i32()?,
            7 => self.IsUpdateBossHpNum = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossPilingSettleRequest {}

impl BossPilingSettleRequest {
    pub const MSG_ID: u16 = 19863;
}

impl PbMessage for BossPilingSettleRequest {
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
pub struct BossPilingSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossPilingSettleResponse {
    pub const MSG_ID: u16 = 17879;
}

impl PbMessage for BossPilingSettleResponse {
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
pub struct BossPilingTaskInfoUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl BossPilingTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 15232;
}

impl PbMessage for BossPilingTaskInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 7, item);
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
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushBatchClaimRewardRequest {
    pub ActivityId: i32,
    pub TaskRewardIds: Vec<i32>,
}

impl BossRushBatchClaimRewardRequest {
    pub const MSG_ID: u16 = 27431;
}

impl PbMessage for BossRushBatchClaimRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TaskRewardIds {
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
            6 => self.ActivityId = r.read_varint_i32()?,
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskRewardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushBatchClaimRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushBatchClaimRewardResponse {
    pub const MSG_ID: u16 = 26097;
}

impl PbMessage for BossRushBatchClaimRewardResponse {
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
pub struct BossRushBuffChooseResultRequest {
    pub Index: i32,
}

impl BossRushBuffChooseResultRequest {
    pub const MSG_ID: u16 = 23159;
}

impl PbMessage for BossRushBuffChooseResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushBuffChooseResultResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushBuffChooseResultResponse {
    pub const MSG_ID: u16 = 24781;
}

impl PbMessage for BossRushBuffChooseResultResponse {
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
pub struct BossRushChooseBuffRequest {}

impl BossRushChooseBuffRequest {
    pub const MSG_ID: u16 = 29389;
}

impl PbMessage for BossRushChooseBuffRequest {
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
pub struct BossRushChooseBuffResponse {
    pub BuffIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushChooseBuffResponse {
    pub const MSG_ID: u16 = 19084;
}

impl PbMessage for BossRushChooseBuffResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct BossRushClaimRewardRequest {
    pub RewardType: Option<BossRushRewardType>,
    pub ActivityId: i32,
    pub RewardId: i32,
}

impl BossRushClaimRewardRequest {
    pub const MSG_ID: u16 = 17500;
}

impl PbMessage for BossRushClaimRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
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
                self.RewardType = Some(
                    <BossRushRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            14 => self.RewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushClaimRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushClaimRewardResponse {
    pub const MSG_ID: u16 = 23469;
}

impl PbMessage for BossRushClaimRewardResponse {
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
pub struct BossRushCtx {
    pub Id: i32,
    pub ActivityId: i32,
    pub BuffSelections: Vec<BuffSelection>,
    pub ScoreBuff: Vec<i32>,
}

impl PbMessage for BossRushCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.BuffSelections {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.ScoreBuff {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.ActivityId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BuffSelections
                    .push(<BuffSelection as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreBuff, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushFailResultNotify {}

impl BossRushFailResultNotify {
    pub const MSG_ID: u16 = 19850;
}

impl PbMessage for BossRushFailResultNotify {
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
pub struct BossRushLevelData {
    pub InstId: i32,
    pub StartTime: i32,
    pub IsOpen: bool,
    pub Score: i32,
    pub CharacterSelectionRecord: Vec<i32>,
    pub BuffSelectionRecord: Vec<BuffSelection>,
    pub LevelRewardClaimStatus: Option<BossRushRewardClaimStatus>,
    pub ScoreBuff: Vec<i32>,
    pub LevelScoreRewardStatus: Vec<BossRushRewardClaimStatus>,
}

impl PbMessage for BossRushLevelData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartTime);
        }
        if self.IsOpen {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        for item in &self.CharacterSelectionRecord {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BuffSelectionRecord {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.LevelRewardClaimStatus {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ScoreBuff {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LevelScoreRewardStatus {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.StartTime = r.read_varint_i32()?,
            3 => self.IsOpen = r.read_bool()?,
            4 => self.Score = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CharacterSelectionRecord,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.BuffSelectionRecord
                    .push(<BuffSelection as PbMessage>::decode(bytes)?);
            }
            7 => {
                self.LevelRewardClaimStatus = Some(
                    <BossRushRewardClaimStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreBuff, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LevelScoreRewardStatus,
                |sr| {
                    Ok(
                        <BossRushRewardClaimStatus as crate::wire::PbEnum>::from_i32_or_default(
                            sr.read_varint_i32()?,
                        ),
                    )
                },
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushLevelInfoNotify {
    pub BossRushScoreScoreRewardInfo: Vec<BossRushScoreRewardData>,
    pub ActivityId: i32,
    pub BossRushLevelInfo: Vec<BossRushLevelData>,
    pub UnlockedBuffIndices: Vec<i32>,
}

impl BossRushLevelInfoNotify {
    pub const MSG_ID: u16 = 28132;
}

impl PbMessage for BossRushLevelInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BossRushScoreScoreRewardInfo {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.BossRushLevelInfo {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.UnlockedBuffIndices {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BossRushScoreScoreRewardInfo
                    .push(<BossRushScoreRewardData as PbMessage>::decode(bytes)?);
            }
            8 => self.ActivityId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.BossRushLevelInfo
                    .push(<BossRushLevelData as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedBuffIndices,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushLevelScoreRewardRequest {
    pub BossRushActivityId: i32,
    pub Index: i32,
}

impl BossRushLevelScoreRewardRequest {
    pub const MSG_ID: u16 = 24337;
}

impl PbMessage for BossRushLevelScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BossRushActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossRushActivityId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            4 => self.BossRushActivityId = r.read_varint_i32()?,
            10 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushLevelScoreRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl BossRushLevelScoreRewardResponse {
    pub const MSG_ID: u16 = 17469;
}

impl PbMessage for BossRushLevelScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushResultNotify {
    pub KilMonsterScore: i32,
    pub LeftTime: i32,
    pub ActivityId: i32,
    pub InstId: i32,
    pub TimerScore: i32,
    pub HighScore: i32,
    pub TechnicalScore: i32,
}

impl BossRushResultNotify {
    pub const MSG_ID: u16 = 16040;
}

impl PbMessage for BossRushResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.KilMonsterScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KilMonsterScore);
        }
        if self.LeftTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeftTime);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.TimerScore != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimerScore);
        }
        if self.HighScore != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HighScore);
        }
        if self.TechnicalScore != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TechnicalScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.KilMonsterScore = r.read_varint_i32()?,
            2 => self.LeftTime = r.read_varint_i32()?,
            3 => self.ActivityId = r.read_varint_i32()?,
            4 => self.InstId = r.read_varint_i32()?,
            7 => self.TimerScore = r.read_varint_i32()?,
            9 => self.HighScore = r.read_varint_i32()?,
            13 => self.TechnicalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushSelectBuffNotify {
    pub SelectBuffIndices: Vec<i32>,
    pub InstId: i32,
    pub ActivityId: i32,
}

impl BossRushSelectBuffNotify {
    pub const MSG_ID: u16 = 23543;
}

impl PbMessage for BossRushSelectBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectBuffIndices {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectBuffIndices,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.InstId = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushSettlementRequest {}

impl BossRushSettlementRequest {
    pub const MSG_ID: u16 = 19575;
}

impl PbMessage for BossRushSettlementRequest {
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
pub struct BossRushSettlementResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushSettlementResponse {
    pub const MSG_ID: u16 = 25837;
}

impl PbMessage for BossRushSettlementResponse {
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
pub struct BossRushTaskRewardRequest {
    pub TaskId: i32,
}

impl BossRushTaskRewardRequest {
    pub const MSG_ID: u16 = 25214;
}

impl PbMessage for BossRushTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BossRushTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BossRushTaskRewardResponse {
    pub const MSG_ID: u16 = 24143;
}

impl PbMessage for BossRushTaskRewardResponse {
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
pub struct BossRushTaskUpdateNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl BossRushTaskUpdateNotify {
    pub const MSG_ID: u16 = 28139;
}

impl PbMessage for BossRushTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BrochureRewardRequest {
    pub BrochureId: i32,
    pub ActivityId: i32,
    pub BookItems: Vec<i32>,
}

impl BrochureRewardRequest {
    pub const MSG_ID: u16 = 20771;
}

impl PbMessage for BrochureRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BrochureId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BrochureId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.BookItems {
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
            3 => self.BrochureId = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BookItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BrochureRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl BrochureRewardResponse {
    pub const MSG_ID: u16 = 22634;
}

impl PbMessage for BrochureRewardResponse {
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
pub struct BtCancelRangeFailWarningNotify {
    pub TreeIncId: i64,
}

impl BtCancelRangeFailWarningNotify {
    pub const MSG_ID: u16 = 29677;
}

impl PbMessage for BtCancelRangeFailWarningNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            12 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtForcedOccupationRequest {
    pub TreeIncId: i64,
}

impl BtForcedOccupationRequest {
    pub const MSG_ID: u16 = 25189;
}

impl PbMessage for BtForcedOccupationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtForcedOccupationResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl BtForcedOccupationResponse {
    pub const MSG_ID: u16 = 24742;
}

impl PbMessage for BtForcedOccupationResponse {
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
pub struct BtObjYuXueModeInfo {
    pub BtType: Option<BtType>,
    pub BtObjId: i32,
    pub BtObjSetting: i32,
}

impl PbMessage for BtObjYuXueModeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BtType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BtObjId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjId);
        }
        if self.BtObjSetting != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjSetting);
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
                self.BtType = Some(<BtType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.BtObjId = r.read_varint_i32()?,
            3 => self.BtObjSetting = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRangeFailWarningNotify {
    pub TreeIncId: i64,
}

impl BtRangeFailWarningNotify {
    pub const MSG_ID: u16 = 22612;
}

impl PbMessage for BtRangeFailWarningNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            14 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRollbackInfo {
    pub FailRollbackData: Option<FailRollbackData>,
    pub ForceClosedRollbackData: Option<ForceClosedRollbackData>,
}

impl PbMessage for BtRollbackInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FailRollbackData {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ForceClosedRollbackData {
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
                self.FailRollbackData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ForceClosedRollbackData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRollbackInfoNotify {
    pub TreeInfo: Option<TreeInfo>,
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
}

impl BtRollbackInfoNotify {
    pub const MSG_ID: u16 = 26092;
}

impl PbMessage for BtRollbackInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TreeInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.TreeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.TreeOwnerId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRollbackNotify {
    pub BtRollbackInfo: Option<BtRollbackInfo>,
    pub FailReason: Option<FailReason>,
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
    pub FailNodeId: i32,
}

impl BtRollbackNotify {
    pub const MSG_ID: u16 = 18266;
}

impl PbMessage for BtRollbackNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BtRollbackInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.FailReason {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.FailNodeId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailNodeId);
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
                self.BtRollbackInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                self.FailReason = Some(<FailReason as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.TreeIncId = r.read_varint_i64()?,
            9 => self.TreeOwnerId = r.read_varint_i32()?,
            15 => self.FailNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtRollbackStartNotify {
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub IsRollbackSubLevel: bool,
    pub IsRollbackPos: bool,
}

impl BtRollbackStartNotify {
    pub const MSG_ID: u16 = 26088;
}

impl PbMessage for BtRollbackStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.IsRollbackSubLevel {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRollbackPos {
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
            4 => self.TreeOwnerId = r.read_varint_i32()?,
            8 => self.TreeIncId = r.read_varint_i64()?,
            10 => self.IsRollbackSubLevel = r.read_bool()?,
            11 => self.IsRollbackPos = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtSuspendNotify {
    pub NodeId: i32,
    pub SuspendType: i32,
    pub OccupationInfo: Vec<OccupationPbInfo>,
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub IsScreenOccupy: bool,
}

impl BtSuspendNotify {
    pub const MSG_ID: u16 = 29172;
}

impl PbMessage for BtSuspendNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.SuspendType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SuspendType);
        }
        for item in &self.OccupationInfo {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.IsScreenOccupy {
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
            2 => self.NodeId = r.read_varint_i32()?,
            3 => self.SuspendType = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.OccupationInfo
                    .push(<OccupationPbInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.TreeOwnerId = r.read_varint_i32()?,
            12 => self.TreeIncId = r.read_varint_i64()?,
            13 => self.IsScreenOccupy = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtVarUpdateNotify {
    pub TreeIncId: i64,
    pub VarName: Vec<u8>,
    pub VarDefine: Option<VarDefinePb>,
    pub TreeOwnerId: i32,
}

impl BtVarUpdateNotify {
    pub const MSG_ID: u16 = 25834;
}

impl PbMessage for BtVarUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if !self.VarName.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VarName);
        }
        if let Some(v) = &self.VarDefine {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            2 => self.TreeIncId = r.read_varint_i64()?,
            5 => self.VarName = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.VarDefine
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectExecuteRequest {
    pub Context: Option<FightBuffEffectContext>,
    pub Index: i32,
}

impl BuffEffectExecuteRequest {
    pub const MSG_ID: u16 = 19635;
}

impl PbMessage for BuffEffectExecuteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Context {
            crate::wire::write_submessage(out, 7, v);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Context
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectExecuteResponse {
    pub Code: Option<ErrorCode>,
}

impl BuffEffectExecuteResponse {
    pub const MSG_ID: u16 = 27923;
}

impl PbMessage for BuffEffectExecuteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct BuffItemRequest {
    pub ItemId: i32,
    pub Num: i32,
    pub RoleId: i32,
}

impl BuffItemRequest {
    pub const MSG_ID: u16 = 17824;
}

impl PbMessage for BuffItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
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
            5 => self.ItemId = r.read_varint_i32()?,
            9 => self.Num = r.read_varint_i32()?,
            14 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffItemResponse {
    pub BuffItemInfo: Option<BuffItem>,
    pub RoleId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl BuffItemResponse {
    pub const MSG_ID: u16 = 21929;
}

impl PbMessage for BuffItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BuffItemInfo {
            crate::wire::write_submessage(out, 12, v);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.BuffItemInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.RoleId = r.read_varint_i32()?,
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
pub struct BuffItemUpdateNotify {
    pub BuffItem: Option<BuffItem>,
    pub EquipItemList: Vec<EquipBuffItem>,
}

impl BuffItemUpdateNotify {
    pub const MSG_ID: u16 = 15200;
}

impl PbMessage for BuffItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BuffItem {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.EquipItemList {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BuffItem
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.EquipItemList
                    .push(<EquipBuffItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffSelection {
    pub BuffId: i32,
    pub Slot: i32,
    pub BuffSelectionStatus: Option<BossRushBuffSelectionStatus>,
}

impl PbMessage for BuffSelection {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
        }
        if self.Slot != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Slot);
        }
        if let Some(v) = &self.BuffSelectionStatus {
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
            1 => self.BuffId = r.read_varint_i32()?,
            2 => self.Slot = r.read_varint_i32()?,
            3 => {
                self.BuffSelectionStatus = Some(
                    <BossRushBuffSelectionStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct BuildingInfo {
    pub BuildingId: i32,
    pub Level: i32,
    pub IsUnlock: bool,
}

impl PbMessage for BuildingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuildingId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuildingId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.IsUnlock {
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
            1 => self.BuildingId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingLevelUpRequest {
    pub BuildingId: i32,
}

impl BuildingLevelUpRequest {
    pub const MSG_ID: u16 = 22480;
}

impl PbMessage for BuildingLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuildingId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuildingId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.BuildingId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingLevelUpResponse {
    pub BuildingInfo: Option<BuildingInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub PeopleLike: i32,
}

impl BuildingLevelUpResponse {
    pub const MSG_ID: u16 = 26925;
}

impl PbMessage for BuildingLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BuildingInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PeopleLike != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PeopleLike);
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
                self.BuildingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.PeopleLike = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingUnLockRequest {
    pub BuildingId: i32,
}

impl BuildingUnLockRequest {
    pub const MSG_ID: u16 = 26985;
}

impl PbMessage for BuildingUnLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuildingId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuildingId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.BuildingId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuildingUnLockResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PeopleLike: i32,
}

impl BuildingUnLockResponse {
    pub const MSG_ID: u16 = 28422;
}

impl PbMessage for BuildingUnLockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PeopleLike != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PeopleLike);
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
            12 => self.PeopleLike = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BulletScreenTimes {
    pub BulletScreenId: i32,
    pub Num: i32,
}

impl PbMessage for BulletScreenTimes {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletScreenId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletScreenId);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
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
            2 => self.Num = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonIsEnableRequest {
    pub Types_: Vec<ButtonType>,
    pub Type: Option<ButtonType>,
}

impl ButtonIsEnableRequest {
    pub const MSG_ID: u16 = 18232;
}

impl PbMessage for ButtonIsEnableRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Types_ {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        if let Some(v) = &self.Type {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Types_, |sr| {
                Ok(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    sr.read_varint_i32()?,
                ))
            })?,
            9 => {
                self.Type = Some(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonIsEnableResponse {
    pub Type: Option<ButtonType>,
    pub ErrorCode: Option<ErrorCode>,
    pub Result: Vec<ButtonEnableResult>,
    pub Enable: bool,
}

impl ButtonIsEnableResponse {
    pub const MSG_ID: u16 = 25924;
}

impl PbMessage for ButtonIsEnableResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Result {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.Enable {
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
            2 => {
                self.Type = Some(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .push(<ButtonEnableResult as PbMessage>::decode(bytes)?);
            }
            14 => self.Enable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashDevelopRewardUnlockNotify {
    pub UnlockedDevelopReward: Option<CalabashDevelopInfo>,
    pub IncrId: i32,
    pub ItemId: i32,
}

impl CalabashDevelopRewardUnlockNotify {
    pub const MSG_ID: u16 = 17081;
}

impl PbMessage for CalabashDevelopRewardUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UnlockedDevelopReward {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
                let bytes = r.read_len_delimited()?;
                self.UnlockedDevelopReward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.IncrId = r.read_varint_i32()?,
            13 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashExpAddNotify {
    pub CalabashCfg: Option<CalabashCfg>,
    pub CurLevel: i32,
    pub AddExp: i32,
    pub CurExp: i32,
}

impl CalabashExpAddNotify {
    pub const MSG_ID: u16 = 20523;
}

impl PbMessage for CalabashExpAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CalabashCfg {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.CurLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLevel);
        }
        if self.AddExp != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddExp);
        }
        if self.CurExp != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurExp);
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
                self.CalabashCfg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.CurLevel = r.read_varint_i32()?,
            13 => self.AddExp = r.read_varint_i32()?,
            14 => self.CurExp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashMsgRequest {}

impl CalabashMsgRequest {
    pub const MSG_ID: u16 = 23687;
}

impl PbMessage for CalabashMsgRequest {
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
pub struct CalabashMsgResponse {
    pub CalabashMsg: Option<CalabashMsg>,
    pub CalabashCfg: Option<CalabashCfg>,
    pub ErrCode: Option<ErrorCode>,
}

impl CalabashMsgResponse {
    pub const MSG_ID: u16 = 15659;
}

impl PbMessage for CalabashMsgResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CalabashMsg {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.CalabashCfg {
            crate::wire::write_submessage(out, 6, v);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CalabashMsg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CalabashCfg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct CalabashSkinTakeOnNotify {
    pub SkinId: i32,
}

impl CalabashSkinTakeOnNotify {
    pub const MSG_ID: u16 = 22784;
}

impl PbMessage for CalabashSkinTakeOnNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashSkinUpdateNotify {
    pub SkinIds: Vec<i32>,
}

impl CalabashSkinUpdateNotify {
    pub const MSG_ID: u16 = 18523;
}

impl PbMessage for CalabashSkinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancelMatchNotify {
    pub PlayerId: i32,
}

impl CancelMatchNotify {
    pub const MSG_ID: u16 = 18579;
}

impl PbMessage for CancelMatchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CancelMatchRequest {}

impl CancelMatchRequest {
    pub const MSG_ID: u16 = 15153;
}

impl PbMessage for CancelMatchRequest {
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
pub struct CancelMatchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CancelMatchResponse {
    pub const MSG_ID: u16 = 17155;
}

impl PbMessage for CancelMatchResponse {
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
pub struct CaptureBatchEntityRequest {
    pub InstId: i32,
    pub Ids: Vec<i64>,
}

impl CaptureBatchEntityRequest {
    pub const MSG_ID: u16 = 28951;
}

impl PbMessage for CaptureBatchEntityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.InstId = r.read_varint_i32()?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaptureBatchEntityResponse {
    pub ErrCode: Option<ErrorCode>,
    pub Ids: Vec<i64>,
}

impl CaptureBatchEntityResponse {
    pub const MSG_ID: u16 = 25668;
}

impl PbMessage for CaptureBatchEntityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
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
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaptureEntityRequest {
    pub Id: i64,
}

impl CaptureEntityRequest {
    pub const MSG_ID: u16 = 15300;
}

impl PbMessage for CaptureEntityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaptureEntityResponse {
    pub ErrCode: Option<ErrorCode>,
    pub Id: i64,
}

impl CaptureEntityResponse {
    pub const MSG_ID: u16 = 15289;
}

impl PbMessage for CaptureEntityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
            13 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CertificateLevelRewardRequest {}

impl CertificateLevelRewardRequest {
    pub const MSG_ID: u16 = 15228;
}

impl PbMessage for CertificateLevelRewardRequest {
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
pub struct CertificateLevelRewardResponse {
    pub Code: Option<ErrorCode>,
}

impl CertificateLevelRewardResponse {
    pub const MSG_ID: u16 = 25731;
}

impl PbMessage for CertificateLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct ChampionExtra {
    pub FirstCount: i32,
    pub AbsoluteRule: bool,
    pub Destroyer: bool,
}

impl PbMessage for ChampionExtra {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FirstCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstCount);
        }
        if self.AbsoluteRule {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Destroyer {
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
            1 => self.FirstCount = r.read_varint_i32()?,
            2 => self.AbsoluteRule = r.read_bool()?,
            3 => self.Destroyer = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeControlRoleNotify {
    pub PlayerId: i32,
    pub Location: Option<Vector>,
    pub NeedSetLocation: bool,
    pub DownId: i64,
    pub UpId: i64,
}

impl ChangeControlRoleNotify {
    pub const MSG_ID: u16 = 29562;
}

impl PbMessage for ChangeControlRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.NeedSetLocation {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DownId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DownId);
        }
        if self.UpId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpId);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.NeedSetLocation = r.read_bool()?,
            9 => self.DownId = r.read_varint_i64()?,
            13 => self.UpId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEntityRoleRequest {
    pub Id: i64,
    pub PlayerId: i32,
}

impl ChangeEntityRoleRequest {
    pub const MSG_ID: u16 = 24621;
}

impl PbMessage for ChangeEntityRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            3 => self.Id = r.read_varint_i64()?,
            5 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEntityRoleResponse {
    pub Success: bool,
    pub PlayerId: i32,
    pub Id: i64,
}

impl ChangeEntityRoleResponse {
    pub const MSG_ID: u16 = 28494;
}

impl PbMessage for ChangeEntityRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Success {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
            9 => self.PlayerId = r.read_varint_i32()?,
            11 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEntityStateRequest {
    pub StateTagId: i32,
    pub EntityId: i32,
}

impl ChangeEntityStateRequest {
    pub const MSG_ID: u16 = 18733;
}

impl PbMessage for ChangeEntityStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StateTagId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateTagId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            10 => self.StateTagId = r.read_varint_i32()?,
            15 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEntityStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeEntityStateResponse {
    pub const MSG_ID: u16 = 21818;
}

impl PbMessage for ChangeEntityStateResponse {
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
pub struct ChangeEventByPosOp {
    pub ChangeEventDatas: Vec<ChangeEventData>,
}

impl PbMessage for ChangeEventByPosOp {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChangeEventDatas {
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
                self.ChangeEventDatas
                    .push(<ChangeEventData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeEventData {
    pub Index: i32,
    pub RogueResGridData: Option<RogueResGridData>,
}

impl PbMessage for ChangeEventData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.RogueResGridData {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RogueResGridData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeMonsterRoleNotify {
    pub Id: i64,
    pub PlayerId: i32,
}

impl ChangeMonsterRoleNotify {
    pub const MSG_ID: u16 = 22631;
}

impl PbMessage for ChangeMonsterRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            2 => self.Id = r.read_varint_i64()?,
            4 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangePhantomEquipGroupNameRequest {
    pub RoleId: i32,
    pub Index: i32,
    pub Name: Vec<u8>,
}

impl ChangePhantomEquipGroupNameRequest {
    pub const MSG_ID: u16 = 18907;
}

impl PbMessage for ChangePhantomEquipGroupNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.RoleId = r.read_varint_i32()?,
            9 => self.Index = r.read_varint_i32()?,
            13 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangePhantomEquipGroupNameResponse {
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangePhantomEquipGroupNameResponse {
    pub const MSG_ID: u16 = 16060;
}

impl PbMessage for ChangePhantomEquipGroupNameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomEquipGroupInfos {
            crate::wire::write_submessage(out, 6, item);
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
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
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
pub struct ChangePlayerFightRoleNotify {
    pub IsPhantom: bool,
    pub FightRoleInfos: Vec<FightRoleInformation>,
    pub PlayerId: i32,
}

impl ChangePlayerFightRoleNotify {
    pub const MSG_ID: u16 = 25527;
}

impl PbMessage for ChangePlayerFightRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsPhantom {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.FightRoleInfos {
            crate::wire::write_submessage(out, 10, item);
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
            3 => self.IsPhantom = r.read_bool()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfos
                    .push(<FightRoleInformation as PbMessage>::decode(bytes)?);
            }
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeSceneModeEndNotify {}

impl ChangeSceneModeEndNotify {
    pub const MSG_ID: u16 = 17107;
}

impl PbMessage for ChangeSceneModeEndNotify {
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
pub struct ChangeSceneModeNotify {
    pub Mode: Option<SceneMode>,
    pub SceneId: Vec<u8>,
}

impl ChangeSceneModeNotify {
    pub const MSG_ID: u16 = 20701;
}

impl PbMessage for ChangeSceneModeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Mode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SceneId);
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
                self.Mode = Some(<SceneMode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.SceneId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeSceneRoadNotify {
    pub IsEnable: bool,
    pub EnabledRoads: Vec<i32>,
    pub SceneId: Vec<u8>,
}

impl ChangeSceneRoadNotify {
    pub const MSG_ID: u16 = 21251;
}

impl PbMessage for ChangeSceneRoadNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnable {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.EnabledRoads {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SceneId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.IsEnable = r.read_bool()?,
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EnabledRoads, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => self.SceneId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelChatHistoryNotify {
    pub SubChannelType: Option<SubChatChannelType>,
    pub ChannelType: Option<ChatChannelType>,
    pub MessageInfos: Vec<ChannelChatMessageInfo>,
}

impl ChannelChatHistoryNotify {
    pub const MSG_ID: u16 = 24562;
}

impl PbMessage for ChannelChatHistoryNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SubChannelType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChannelType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.MessageInfos {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MessageInfos
                    .push(<ChannelChatMessageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelChatMessageInfo {
    pub SenderId: i32,
    pub SenderIcon: i32,
    pub SenderName: Vec<u8>,
    pub ChatContentType: Option<ChatContentType>,
    pub Content: Vec<u8>,
    pub NoticeType: Option<ChatChannelNoticeType>,
    pub SendTime: i64,
    pub SenderPsnOnlineId: Vec<u8>,
    pub SenderPsnAccountId: Vec<u8>,
    pub TitleId: i32,
    pub TitleExtraParam: i32,
    pub SenderXboxOnlineId: Vec<u8>,
    pub SenderXboxAccountId: Vec<u8>,
}

impl PbMessage for ChannelChatMessageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SenderId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SenderId);
        }
        if self.SenderIcon != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SenderIcon);
        }
        if !self.SenderName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SenderName);
        }
        if let Some(v) = &self.ChatContentType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
        if let Some(v) = &self.NoticeType {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SendTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SendTime);
        }
        if !self.SenderPsnOnlineId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SenderPsnOnlineId);
        }
        if !self.SenderPsnAccountId.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SenderPsnAccountId);
        }
        if self.TitleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TitleId);
        }
        if self.TitleExtraParam != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TitleExtraParam);
        }
        if !self.SenderXboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SenderXboxOnlineId);
        }
        if !self.SenderXboxAccountId.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SenderXboxAccountId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SenderId = r.read_varint_i32()?,
            2 => self.SenderIcon = r.read_varint_i32()?,
            3 => self.SenderName = r.read_len_delimited()?.to_vec(),
            4 => {
                self.ChatContentType = Some(
                    <ChatContentType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.Content = r.read_len_delimited()?.to_vec(),
            6 => {
                self.NoticeType = Some(
                    <ChatChannelNoticeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.SendTime = r.read_varint_i64()?,
            8 => self.SenderPsnOnlineId = r.read_len_delimited()?.to_vec(),
            9 => self.SenderPsnAccountId = r.read_len_delimited()?.to_vec(),
            10 => self.TitleId = r.read_varint_i32()?,
            11 => self.TitleExtraParam = r.read_varint_i32()?,
            12 => self.SenderXboxOnlineId = r.read_len_delimited()?.to_vec(),
            13 => self.SenderXboxAccountId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChannelChatMessageNotify {
    pub SubChannelType: Option<SubChatChannelType>,
    pub ChannelType: Option<ChatChannelType>,
    pub MessageInfo: Option<ChannelChatMessageInfo>,
}

impl ChannelChatMessageNotify {
    pub const MSG_ID: u16 = 19107;
}

impl PbMessage for ChannelChatMessageNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SubChannelType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChannelType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MessageInfo {
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
            1 => {
                self.SubChannelType = Some(
                    <SubChatChannelType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                self.ChannelType = Some(
                    <ChatChannelType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.MessageInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChapterResultRewardRequest {
    pub ResultId: i32,
    pub ActivityId: i32,
    pub ChapterId: i32,
}

impl ChapterResultRewardRequest {
    pub const MSG_ID: u16 = 24597;
}

impl PbMessage for ChapterResultRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResultId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResultId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.ResultId = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            13 => self.ChapterId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChapterResultRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChapterResultRewardResponse {
    pub const MSG_ID: u16 = 15304;
}

impl PbMessage for ChapterResultRewardResponse {
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
pub struct ChapterUpdateNotify {
    pub ActivityId: i32,
    pub ChapterDatas: Vec<CiacconaChapterPbData>,
}

impl ChapterUpdateNotify {
    pub const MSG_ID: u16 = 25234;
}

impl PbMessage for ChapterUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ChapterDatas {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ChapterDatas
                    .push(<CiacconaChapterPbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterDeathPush {
    pub Result: i32,
    pub RoleId: i32,
}

impl CharacterDeathPush {
    pub const MSG_ID: u16 = 23389;
}

impl PbMessage for CharacterDeathPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Result != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Result);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            8 => self.Result = r.read_varint_i32()?,
            11 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterDeathRequest {
    pub Result: i32,
    pub RoleId: i32,
}

impl CharacterDeathRequest {
    pub const MSG_ID: u16 = 21948;
}

impl PbMessage for CharacterDeathRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Result != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Result);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            9 => self.Result = r.read_varint_i32()?,
            12 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterDeathResponse {}

impl CharacterDeathResponse {
    pub const MSG_ID: u16 = 25795;
}

impl PbMessage for CharacterDeathResponse {
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
pub struct CharacterLookAtNotify {
    pub CharacterLookAtInfos: Vec<CharacterLookAtInfo>,
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
}

impl CharacterLookAtNotify {
    pub const MSG_ID: u16 = 24799;
}

impl PbMessage for CharacterLookAtNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CharacterLookAtInfos {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.CharacterLookAtInfos
                    .push(<CharacterLookAtInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.TreeOwnerId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterMoveToPointConfig {
    pub TargetPos: Option<Vector>,
    pub MoveType: i32,
    pub IsExact: bool,
}

impl PbMessage for CharacterMoveToPointConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TargetPos {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MoveType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveType);
        }
        if self.IsExact {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TargetPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.MoveType = r.read_varint_i32()?,
            3 => self.IsExact = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterMoveToPointEndPush {
    pub IsInterrupt: bool,
    pub EntityId: i64,
}

impl CharacterMoveToPointEndPush {
    pub const MSG_ID: u16 = 27483;
}

impl PbMessage for CharacterMoveToPointEndPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsInterrupt {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            14 => self.IsInterrupt = r.read_bool()?,
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterMoveToPointNotify {
    pub MoveStatus: Option<MoveStatus>,
    pub EntityId: i64,
    pub PlayerId: i32,
    pub Config: Option<CharacterMoveToPointConfig>,
}

impl CharacterMoveToPointNotify {
    pub const MSG_ID: u16 = 29143;
}

impl PbMessage for CharacterMoveToPointNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MoveStatus {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Config {
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
                self.MoveStatus = Some(<MoveStatus as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.EntityId = r.read_varint_i64()?,
            11 => self.PlayerId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Config
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatMutePlayerListNotify {
    pub PlayerId: Vec<i32>,
}

impl ChatMutePlayerListNotify {
    pub const MSG_ID: u16 = 29806;
}

impl PbMessage for ChatMutePlayerListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PlayerId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatMutePlayerRequest {
    pub Mute: bool,
    pub TargetUID: i32,
}

impl ChatMutePlayerRequest {
    pub const MSG_ID: u16 = 26518;
}

impl PbMessage for ChatMutePlayerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Mute {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TargetUID != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUID);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Mute = r.read_bool()?,
            7 => self.TargetUID = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatMutePlayerResponse {
    pub RemoveMutes: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ChatMutePlayerResponse {
    pub const MSG_ID: u16 = 26851;
}

impl PbMessage for ChatMutePlayerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RemoveMutes {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RemoveMutes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct ChatNotify {
    pub Content: Vec<u8>,
}

impl ChatNotify {
    pub const MSG_ID: u16 = 27064;
}

impl PbMessage for ChatNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
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
            14 => self.Content = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckClientPush {
    pub AppVersion: Vec<u8>,
    pub LauncherVersion: Vec<u8>,
    pub ResourceVersion: Vec<u8>,
}

impl CheckClientPush {
    pub const MSG_ID: u16 = 116;
}

impl PbMessage for CheckClientPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AppVersion.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AppVersion);
        }
        if !self.LauncherVersion.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LauncherVersion);
        }
        if !self.ResourceVersion.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourceVersion);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AppVersion = r.read_len_delimited()?.to_vec(),
            2 => self.LauncherVersion = r.read_len_delimited()?.to_vec(),
            3 => self.ResourceVersion = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckGearRequest {
    pub EntityId: i64,
}

impl CheckGearRequest {
    pub const MSG_ID: u16 = 27460;
}

impl PbMessage for CheckGearRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckGearResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CheckGearResponse {
    pub const MSG_ID: u16 = 26915;
}

impl PbMessage for CheckGearResponse {
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
pub struct CheckPublicResourceVersionRequest {
    pub PublicResourceVersionInfo: Option<PublicResourceVersionInfo>,
}

impl CheckPublicResourceVersionRequest {
    pub const MSG_ID: u16 = 17400;
}

impl PbMessage for CheckPublicResourceVersionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PublicResourceVersionInfo {
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
                self.PublicResourceVersionInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckPublicResourceVersionResponse {
    pub ErrorId: Option<ErrorCode>,
    pub TipLevel: i32,
}

impl CheckPublicResourceVersionResponse {
    pub const MSG_ID: u16 = 21060;
}

impl PbMessage for CheckPublicResourceVersionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TipLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TipLevel);
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
            8 => self.TipLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckUseSoundBoxSkillRequest {}

impl CheckUseSoundBoxSkillRequest {
    pub const MSG_ID: u16 = 16116;
}

impl PbMessage for CheckUseSoundBoxSkillRequest {
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
pub struct CheckUseSoundBoxSkillResponse {
    pub Code: Option<ErrorCode>,
}

impl CheckUseSoundBoxSkillResponse {
    pub const MSG_ID: u16 = 18835;
}

impl PbMessage for CheckUseSoundBoxSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct CiacconaActivityPbData {
    pub ChapterDatas: Vec<CiacconaChapterPbData>,
    pub ScheduleRewardDatas: Vec<CiacconaScheduleRewardPbData>,
    pub ActivityResultDatas: Vec<CiacconaActivityResultPbData>,
    pub InspirationData: Option<CiacconaInspirationPbData>,
    pub State1Unlock: bool,
    pub State2Unlock: bool,
    pub LimitedStartTime: i64,
    pub LimitedEndTime: i64,
}

impl PbMessage for CiacconaActivityPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChapterDatas {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ScheduleRewardDatas {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.ActivityResultDatas {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.InspirationData {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.State1Unlock {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.State2Unlock {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LimitedStartTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedStartTime);
        }
        if self.LimitedEndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedEndTime);
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
                self.ChapterDatas
                    .push(<CiacconaChapterPbData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ScheduleRewardDatas
                    .push(<CiacconaScheduleRewardPbData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityResultDatas
                    .push(<CiacconaActivityResultPbData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InspirationData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.State1Unlock = r.read_bool()?,
            6 => self.State2Unlock = r.read_bool()?,
            7 => self.LimitedStartTime = r.read_varint_i64()?,
            8 => self.LimitedEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaActivityResultPbData {
    pub ResultId: i32,
    pub TakeReward: bool,
}

impl PbMessage for CiacconaActivityResultPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResultId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResultId);
        }
        if self.TakeReward {
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
            1 => self.ResultId = r.read_varint_i32()?,
            2 => self.TakeReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaActivityStateUnlockUpdateNotify {
    pub State1Unlock: bool,
    pub ActivityId: i32,
    pub State2Unlock: bool,
}

impl CiacconaActivityStateUnlockUpdateNotify {
    pub const MSG_ID: u16 = 17332;
}

impl PbMessage for CiacconaActivityStateUnlockUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.State1Unlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.State2Unlock {
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
            2 => self.State1Unlock = r.read_bool()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            14 => self.State2Unlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaChapterChoicePbData {
    pub ChoiceId: i32,
    pub InspirationUnlock: bool,
    pub ConditionUnlock: bool,
}

impl PbMessage for CiacconaChapterChoicePbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChoiceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChoiceId);
        }
        if self.InspirationUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ConditionUnlock {
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
            1 => self.ChoiceId = r.read_varint_i32()?,
            2 => self.InspirationUnlock = r.read_bool()?,
            3 => self.ConditionUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaChapterPbData {
    pub CharacterId: i32,
    pub Unlock: bool,
    pub ResultDatas: Vec<CiacconaChapterResultPbData>,
    pub ChoiceDatas: Vec<CiacconaChapterChoicePbData>,
}

impl PbMessage for CiacconaChapterPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CharacterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CharacterId);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ResultDatas {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.ChoiceDatas {
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
            1 => self.CharacterId = r.read_varint_i32()?,
            2 => self.Unlock = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ResultDatas
                    .push(<CiacconaChapterResultPbData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ChoiceDatas
                    .push(<CiacconaChapterChoicePbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaChapterResultPbData {
    pub ResultId: i32,
    pub Finish: bool,
    pub TakeReward: bool,
}

impl PbMessage for CiacconaChapterResultPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResultId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResultId);
        }
        if self.Finish {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TakeReward {
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
            1 => self.ResultId = r.read_varint_i32()?,
            2 => self.Finish = r.read_bool()?,
            3 => self.TakeReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaInspirationPbData {
    pub Inspiration: i32,
    pub RefreshTime: i64,
}

impl PbMessage for CiacconaInspirationPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Inspiration != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Inspiration);
        }
        if self.RefreshTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RefreshTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Inspiration = r.read_varint_i32()?,
            2 => self.RefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CiacconaScheduleRewardPbData {
    pub RewardId: i32,
    pub CanReward: bool,
    pub TakeReward: bool,
}

impl PbMessage for CiacconaScheduleRewardPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
        }
        if self.CanReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TakeReward {
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
            1 => self.RewardId = r.read_varint_i32()?,
            2 => self.CanReward = r.read_bool()?,
            3 => self.TakeReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CircumFluenceData {
    pub ActivityTasks: Vec<ActivityTask>,
    pub SignRewards: Vec<ActivityTask>,
    pub ScoreRewards: Vec<i32>,
    pub NowRoundOpen: bool,
    pub EndTime: i64,
    pub NextRefreshTime: i64,
}

impl PbMessage for CircumFluenceData {
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
        if self.NowRoundOpen {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextRefreshTime);
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
            5 => self.NowRoundOpen = r.read_bool()?,
            6 => self.EndTime = r.read_varint_i64()?,
            7 => self.NextRefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CircumFluenceTaskRewardRequest {
    pub ActivityId: i32,
    pub Ids: Vec<i32>,
    pub RewardType: Option<CircumFluenceRewardType>,
}

impl CircumFluenceTaskRewardRequest {
    pub const MSG_ID: u16 = 25593;
}

impl PbMessage for CircumFluenceTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.RewardType {
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
            5 => self.ActivityId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                self.RewardType = Some(
                    <CircumFluenceRewardType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct CircumFluenceTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CircumFluenceTaskRewardResponse {
    pub const MSG_ID: u16 = 21293;
}

impl PbMessage for CircumFluenceTaskRewardResponse {
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
pub struct ClientBasicInfoRequest {
    pub ClientBasicInfo: Option<ClientBasicInfo>,
}

impl ClientBasicInfoRequest {
    pub const MSG_ID: u16 = 23592;
}

impl PbMessage for ClientBasicInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ClientBasicInfo {
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
                self.ClientBasicInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientBasicInfoResponse {}

impl ClientBasicInfoResponse {
    pub const MSG_ID: u16 = 19121;
}

impl PbMessage for ClientBasicInfoResponse {
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
pub struct ClientTriggerTeleportPb {
    pub ActionCtx: Option<GameCtxPb>,
    pub Index: i32,
    pub StreamingSourceConditionIndex: i32,
}

impl PbMessage for ClientTriggerTeleportPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActionCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.StreamingSourceConditionIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StreamingSourceConditionIndex);
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
                self.ActionCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Index = r.read_varint_i32()?,
            3 => self.StreamingSourceConditionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientVersionNoMatchNotify {
    pub TeamType: Option<TeamType>,
    pub PlayerId: i32,
}

impl ClientVersionNoMatchNotify {
    pub const MSG_ID: u16 = 16267;
}

impl PbMessage for ClientVersionNoMatchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TeamType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => {
                self.TeamType = Some(<TeamType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombineEntitiesRequest {
    pub TargetEntity: i64,
    pub CombinePartInfo: Option<CombinePartInfo>,
}

impl CombineEntitiesRequest {
    pub const MSG_ID: u16 = 28910;
}

impl PbMessage for CombineEntitiesRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetEntity != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntity);
        }
        if let Some(v) = &self.CombinePartInfo {
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
            3 => self.TargetEntity = r.read_varint_i64()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CombinePartInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombineEntitiesResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CombineEntitiesResponse {
    pub const MSG_ID: u16 = 16600;
}

impl PbMessage for CombineEntitiesResponse {
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
pub struct CombineEntityInfoPb {
    pub CombinePartInfoList: Vec<CombinePartInfo>,
    pub TargetEntityId: i64,
}

impl PbMessage for CombineEntityInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CombinePartInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
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
                self.CombinePartInfoList
                    .push(<CombinePartInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombinePartInfo {
    pub CombinerEntityId: i64,
    pub OffsetPos: Option<Vector>,
    pub OffsetRotate: Option<Rotator>,
    pub PartId: i32,
}

impl PbMessage for CombinePartInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CombinerEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CombinerEntityId);
        }
        if let Some(v) = &self.OffsetPos {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.OffsetRotate {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.PartId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CombinerEntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.OffsetPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.OffsetRotate
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.PartId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComboParamInfo {
    pub KeepTime: i32,
    pub AddTime: i32,
}

impl PbMessage for ComboParamInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.KeepTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KeepTime);
        }
        if self.AddTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.KeepTime = r.read_varint_i32()?,
            2 => self.AddTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommitBoardRequest {
    pub BoardEntityId: i64,
}

impl CommitBoardRequest {
    pub const MSG_ID: u16 = 22677;
}

impl PbMessage for CommitBoardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoardEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.BoardEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommitBoardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl CommitBoardResponse {
    pub const MSG_ID: u16 = 16328;
}

impl PbMessage for CommitBoardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct CommitBoardSuccessNotify {
    pub BoardEntityId: i64,
}

impl CommitBoardSuccessNotify {
    pub const MSG_ID: u16 = 21576;
}

impl PbMessage for CommitBoardSuccessNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoardEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.BoardEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonOrganActionListNotify {
    pub Id: i32,
    pub ActionIndex: i32,
}

impl CommonOrganActionListNotify {
    pub const MSG_ID: u16 = 25907;
}

impl PbMessage for CommonOrganActionListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ActionIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.Id = r.read_varint_i32()?,
            14 => self.ActionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonOrganResetTimeRequest {
    pub EntityId: i64,
}

impl CommonOrganResetTimeRequest {
    pub const MSG_ID: u16 = 22989;
}

impl PbMessage for CommonOrganResetTimeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonOrganResetTimeResponse {
    pub EntityId: i64,
    pub ErrorCode: Option<ErrorCode>,
    pub ResetTime: i32,
}

impl CommonOrganResetTimeResponse {
    pub const MSG_ID: u16 = 18085;
}

impl PbMessage for CommonOrganResetTimeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ResetTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResetTime);
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.ResetTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonTouchUiSettingUpdateRequest {
    pub MobileButtonSettings: Vec<MobileButtonSetting>,
}

impl CommonTouchUiSettingUpdateRequest {
    pub const MSG_ID: u16 = 21364;
}

impl PbMessage for CommonTouchUiSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MobileButtonSettings {
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
                self.MobileButtonSettings
                    .push(<MobileButtonSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CommonTouchUiSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CommonTouchUiSettingUpdateResponse {
    pub const MSG_ID: u16 = 19580;
}

impl PbMessage for CommonTouchUiSettingUpdateResponse {
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
pub struct CommonUiSettingNotify {
    pub CommonUiButtonSetting: Vec<MobileButtonSetting>,
}

impl CommonUiSettingNotify {
    pub const MSG_ID: u16 = 16208;
}

impl PbMessage for CommonUiSettingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CommonUiButtonSetting {
            crate::wire::write_submessage(out, 7, item);
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
                let bytes = r.read_len_delimited()?;
                self.CommonUiButtonSetting
                    .push(<MobileButtonSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CompleteMarkNotify {
    pub MarkIds: Vec<i32>,
}

impl CompleteMarkNotify {
    pub const MSG_ID: u16 = 27647;
}

impl PbMessage for CompleteMarkNotify {
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
pub struct CompleteQuestSyncDataPb {
    pub QuestId: Vec<i32>,
}

impl PbMessage for CompleteQuestSyncDataPb {
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
pub struct CondInfo {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
}

impl PbMessage for CondInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Current != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Current);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Target);
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
            2 => self.Current = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionTaskData {
    pub Id: i32,
    pub Status: Option<ConditionTaskStatus>,
    pub Rate: i32,
    pub Target: i32,
    pub IsConditionUnlock: bool,
}

impl PbMessage for ConditionTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Rate != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rate);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Target);
        }
        if self.IsConditionUnlock {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <ConditionTaskStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Rate = r.read_varint_i32()?,
            4 => self.Target = r.read_varint_i32()?,
            6 => self.IsConditionUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectEntityInfo {
    pub SrcEntityId: i64,
    pub TargetEntityInfos: Vec<ConnectEntityTarget>,
    pub IsConnect: bool,
}

impl PbMessage for ConnectEntityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SrcEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SrcEntityId);
        }
        for item in &self.TargetEntityInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.IsConnect {
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
            1 => self.SrcEntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TargetEntityInfos
                    .push(<ConnectEntityTarget as PbMessage>::decode(bytes)?);
            }
            3 => self.IsConnect = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectEntityTarget {
    pub TargetId: i64,
    pub ConnectPortalIds: Vec<i64>,
}

impl PbMessage for ConnectEntityTarget {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        for item in &self.ConnectPortalIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetId = r.read_varint_i64()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ConnectPortalIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectorCanConnectRequest {
    pub IsEnter: bool,
    pub EntityId: i64,
}

impl ConnectorCanConnectRequest {
    pub const MSG_ID: u16 = 27563;
}

impl PbMessage for ConnectorCanConnectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnter {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            4 => self.IsEnter = r.read_bool()?,
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectorCanConnectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ConnectorCanConnectResponse {
    pub const MSG_ID: u16 = 17977;
}

impl PbMessage for ConnectorCanConnectResponse {
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
pub struct ConnectorConnectEntityNotify {
    pub ConnectEntityInfos: Vec<ConnectEntityInfo>,
}

impl ConnectorConnectEntityNotify {
    pub const MSG_ID: u16 = 29082;
}

impl PbMessage for ConnectorConnectEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConnectEntityInfos {
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
                self.ConnectEntityInfos
                    .push(<ConnectEntityInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectorConnectEntityRequest {
    pub ConnectEntityInfo: Option<ConnectEntityInfo>,
}

impl ConnectorConnectEntityRequest {
    pub const MSG_ID: u16 = 29745;
}

impl PbMessage for ConnectorConnectEntityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ConnectEntityInfo {
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
                self.ConnectEntityInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConnectorConnectEntityResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ConnectorConnectEntityResponse {
    pub const MSG_ID: u16 = 15177;
}

impl PbMessage for ConnectorConnectEntityResponse {
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
pub struct ConsoleNotify {
    pub Type: Vec<u8>,
    pub Content: Vec<u8>,
}

impl ConsoleNotify {
    pub const MSG_ID: u16 = 19286;
}

impl PbMessage for ConsoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Type.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Type);
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
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
            8 => self.Type = r.read_len_delimited()?.to_vec(),
            10 => self.Content = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConstructTrapRequest {
    pub TrapGridPlacementPbInfo: Option<GridPlacementPbInfo>,
    pub ConfigId: i32,
}

impl ConstructTrapRequest {
    pub const MSG_ID: u16 = 17132;
}

impl PbMessage for ConstructTrapRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TrapGridPlacementPbInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TrapGridPlacementPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConstructTrapResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Id: i64,
}

impl ConstructTrapResponse {
    pub const MSG_ID: u16 = 23882;
}

impl PbMessage for ConstructTrapResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
            10 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveActivityInfo {
    pub ActivityId: i32,
    pub Tasks: Vec<ConsumptiveTaskInfo>,
    pub TotalScore: i32,
}

impl PbMessage for ConsumptiveActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Tasks
                    .push(<ConsumptiveTaskInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.TotalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveActivityInfoRequest {}

impl ConsumptiveActivityInfoRequest {
    pub const MSG_ID: u16 = 24785;
}

impl PbMessage for ConsumptiveActivityInfoRequest {
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
pub struct ConsumptiveActivityInfoResponse {
    pub ConsumptiveActivityInfo: Option<ConsumptiveActivityInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ConsumptiveActivityInfoResponse {
    pub const MSG_ID: u16 = 18014;
}

impl PbMessage for ConsumptiveActivityInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ConsumptiveActivityInfo {
            crate::wire::write_submessage(out, 12, v);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ConsumptiveActivityInfo
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
pub struct ConsumptiveActivityScoreRequest {
    pub ActivityId: i32,
}

impl ConsumptiveActivityScoreRequest {
    pub const MSG_ID: u16 = 21065;
}

impl PbMessage for ConsumptiveActivityScoreRequest {
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
pub struct ConsumptiveActivityScoreResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TotalScore: i32,
}

impl ConsumptiveActivityScoreResponse {
    pub const MSG_ID: u16 = 15654;
}

impl PbMessage for ConsumptiveActivityScoreResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
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
            8 => self.TotalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveRewardRequest {
    pub ActivityId: i32,
    pub TaskIds: Vec<i32>,
}

impl ConsumptiveRewardRequest {
    pub const MSG_ID: u16 = 26335;
}

impl PbMessage for ConsumptiveRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
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
            5 => self.ActivityId = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskInfos: Vec<ConsumptiveTaskInfo>,
    pub TaskInfo: Option<ConsumptiveTaskInfo>,
}

impl ConsumptiveRewardResponse {
    pub const MSG_ID: u16 = 25276;
}

impl PbMessage for ConsumptiveRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.TaskInfo {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TaskInfos
                    .push(<ConsumptiveTaskInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TaskInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveTaskInfo {
    pub Id: i32,
    pub Type: Option<ConsumptiveTaskType>,
    pub Process: Option<ConsumptiveTaskProcess>,
    pub Reward: Option<ConsumptiveTaskReward>,
}

impl PbMessage for ConsumptiveTaskInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Process {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Reward {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <ConsumptiveTaskType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Process
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveTaskInfoNotify {
    pub ConsumptiveTaskInfo: Option<ConsumptiveTaskInfo>,
}

impl ConsumptiveTaskInfoNotify {
    pub const MSG_ID: u16 = 22181;
}

impl PbMessage for ConsumptiveTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ConsumptiveTaskInfo {
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
                self.ConsumptiveTaskInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveTaskProcess {
    pub Current: i32,
    pub Target: i32,
}

impl PbMessage for ConsumptiveTaskProcess {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Current != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Current);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Target);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Current = r.read_varint_i32()?,
            2 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveTaskReward {
    pub Rewarded: i32,
    pub WaitReward: i32,
    pub MaxReward: i32,
}

impl PbMessage for ConsumptiveTaskReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Rewarded != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rewarded);
        }
        if self.WaitReward != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaitReward);
        }
        if self.MaxReward != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReward);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Rewarded = r.read_varint_i32()?,
            2 => self.WaitReward = r.read_varint_i32()?,
            3 => self.MaxReward = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConsumptiveTotalScoreNotify {
    pub TotalScore: i32,
}

impl ConsumptiveTotalScoreNotify {
    pub const MSG_ID: u16 = 22825;
}

impl PbMessage for ConsumptiveTotalScoreNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.TotalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ContextIdNotify {
    pub Id: i64,
}

impl ContextIdNotify {
    pub const MSG_ID: u16 = 17290;
}

impl PbMessage for ContextIdNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ControlPointData {
    pub ControlPointIndex: i32,
    pub LeftEnable: bool,
    pub RightEnable: bool,
}

impl PbMessage for ControlPointData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ControlPointIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ControlPointIndex);
        }
        if self.LeftEnable {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RightEnable {
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
            1 => self.ControlPointIndex = r.read_varint_i32()?,
            2 => self.LeftEnable = r.read_bool()?,
            3 => self.RightEnable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopRoleInfoUpdateNotify {
    pub CoopTaskCompleteInfo: Option<CoopTaskCompleteInfo>,
    pub CoopRoleInfo: Option<CoopRoleInfo>,
    pub PreCompleteId: i32,
    pub ActivityId: i32,
}

impl CoopRoleInfoUpdateNotify {
    pub const MSG_ID: u16 = 27573;
}

impl PbMessage for CoopRoleInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CoopTaskCompleteInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.CoopRoleInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.PreCompleteId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreCompleteId);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CoopTaskCompleteInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CoopRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.PreCompleteId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopRoleRewardRequest {
    pub CoopId: i32,
}

impl CoopRoleRewardRequest {
    pub const MSG_ID: u16 = 25686;
}

impl PbMessage for CoopRoleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CoopId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CoopId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.CoopId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopRoleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CoopRoleRewardResponse {
    pub const MSG_ID: u16 = 25354;
}

impl PbMessage for CoopRoleRewardResponse {
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
pub struct CoopSpRewardRequest {
    pub RewardIds: Vec<i32>,
}

impl CoopSpRewardRequest {
    pub const MSG_ID: u16 = 22261;
}

impl PbMessage for CoopSpRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CoopSpRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CoopSpRewardResponse {
    pub const MSG_ID: u16 = 19720;
}

impl PbMessage for CoopSpRewardResponse {
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
pub struct CorniceChallengeEndNotify {
    pub NewScoreRecord: bool,
    pub RemainingTime: i32,
    pub Score: i32,
    pub NewRemainingTimeRecord: bool,
    pub LevelPlayId: i32,
}

impl CorniceChallengeEndNotify {
    pub const MSG_ID: u16 = 18364;
}

impl PbMessage for CorniceChallengeEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NewScoreRecord {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RemainingTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainingTime);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.NewRemainingTimeRecord {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.NewScoreRecord = r.read_bool()?,
            8 => self.RemainingTime = r.read_varint_i32()?,
            12 => self.Score = r.read_varint_i32()?,
            13 => self.NewRemainingTimeRecord = r.read_bool()?,
            15 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorniceChallengeTransRequest {
    pub LevelPlayId: i32,
}

impl CorniceChallengeTransRequest {
    pub const MSG_ID: u16 = 23951;
}

impl PbMessage for CorniceChallengeTransRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorniceChallengeTransResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CorniceChallengeTransResponse {
    pub const MSG_ID: u16 = 28762;
}

impl PbMessage for CorniceChallengeTransResponse {
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
pub struct CorniceLevelPlayEnrty {
    pub MaxScore: i32,
    pub RemainingTime: i32,
    pub UnlockTime: i64,
    pub GetRewardScoreIndexList: Vec<i32>,
}

impl PbMessage for CorniceLevelPlayEnrty {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.RemainingTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainingTime);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        for item in &self.GetRewardScoreIndexList {
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
            1 => self.MaxScore = r.read_varint_i32()?,
            2 => self.RemainingTime = r.read_varint_i32()?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.GetRewardScoreIndexList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorniceMeetingData {
    pub QuestUnlockTime: i64,
    pub CorninceDataList: BTreeMap<i32, CorniceLevelPlayEnrty>,
}

impl PbMessage for CorniceMeetingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestUnlockTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.QuestUnlockTime);
        }
        if !self.CorninceDataList.is_empty() {
            crate::wire::write_map(out, 2, &self.CorninceDataList);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.QuestUnlockTime = r.read_varint_i64()?,
            2 => crate::wire::decode_map_entry(r, &mut self.CorninceDataList)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorniceMeetingRewardRequest {
    pub ScoreIndex: i32,
    pub LevelPlayId: i32,
}

impl CorniceMeetingRewardRequest {
    pub const MSG_ID: u16 = 18385;
}

impl PbMessage for CorniceMeetingRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ScoreIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScoreIndex);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ScoreIndex = r.read_varint_i32()?,
            9 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CorniceMeetingRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CorniceMeetingRewardResponse {
    pub const MSG_ID: u16 = 18253;
}

impl PbMessage for CorniceMeetingRewardResponse {
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
pub struct CorniceMeetingRewardScoreList {
    pub ScoreIndexes: Vec<i32>,
}

impl PbMessage for CorniceMeetingRewardScoreList {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreIndexes {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreIndexes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CostPhantomAttrRecommendInfo {
    pub Cost: i32,
    pub MainAttrRecommendInfos: Vec<PhantomAttrRecommendInfo>,
    pub ViceAttrRecommendInfos: Vec<PhantomAttrRecommendInfo>,
}

impl PbMessage for CostPhantomAttrRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Cost != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cost);
        }
        for item in &self.MainAttrRecommendInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.ViceAttrRecommendInfos {
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
            1 => self.Cost = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MainAttrRecommendInfos
                    .push(<PhantomAttrRecommendInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ViceAttrRecommendInfos
                    .push(<PhantomAttrRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreatStoryCharacterRequest {}

impl CreatStoryCharacterRequest {
    pub const MSG_ID: u16 = 29348;
}

impl PbMessage for CreatStoryCharacterRequest {
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
pub struct CreatStoryCharacterResponse {
    pub EntityId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl CreatStoryCharacterResponse {
    pub const MSG_ID: u16 = 24319;
}

impl PbMessage for CreatStoryCharacterResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            5 => self.EntityId = r.read_varint_i64()?,
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
pub struct CreateBulletPush {
    pub Handle: Option<ActiveBulletHandle>,
    pub RandomInitPosOffset: Option<Vector>,
    pub SkillId: i64,
    pub SpawnVelocityEntityId: i64,
    pub CombatCommon: Option<CombatCommon>,
    pub RandomInitSpeed: Option<Vector>,
    pub TargetId: i64,
    pub OwnerEntityId: i64,
    pub ParentHandle: Option<ActiveBulletHandle>,
    pub IsLocal: bool,
    pub DtType: i32,
    pub BulletId: i64,
    pub SpawnEntityId: i64,
    pub Rotation: Option<Rotator>,
    pub Location: Option<Vector>,
}

impl CreateBulletPush {
    pub const MSG_ID: u16 = 26882;
}

impl PbMessage for CreateBulletPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.RandomInitPosOffset {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.SpawnVelocityEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnVelocityEntityId);
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.RandomInitSpeed {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.OwnerEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerEntityId);
        }
        if let Some(v) = &self.ParentHandle {
            crate::wire::write_submessage(out, 9, v);
        }
        if self.IsLocal {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DtType != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DtType);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if self.SpawnEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnEntityId);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.Location {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RandomInitPosOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.SkillId = r.read_varint_i64()?,
            4 => self.SpawnVelocityEntityId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RandomInitSpeed
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.TargetId = r.read_varint_i64()?,
            8 => self.OwnerEntityId = r.read_varint_i64()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ParentHandle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.IsLocal = r.read_bool()?,
            11 => self.DtType = r.read_varint_i32()?,
            12 => self.BulletId = r.read_varint_i64()?,
            13 => self.SpawnEntityId = r.read_varint_i64()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
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
pub struct CreatureDetailRequest {
    pub Id: i64,
}

impl CreatureDetailRequest {
    pub const MSG_ID: u16 = 21574;
}

impl PbMessage for CreatureDetailRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreatureDetailResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ConfigIndex: i32,
    pub Id: i64,
    pub AliveEntities: Vec<DynamicEntityInformation>,
    pub CurCount: i32,
    pub ConditionSatisfy: bool,
    pub ExistCount: i32,
    pub AreaIndex: i32,
    pub NextRefreshTime: i64,
}

impl CreatureDetailResponse {
    pub const MSG_ID: u16 = 16069;
}

impl PbMessage for CreatureDetailResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ConfigIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigIndex);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.AliveEntities {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.CurCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCount);
        }
        if self.ConditionSatisfy {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ExistCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExistCount);
        }
        if self.AreaIndex != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaIndex);
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextRefreshTime);
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
            5 => self.ConfigIndex = r.read_varint_i32()?,
            6 => self.Id = r.read_varint_i64()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AliveEntities
                    .push(<DynamicEntityInformation as PbMessage>::decode(bytes)?);
            }
            8 => self.CurCount = r.read_varint_i32()?,
            10 => self.ConditionSatisfy = r.read_bool()?,
            11 => self.ExistCount = r.read_varint_i32()?,
            12 => self.AreaIndex = r.read_varint_i32()?,
            14 => self.NextRefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyAdventurePtRewardRequest {
    pub Id: i32,
}

impl DailyAdventurePtRewardRequest {
    pub const MSG_ID: u16 = 24837;
}

impl PbMessage for DailyAdventurePtRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyAdventurePtRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DailyAdventurePtRewardResponse {
    pub const MSG_ID: u16 = 17678;
}

impl PbMessage for DailyAdventurePtRewardResponse {
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
pub struct DailyAdventureTaskRewardRequest {
    pub Id: i32,
}

impl DailyAdventureTaskRewardRequest {
    pub const MSG_ID: u16 = 23720;
}

impl PbMessage for DailyAdventureTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DailyAdventureTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DailyAdventureTaskRewardResponse {
    pub const MSG_ID: u16 = 17499;
}

impl PbMessage for DailyAdventureTaskRewardResponse {
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
pub struct DailyAdventureTaskUpdateNotify {
    pub DailyAdventureActivityTasks: Vec<DailyAdventureActivityTask>,
    pub ResetDaily: bool,
}

impl DailyAdventureTaskUpdateNotify {
    pub const MSG_ID: u16 = 28227;
}

impl PbMessage for DailyAdventureTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DailyAdventureActivityTasks {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.ResetDaily {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DailyAdventureActivityTasks
                    .push(<DailyAdventureActivityTask as PbMessage>::decode(bytes)?);
            }
            7 => self.ResetDaily = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlActivityLimitCoinUpdateNotify {
    pub LimitCoinCost: i32,
    pub ActivityId: i32,
}

impl DangoBrawlActivityLimitCoinUpdateNotify {
    pub const MSG_ID: u16 = 25541;
}

impl PbMessage for DangoBrawlActivityLimitCoinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LimitCoinCost != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitCoinCost);
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
            11 => self.LimitCoinCost = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagInfo {
    pub NormalItems: Vec<DangoBrawlNormalBagItemInfo>,
    pub BenchItems: Vec<DangoBrawlBenchItemInfo>,
}

impl PbMessage for DangoBrawlBagInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NormalItems {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BenchItems {
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
                self.NormalItems
                    .push(<DangoBrawlNormalBagItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BenchItems
                    .push(<DangoBrawlBenchItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagOpInfo {
    pub OpType: Option<DangoBrawlBagOpType>,
    pub FromUid: i32,
    pub Slot: Option<DangoBrawlBagSlotInfo>,
}

impl PbMessage for DangoBrawlBagOpInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OpType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FromUid != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromUid);
        }
        if let Some(v) = &self.Slot {
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
            1 => {
                self.OpType = Some(
                    <DangoBrawlBagOpType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.FromUid = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Slot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagOperateRequest {
    pub Ops: Vec<DangoBrawlBagOpInfo>,
}

impl DangoBrawlBagOperateRequest {
    pub const MSG_ID: u16 = 27984;
}

impl PbMessage for DangoBrawlBagOperateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ops {
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
                self.Ops
                    .push(<DangoBrawlBagOpInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagOperateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlBagOperateResponse {
    pub const MSG_ID: u16 = 17507;
}

impl PbMessage for DangoBrawlBagOperateResponse {
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
