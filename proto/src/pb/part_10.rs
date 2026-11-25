use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchTeamStateNotify {
    pub TeamState: Option<MatchTeamState>,
}

impl MatchTeamStateNotify {
    pub const MSG_ID: u16 = 19812;
}

impl PbMessage for MatchTeamStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TeamState {
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
                self.TeamState = Some(
                    <MatchTeamState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct MatchThreeEndRequest {
    pub LevelId: i32,
    pub Results: BTreeMap<i32, i32>,
    pub TraceId: Vec<u8>,
}

impl MatchThreeEndRequest {
    pub const MSG_ID: u16 = 29259;
}

impl PbMessage for MatchThreeEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if !self.Results.is_empty() {
            crate::wire::write_map(out, 8, &self.Results);
        }
        if !self.TraceId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TraceId);
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
            8 => crate::wire::decode_map_entry(r, &mut self.Results)?,
            10 => self.TraceId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchThreeEndResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Success: bool,
}

impl MatchThreeEndResponse {
    pub const MSG_ID: u16 = 20119;
}

impl PbMessage for MatchThreeEndResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Success {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.Success = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchThreeStarRequest {
    pub LevelId: i32,
    pub JoinRole: i32,
}

impl MatchThreeStarRequest {
    pub const MSG_ID: u16 = 19959;
}

impl PbMessage for MatchThreeStarRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.JoinRole != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JoinRole);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.LevelId = r.read_varint_i32()?,
            6 => self.JoinRole = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchThreeStarResponse {
    pub TraceId: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MatchThreeStarResponse {
    pub const MSG_ID: u16 = 28482;
}

impl PbMessage for MatchThreeStarResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TraceId.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TraceId);
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
            4 => self.TraceId = r.read_len_delimited()?.to_vec(),
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
pub struct MatchingNotify {}

impl MatchingNotify {
    pub const MSG_ID: u16 = 20424;
}

impl PbMessage for MatchingNotify {
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
pub struct MaterialReplaceRequest {
    pub ConsumeItemId: i32,
    pub TargetReplaceItemId: i32,
    pub ConsumeItemNum: i32,
}

impl MaterialReplaceRequest {
    pub const MSG_ID: u16 = 26427;
}

impl PbMessage for MaterialReplaceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConsumeItemId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConsumeItemId);
        }
        if self.TargetReplaceItemId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetReplaceItemId);
        }
        if self.ConsumeItemNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConsumeItemNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.ConsumeItemId = r.read_varint_i32()?,
            13 => self.TargetReplaceItemId = r.read_varint_i32()?,
            15 => self.ConsumeItemNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MaterialReplaceResponse {
    pub Code: Option<ErrorCode>,
}

impl MaterialReplaceResponse {
    pub const MSG_ID: u16 = 17042;
}

impl PbMessage for MaterialReplaceResponse {
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
pub struct MaxMessageIdPush {}

impl MaxMessageIdPush {
    pub const MSG_ID: u16 = 22976;
}

impl PbMessage for MaxMessageIdPush {
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
pub struct MessageDecodeFailNotify {
    pub Conv: u32,
    pub Code: Option<ErrorCode>,
    pub ChannelId: Vec<u8>,
    pub SeqNo: i32,
}

impl MessageDecodeFailNotify {
    pub const MSG_ID: u16 = 25607;
}

impl PbMessage for MessageDecodeFailNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Conv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.Conv));
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ChannelId.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ChannelId);
        }
        if self.SeqNo != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeqNo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Conv = r.read_varint_u32()?,
            8 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.ChannelId = r.read_len_delimited()?.to_vec(),
            13 => self.SeqNo = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MingSuHandRewardRequest {
    pub DragonPoolId: i32,
    pub InteractEntityId: i64,
}

impl MingSuHandRewardRequest {
    pub const MSG_ID: u16 = 28370;
}

impl PbMessage for MingSuHandRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
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
            2 => self.DragonPoolId = r.read_varint_i32()?,
            12 => self.InteractEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MingSuHandRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LevelGains: i32,
}

impl MingSuHandRewardResponse {
    pub const MSG_ID: u16 = 22679;
}

impl PbMessage for MingSuHandRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LevelGains != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGains);
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
            10 => self.LevelGains = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiniLetterContentPb {
    pub LetterId: i32,
    pub ContentKey: Vec<Vec<u8>>,
}

impl PbMessage for MiniLetterContentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LetterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LetterId);
        }
        for item in &self.ContentKey {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
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
            1 => self.LetterId = r.read_varint_i32()?,
            2 => self.ContentKey.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiniWriteLetterLoginNotify {
    pub LetterContent: Vec<MiniLetterContentPb>,
}

impl MiniWriteLetterLoginNotify {
    pub const MSG_ID: u16 = 21985;
}

impl PbMessage for MiniWriteLetterLoginNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LetterContent {
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
                self.LetterContent
                    .push(<MiniLetterContentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiniWriteLetterSaveContentRequest {
    pub LetterContent: Vec<MiniLetterContentPb>,
}

impl MiniWriteLetterSaveContentRequest {
    pub const MSG_ID: u16 = 26392;
}

impl PbMessage for MiniWriteLetterSaveContentRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LetterContent {
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
                self.LetterContent
                    .push(<MiniLetterContentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MiniWriteLetterSaveContentResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl MiniWriteLetterSaveContentResponse {
    pub const MSG_ID: u16 = 17140;
}

impl PbMessage for MiniWriteLetterSaveContentResponse {
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
pub struct MixDrinkRoleInviteRequest {
    pub RoleId: i32,
}

impl MixDrinkRoleInviteRequest {
    pub const MSG_ID: u16 = 20628;
}

impl PbMessage for MixDrinkRoleInviteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            11 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MixDrinkRoleInviteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MixDrinkRoleInviteResponse {
    pub const MSG_ID: u16 = 18245;
}

impl PbMessage for MixDrinkRoleInviteResponse {
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
pub struct MixDrinkRoleRewardRequest {
    pub RoleInviteId: i32,
}

impl MixDrinkRoleRewardRequest {
    pub const MSG_ID: u16 = 21940;
}

impl PbMessage for MixDrinkRoleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleInviteId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleInviteId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.RoleInviteId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MixDrinkRoleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MixDrinkRoleRewardResponse {
    pub const MSG_ID: u16 = 21453;
}

impl PbMessage for MixDrinkRoleRewardResponse {
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
pub struct MixDrinkSettleRequest {
    pub LikePoint: i32,
    pub Orna: i32,
    pub RequireId: i32,
    pub Batching: Vec<i32>,
    pub BaseDrinks: Vec<i32>,
}

impl MixDrinkSettleRequest {
    pub const MSG_ID: u16 = 28130;
}

impl PbMessage for MixDrinkSettleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LikePoint != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LikePoint);
        }
        if self.Orna != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Orna);
        }
        if self.RequireId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RequireId);
        }
        for item in &self.Batching {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BaseDrinks {
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
            1 => self.LikePoint = r.read_varint_i32()?,
            5 => self.Orna = r.read_varint_i32()?,
            8 => self.RequireId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Batching, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BaseDrinks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MixDrinkSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MixDrinkSettleResponse {
    pub const MSG_ID: u16 = 19105;
}

impl PbMessage for MixDrinkSettleResponse {
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
pub struct MixDrinkTopicSelectRequest {
    pub TopicId: i32,
}

impl MixDrinkTopicSelectRequest {
    pub const MSG_ID: u16 = 27388;
}

impl PbMessage for MixDrinkTopicSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TopicId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TopicId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.TopicId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MixDrinkTopicSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MixDrinkTopicSelectResponse {
    pub const MSG_ID: u16 = 26371;
}

impl PbMessage for MixDrinkTopicSelectResponse {
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
pub struct MobileButtonSettingUpdateRequest {
    pub MobileButtonSettings: Vec<MobileButtonSetting>,
}

impl MobileButtonSettingUpdateRequest {
    pub const MSG_ID: u16 = 25947;
}

impl PbMessage for MobileButtonSettingUpdateRequest {
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
pub struct MobileButtonSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MobileButtonSettingUpdateResponse {
    pub const MSG_ID: u16 = 22058;
}

impl PbMessage for MobileButtonSettingUpdateResponse {
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
pub struct ModeInfo {
    pub IsUnlock: bool,
    pub ConditionGroupId: i32,
    pub WaveId: i32,
    pub KillMonsterCount: i32,
    pub IsFinish: bool,
}

impl PbMessage for ModeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsUnlock {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ConditionGroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionGroupId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
        if self.KillMonsterCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillMonsterCount);
        }
        if self.IsFinish {
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
            1 => self.IsUnlock = r.read_bool()?,
            2 => self.ConditionGroupId = r.read_varint_i32()?,
            3 => self.WaveId = r.read_varint_i32()?,
            4 => self.KillMonsterCount = r.read_varint_i32()?,
            5 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterAttributeArrayNotify {
    pub Monsters: Vec<MonsterAttributeNotify>,
}

impl MonsterAttributeArrayNotify {
    pub const MSG_ID: u16 = 24398;
}

impl PbMessage for MonsterAttributeArrayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Monsters {
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
                self.Monsters
                    .push(<MonsterAttributeNotify as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterAttributeNotify {
    pub Id: i64,
    pub Attributes: BTreeMap<i32, i32>,
}

impl MonsterAttributeNotify {
    pub const MSG_ID: u16 = 25923;
}

impl PbMessage for MonsterAttributeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if !self.Attributes.is_empty() {
            crate::wire::write_map(out, 14, &self.Attributes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.Id = r.read_varint_i64()?,
            14 => crate::wire::decode_map_entry(r, &mut self.Attributes)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterDetectionInfoRequest {
    pub Params: Vec<DetectionTarget>,
}

impl MonsterDetectionInfoRequest {
    pub const MSG_ID: u16 = 28163;
}

impl PbMessage for MonsterDetectionInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
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
                self.Params
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterDetectionInfoResponse {
    pub DetectionTarget: Vec<DetectionTarget>,
}

impl MonsterDetectionInfoResponse {
    pub const MSG_ID: u16 = 17979;
}

impl PbMessage for MonsterDetectionInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DetectionTarget {
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
                self.DetectionTarget
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterGachaDataChangeNotify {
    pub MonsterGachaDataPb: Option<MonsterGachaDataPb>,
}

impl MonsterGachaDataChangeNotify {
    pub const MSG_ID: u16 = 15207;
}

impl PbMessage for MonsterGachaDataChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MonsterGachaDataPb {
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
                self.MonsterGachaDataPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterLevelArrayNotify {
    pub Monsters: Vec<MonsterLevelNotify>,
}

impl MonsterLevelArrayNotify {
    pub const MSG_ID: u16 = 19542;
}

impl PbMessage for MonsterLevelArrayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Monsters {
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
                self.Monsters
                    .push(<MonsterLevelNotify as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterLevelNotify {
    pub Id: i64,
    pub Level: i32,
}

impl MonsterLevelNotify {
    pub const MSG_ID: u16 = 28638;
}

impl PbMessage for MonsterLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
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
            8 => self.Id = r.read_varint_i64()?,
            15 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterPatrolPointReachedRequest {
    pub SplineEntityConfigId: i32,
    pub Index: i32,
    pub EntityId: i64,
}

impl MonsterPatrolPointReachedRequest {
    pub const MSG_ID: u16 = 27779;
}

impl PbMessage for MonsterPatrolPointReachedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SplineEntityConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityConfigId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
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
            3 => self.SplineEntityConfigId = r.read_varint_i32()?,
            4 => self.Index = r.read_varint_i32()?,
            5 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterPatrolPointReachedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MonsterPatrolPointReachedResponse {
    pub const MSG_ID: u16 = 21552;
}

impl PbMessage for MonsterPatrolPointReachedResponse {
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
pub struct MontagePlayRequest {
    pub MontageIndex: i32,
}

impl MontagePlayRequest {
    pub const MSG_ID: u16 = 27974;
}

impl PbMessage for MontagePlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.MontageIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MontagePlayResponse {
    pub Error: Option<DErrorResult>,
    pub MontageIndex: i32,
    pub SkillId: i64,
}

impl MontagePlayResponse {
    pub const MSG_ID: u16 = 19825;
}

impl PbMessage for MontagePlayResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
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
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.MontageIndex = r.read_varint_i32()?,
            11 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonthCardUseNotify {
    pub ExtendedDays: i32,
    pub ItemId: i32,
    pub Days: i32,
    pub DailyRewardItemId: i32,
    pub DailyRewardItemCount: i32,
    pub Count: i32,
}

impl MonthCardUseNotify {
    pub const MSG_ID: u16 = 17882;
}

impl PbMessage for MonthCardUseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExtendedDays != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtendedDays);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Days != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Days);
        }
        if self.DailyRewardItemId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DailyRewardItemId);
        }
        if self.DailyRewardItemCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DailyRewardItemCount);
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
            1 => self.ExtendedDays = r.read_varint_i32()?,
            4 => self.ItemId = r.read_varint_i32()?,
            7 => self.Days = r.read_varint_i32()?,
            8 => self.DailyRewardItemId = r.read_varint_i32()?,
            9 => self.DailyRewardItemCount = r.read_varint_i32()?,
            14 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoonPhaseActivityData {
    pub MoonPhaseSelects: Vec<MoonPhaseSelect>,
    pub MoonPhaseSpReward: bool,
    pub NowMoonParse: i32,
}

impl PbMessage for MoonPhaseActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MoonPhaseSelects {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.MoonPhaseSpReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NowMoonParse != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NowMoonParse);
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
                self.MoonPhaseSelects
                    .push(<MoonPhaseSelect as PbMessage>::decode(bytes)?);
            }
            2 => self.MoonPhaseSpReward = r.read_bool()?,
            3 => self.NowMoonParse = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoonPhaseRandomRequest {}

impl MoonPhaseRandomRequest {
    pub const MSG_ID: u16 = 15067;
}

impl PbMessage for MoonPhaseRandomRequest {
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
pub struct MoonPhaseRandomResponse {
    pub Result: Option<MoonPhaseSelect>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MoonPhaseRandomResponse {
    pub const MSG_ID: u16 = 18392;
}

impl PbMessage for MoonPhaseRandomResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 8, v);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Result
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
pub struct MoonPhaseRewardRequest {}

impl MoonPhaseRewardRequest {
    pub const MSG_ID: u16 = 21365;
}

impl PbMessage for MoonPhaseRewardRequest {
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
pub struct MoonPhaseRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MoonPhaseRewardResponse {
    pub const MSG_ID: u16 = 15018;
}

impl PbMessage for MoonPhaseRewardResponse {
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
pub struct MoonPhaseSelect {
    pub MoonPhaseId: i32,
    pub Label1: i32,
    pub Label2: i32,
}

impl PbMessage for MoonPhaseSelect {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoonPhaseId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoonPhaseId);
        }
        if self.Label1 != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Label1);
        }
        if self.Label2 != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Label2);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MoonPhaseId = r.read_varint_i32()?,
            2 => self.Label1 = r.read_varint_i32()?,
            3 => self.Label2 = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleActivityData {
    pub MoraleAreaInfos: Vec<MoraleAreaInfo>,
    pub RewardedIds: Vec<i32>,
    pub FlagToBoxInfos: Vec<FlagToBoxInfo>,
}

impl PbMessage for MoraleActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MoraleAreaInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardedIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FlagToBoxInfos {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.MoraleAreaInfos
                    .push(<MoraleAreaInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FlagToBoxInfos
                    .push(<FlagToBoxInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleAreaInfo {
    pub MoraleArea: i32,
    pub ExploreBoxGetCount: i32,
}

impl PbMessage for MoraleAreaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoraleArea != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoraleArea);
        }
        if self.ExploreBoxGetCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreBoxGetCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MoraleArea = r.read_varint_i32()?,
            2 => self.ExploreBoxGetCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleInfoNotify {
    pub Reason: i32,
    pub TempMoraleExp: i32,
    pub IndomitableLevel: i32,
    pub IsStart: bool,
    pub KeepMoraleLevel: i32,
    pub ExpRatio: i32,
    pub MoraleConfigId: i32,
    pub PlayerId: i32,
    pub TempMoraleLevel: i32,
    pub KeepMoraleExp: i32,
}

impl MoraleInfoNotify {
    pub const MSG_ID: u16 = 21439;
}

impl PbMessage for MoraleInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Reason != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.TempMoraleExp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TempMoraleExp);
        }
        if self.IndomitableLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IndomitableLevel);
        }
        if self.IsStart {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.KeepMoraleLevel != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KeepMoraleLevel);
        }
        if self.ExpRatio != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExpRatio);
        }
        if self.MoraleConfigId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoraleConfigId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.TempMoraleLevel != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TempMoraleLevel);
        }
        if self.KeepMoraleExp != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KeepMoraleExp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Reason = r.read_varint_i32()?,
            3 => self.TempMoraleExp = r.read_varint_i32()?,
            6 => self.IndomitableLevel = r.read_varint_i32()?,
            8 => self.IsStart = r.read_bool()?,
            9 => self.KeepMoraleLevel = r.read_varint_i32()?,
            10 => self.ExpRatio = r.read_varint_i32()?,
            12 => self.MoraleConfigId = r.read_varint_i32()?,
            13 => self.PlayerId = r.read_varint_i32()?,
            14 => self.TempMoraleLevel = r.read_varint_i32()?,
            15 => self.KeepMoraleExp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoralePosRequest {}

impl MoralePosRequest {
    pub const MSG_ID: u16 = 17542;
}

impl PbMessage for MoralePosRequest {
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
pub struct MoralePosResponse {
    pub AreaPos: i32,
}

impl MoralePosResponse {
    pub const MSG_ID: u16 = 29731;
}

impl PbMessage for MoralePosResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaPos != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaPos);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.AreaPos = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleTreasureBoxTraceRequest {
    pub AreaPos: i32,
}

impl MoraleTreasureBoxTraceRequest {
    pub const MSG_ID: u16 = 26624;
}

impl PbMessage for MoraleTreasureBoxTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaPos != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaPos);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.AreaPos = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoraleTreasureBoxTraceResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AreaPos: i32,
    pub EntityIds: Vec<i32>,
}

impl MoraleTreasureBoxTraceResponse {
    pub const MSG_ID: u16 = 23584;
}

impl PbMessage for MoraleTreasureBoxTraceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AreaPos != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaPos);
        }
        for item in &self.EntityIds {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.AreaPos = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorAnimEntity {
    pub EntityId: i64,
    pub MotorAnimInfos: Vec<MotorAnimInfo>,
}

impl PbMessage for MotorAnimEntity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.MotorAnimInfos {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MotorAnimInfos
                    .push(<MotorAnimInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorAnimInfo {
    pub FrontWheelInfo: Option<MotorWheelInfo>,
    pub BackWheelInfo: Option<MotorWheelInfo>,
    pub IsBeingImpacted: bool,
    pub ImpactedVelocity: Option<Vector>,
    pub ImpactedNormal: Option<Vector>,
    pub MoveMix: Option<Vector>,
}

impl PbMessage for MotorAnimInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FrontWheelInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.BackWheelInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.IsBeingImpacted {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ImpactedVelocity {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ImpactedNormal {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.MoveMix {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.FrontWheelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BackWheelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.IsBeingImpacted = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ImpactedVelocity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ImpactedNormal
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MoveMix
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorAnimReplaySampleUdpNotify {
    pub MotorAnimEntities: Vec<MotorAnimEntity>,
}

impl MotorAnimReplaySampleUdpNotify {
    pub const MSG_ID: u16 = 17840;
}

impl PbMessage for MotorAnimReplaySampleUdpNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorAnimEntities {
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
                self.MotorAnimEntities
                    .push(<MotorAnimEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorAnimReplaySampleUdpPush {
    pub SceneOwnerId: i32,
    pub MotorAnimEntities: Vec<MotorAnimEntity>,
}

impl MotorAnimReplaySampleUdpPush {
    pub const MSG_ID: u16 = 18728;
}

impl PbMessage for MotorAnimReplaySampleUdpPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
        }
        for item in &self.MotorAnimEntities {
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
            11 => self.SceneOwnerId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.MotorAnimEntities
                    .push(<MotorAnimEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorBattleContextNotify {}

impl MotorBattleContextNotify {
    pub const MSG_ID: u16 = 29330;
}

impl PbMessage for MotorBattleContextNotify {
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
pub struct MotorCycleCruiseStatePush {
    pub IsStart: bool,
}

impl MotorCycleCruiseStatePush {
    pub const MSG_ID: u16 = 28534;
}

impl PbMessage for MotorCycleCruiseStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsStart {
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
            9 => self.IsStart = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDataNotify {
    pub Motor: Option<MotorPb>,
}

impl MotorDataNotify {
    pub const MSG_ID: u16 = 25272;
}

impl PbMessage for MotorDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Motor {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Motor
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDecalMotorIpRequest {
    pub ActivityId: i32,
}

impl MotorDecalMotorIpRequest {
    pub const MSG_ID: u16 = 27551;
}

impl PbMessage for MotorDecalMotorIpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDecalMotorIpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorIp: i32,
}

impl MotorDecalMotorIpResponse {
    pub const MSG_ID: u16 = 27523;
}

impl PbMessage for MotorDecalMotorIpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MotorIp != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorIp);
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
            15 => self.MotorIp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDecalTaskRequest {
    pub MotorIp: i32,
}

impl MotorDecalTaskRequest {
    pub const MSG_ID: u16 = 17341;
}

impl PbMessage for MotorDecalTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorIp != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorIp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.MotorIp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDecalTaskResponse {
    pub Ids: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorDecalTaskResponse {
    pub const MSG_ID: u16 = 22439;
}

impl PbMessage for MotorDecalTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
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
pub struct MotorDecalUpdateNotify {
    pub Task: Option<ConditionTask>,
}

impl MotorDecalUpdateNotify {
    pub const MSG_ID: u16 = 20711;
}

impl PbMessage for MotorDecalUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
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
pub struct MotorDevelopTaskRewardRequest {
    pub Ids: Vec<i32>,
}

impl MotorDevelopTaskRewardRequest {
    pub const MSG_ID: u16 = 16150;
}

impl PbMessage for MotorDevelopTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDevelopTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorDevelopTaskRewardResponse {
    pub const MSG_ID: u16 = 29442;
}

impl PbMessage for MotorDevelopTaskRewardResponse {
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
pub struct MotorDevelopTaskUpdateNotify {
    pub Task: Vec<ConditionTask>,
}

impl MotorDevelopTaskUpdateNotify {
    pub const MSG_ID: u16 = 27674;
}

impl PbMessage for MotorDevelopTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
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
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorExpAddNotify {
    pub MotorExpLimitGainDaily: i32,
    pub CurExp: i32,
    pub AddExp: i32,
    pub CurLevel: i32,
    pub MotorExpMonsterDropDailyLimit: i32,
}

impl MotorExpAddNotify {
    pub const MSG_ID: u16 = 20982;
}

impl PbMessage for MotorExpAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorExpLimitGainDaily != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorExpLimitGainDaily);
        }
        if self.CurExp != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurExp);
        }
        if self.AddExp != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddExp);
        }
        if self.CurLevel != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLevel);
        }
        if self.MotorExpMonsterDropDailyLimit != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorExpMonsterDropDailyLimit);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.MotorExpLimitGainDaily = r.read_varint_i32()?,
            12 => self.CurExp = r.read_varint_i32()?,
            13 => self.AddExp = r.read_varint_i32()?,
            14 => self.CurLevel = r.read_varint_i32()?,
            15 => self.MotorExpMonsterDropDailyLimit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightAddBuffNotify {
    pub Buffs: Vec<MotorFightRoundBuffPb>,
}

impl MotorFightAddBuffNotify {
    pub const MSG_ID: u16 = 18353;
}

impl PbMessage for MotorFightAddBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Buffs {
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
                self.Buffs
                    .push(<MotorFightRoundBuffPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightAddCollectionNotify {
    pub Collections: Vec<MotorFightRoundCollectionPb>,
}

impl MotorFightAddCollectionNotify {
    pub const MSG_ID: u16 = 27020;
}

impl PbMessage for MotorFightAddCollectionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Collections {
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
                self.Collections
                    .push(<MotorFightRoundCollectionPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightAddEnemyBuffNotify {
    pub EnemyBuffs: Vec<MotorFightRoundBuffPb>,
}

impl MotorFightAddEnemyBuffNotify {
    pub const MSG_ID: u16 = 17969;
}

impl PbMessage for MotorFightAddEnemyBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EnemyBuffs {
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
                self.EnemyBuffs
                    .push(<MotorFightRoundBuffPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightBossDropNotify {
    pub SubLevelIndex: i32,
    pub WaveGroupIndex: i32,
    pub Drops: Vec<MotorFightCollectionDrop>,
    pub BossId: i32,
}

impl MotorFightBossDropNotify {
    pub const MSG_ID: u16 = 15088;
}

impl PbMessage for MotorFightBossDropNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        for item in &self.Drops {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.BossId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SubLevelIndex = r.read_varint_i32()?,
            4 => self.WaveGroupIndex = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Drops
                    .push(<MotorFightCollectionDrop as PbMessage>::decode(bytes)?);
            }
            9 => self.BossId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightBossPb {
    pub BossId: i32,
    pub WaveId: i32,
}

impl PbMessage for MotorFightBossPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BossId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BossId = r.read_varint_i32()?,
            2 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightBuffGatePb {
    pub BuffGateId: i32,
    pub WaveId: i32,
}

impl PbMessage for MotorFightBuffGatePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffGateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffGateId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BuffGateId = r.read_varint_i32()?,
            2 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightCollectItemInfo {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for MotorFightCollectItemInfo {
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
pub struct MotorFightCollectionDrop {
    pub Pos: Option<MotorFightCollectionPos>,
    pub CollectionId: i32,
}

impl PbMessage for MotorFightCollectionDrop {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CollectionId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CollectionId);
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
                self.Pos = Some(
                    <MotorFightCollectionPos as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.CollectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightCtx {
    pub Continue: bool,
    pub LevelId: i32,
    pub RoleId: i32,
}

impl PbMessage for MotorFightCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Continue {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.Continue = r.read_bool()?,
            2 => self.LevelId = r.read_varint_i32()?,
            3 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightEnterSubLevelRequest {
    pub SubLevelIndex: i32,
}

impl MotorFightEnterSubLevelRequest {
    pub const MSG_ID: u16 = 28999;
}

impl PbMessage for MotorFightEnterSubLevelRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SubLevelIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightEnterSubLevelResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightEnterSubLevelResponse {
    pub const MSG_ID: u16 = 21543;
}

impl PbMessage for MotorFightEnterSubLevelResponse {
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
pub struct MotorFightGameOverNotify {
    pub Result: Option<MotorFightResult>,
    pub Win: bool,
}

impl MotorFightGameOverNotify {
    pub const MSG_ID: u16 = 15356;
}

impl PbMessage for MotorFightGameOverNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.Win {
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
                let bytes = r.read_len_delimited()?;
                self.Result
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Win = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightItemUpdateNotify {
    pub UnlockedItem: Vec<i32>,
}

impl MotorFightItemUpdateNotify {
    pub const MSG_ID: u16 = 22316;
}

impl PbMessage for MotorFightItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedItem {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedItem, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightKillRequest {
    pub WaveGroupIndex: i32,
    pub IsKilled: bool,
    pub EntityId: i32,
    pub IsBoss: bool,
}

impl MotorFightKillRequest {
    pub const MSG_ID: u16 = 24452;
}

impl PbMessage for MotorFightKillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        if self.IsKilled {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
        }
        if self.IsBoss {
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
            1 => self.WaveGroupIndex = r.read_varint_i32()?,
            2 => self.IsKilled = r.read_bool()?,
            11 => self.EntityId = r.read_varint_i32()?,
            14 => self.IsBoss = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightKillResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightKillResponse {
    pub const MSG_ID: u16 = 16884;
}

impl PbMessage for MotorFightKillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct MotorFightLastSaveData {
    pub LevelId: i32,
    pub CostTime: i64,
    pub Score: i32,
}

impl PbMessage for MotorFightLastSaveData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTime);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.CostTime = r.read_varint_i64()?,
            3 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightLastSaveDataRequest {}

impl MotorFightLastSaveDataRequest {
    pub const MSG_ID: u16 = 16858;
}

impl PbMessage for MotorFightLastSaveDataRequest {
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
pub struct MotorFightLastSaveDataResponse {
    pub LastSaveData: Option<MotorFightLastSaveData>,
    pub Exists: bool,
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightLastSaveDataResponse {
    pub const MSG_ID: u16 = 19578;
}

impl PbMessage for MotorFightLastSaveDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LastSaveData {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.Exists {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrCode {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.LastSaveData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.Exists = r.read_bool()?,
            9 => {
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
pub struct MotorFightLevelUpdateNotify {
    pub MotorFightLevelPb: Vec<MotorFightLevelPb>,
}

impl MotorFightLevelUpdateNotify {
    pub const MSG_ID: u16 = 26610;
}

impl PbMessage for MotorFightLevelUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorFightLevelPb {
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
                self.MotorFightLevelPb
                    .push(<MotorFightLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightMonsterDropNotify {
    pub MonsterId: i32,
    pub SubLevelIndex: i32,
    pub WaveGroupIndex: i32,
    pub BuffGateId: i32,
}

impl MotorFightMonsterDropNotify {
    pub const MSG_ID: u16 = 27596;
}

impl PbMessage for MotorFightMonsterDropNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        if self.BuffGateId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffGateId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.MonsterId = r.read_varint_i32()?,
            11 => self.SubLevelIndex = r.read_varint_i32()?,
            14 => self.WaveGroupIndex = r.read_varint_i32()?,
            15 => self.BuffGateId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightMonsterPb {
    pub MonsterId: i32,
    pub WaveId: i32,
}

impl PbMessage for MotorFightMonsterPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MonsterId = r.read_varint_i32()?,
            2 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightOneKeyRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl MotorFightOneKeyRewardRequest {
    pub const MSG_ID: u16 = 27302;
}

impl PbMessage for MotorFightOneKeyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightOneKeyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorFightOneKeyRewardResponse {
    pub const MSG_ID: u16 = 18713;
}

impl PbMessage for MotorFightOneKeyRewardResponse {
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
pub struct MotorFightPlayerDeadRequest {}

impl MotorFightPlayerDeadRequest {
    pub const MSG_ID: u16 = 22425;
}

impl PbMessage for MotorFightPlayerDeadRequest {
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
pub struct MotorFightPlayerDeadResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightPlayerDeadResponse {
    pub const MSG_ID: u16 = 23704;
}

impl PbMessage for MotorFightPlayerDeadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct MotorFightPlayerRankingInfo {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub ResultCommon: Option<MotorFightResultBase>,
}

impl PbMessage for MotorFightPlayerRankingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if let Some(v) = &self.ResultCommon {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ResultCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRankListRequest {
    pub ActivityId: i32,
}

impl MotorFightRankListRequest {
    pub const MSG_ID: u16 = 22743;
}

impl PbMessage for MotorFightRankListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRankListResponse {
    pub FriendData: Vec<MotorFightPlayerRankingInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub SelfData: Option<MotorFightPlayerRankingInfo>,
}

impl MotorFightRankListResponse {
    pub const MSG_ID: u16 = 22100;
}

impl PbMessage for MotorFightRankListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FriendData {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FriendData
                    .push(<MotorFightPlayerRankingInfo as PbMessage>::decode(bytes)?);
            }
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
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
pub struct MotorFightRankListSelfDataRequest {
    pub ActivityId: i32,
}

impl MotorFightRankListSelfDataRequest {
    pub const MSG_ID: u16 = 28172;
}

impl PbMessage for MotorFightRankListSelfDataRequest {
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
pub struct MotorFightRankListSelfDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SelfData: Option<MotorFightPlayerRankingInfo>,
}

impl MotorFightRankListSelfDataResponse {
    pub const MSG_ID: u16 = 18113;
}

impl PbMessage for MotorFightRankListSelfDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.SelfData {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
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
pub struct MotorFightRefreshBossPush {
    pub WaveGroupIndex: i32,
    pub SubLevelIndex: i32,
    pub RefreshId: i32,
}

impl MotorFightRefreshBossPush {
    pub const MSG_ID: u16 = 23084;
}

impl PbMessage for MotorFightRefreshBossPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.RefreshId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.WaveGroupIndex = r.read_varint_i32()?,
            5 => self.SubLevelIndex = r.read_varint_i32()?,
            8 => self.RefreshId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRefreshCollectionRequest {
    pub SubLevelIndex: i32,
    pub BossId: i32,
    pub WaveGroupIndex: i32,
}

impl MotorFightRefreshCollectionRequest {
    pub const MSG_ID: u16 = 22653;
}

impl PbMessage for MotorFightRefreshCollectionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.BossId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossId);
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SubLevelIndex = r.read_varint_i32()?,
            12 => self.BossId = r.read_varint_i32()?,
            14 => self.WaveGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRefreshCollectionResponse {
    pub Drops: Vec<MotorFightCollectionDrop>,
    pub RefreshCollectionTimes: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightRefreshCollectionResponse {
    pub const MSG_ID: u16 = 18221;
}

impl PbMessage for MotorFightRefreshCollectionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Drops {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RefreshCollectionTimes != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCollectionTimes);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Drops
                    .push(<MotorFightCollectionDrop as PbMessage>::decode(bytes)?);
            }
            8 => self.RefreshCollectionTimes = r.read_varint_i32()?,
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
pub struct MotorFightRefreshCollectionTimesChangeNotify {
    pub RefreshCollectionTimes: i32,
}

impl MotorFightRefreshCollectionTimesChangeNotify {
    pub const MSG_ID: u16 = 16712;
}

impl PbMessage for MotorFightRefreshCollectionTimesChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RefreshCollectionTimes != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCollectionTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RefreshCollectionTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRefreshNextWaveGroupRequest {
    pub WaveGroupIndex: i32,
}

impl MotorFightRefreshNextWaveGroupRequest {
    pub const MSG_ID: u16 = 16568;
}

impl PbMessage for MotorFightRefreshNextWaveGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.WaveGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRefreshNextWaveGroupResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightRefreshNextWaveGroupResponse {
    pub const MSG_ID: u16 = 27635;
}

impl PbMessage for MotorFightRefreshNextWaveGroupResponse {
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
pub struct MotorFightRefreshSubLevelNotify {
    pub SubLevelIndex: i32,
    pub WaveGroups: Vec<MotorFightWaveGroupPb>,
}

impl MotorFightRefreshSubLevelNotify {
    pub const MSG_ID: u16 = 22478;
}

impl PbMessage for MotorFightRefreshSubLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        for item in &self.WaveGroups {
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
            3 => self.SubLevelIndex = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.WaveGroups
                    .push(<MotorFightWaveGroupPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightResult {
    pub ResultCommon: Option<MotorFightResultBase>,
    pub IsBestScore: bool,
    pub RewardItemMap: BTreeMap<i32, i32>,
    pub IsFirstClear: bool,
    pub CostTime: i64,
}

impl PbMessage for MotorFightResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ResultCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.IsBestScore {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.RewardItemMap.is_empty() {
            crate::wire::write_map(out, 9, &self.RewardItemMap);
        }
        if self.IsFirstClear {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTime);
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
                self.ResultCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.IsBestScore = r.read_bool()?,
            9 => crate::wire::decode_map_entry(r, &mut self.RewardItemMap)?,
            10 => self.IsFirstClear = r.read_bool()?,
            11 => self.CostTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightResultBase {
    pub LevelId: i32,
    pub PassedWaveNum: i32,
    pub RoleId: i32,
    pub Items: Vec<MotorFightCollectItemInfo>,
    pub TotalScore: i32,
    pub ScoreDetail: Option<MotorFightScoreDetailPb>,
    pub KillMonsterCount: i32,
    pub SelectedBuffGateCount: i32,
}

impl PbMessage for MotorFightResultBase {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.PassedWaveNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassedWaveNum);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
        if let Some(v) = &self.ScoreDetail {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.KillMonsterCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillMonsterCount);
        }
        if self.SelectedBuffGateCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectedBuffGateCount);
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
            3 => self.PassedWaveNum = r.read_varint_i32()?,
            4 => self.RoleId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<MotorFightCollectItemInfo as PbMessage>::decode(bytes)?);
            }
            6 => self.TotalScore = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ScoreDetail
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.KillMonsterCount = r.read_varint_i32()?,
            9 => self.SelectedBuffGateCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRoleUpdateNotify {
    pub UnlockedRole: Vec<i32>,
}

impl MotorFightRoleUpdateNotify {
    pub const MSG_ID: u16 = 19840;
}

impl PbMessage for MotorFightRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedRole {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedRole, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRoundBuffPb {
    pub BuffId: i32,
    pub SubLevelIndex: i32,
    pub WaveGroupIndex: i32,
}

impl PbMessage for MotorFightRoundBuffPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
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
            2 => self.SubLevelIndex = r.read_varint_i32()?,
            3 => self.WaveGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRoundCollectionPb {
    pub CollectionId: i32,
    pub SubLevelIndex: i32,
    pub WaveGroupIndex: i32,
}

impl PbMessage for MotorFightRoundCollectionPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CollectionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CollectionId);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CollectionId = r.read_varint_i32()?,
            2 => self.SubLevelIndex = r.read_varint_i32()?,
            3 => self.WaveGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightRoundInfoNotify {
    pub Buffs: Vec<MotorFightRoundBuffPb>,
    pub BossIds: Vec<i32>,
    pub Collections: Vec<MotorFightRoundCollectionPb>,
    pub PlayerHp: i32,
    pub Score: i32,
    pub RefreshCollectionTimes: i32,
    pub GameTime: i32,
    pub PassSubLevelIndex: i32,
    pub EnemyBuffs: Vec<MotorFightRoundBuffPb>,
}

impl MotorFightRoundInfoNotify {
    pub const MSG_ID: u16 = 26380;
}

impl PbMessage for MotorFightRoundInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Buffs {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.BossIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Collections {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.PlayerHp != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerHp);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.RefreshCollectionTimes != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCollectionTimes);
        }
        if self.GameTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GameTime);
        }
        if self.PassSubLevelIndex != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassSubLevelIndex);
        }
        for item in &self.EnemyBuffs {
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
                let bytes = r.read_len_delimited()?;
                self.Buffs
                    .push(<MotorFightRoundBuffPb as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BossIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Collections
                    .push(<MotorFightRoundCollectionPb as PbMessage>::decode(bytes)?);
            }
            6 => self.PlayerHp = r.read_varint_i32()?,
            9 => self.Score = r.read_varint_i32()?,
            10 => self.RefreshCollectionTimes = r.read_varint_i32()?,
            11 => self.GameTime = r.read_varint_i32()?,
            12 => self.PassSubLevelIndex = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.EnemyBuffs
                    .push(<MotorFightRoundBuffPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightScoreDetailPb {
    pub CollectionScore: i32,
    pub WaveScore: i32,
    pub MonsterScore: i32,
    pub BuffGateScore: i32,
}

impl PbMessage for MotorFightScoreDetailPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CollectionScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CollectionScore);
        }
        if self.WaveScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveScore);
        }
        if self.MonsterScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterScore);
        }
        if self.BuffGateScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffGateScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CollectionScore = r.read_varint_i32()?,
            2 => self.WaveScore = r.read_varint_i32()?,
            3 => self.MonsterScore = r.read_varint_i32()?,
            4 => self.BuffGateScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightScoreNotify {
    pub Score: i32,
}

impl MotorFightScoreNotify {
    pub const MSG_ID: u16 = 18613;
}

impl PbMessage for MotorFightScoreNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Score != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightSelectBuffGateRequest {
    pub WaveGroupIndex: i32,
    pub BuffGateId: i32,
    pub SubLevelIndex: i32,
    pub BuffIds: Vec<i32>,
}

impl MotorFightSelectBuffGateRequest {
    pub const MSG_ID: u16 = 16560;
}

impl PbMessage for MotorFightSelectBuffGateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        if self.BuffGateId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffGateId);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        for item in &self.BuffIds {
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
            4 => self.WaveGroupIndex = r.read_varint_i32()?,
            6 => self.BuffGateId = r.read_varint_i32()?,
            7 => self.SubLevelIndex = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightSelectBuffGateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightSelectBuffGateResponse {
    pub const MSG_ID: u16 = 22911;
}

impl PbMessage for MotorFightSelectBuffGateResponse {
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
pub struct MotorFightSelectCollectionRequest {
    pub PlayerHp: i32,
    pub Pos: Option<MotorFightCollectionPos>,
    pub WaveGroupIndex: i32,
    pub SubLevelIndex: i32,
    pub BossId: i32,
}

impl MotorFightSelectCollectionRequest {
    pub const MSG_ID: u16 = 19245;
}

impl PbMessage for MotorFightSelectCollectionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerHp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerHp);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.WaveGroupIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupIndex);
        }
        if self.SubLevelIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubLevelIndex);
        }
        if self.BossId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.PlayerHp = r.read_varint_i32()?,
            3 => {
                self.Pos = Some(
                    <MotorFightCollectionPos as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            9 => self.WaveGroupIndex = r.read_varint_i32()?,
            10 => self.SubLevelIndex = r.read_varint_i32()?,
            13 => self.BossId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightSelectCollectionResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightSelectCollectionResponse {
    pub const MSG_ID: u16 = 18857;
}

impl PbMessage for MotorFightSelectCollectionResponse {
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
pub struct MotorFightSettleRequest {}

impl MotorFightSettleRequest {
    pub const MSG_ID: u16 = 15544;
}

impl PbMessage for MotorFightSettleRequest {
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
pub struct MotorFightSettleResponse {
    pub Win: bool,
    pub Result: Option<MotorFightResult>,
    pub ErrCode: Option<ErrorCode>,
}

impl MotorFightSettleResponse {
    pub const MSG_ID: u16 = 19184;
}

impl PbMessage for MotorFightSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Win {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Result {
            crate::wire::write_submessage(out, 10, v);
        }
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
            8 => self.Win = r.read_bool()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct MotorFightTalentTreeUpdateNotify {
    pub Talent: Vec<MotorFightTalentPb>,
}

impl MotorFightTalentTreeUpdateNotify {
    pub const MSG_ID: u16 = 21599;
}

impl PbMessage for MotorFightTalentTreeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Talent {
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
                self.Talent
                    .push(<MotorFightTalentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightTalentUpRequest {
    pub TalentId: i32,
}

impl MotorFightTalentUpRequest {
    pub const MSG_ID: u16 = 28481;
}

impl PbMessage for MotorFightTalentUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.TalentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightTalentUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorFightTalentUpResponse {
    pub const MSG_ID: u16 = 24183;
}

impl PbMessage for MotorFightTalentUpResponse {
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
pub struct MotorFightTaskUpdateNotify {
    pub Task: Vec<ConditionTask>,
}

impl MotorFightTaskUpdateNotify {
    pub const MSG_ID: u16 = 28108;
}

impl PbMessage for MotorFightTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
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
                self.Task.push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorFightWaveGroupPb {
    pub WaveGroupId: i32,
    pub Monsters: Vec<MotorFightMonsterPb>,
    pub Bosses: Vec<MotorFightBossPb>,
    pub BuffGates: Vec<MotorFightBuffGatePb>,
}

impl PbMessage for MotorFightWaveGroupPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveGroupId);
        }
        for item in &self.Monsters {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Bosses {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.BuffGates {
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
            1 => self.WaveGroupId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Monsters
                    .push(<MotorFightMonsterPb as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Bosses
                    .push(<MotorFightBossPb as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BuffGates
                    .push(<MotorFightBuffGatePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorLinkTaskRequest {
    pub MotorIp: i32,
}

impl MotorLinkTaskRequest {
    pub const MSG_ID: u16 = 23517;
}

impl PbMessage for MotorLinkTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorIp != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorIp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.MotorIp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorLinkTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Ids: Vec<i32>,
}

impl MotorLinkTaskResponse {
    pub const MSG_ID: u16 = 22394;
}

impl PbMessage for MotorLinkTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Ids {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOnlineParkPlayerRuleInfo {
    pub PlayerId: i32,
    pub RuleIds: Vec<i32>,
}

impl PbMessage for MotorOnlineParkPlayerRuleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.RuleIds {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RuleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOnlineProgressReportPush {
    pub Progress: i32,
    pub InstId: i32,
}

impl MotorOnlineProgressReportPush {
    pub const MSG_ID: u16 = 18346;
}

impl PbMessage for MotorOnlineProgressReportPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Progress != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            1 => self.Progress = r.read_varint_i32()?,
            11 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOnlineRankPlayerInfo {
    pub PlayerId: i32,
    pub Rank: i32,
}

impl PbMessage for MotorOnlineRankPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Rank != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rank);
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
            2 => self.Rank = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOnlineRankRuleIdsNotify {
    pub InstId: i32,
    pub MotorOnlinePlayerInfoList: Vec<MotorOnlineParkPlayerRuleInfo>,
}

impl MotorOnlineRankRuleIdsNotify {
    pub const MSG_ID: u16 = 29052;
}

impl PbMessage for MotorOnlineRankRuleIdsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.MotorOnlinePlayerInfoList {
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
            5 => self.InstId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOnlinePlayerInfoList
                    .push(<MotorOnlineParkPlayerRuleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOnlineRankSnapshotNotify {
    pub SnapShotVersion: i32,
    pub MotorOnlinePlayerInfoList: Vec<MotorOnlineRankPlayerInfo>,
    pub InstId: i32,
}

impl MotorOnlineRankSnapshotNotify {
    pub const MSG_ID: u16 = 21549;
}

impl PbMessage for MotorOnlineRankSnapshotNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SnapShotVersion != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SnapShotVersion);
        }
        for item in &self.MotorOnlinePlayerInfoList {
            crate::wire::write_submessage(out, 8, item);
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
            7 => self.SnapShotVersion = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOnlinePlayerInfoList
                    .push(<MotorOnlineRankPlayerInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookAddNotify {
    pub MotorDecoration: Vec<i32>,
    pub MotorSticker: Vec<i32>,
    pub MotorFrame: Vec<i32>,
    pub MotorSkin: Vec<i32>,
}

impl MotorOutlookAddNotify {
    pub const MSG_ID: u16 = 24099;
}

impl PbMessage for MotorOutlookAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorDecoration {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MotorSticker {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MotorFrame {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MotorSkin {
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MotorDecoration,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MotorSticker, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MotorFrame, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MotorSkin, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookEquippedPb {
    pub SkinEquipped: i32,
    pub StickerEquipped: Vec<i32>,
    pub DecorationsEquipped: Vec<i32>,
    pub FrameEquipped: i32,
}

impl PbMessage for MotorOutlookEquippedPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinEquipped != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinEquipped);
        }
        for item in &self.StickerEquipped {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DecorationsEquipped {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FrameEquipped != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FrameEquipped);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SkinEquipped = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.StickerEquipped,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DecorationsEquipped,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => self.FrameEquipped = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookFullChangeNotify {
    pub MotorOutlook: Option<MotorOutlookPb>,
}

impl MotorOutlookFullChangeNotify {
    pub const MSG_ID: u16 = 16805;
}

impl PbMessage for MotorOutlookFullChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorOutlook {
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
                self.MotorOutlook
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookInfoRequest {}

impl MotorOutlookInfoRequest {
    pub const MSG_ID: u16 = 16618;
}

impl PbMessage for MotorOutlookInfoRequest {
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
pub struct MotorOutlookInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorOutlook: Option<MotorOutlookPb>,
}

impl MotorOutlookInfoResponse {
    pub const MSG_ID: u16 = 18366;
}

impl PbMessage for MotorOutlookInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MotorOutlook {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlook
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookOwnedChangeNotify {
    pub MotorOutlookOwned: Option<MotorOutlookOwnedPb>,
}

impl MotorOutlookOwnedChangeNotify {
    pub const MSG_ID: u16 = 22922;
}

impl PbMessage for MotorOutlookOwnedChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorOutlookOwned {
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
                self.MotorOutlookOwned
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookOwnedPb {
    pub StickerOwned: Vec<i32>,
    pub SkinOwned: Vec<i32>,
    pub DecorationsOwned: Vec<i32>,
    pub FrameOwned: Vec<i32>,
}

impl PbMessage for MotorOutlookOwnedPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StickerOwned {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkinOwned {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DecorationsOwned {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FrameOwned {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StickerOwned, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinOwned, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DecorationsOwned,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FrameOwned, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookPb {
    pub MotorOutlookOwned: Option<MotorOutlookOwnedPb>,
    pub MotorOutlookEquipped: Option<MotorOutlookEquippedPb>,
    pub MotorOutlookPreset: Option<MotorOutlookPlayerPresetPb>,
    pub LatestMotorSkinSuit: Vec<MotorOutlookEquippedPb>,
    pub SceneInUse: i32,
}

impl PbMessage for MotorOutlookPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorOutlookOwned {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.MotorOutlookEquipped {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.MotorOutlookPreset {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.LatestMotorSkinSuit {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.SceneInUse != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneInUse);
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
                self.MotorOutlookOwned
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlookEquipped
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlookPreset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.LatestMotorSkinSuit
                    .push(<MotorOutlookEquippedPb as PbMessage>::decode(bytes)?);
            }
            5 => self.SceneInUse = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourCtx {
    pub MotorParkourId: i32,
}

impl PbMessage for MotorParkourCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorParkourId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorParkourId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MotorParkourId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourLevelRewardRequest {
    pub MotorParkourId: i32,
    pub RewardIdxs: Vec<i32>,
}

impl MotorParkourLevelRewardRequest {
    pub const MSG_ID: u16 = 18192;
}

impl PbMessage for MotorParkourLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MotorParkourId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorParkourId);
        }
        for item in &self.RewardIdxs {
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
            6 => self.MotorParkourId = r.read_varint_i32()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIdxs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourLevelRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl MotorParkourLevelRewardResponse {
    pub const MSG_ID: u16 = 21142;
}

impl PbMessage for MotorParkourLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
pub struct MotorParkourLevelRewardUpdateNotify {
    pub MotorParkourLevelInfos: Vec<MotorParkourLevelInfo>,
}

impl MotorParkourLevelRewardUpdateNotify {
    pub const MSG_ID: u16 = 16941;
}

impl PbMessage for MotorParkourLevelRewardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorParkourLevelInfos {
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
                self.MotorParkourLevelInfos
                    .push(<MotorParkourLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourResultNotify {
    pub TimeRecord: i32,
    pub MotorParkourId: i32,
    pub IsSuccess: bool,
}

impl MotorParkourResultNotify {
    pub const MSG_ID: u16 = 25444;
}

impl PbMessage for MotorParkourResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeRecord != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeRecord);
        }
        if self.MotorParkourId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MotorParkourId);
        }
        if self.IsSuccess {
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
            4 => self.TimeRecord = r.read_varint_i32()?,
            9 => self.MotorParkourId = r.read_varint_i32()?,
            11 => self.IsSuccess = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorParkourStartNotify {
    pub VariableKeyNames: BTreeMap<Vec<u8>, Vec<u8>>,
    pub FlowTime: i64,
}

impl MotorParkourStartNotify {
    pub const MSG_ID: u16 = 27385;
}

impl PbMessage for MotorParkourStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.VariableKeyNames.is_empty() {
            crate::wire::write_map(out, 3, &self.VariableKeyNames);
        }
        if self.FlowTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_map_entry(r, &mut self.VariableKeyNames)?,
            15 => self.FlowTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorPositionStateNotify {
    pub EntityId: i64,
    pub PositionState: i32,
}

impl MotorPositionStateNotify {
    pub const MSG_ID: u16 = 20614;
}

impl PbMessage for MotorPositionStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PositionState != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PositionState);
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
            5 => self.PositionState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorPositionStatePush {
    pub PositionState: i32,
    pub EntityId: i64,
}

impl MotorPositionStatePush {
    pub const MSG_ID: u16 = 28875;
}

impl PbMessage for MotorPositionStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PositionState != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PositionState);
        }
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
            7 => self.PositionState = r.read_varint_i32()?,
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorRaceChallengeData {
    pub RankS: i32,
    pub RankA: i32,
    pub RankB: i32,
    pub Score: i32,
}

impl PbMessage for MotorRaceChallengeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RankS != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankS);
        }
        if self.RankA != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankA);
        }
        if self.RankB != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankB);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.RankS = r.read_varint_i32()?,
            2 => self.RankA = r.read_varint_i32()?,
            3 => self.RankB = r.read_varint_i32()?,
            4 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorSliderRequest {
    pub InteractType: Option<MotorSliderInteractType>,
    pub EntityId: i64,
}

impl MotorSliderRequest {
    pub const MSG_ID: u16 = 22317;
}

impl PbMessage for MotorSliderRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InteractType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => {
                self.InteractType = Some(
                    <MotorSliderInteractType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorSliderResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorSliderResponse {
    pub const MSG_ID: u16 = 16957;
}

impl PbMessage for MotorSliderResponse {
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
pub struct MotorTaskInfoNotify {
    pub TaskTrees: Vec<MotorTaskTreePb>,
}

impl MotorTaskInfoNotify {
    pub const MSG_ID: u16 = 22808;
}

impl PbMessage for MotorTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskTrees {
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
                self.TaskTrees
                    .push(<MotorTaskTreePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechTreeUnlockNotify {
    pub TreeId: i32,
}

impl MotorTechTreeUnlockNotify {
    pub const MSG_ID: u16 = 20699;
}

impl PbMessage for MotorTechTreeUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.TreeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorTechUnlockNotify {
    pub TechId: Vec<i32>,
}

impl MotorTechUnlockNotify {
    pub const MSG_ID: u16 = 17862;
}

impl PbMessage for MotorTechUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TechId {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TechId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorWheelInfo {
    pub Accel: f32,
    pub Speed: f32,
    pub Location: Option<Vector>,
}

impl PbMessage for MotorWheelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Accel != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Accel).to_bits().to_le_bytes());
        }
        if self.Speed != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Speed).to_bits().to_le_bytes());
        }
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
            1 => self.Accel = r.read_f32()?,
            2 => self.Speed = r.read_f32()?,
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
pub struct MotorcycleTaskUpdateNotify {
    pub Task: Option<ConditionTask>,
}

impl MotorcycleTaskUpdateNotify {
    pub const MSG_ID: u16 = 20545;
}

impl PbMessage for MotorcycleTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
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
pub struct MoveOp {
    pub Paths: Vec<i32>,
    pub GridVisions: Vec<GridVision>,
}

impl PbMessage for MoveOp {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Paths {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.GridVisions {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Paths, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GridVisions
                    .push(<GridVision as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovePlacementNotify {
    pub TargetBoardEntityId: i64,
    pub PlacementEntityId: i64,
    pub PosRot: Option<PositionAndRotate>,
    pub PosOnBoard: Option<BoardGridPositionInfo>,
}

impl MovePlacementNotify {
    pub const MSG_ID: u16 = 21148;
}

impl PbMessage for MovePlacementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetBoardEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetBoardEntityId);
        }
        if self.PlacementEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlacementEntityId);
        }
        if let Some(v) = &self.PosRot {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.PosOnBoard {
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
            2 => self.TargetBoardEntityId = r.read_varint_i64()?,
            8 => self.PlacementEntityId = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PosRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PosOnBoard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovePlacementRequest {
    pub PosRot: Option<PositionAndRotate>,
    pub TargetBoardEntityId: i64,
    pub PlacementEntityId: i64,
    pub PosOnBoard: Option<BoardGridPositionInfo>,
}

impl MovePlacementRequest {
    pub const MSG_ID: u16 = 26801;
}

impl PbMessage for MovePlacementRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PosRot {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TargetBoardEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetBoardEntityId);
        }
        if self.PlacementEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlacementEntityId);
        }
        if let Some(v) = &self.PosOnBoard {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.PosRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.TargetBoardEntityId = r.read_varint_i64()?,
            4 => self.PlacementEntityId = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PosOnBoard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovePlacementResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl MovePlacementResponse {
    pub const MSG_ID: u16 = 29380;
}

impl PbMessage for MovePlacementResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct MoveSampleSceneItem {
    pub LinearVelocity: Option<Vector>,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub TimeStamp: f32,
}

impl PbMessage for MoveSampleSceneItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LinearVelocity {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
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
                self.LinearVelocity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.TimeStamp = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveSceneItemNotify {
    pub MoveInfos: Vec<MoveSampleSceneItem>,
    pub CombatCommon: Option<CombatCommon>,
}

impl MoveSceneItemNotify {
    pub const MSG_ID: u16 = 26459;
}

impl PbMessage for MoveSceneItemNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MoveInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.CombatCommon {
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
                self.MoveInfos
                    .push(<MoveSampleSceneItem as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveSceneItemPush {
    pub CombatCommon: Option<CombatCommon>,
    pub MoveInfos: Vec<MoveSampleSceneItem>,
}

impl MoveSceneItemPush {
    pub const MSG_ID: u16 = 20741;
}

impl PbMessage for MoveSceneItemPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 5, v);
        }
        for item in &self.MoveInfos {
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
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.MoveInfos
                    .push(<MoveSampleSceneItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveSceneItemResetPositionRequest {
    pub ResetEntityId: i64,
    pub SceneOwnerId: i32,
    pub EntityId: i64,
}

impl MoveSceneItemResetPositionRequest {
    pub const MSG_ID: u16 = 27648;
}

impl PbMessage for MoveSceneItemResetPositionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResetEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ResetEntityId);
        }
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
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
            1 => self.ResetEntityId = r.read_varint_i64()?,
            3 => self.SceneOwnerId = r.read_varint_i32()?,
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveSceneItemResetPositionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MoveSceneItemResetPositionResponse {
    pub const MSG_ID: u16 = 20105;
}

impl PbMessage for MoveSceneItemResetPositionResponse {
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
pub struct MoveSplineStatusNotify {
    pub SceneItemRuntimeData: Option<SceneItemSplineRuntimeData>,
    pub MoveStatus: Option<MoveStatus>,
    pub EntityId: i64,
    pub SplineEntityId: i32,
    pub MoveSplineConfig: Option<MoveSplineConfig>,
}

impl MoveSplineStatusNotify {
    pub const MSG_ID: u16 = 17431;
}

impl PbMessage for MoveSplineStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SceneItemRuntimeData {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.MoveStatus {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
        }
        if let Some(v) = &self.MoveSplineConfig {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SceneItemRuntimeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.MoveStatus = Some(<MoveStatus as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.EntityId = r.read_varint_i64()?,
            11 => self.SplineEntityId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.MoveSplineConfig
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerActivityInfo {
    pub MowTowerLevelInfos: Vec<MowTowerLevelsInfo>,
}

impl PbMessage for MowTowerActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MowTowerLevelInfos {
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
                self.MowTowerLevelInfos
                    .push(<MowTowerLevelsInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerCtx {
    pub LevelsId: i32,
    pub SelectBuff: Vec<i32>,
    pub SecondRoles: Vec<i32>,
}

impl PbMessage for MowTowerCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        for item in &self.SelectBuff {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SecondRoles {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SecondRoles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerFirstScoreNotify {
    pub LevelsId: i32,
    pub FirstKilMonsterScore: i32,
    pub FirstTimerScore: i32,
}

impl MowTowerFirstScoreNotify {
    pub const MSG_ID: u16 = 28712;
}

impl PbMessage for MowTowerFirstScoreNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        if self.FirstKilMonsterScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstKilMonsterScore);
        }
        if self.FirstTimerScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstTimerScore);
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
            4 => self.FirstKilMonsterScore = r.read_varint_i32()?,
            8 => self.FirstTimerScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerLevelsInfoUpdateNotify {
    pub MowTowerLevelsInfos: Vec<MowTowerLevelsInfo>,
}

impl MowTowerLevelsInfoUpdateNotify {
    pub const MSG_ID: u16 = 16031;
}

impl PbMessage for MowTowerLevelsInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MowTowerLevelsInfos {
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
                self.MowTowerLevelsInfos
                    .push(<MowTowerLevelsInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerLevelsRewardRequest {
    pub RewardId: i32,
}

impl MowTowerLevelsRewardRequest {
    pub const MSG_ID: u16 = 16582;
}

impl PbMessage for MowTowerLevelsRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.RewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerLevelsRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MowTowerLevelsRewardResponse {
    pub const MSG_ID: u16 = 17233;
}

impl PbMessage for MowTowerLevelsRewardResponse {
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
pub struct MowTowerResultNotify {
    pub LevelsId: i32,
    pub FirstTimerScore: i32,
    pub HighScore: i32,
    pub SecondTimerScore: i32,
    pub SecondKilMonsterScore: i32,
    pub FirstKilMonsterScore: i32,
}

impl MowTowerResultNotify {
    pub const MSG_ID: u16 = 17664;
}

impl PbMessage for MowTowerResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        if self.FirstTimerScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstTimerScore);
        }
        if self.HighScore != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HighScore);
        }
        if self.SecondTimerScore != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondTimerScore);
        }
        if self.SecondKilMonsterScore != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondKilMonsterScore);
        }
        if self.FirstKilMonsterScore != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstKilMonsterScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.LevelsId = r.read_varint_i32()?,
            3 => self.FirstTimerScore = r.read_varint_i32()?,
            9 => self.HighScore = r.read_varint_i32()?,
            11 => self.SecondTimerScore = r.read_varint_i32()?,
            14 => self.SecondKilMonsterScore = r.read_varint_i32()?,
            15 => self.FirstKilMonsterScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MowTowerSettlementRequest {}

impl MowTowerSettlementRequest {
    pub const MSG_ID: u16 = 26061;
}

impl PbMessage for MowTowerSettlementRequest {
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
pub struct MowTowerSettlementResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MowTowerSettlementResponse {
    pub const MSG_ID: u16 = 27490;
}

impl PbMessage for MowTowerSettlementResponse {
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
pub struct MulCalabashLevelRewardRequest {
    pub Level: Vec<i32>,
}

impl MulCalabashLevelRewardRequest {
    pub const MSG_ID: u16 = 17463;
}

impl PbMessage for MulCalabashLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Level {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Level, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulCalabashLevelRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Level: Vec<i32>,
}

impl MulCalabashLevelRewardResponse {
    pub const MSG_ID: u16 = 17527;
}

impl PbMessage for MulCalabashLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Level {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Level, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulCorniceMeetingRewardRequest {
    pub ActivityId: i32,
    pub CorniceMeetingRewardScoreLists: BTreeMap<i32, CorniceMeetingRewardScoreList>,
}

impl MulCorniceMeetingRewardRequest {
    pub const MSG_ID: u16 = 20556;
}

impl PbMessage for MulCorniceMeetingRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if !self.CorniceMeetingRewardScoreLists.is_empty() {
            crate::wire::write_map(out, 13, &self.CorniceMeetingRewardScoreLists);
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
            13 => crate::wire::decode_map_entry(r, &mut self.CorniceMeetingRewardScoreLists)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulCorniceMeetingRewardResponse {
    pub CorniceMeetingRewardScoreLists: BTreeMap<i32, CorniceMeetingRewardScoreList>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulCorniceMeetingRewardResponse {
    pub const MSG_ID: u16 = 22551;
}

impl PbMessage for MulCorniceMeetingRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CorniceMeetingRewardScoreLists.is_empty() {
            crate::wire::write_map(out, 11, &self.CorniceMeetingRewardScoreLists);
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
            11 => crate::wire::decode_map_entry(r, &mut self.CorniceMeetingRewardScoreLists)?,
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
pub struct MulExploreScoreRewardRequest {
    pub Rewards: BTreeMap<i32, i32>,
}

impl MulExploreScoreRewardRequest {
    pub const MSG_ID: u16 = 20889;
}

impl PbMessage for MulExploreScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Rewards.is_empty() {
            crate::wire::write_map(out, 13, &self.Rewards);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.Rewards)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulExploreScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MulExploreScoreRewardResponse {
    pub const MSG_ID: u16 = 27486;
}

impl PbMessage for MulExploreScoreRewardResponse {
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
pub struct MulFishingIllustratedRewardRequest {
    pub RewardId: Vec<i32>,
}

impl MulFishingIllustratedRewardRequest {
    pub const MSG_ID: u16 = 25002;
}

impl PbMessage for MulFishingIllustratedRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardId {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulFishingIllustratedRewardResponse {
    pub RewardId: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulFishingIllustratedRewardResponse {
    pub const MSG_ID: u16 = 28177;
}

impl PbMessage for MulFishingIllustratedRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardId, |sr| {
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
pub struct MulGetAdventureRewardRequest {
    pub Id: Vec<i32>,
}

impl MulGetAdventureRewardRequest {
    pub const MSG_ID: u16 = 21424;
}

impl PbMessage for MulGetAdventureRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Id {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Id, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulGetAdventureRewardResponse {
    pub Code: Option<ErrorCode>,
    pub Id: Vec<i32>,
}

impl MulGetAdventureRewardResponse {
    pub const MSG_ID: u16 = 24375;
}

impl PbMessage for MulGetAdventureRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Id {
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
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Id, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulMapTravelSoarRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl MulMapTravelSoarRewardRequest {
    pub const MSG_ID: u16 = 19711;
}

impl PbMessage for MulMapTravelSoarRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
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
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulMapTravelSoarRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TaskIds: Vec<i32>,
}

impl MulMapTravelSoarRewardResponse {
    pub const MSG_ID: u16 = 16779;
}

impl PbMessage for MulMapTravelSoarRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskIds {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulMapTravelTaskRewardRequest {
    pub TaskId: Vec<i32>,
}

impl MulMapTravelTaskRewardRequest {
    pub const MSG_ID: u16 = 22390;
}

impl PbMessage for MulMapTravelTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskId {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulMapTravelTaskRewardResponse {
    pub TaskId: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulMapTravelTaskRewardResponse {
    pub const MSG_ID: u16 = 16436;
}

impl PbMessage for MulMapTravelTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskId {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct MulPhantomBattleMasterLevelRewardRequest {
    pub PhantomBattleMasterLevelId: Vec<i32>,
    pub ActivityId: i32,
}

impl MulPhantomBattleMasterLevelRewardRequest {
    pub const MSG_ID: u16 = 18455;
}

impl PbMessage for MulPhantomBattleMasterLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleMasterLevelId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomBattleMasterLevelId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulPhantomBattleMasterLevelRewardResponse {
    pub PhantomBattleMasterLevelId: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulPhantomBattleMasterLevelRewardResponse {
    pub const MSG_ID: u16 = 15639;
}

impl PbMessage for MulPhantomBattleMasterLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleMasterLevelId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomBattleMasterLevelId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
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
pub struct MulRogueWeeklyRewardRequest {
    pub ActivityId: i32,
    pub ConfigId: Vec<i32>,
}

impl MulRogueWeeklyRewardRequest {
    pub const MSG_ID: u16 = 16599;
}

impl PbMessage for MulRogueWeeklyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ConfigId {
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
            5 => self.ActivityId = r.read_varint_i32()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ConfigId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulRogueWeeklyRewardResponse {
    pub ConfigId: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulRogueWeeklyRewardResponse {
    pub const MSG_ID: u16 = 28198;
}

impl PbMessage for MulRogueWeeklyRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConfigId {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ConfigId, |sr| {
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
pub struct MulRoguelikeWriteCatRewardRequest {
    pub Index: Vec<i32>,
    pub ActivityId: i32,
}

impl MulRoguelikeWriteCatRewardRequest {
    pub const MSG_ID: u16 = 24493;
}

impl PbMessage for MulRoguelikeWriteCatRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Index {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Index, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MulRoguelikeWriteCatRewardResponse {
    pub Index: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MulRoguelikeWriteCatRewardResponse {
    pub const MSG_ID: u16 = 19677;
}

impl PbMessage for MulRoguelikeWriteCatRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Index {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Index, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct MultiMapInfoNotify {
    pub UnlockMultiMapId: Vec<i32>,
    pub UnlockBlockId: Vec<i32>,
}

impl MultiMapInfoNotify {
    pub const MSG_ID: u16 = 20261;
}

impl PbMessage for MultiMapInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockMultiMapId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockBlockId {
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockMultiMapId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBlockId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MutiplayerInfo {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub IconId: i32,
    pub Level: i32,
    pub GuildName: Vec<u8>,
    pub GuildIntro: Vec<u8>,
}

impl PbMessage for MutiplayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.IconId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IconId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if !self.GuildName.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GuildName);
        }
        if !self.GuildIntro.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GuildIntro);
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
            3 => self.IconId = r.read_varint_i32()?,
            4 => self.Level = r.read_varint_i32()?,
            5 => self.GuildName = r.read_len_delimited()?.to_vec(),
            6 => self.GuildIntro = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MutiplayerTeamRefreshNearListRequest {}

impl MutiplayerTeamRefreshNearListRequest {
    pub const MSG_ID: u16 = 17426;
}

impl PbMessage for MutiplayerTeamRefreshNearListRequest {
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
pub struct MutiplayerTeamRefreshNearListResponse {
    pub ErrorParams: Vec<Vec<u8>>,
    pub InfoList: Vec<MutiplayerInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub NextRefreshTime: i32,
}

impl MutiplayerTeamRefreshNearListResponse {
    pub const MSG_ID: u16 = 20108;
}

impl PbMessage for MutiplayerTeamRefreshNearListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.InfoList {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextRefreshTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.InfoList
                    .push(<MutiplayerInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.NextRefreshTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NameModifyNotify {
    pub LastModifyNameTime: i64,
    pub Name: Vec<u8>,
}

impl NameModifyNotify {
    pub const MSG_ID: u16 = 26559;
}

impl PbMessage for NameModifyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LastModifyNameTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastModifyNameTime);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
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
            4 => self.LastModifyNameTime = r.read_varint_i64()?,
            11 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavigationFindPathRequest {
    pub EndPos: Option<Vector>,
    pub MapId: i32,
    pub StartPos: Option<Vector>,
}

impl NavigationFindPathRequest {
    pub const MSG_ID: u16 = 27225;
}

impl PbMessage for NavigationFindPathRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EndPos {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if let Some(v) = &self.StartPos {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.EndPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.MapId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.StartPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NavigationFindPathResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PosList: Vec<Vector>,
}

impl NavigationFindPathResponse {
    pub const MSG_ID: u16 = 27703;
}

impl PbMessage for NavigationFindPathResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PosList {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PosList.push(<Vector as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetTestData {
    pub DataDouble: f64,
    pub DataFloat: f32,
    pub DataInt32: i32,
    pub DataSInt32: i32,
    pub DataUInt32: u32,
    pub DataInt64: i64,
    pub DataSInt64: i64,
    pub DataUInt64: u64,
    pub DataFixed32: u32,
    pub DataFixed64: u64,
    pub DataSFixed32: i32,
    pub DataSFixed64: i64,
    pub DataBool: bool,
    pub DataString: Vec<u8>,
    pub DataBytes: Vec<u8>,
}

impl PbMessage for NetTestData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DataDouble != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.DataDouble).to_bits().to_le_bytes());
        }
        if self.DataFloat != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DataFloat).to_bits().to_le_bytes());
        }
        if self.DataInt32 != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DataInt32);
        }
        if self.DataSInt32 != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, self.DataSInt32 as i32);
        }
        if self.DataUInt32 != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.DataUInt32));
        }
        if self.DataInt64 != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DataInt64);
        }
        if self.DataSInt64 != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_sint64(out, self.DataSInt64 as i64);
        }
        if self.DataUInt64 != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, self.DataUInt64);
        }
        if self.DataFixed32 != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DataFixed32 as u32).to_le_bytes());
        }
        if self.DataFixed64 != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.DataFixed64 as u64).to_le_bytes());
        }
        if self.DataSFixed32 != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DataSFixed32 as i32).to_le_bytes());
        }
        if self.DataSFixed64 != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.DataSFixed64 as i64).to_le_bytes());
        }
        if self.DataBool {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.DataString.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DataString);
        }
        if !self.DataBytes.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DataBytes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DataDouble = r.read_f64()?,
            2 => self.DataFloat = r.read_f32()?,
            3 => self.DataInt32 = r.read_varint_i32()?,
            4 => self.DataSInt32 = r.read_sint32()?,
            5 => self.DataUInt32 = r.read_varint_u32()?,
            6 => self.DataInt64 = r.read_varint_i64()?,
            7 => self.DataSInt64 = r.read_sint64()?,
            8 => self.DataUInt64 = r.read_varint_u64()?,
            9 => self.DataFixed32 = r.read_fixed32()? as u32,
            10 => self.DataFixed64 = r.read_fixed64()? as u64,
            11 => self.DataSFixed32 = (r.read_fixed32()? as i32) as i32,
            12 => self.DataSFixed64 = (r.read_fixed64()? as i64) as i64,
            13 => self.DataBool = r.read_bool()?,
            14 => self.DataString = r.read_len_delimited()?.to_vec(),
            15 => self.DataBytes = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetTestDataNotify {
    pub Data: Option<NetTestData>,
}

impl NetTestDataNotify {
    pub const MSG_ID: u16 = 15406;
}

impl PbMessage for NetTestDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Data {
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
                self.Data
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetTestDataPush {
    pub Data: Option<NetTestData>,
}

impl NetTestDataPush {
    pub const MSG_ID: u16 = 24898;
}

impl PbMessage for NetTestDataPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Data {
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
                self.Data
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetTestDataRequest {
    pub Data: Option<NetTestData>,
}

impl NetTestDataRequest {
    pub const MSG_ID: u16 = 17003;
}

impl PbMessage for NetTestDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Data {
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
                self.Data
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NetTestDataResponse {
    pub Data: Option<NetTestData>,
}

impl NetTestDataResponse {
    pub const MSG_ID: u16 = 23468;
}

impl PbMessage for NetTestDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Data {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewBieCourseRewardNotify {
    pub HadTakeReward: Vec<i32>,
}

impl NewBieCourseRewardNotify {
    pub const MSG_ID: u16 = 18692;
}

impl PbMessage for NewBieCourseRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HadTakeReward {
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
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HadTakeReward, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewBieCourseRewardRequest {
    pub Level: i32,
}

impl NewBieCourseRewardRequest {
    pub const MSG_ID: u16 = 28328;
}

impl PbMessage for NewBieCourseRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewBieCourseRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewBieCourseRewardResponse {
    pub const MSG_ID: u16 = 17644;
}

impl PbMessage for NewBieCourseRewardResponse {
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
pub struct NewHandInItemRequest {
    pub ProjectionMachine: Option<NewHandInItemsProjectionMachinePb>,
}

impl NewHandInItemRequest {
    pub const MSG_ID: u16 = 23060;
}

impl PbMessage for NewHandInItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ProjectionMachine {
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
                self.ProjectionMachine
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewHandInItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewHandInItemResponse {
    pub const MSG_ID: u16 = 24260;
}

impl PbMessage for NewHandInItemResponse {
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
pub struct NewHandInItemsProjectionMachinePb {
    pub EntityCfgId: i32,
    pub ItemId: i32,
}

impl PbMessage for NewHandInItemsProjectionMachinePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityCfgId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityCfgId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.EntityCfgId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewJourneyRequest {
    pub TaskIds: Vec<i32>,
}

impl NewJourneyRequest {
    pub const MSG_ID: u16 = 21748;
}

impl PbMessage for NewJourneyRequest {
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
pub struct NewJourneyResponse {
    pub NewJourneyTask: Vec<NewJourneyTask>,
}

impl NewJourneyResponse {
    pub const MSG_ID: u16 = 17635;
}

impl PbMessage for NewJourneyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NewJourneyTask {
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
                self.NewJourneyTask
                    .push(<NewJourneyTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewJourneyTask {
    pub TaskIdx: i32,
    pub CanJump: bool,
    pub Complete: bool,
}

impl PbMessage for NewJourneyTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskIdx != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskIdx);
        }
        if self.CanJump {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Complete {
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
            1 => self.TaskIdx = r.read_varint_i32()?,
            2 => self.CanJump = r.read_bool()?,
            3 => self.Complete = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewLinkNotify {
    pub Current: Option<ENewLinkStage>,
    pub PlayerId: i32,
    pub LinkId: i32,
}

impl NewLinkNotify {
    pub const MSG_ID: u16 = 21875;
}

impl PbMessage for NewLinkNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Current {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.LinkId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            1 => {
                self.Current = Some(<ENewLinkStage as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.PlayerId = r.read_varint_i32()?,
            10 => self.LinkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportActivityTaskUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl NewPlayerSupportActivityTaskUpdateNotify {
    pub const MSG_ID: u16 = 19012;
}

impl PbMessage for NewPlayerSupportActivityTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportActivityTrialRoleUpdateNotify {
    pub TrialRoleId: Vec<i32>,
    pub CurUseRoleInfo: Option<roleInfo>,
}

impl NewPlayerSupportActivityTrialRoleUpdateNotify {
    pub const MSG_ID: u16 = 27128;
}

impl PbMessage for NewPlayerSupportActivityTrialRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrialRoleId {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.CurUseRoleInfo {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TrialRoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
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
pub struct NewPlayerSupportActivityTrialRoleUpdateV2Notify {
    pub CurUseRoleInfo: Option<roleInfo>,
    pub ActivityId: i32,
    pub TrialRoleId: Vec<i32>,
}

impl NewPlayerSupportActivityTrialRoleUpdateV2Notify {
    pub const MSG_ID: u16 = 15348;
}

impl PbMessage for NewPlayerSupportActivityTrialRoleUpdateV2Notify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TrialRoleId {
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
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ActivityId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TrialRoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportRewardTaskRequest {
    pub TaskIds: Vec<i32>,
}

impl NewPlayerSupportRewardTaskRequest {
    pub const MSG_ID: u16 = 15421;
}

impl PbMessage for NewPlayerSupportRewardTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportRewardTaskResponse {
    pub RoleId: Vec<i32>,
    pub Code: Option<ErrorCode>,
}

impl NewPlayerSupportRewardTaskResponse {
    pub const MSG_ID: u16 = 25832;
}

impl PbMessage for NewPlayerSupportRewardTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Code {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
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
pub struct NewPlayerSupportSetCurUseTrialRoleRequest {
    pub TrialRoleId: i32,
}

impl NewPlayerSupportSetCurUseTrialRoleRequest {
    pub const MSG_ID: u16 = 22855;
}

impl PbMessage for NewPlayerSupportSetCurUseTrialRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportSetCurUseTrialRoleResponse {
    pub RoleId: i32,
    pub ReplacedCurRole: bool,
    pub Code: Option<ErrorCode>,
    pub CurUseRoleInfo: Option<roleInfo>,
    pub ReplacedRoleId: i32,
}

impl NewPlayerSupportSetCurUseTrialRoleResponse {
    pub const MSG_ID: u16 = 29875;
}

impl PbMessage for NewPlayerSupportSetCurUseTrialRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.ReplacedCurRole {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.ReplacedRoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReplacedRoleId);
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
            6 => self.ReplacedCurRole = r.read_bool()?,
            8 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.ReplacedRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportSetCurUseTrialRoleV2Request {
    pub TrialRoleId: i32,
}

impl NewPlayerSupportSetCurUseTrialRoleV2Request {
    pub const MSG_ID: u16 = 19403;
}

impl PbMessage for NewPlayerSupportSetCurUseTrialRoleV2Request {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportSetCurUseTrialRoleV2Response {
    pub Code: Option<ErrorCode>,
    pub CurUseRoleInfo: Option<roleInfo>,
    pub RoleId: i32,
    pub ReplacedRoleId: i32,
    pub ReplacedCurRole: bool,
}

impl NewPlayerSupportSetCurUseTrialRoleV2Response {
    pub const MSG_ID: u16 = 19230;
}

impl PbMessage for NewPlayerSupportSetCurUseTrialRoleV2Response {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CurUseRoleInfo {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.ReplacedRoleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReplacedRoleId);
        }
        if self.ReplacedCurRole {
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
            5 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.CurUseRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.RoleId = r.read_varint_i32()?,
            13 => self.ReplacedRoleId = r.read_varint_i32()?,
            15 => self.ReplacedCurRole = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportTrialRoleLvUpRequest {
    pub TrialRoleId: i32,
}

impl NewPlayerSupportTrialRoleLvUpRequest {
    pub const MSG_ID: u16 = 24142;
}

impl PbMessage for NewPlayerSupportTrialRoleLvUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportTrialRoleLvUpResponse {
    pub Code: Option<ErrorCode>,
    pub RoleId: i32,
    pub CurUseRoleInfo: Option<roleInfo>,
}

impl NewPlayerSupportTrialRoleLvUpResponse {
    pub const MSG_ID: u16 = 22280;
}

impl PbMessage for NewPlayerSupportTrialRoleLvUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.CurUseRoleInfo {
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
            9 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.RoleId = r.read_varint_i32()?,
            15 => {
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
pub struct NewPlayerSupportTrialRoleLvUpV2Request {
    pub TrialRoleId: i32,
}

impl NewPlayerSupportTrialRoleLvUpV2Request {
    pub const MSG_ID: u16 = 17058;
}

impl PbMessage for NewPlayerSupportTrialRoleLvUpV2Request {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrialRoleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.TrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewPlayerSupportTrialRoleLvUpV2Response {
    pub RoleId: i32,
    pub Code: Option<ErrorCode>,
    pub CurUseRoleInfo: Option<roleInfo>,
}

impl NewPlayerSupportTrialRoleLvUpV2Response {
    pub const MSG_ID: u16 = 25380;
}

impl PbMessage for NewPlayerSupportTrialRoleLvUpV2Response {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CurUseRoleInfo {
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
            2 => self.RoleId = r.read_varint_i32()?,
            4 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
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
pub struct NewPlayerSupportV2GachaRoleUpdateNotify {
    pub DeduplicateGachaRoleIds: Vec<i32>,
    pub ActivityId: i32,
}

impl NewPlayerSupportV2GachaRoleUpdateNotify {
    pub const MSG_ID: u16 = 20716;
}

impl PbMessage for NewPlayerSupportV2GachaRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DeduplicateGachaRoleIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DeduplicateGachaRoleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerClearRecordRequest {
    pub LevelId: i32,
}

impl NewTowerClearRecordRequest {
    pub const MSG_ID: u16 = 23758;
}

impl PbMessage for NewTowerClearRecordRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerClearRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerClearRecordResponse {
    pub const MSG_ID: u16 = 19209;
}

impl PbMessage for NewTowerClearRecordResponse {
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
pub struct NewTowerClimbingCtx {
    pub Param: Option<NewTowerCycleStartParam>,
}

impl PbMessage for NewTowerClimbingCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Param {
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
                self.Param
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerCycleRecordPb {
    pub CycleId: i32,
    pub NewTowerLevelRecordPbs: Vec<NewTowerLevelRecordPb>,
}

impl PbMessage for NewTowerCycleRecordPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        for item in &self.NewTowerLevelRecordPbs {
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
            1 => self.CycleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NewTowerLevelRecordPbs
                    .push(<NewTowerLevelRecordPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerCycleStartParam {
    pub LevelId: i32,
    pub Index: i32,
    pub BuffIds: Vec<i32>,
    pub RoleSaveInfos: Vec<RoleSaveInfo>,
}

impl PbMessage for NewTowerCycleStartParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RoleSaveInfos {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.Index = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoleSaveInfos
                    .push(<RoleSaveInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerCycleStartRequest {
    pub Param: Option<NewTowerCycleStartParam>,
}

impl NewTowerCycleStartRequest {
    pub const MSG_ID: u16 = 16812;
}

impl PbMessage for NewTowerCycleStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Param {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Param
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerCycleStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerCycleStartResponse {
    pub const MSG_ID: u16 = 15315;
}

impl PbMessage for NewTowerCycleStartResponse {
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
pub struct NewTowerLastCycleInfoRequest {}

impl NewTowerLastCycleInfoRequest {
    pub const MSG_ID: u16 = 17540;
}

impl PbMessage for NewTowerLastCycleInfoRequest {
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
pub struct NewTowerLastCycleInfoResponse {
    pub NewTowerCycleRecordPb: Option<NewTowerCycleRecordPb>,
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerLastCycleInfoResponse {
    pub const MSG_ID: u16 = 17319;
}

impl PbMessage for NewTowerLastCycleInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewTowerCycleRecordPb {
            crate::wire::write_submessage(out, 7, v);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.NewTowerCycleRecordPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
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
pub struct NewTowerLevelRecordPb {
    pub LevelId: i32,
    pub NewTowerTeamRecordPbs: Vec<NewTowerTeamRecordPb>,
}

impl PbMessage for NewTowerLevelRecordPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.NewTowerTeamRecordPbs {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NewTowerTeamRecordPbs
                    .push(<NewTowerTeamRecordPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerLevelUnlockNotify {
    pub Record: Option<NewTowerClimbingLevelRecord>,
}

impl NewTowerLevelUnlockNotify {
    pub const MSG_ID: u16 = 23956;
}

impl PbMessage for NewTowerLevelUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Record {
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
                self.Record
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerMedalInfo {
    pub MedalGroupId: i32,
    pub SeasonId: i32,
    pub CycleId: i32,
    pub Progress: i32,
    pub CompleteTime: i64,
}

impl PbMessage for NewTowerMedalInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MedalGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MedalGroupId);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.CompleteTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CompleteTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MedalGroupId = r.read_varint_i32()?,
            2 => self.SeasonId = r.read_varint_i32()?,
            3 => self.CycleId = r.read_varint_i32()?,
            4 => self.Progress = r.read_varint_i32()?,
            5 => self.CompleteTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerMedalInfoRequest {}

impl NewTowerMedalInfoRequest {
    pub const MSG_ID: u16 = 26935;
}

impl PbMessage for NewTowerMedalInfoRequest {
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
pub struct NewTowerMedalInfoResponse {
    pub NewTowerMedalInfos: Vec<NewTowerMedalInfo>,
}

impl NewTowerMedalInfoResponse {
    pub const MSG_ID: u16 = 27000;
}

impl PbMessage for NewTowerMedalInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NewTowerMedalInfos {
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
                self.NewTowerMedalInfos
                    .push(<NewTowerMedalInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerMonsterHpInfo {
    pub WaveConfigId: i32,
    pub IncId: i64,
    pub MaxHp: i64,
    pub CurHp: i64,
}

impl PbMessage for NewTowerMonsterHpInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveConfigId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.IncId);
        }
        if self.MaxHp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxHp);
        }
        if self.CurHp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurHp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.WaveConfigId = r.read_varint_i32()?,
            2 => self.IncId = r.read_varint_i64()?,
            3 => self.MaxHp = r.read_varint_i64()?,
            4 => self.CurHp = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerMonsterHpInfoNotify {
    pub MonsterHpInfos: Vec<NewTowerMonsterHpInfo>,
}

impl NewTowerMonsterHpInfoNotify {
    pub const MSG_ID: u16 = 25553;
}

impl PbMessage for NewTowerMonsterHpInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MonsterHpInfos {
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
                self.MonsterHpInfos
                    .push(<NewTowerMonsterHpInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRecommendData {
    pub RecommendLevels: Vec<NewTowerRecommendLevel>,
}

impl PbMessage for NewTowerRecommendData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecommendLevels {
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
                self.RecommendLevels
                    .push(<NewTowerRecommendLevel as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRecommendLevel {
    pub LevelId: i32,
    pub RecommendId: i32,
    pub RecommendTeams: Vec<NewTowerRecommendTeam>,
}

impl PbMessage for NewTowerRecommendLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.RecommendId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommendId);
        }
        for item in &self.RecommendTeams {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.RecommendId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RecommendTeams
                    .push(<NewTowerRecommendTeam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRecommendRequest {}

impl NewTowerRecommendRequest {
    pub const MSG_ID: u16 = 23165;
}

impl PbMessage for NewTowerRecommendRequest {
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
pub struct NewTowerRecommendResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub NewTowerRecommendData: Option<NewTowerRecommendData>,
}

impl NewTowerRecommendResponse {
    pub const MSG_ID: u16 = 15290;
}

impl PbMessage for NewTowerRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.NewTowerRecommendData {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.NewTowerRecommendData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRecommendTeam {
    pub RoleIds: Vec<i32>,
    pub SkillBranchIds: Vec<i32>,
    pub BuffIds: Vec<i32>,
    pub UsageRate: i32,
}

impl PbMessage for NewTowerRecommendTeam {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkillBranchIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.UsageRate != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UsageRate);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.UsageRate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerResultNotify {
    pub TeamChallengeInfo: Option<TeamChallengeInfo>,
    pub BeforeLevelScore: i32,
    pub BeforeTeamScore: i32,
    pub AfterTeamScore: i32,
    pub MonsterInfoPreviews: Vec<MonsterInfoPreview>,
    pub CompletedSeasonTaskIds: Vec<i32>,
    pub AfterLevelScore: i32,
    pub LevelId: i32,
    pub Score: i32,
    pub TotalScore: i32,
}

impl NewTowerResultNotify {
    pub const MSG_ID: u16 = 23657;
}

impl PbMessage for NewTowerResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TeamChallengeInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.BeforeLevelScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BeforeLevelScore);
        }
        if self.BeforeTeamScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BeforeTeamScore);
        }
        if self.AfterTeamScore != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AfterTeamScore);
        }
        for item in &self.MonsterInfoPreviews {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.CompletedSeasonTaskIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.AfterLevelScore != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AfterLevelScore);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TeamChallengeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.BeforeLevelScore = r.read_varint_i32()?,
            4 => self.BeforeTeamScore = r.read_varint_i32()?,
            5 => self.AfterTeamScore = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MonsterInfoPreviews
                    .push(<MonsterInfoPreview as PbMessage>::decode(bytes)?);
            }
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompletedSeasonTaskIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.AfterLevelScore = r.read_varint_i32()?,
            11 => self.LevelId = r.read_varint_i32()?,
            12 => self.Score = r.read_varint_i32()?,
            13 => self.TotalScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRewardRequest {
    pub Indexs: Vec<i32>,
}

impl NewTowerRewardRequest {
    pub const MSG_ID: u16 = 19524;
}

impl PbMessage for NewTowerRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Indexs {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Indexs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerRewardResponse {
    pub const MSG_ID: u16 = 15099;
}

impl PbMessage for NewTowerRewardResponse {
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
pub struct NewTowerRoleEnergyRequest {
    pub CycleId: i32,
}

impl NewTowerRoleEnergyRequest {
    pub const MSG_ID: u16 = 24038;
}

impl PbMessage for NewTowerRoleEnergyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.CycleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerRoleEnergyResponse {
    pub RecordRoleEnergys: Vec<RecordRoleEnergy>,
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerRoleEnergyResponse {
    pub const MSG_ID: u16 = 25538;
}

impl PbMessage for NewTowerRoleEnergyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecordRoleEnergys {
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
                self.RecordRoleEnergys
                    .push(<RecordRoleEnergy as PbMessage>::decode(bytes)?);
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
pub struct NewTowerRoundUpdateNotify {
    pub Round: i32,
}

impl NewTowerRoundUpdateNotify {
    pub const MSG_ID: u16 = 23161;
}

impl PbMessage for NewTowerRoundUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Round != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Round);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.Round = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerSeasonRewardRequest {
    pub Indexs: Vec<i32>,
}

impl NewTowerSeasonRewardRequest {
    pub const MSG_ID: u16 = 23587;
}

impl PbMessage for NewTowerSeasonRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Indexs {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Indexs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerSeasonRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerSeasonRewardResponse {
    pub const MSG_ID: u16 = 20020;
}

impl PbMessage for NewTowerSeasonRewardResponse {
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
pub struct NewTowerSeasonScoreRewardRequest {
    pub Indexs: Vec<i32>,
}

impl NewTowerSeasonScoreRewardRequest {
    pub const MSG_ID: u16 = 26245;
}

impl PbMessage for NewTowerSeasonScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Indexs {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Indexs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerSeasonScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewTowerSeasonScoreRewardResponse {
    pub const MSG_ID: u16 = 24617;
}

impl PbMessage for NewTowerSeasonScoreRewardResponse {
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
pub struct NewTowerSeasonTaskDataUpdateNotify {
    pub SeasonTasks: Vec<ActivityTask>,
    pub Removes: Vec<i32>,
    pub Adds: Vec<ActivityTask>,
}

impl NewTowerSeasonTaskDataUpdateNotify {
    pub const MSG_ID: u16 = 24370;
}

impl PbMessage for NewTowerSeasonTaskDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SeasonTasks {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.Removes {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Adds {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SeasonTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Adds.push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerSeasonTaskDataUpdateRequest {}

impl NewTowerSeasonTaskDataUpdateRequest {
    pub const MSG_ID: u16 = 18463;
}

impl PbMessage for NewTowerSeasonTaskDataUpdateRequest {
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
pub struct NewTowerSeasonTaskDataUpdateResponse {
    pub SeasonTasks: Vec<ActivityTask>,
}

impl NewTowerSeasonTaskDataUpdateResponse {
    pub const MSG_ID: u16 = 28342;
}

impl PbMessage for NewTowerSeasonTaskDataUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SeasonTasks {
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
                self.SeasonTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerTaskDataUpdateNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl NewTowerTaskDataUpdateNotify {
    pub const MSG_ID: u16 = 20235;
}

impl PbMessage for NewTowerTaskDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerTeamRecordPb {
    pub Roles: Vec<i32>,
    pub Buffs: Vec<i32>,
    pub Score: i32,
}

impl PbMessage for NewTowerTeamRecordPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Buffs {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Roles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Buffs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerUpdateRecordNotify {
    pub Record: Option<NewTowerClimbingLevelRecord>,
}

impl NewTowerUpdateRecordNotify {
    pub const MSG_ID: u16 = 28883;
}

impl PbMessage for NewTowerUpdateRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Record {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Record
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerUpdateRecordRequest {
    pub LevelId: i32,
}

impl NewTowerUpdateRecordRequest {
    pub const MSG_ID: u16 = 16212;
}

impl PbMessage for NewTowerUpdateRecordRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTowerUpdateRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Record: Option<NewTowerClimbingLevelRecord>,
}

impl NewTowerUpdateRecordResponse {
    pub const MSG_ID: u16 = 25854;
}

impl PbMessage for NewTowerUpdateRecordResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Record {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Record
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewTrialRoleGetNightmarePhantomInstInfoRequest {}

impl NewTrialRoleGetNightmarePhantomInstInfoRequest {
    pub const MSG_ID: u16 = 17854;
}

impl PbMessage for NewTrialRoleGetNightmarePhantomInstInfoRequest {
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
pub struct NewTrialRoleGetNightmarePhantomInstInfoResponse {
    pub Code: Option<ErrorCode>,
    pub InsInfoList: Vec<NightmarePhantomInstInfo>,
    pub SaveInsIdList: Vec<i32>,
}

impl NewTrialRoleGetNightmarePhantomInstInfoResponse {
    pub const MSG_ID: u16 = 19481;
}

impl PbMessage for NewTrialRoleGetNightmarePhantomInstInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.InsInfoList {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.SaveInsIdList {
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
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InsInfoList
                    .push(<NightmarePhantomInstInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SaveInsIdList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureChapterDropRequest {
    pub ActivityId: i32,
    pub ChapterId: i32,
}

impl NewbieAdventureChapterDropRequest {
    pub const MSG_ID: u16 = 16662;
}

impl PbMessage for NewbieAdventureChapterDropRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            14 => self.ActivityId = r.read_varint_i32()?,
            15 => self.ChapterId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureChapterDropResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieAdventureChapterDropResponse {
    pub const MSG_ID: u16 = 19337;
}

impl PbMessage for NewbieAdventureChapterDropResponse {
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
pub struct NewbieAdventureRoleRewardRequest {
    pub ChapterId: i32,
    pub RoleId: i32,
    pub ActivityId: i32,
}

impl NewbieAdventureRoleRewardRequest {
    pub const MSG_ID: u16 = 16825;
}

impl PbMessage for NewbieAdventureRoleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            2 => self.ChapterId = r.read_varint_i32()?,
            5 => self.RoleId = r.read_varint_i32()?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureRoleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieAdventureRoleRewardResponse {
    pub const MSG_ID: u16 = 17073;
}

impl PbMessage for NewbieAdventureRoleRewardResponse {
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
pub struct NewbieAdventureTaskRewardRequest {
    pub ActivityId: i32,
    pub TaskIds: Vec<i32>,
}

impl NewbieAdventureTaskRewardRequest {
    pub const MSG_ID: u16 = 29376;
}

impl PbMessage for NewbieAdventureTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            5 => self.ActivityId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieAdventureTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieAdventureTaskRewardResponse {
    pub const MSG_ID: u16 = 15720;
}

impl PbMessage for NewbieAdventureTaskRewardResponse {
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
pub struct NewbieAdventureTaskUpdateNotify {
    pub ActivityId: i32,
    pub Chapter: Vec<NewbieAdventureV2ChapterPb>,
}

impl NewbieAdventureTaskUpdateNotify {
    pub const MSG_ID: u16 = 27178;
}

impl PbMessage for NewbieAdventureTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Chapter {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Chapter
                    .push(<NewbieAdventureV2ChapterPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCarnivalAwardRequest {
    pub ActivityId: i32,
    pub Config: i32,
}

impl NewbieCarnivalAwardRequest {
    pub const MSG_ID: u16 = 27569;
}

impl PbMessage for NewbieCarnivalAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Config != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Config);
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
            4 => self.Config = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCarnivalAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieCarnivalAwardResponse {
    pub const MSG_ID: u16 = 19176;
}

impl PbMessage for NewbieCarnivalAwardResponse {
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
pub struct NewbieCarnivalData {
    pub RoleId: i32,
    pub ActivityTaskData: Option<ActivityTaskData>,
    pub JumpTaskIds: Vec<i32>,
    pub JumpTaskCondInfos: Vec<JumpTaskCondInfo>,
}

impl PbMessage for NewbieCarnivalData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.ActivityTaskData {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.JumpTaskIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.JumpTaskCondInfos {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.JumpTaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.JumpTaskCondInfos
                    .push(<JumpTaskCondInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCarnivalSwitchRoleRequest {
    pub ActivityId: i32,
    pub RoleId: i32,
}

impl NewbieCarnivalSwitchRoleRequest {
    pub const MSG_ID: u16 = 26924;
}

impl PbMessage for NewbieCarnivalSwitchRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            5 => self.ActivityId = r.read_varint_i32()?,
            9 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCarnivalSwitchRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieCarnivalSwitchRoleResponse {
    pub const MSG_ID: u16 = 27062;
}

impl PbMessage for NewbieCarnivalSwitchRoleResponse {
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
pub struct NewbieCarnivalTaskDataUpdateNotify {
    pub ActivityTaskData: Option<ActivityTaskData>,
}

impl NewbieCarnivalTaskDataUpdateNotify {
    pub const MSG_ID: u16 = 23985;
}

impl PbMessage for NewbieCarnivalTaskDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTaskData {
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTaskData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCarnivalTaskJumpUpdateNotify {
    pub JumpTaskIds: Vec<i32>,
    pub JumpTaskCondInfos: Vec<JumpTaskCondInfo>,
}

impl NewbieCarnivalTaskJumpUpdateNotify {
    pub const MSG_ID: u16 = 26302;
}

impl PbMessage for NewbieCarnivalTaskJumpUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.JumpTaskIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.JumpTaskCondInfos {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.JumpTaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.JumpTaskCondInfos
                    .push(<JumpTaskCondInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCourseV2RewardNotify {
    pub ActivityId: i32,
    pub HadTakeReward: Vec<i32>,
}

impl NewbieCourseV2RewardNotify {
    pub const MSG_ID: u16 = 28195;
}

impl PbMessage for NewbieCourseV2RewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.HadTakeReward {
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
            11 => self.ActivityId = r.read_varint_i32()?,
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HadTakeReward, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCourseV2RewardRequest {
    pub Level: Vec<i32>,
    pub ActivityId: i32,
}

impl NewbieCourseV2RewardRequest {
    pub const MSG_ID: u16 = 27099;
}

impl PbMessage for NewbieCourseV2RewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Level {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Level, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieCourseV2RewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl NewbieCourseV2RewardResponse {
    pub const MSG_ID: u16 = 16170;
}

impl PbMessage for NewbieCourseV2RewardResponse {
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
pub struct NewbieFetchScoreRewardRequest {
    pub ActivityId: i32,
    pub ScoreRewardIds: Vec<i32>,
}

impl NewbieFetchScoreRewardRequest {
    pub const MSG_ID: u16 = 25135;
}

impl PbMessage for NewbieFetchScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ScoreRewardIds {
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
            13 => self.ActivityId = r.read_varint_i32()?,
            15 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreRewardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NewbieFetchScoreRewardResponse {
    pub Code: Option<ErrorCode>,
}

impl NewbieFetchScoreRewardResponse {
    pub const MSG_ID: u16 = 29217;
}

impl PbMessage for NewbieFetchScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
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
pub struct NewbieMainTaskUpdateNotify {
    pub ActivityId: i32,
    pub ProgressScore: i32,
    pub NewbieMainTabs: Vec<NewbieMainTabPb>,
}

impl NewbieMainTaskUpdateNotify {
    pub const MSG_ID: u16 = 27645;
}

impl PbMessage for NewbieMainTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ProgressScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProgressScore);
        }
        for item in &self.NewbieMainTabs {
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
            2 => self.ActivityId = r.read_varint_i32()?,
            8 => self.ProgressScore = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.NewbieMainTabs
                    .push(<NewbieMainTabPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NightmarePhantomInstInfo {
    pub InsId: i32,
    pub KillNum: i64,
}

impl PbMessage for NightmarePhantomInstInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InsId);
        }
        if self.KillNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.KillNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.InsId = r.read_varint_i32()?,
            2 => self.KillNum = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NoRenderPortalCtx {
    pub EntityId: i64,
    pub EntityConfigId: i32,
}

impl PbMessage for NoRenderPortalCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalItemRemoveNotify {
    pub NormalItemIdList: Vec<i32>,
}

impl NormalItemRemoveNotify {
    pub const MSG_ID: u16 = 19848;
}

impl PbMessage for NormalItemRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NormalItemIdList {
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
                &mut self.NormalItemIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalMonsterManualInfoRequest {
    pub DetectionId: i32,
}

impl NormalMonsterManualInfoRequest {
    pub const MSG_ID: u16 = 22245;
}

impl PbMessage for NormalMonsterManualInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.DetectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NormalMonsterManualInfoResponse {
    pub Code: Option<ErrorCode>,
    pub DetectionTarget: Vec<DetectionTarget>,
}

impl NormalMonsterManualInfoResponse {
    pub const MSG_ID: u16 = 22708;
}

impl PbMessage for NormalMonsterManualInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DetectionTarget {
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
            10 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionTarget
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcFavorChangeSyncDataPb {
    pub NpcId: i32,
    pub FavorAbsolute: i32,
    pub FavorLevel: i32,
}

impl PbMessage for NpcFavorChangeSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NpcId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NpcId);
        }
        if self.FavorAbsolute != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FavorAbsolute);
        }
        if self.FavorLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FavorLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.NpcId = r.read_varint_i32()?,
            3 => self.FavorAbsolute = r.read_varint_i32()?,
            4 => self.FavorLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPerformComponentPb {
    pub Type: Vec<u8>,
    pub State: Vec<u8>,
}

impl PbMessage for NpcPerformComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Type.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Type);
        }
        if !self.State.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
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
            1 => self.Type = r.read_len_delimited()?.to_vec(),
            2 => self.State = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleAreaLockEffectCtx {
    pub AreaType: Option<BattleAreaType>,
    pub Seal: bool,
    pub UnLockRound: i32,
    pub Camp: Option<PhantomBattleEffectCardCamp>,
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
}

impl PbMessage for NpcPhantomBattleAreaLockEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AreaType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Seal {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnLockRound != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockRound);
        }
        if let Some(v) = &self.Camp {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleFighterInfo {
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
            1 => {
                self.AreaType = Some(
                    <BattleAreaType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Seal = r.read_bool()?,
            3 => self.UnLockRound = r.read_varint_i32()?,
            5 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFighterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleBackCardLibrary {
    pub HandCardNum: i32,
    pub HandCardIndex: i32,
    pub CurCardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleBackCardLibrary {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.HandCardIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardIndex);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HandCardNum = r.read_varint_i32()?,
            2 => self.HandCardIndex = r.read_varint_i32()?,
            3 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleBackSlotCardLibrary {
    pub CardUId: i32,
    pub CurCardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleBackSlotCardLibrary {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardUId = r.read_varint_i32()?,
            2 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleCardAttrInfo {
    pub CardFighterUId: i32,
    pub BattleStatus: BTreeMap<i32, i32>,
}

impl PbMessage for NpcPhantomBattleCardAttrInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardFighterUId);
        }
        if !self.BattleStatus.is_empty() {
            crate::wire::write_map(out, 2, &self.BattleStatus);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardFighterUId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.BattleStatus)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleCardDurableSkillInfo {
    pub CardUId: i32,
    pub CardConfId: i32,
    pub SkillId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub PhantomBattleEffectResultInfo: Option<PhantomBattleEffectResultInfo>,
    pub CurCardLibraryNum: i32,
    pub HandCardNum: i32,
}

impl PbMessage for NpcPhantomBattleCardDurableSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if self.CardConfId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.PhantomBattleEffectResultInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardUId = r.read_varint_i32()?,
            2 => self.CardConfId = r.read_varint_i32()?,
            3 => self.SkillId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleEffectResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.CurCardLibraryNum = r.read_varint_i32()?,
            7 => self.HandCardNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleCardRoleSkillInfo {
    pub SkillId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub PhantomBattleEffectResultInfo: Option<PhantomBattleEffectResultInfo>,
}

impl PbMessage for NpcPhantomBattleCardRoleSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.PhantomBattleEffectResultInfo {
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
            1 => self.SkillId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleEffectResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleCardSelectEffectCtx {
    pub CurHandNum: i32,
    pub CardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleCardSelectEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurHandNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurHandNum);
        }
        if self.CardLibraryNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurHandNum = r.read_varint_i32()?,
            2 => self.CardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleCardSkillInfo {
    pub CardUId: i32,
    pub CardConfId: i32,
    pub SkillId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub PhantomBattleEffectResultInfo: Option<PhantomBattleEffectResultInfo>,
    pub CurCardLibraryNum: i32,
    pub HandCardNum: i32,
}

impl PbMessage for NpcPhantomBattleCardSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if self.CardConfId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.PhantomBattleEffectResultInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardUId = r.read_varint_i32()?,
            2 => self.CardConfId = r.read_varint_i32()?,
            3 => self.SkillId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleEffectResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.CurCardLibraryNum = r.read_varint_i32()?,
            7 => self.HandCardNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleDealEffectCtx {
    pub CurHandNum: i32,
    pub CardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleDealEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurHandNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurHandNum);
        }
        if self.CardLibraryNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurHandNum = r.read_varint_i32()?,
            2 => self.CardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleDiscardInfo {
    pub DiscardNum: i32,
    pub HandCardNum: i32,
    pub CurCardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleDiscardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DiscardNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscardNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DiscardNum = r.read_varint_i32()?,
            2 => self.HandCardNum = r.read_varint_i32()?,
            3 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleDurableEmptyInfo {
    pub CardUid: i32,
    pub DealType: Option<PhantomBattleCardDurableDealType>,
    pub CardHeapSize: i32,
    pub CardHandSize: i32,
}

impl PbMessage for NpcPhantomBattleDurableEmptyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
        if let Some(v) = &self.DealType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CardHeapSize != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardHeapSize);
        }
        if self.CardHandSize != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardHandSize);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardUid = r.read_varint_i32()?,
            2 => {
                self.DealType = Some(
                    <PhantomBattleCardDurableDealType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.CardHeapSize = r.read_varint_i32()?,
            4 => self.CardHandSize = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleEnterSlotInfo {
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
    pub CurHandNum: i32,
}

impl PbMessage for NpcPhantomBattleEnterSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.CurHandNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurHandNum);
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
                self.PhantomBattleFighterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.CurHandNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleEvolveInfo {
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
    pub CanEvolveNum: i32,
    pub HandCardNum: i32,
    pub CardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleEvolveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.CanEvolveNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanEvolveNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.CardLibraryNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardLibraryNum);
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
                self.PhantomBattleFighterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.CanEvolveNum = r.read_varint_i32()?,
            3 => self.HandCardNum = r.read_varint_i32()?,
            4 => self.CardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleFightInfo {
    pub PhantomBattleEvolveInfo: Option<NpcPhantomBattleEvolveInfo>,
    pub PhantomBattleSlotInsteadInfo: Option<NpcPhantomBattleSlotInsteadInfo>,
    pub PhantomBattleSkillTriggerInfo: Option<PhantomBattleSkillTriggerInfo>,
    pub PhantomBattleBuffTriggerInfo: Option<PhantomBattleBuffTriggerInfo>,
    pub NpcPhantomBattleCardSkillInfo: Option<NpcPhantomBattleCardSkillInfo>,
    pub NpcPhantomBattleCardRoleSkillInfo: Option<NpcPhantomBattleCardRoleSkillInfo>,
    pub NpcPhantomBattleGamerStatusInfo: Option<NpcPhantomBattleGamerStatusInfo>,
    pub NpcPhantomBattleCardAttrInfo: Option<NpcPhantomBattleCardAttrInfo>,
    pub NpcPhantomBattleEnterSlotInfo: Option<NpcPhantomBattleEnterSlotInfo>,
    pub NpcPhantomBattleBackCardLibrary: Option<NpcPhantomBattleBackCardLibrary>,
    pub NpcPhantomBattleDiscardInfo: Option<NpcPhantomBattleDiscardInfo>,
    pub NpcPhantomFourCTaskDealCardInfo: Option<NpcPhantomFourCTaskDealCardInfo>,
    pub NpcPhantomLeaveSlot: Option<NpcPhantomLeaveSlot>,
    pub NpcPhantomBattleBackSlotCardLibrary: Option<NpcPhantomBattleBackSlotCardLibrary>,
    pub NpcPhantomBattleReserveCardInfo: Option<NpcPhantomBattleReserveCardInfo>,
    pub PhantomBattleNpcCardUpdateInfo: Option<PhantomBattleNpcCardUpdateInfo>,
    pub NpcPhantomBattleDurableEmptyInfo: Option<NpcPhantomBattleDurableEmptyInfo>,
    pub NpcPhantomBattleCardDurableSkillInfo: Option<NpcPhantomBattleCardDurableSkillInfo>,
}

impl PbMessage for NpcPhantomBattleFightInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleEvolveInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.PhantomBattleSlotInsteadInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.PhantomBattleSkillTriggerInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.PhantomBattleBuffTriggerInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.NpcPhantomBattleCardSkillInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.NpcPhantomBattleCardRoleSkillInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.NpcPhantomBattleGamerStatusInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.NpcPhantomBattleCardAttrInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.NpcPhantomBattleEnterSlotInfo {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.NpcPhantomBattleBackCardLibrary {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.NpcPhantomBattleDiscardInfo {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.NpcPhantomFourCTaskDealCardInfo {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.NpcPhantomLeaveSlot {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.NpcPhantomBattleBackSlotCardLibrary {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.NpcPhantomBattleReserveCardInfo {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.PhantomBattleNpcCardUpdateInfo {
            crate::wire::write_submessage(out, 17, v);
        }
        if let Some(v) = &self.NpcPhantomBattleDurableEmptyInfo {
            crate::wire::write_submessage(out, 18, v);
        }
        if let Some(v) = &self.NpcPhantomBattleCardDurableSkillInfo {
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
                self.PhantomBattleEvolveInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSlotInsteadInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSkillTriggerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleBuffTriggerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleCardSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleCardRoleSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleGamerStatusInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleCardAttrInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleEnterSlotInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleBackCardLibrary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleDiscardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomFourCTaskDealCardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomLeaveSlot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleBackSlotCardLibrary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleReserveCardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleNpcCardUpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleDurableEmptyInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleCardDurableSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleFightInfoNotify {
    pub PhantomBattleFightInfos: Vec<NpcPhantomBattleFightInfo>,
}

impl NpcPhantomBattleFightInfoNotify {
    pub const MSG_ID: u16 = 21331;
}

impl PbMessage for NpcPhantomBattleFightInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleFightInfos {
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
                self.PhantomBattleFightInfos
                    .push(<NpcPhantomBattleFightInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleGamerStatusInfo {
    pub GamerIndex: i32,
    pub BattleStatus: BTreeMap<i32, i32>,
}

impl PbMessage for NpcPhantomBattleGamerStatusInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        if !self.BattleStatus.is_empty() {
            crate::wire::write_map(out, 2, &self.BattleStatus);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GamerIndex = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.BattleStatus)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleReserveCardInfo {
    pub ReserveNum: i32,
    pub HandCardNum: i32,
    pub CurCardLibraryNum: i32,
}

impl PbMessage for NpcPhantomBattleReserveCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReserveNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReserveNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ReserveNum = r.read_varint_i32()?,
            2 => self.HandCardNum = r.read_varint_i32()?,
            3 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomBattleSlotInsteadInfo {
    pub SlotIndexA: i32,
    pub SlotIndexB: i32,
}

impl PbMessage for NpcPhantomBattleSlotInsteadInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotIndexA != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndexA);
        }
        if self.SlotIndexB != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndexB);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SlotIndexA = r.read_varint_i32()?,
            2 => self.SlotIndexB = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomFourCTaskDealCardInfo {
    pub CardConf: i32,
    pub HandCardNum: i32,
    pub CurCardLibraryNum: i32,
}

impl PbMessage for NpcPhantomFourCTaskDealCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardConf != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConf);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CardConf = r.read_varint_i32()?,
            2 => self.HandCardNum = r.read_varint_i32()?,
            3 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPhantomLeaveSlot {
    pub FighterUId: i32,
    pub SlotIndex: i32,
    pub HandCardNum: i32,
}

impl PbMessage for NpcPhantomLeaveSlot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FighterUId);
        }
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FighterUId = r.read_varint_i32()?,
            2 => self.SlotIndex = r.read_varint_i32()?,
            3 => self.HandCardNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcTraceFailedRequest {
    pub TreeIncId: i64,
    pub NodeId: i32,
    pub TreeOwnerId: i32,
}

impl NpcTraceFailedRequest {
    pub const MSG_ID: u16 = 29936;
}

impl PbMessage for NpcTraceFailedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
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
            4 => self.TreeIncId = r.read_varint_i64()?,
            11 => self.NodeId = r.read_varint_i32()?,
            14 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcTraceFailedResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl NpcTraceFailedResponse {
    pub const MSG_ID: u16 = 19981;
}

impl PbMessage for NpcTraceFailedResponse {
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
pub struct NumItem {
    pub Id: i32,
    pub Count: i32,
}

impl PbMessage for NumItem {
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
pub struct OccupationInfoNotify {
    pub OccupationInfo: Vec<OccupationPbInfo>,
}

impl OccupationInfoNotify {
    pub const MSG_ID: u16 = 16517;
}

impl PbMessage for OccupationInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OccupationInfo {
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
                self.OccupationInfo
                    .push(<OccupationPbInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OccupyRewardRequest {
    pub Ids: Vec<i32>,
}

impl OccupyRewardRequest {
    pub const MSG_ID: u16 = 18660;
}

impl PbMessage for OccupyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OccupyRewardResponse {
    pub Ids: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl OccupyRewardResponse {
    pub const MSG_ID: u16 = 24387;
}

impl PbMessage for OccupyRewardResponse {
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
pub struct OneDeviceInputSetting {
    pub Device: Option<InputSettingDevice>,
    pub DeviceSubType: Vec<u8>,
    pub InputAction: Vec<InputAction>,
    pub InputAxis: Vec<InputAxis>,
    pub InputCombinationAction: Vec<CombinationAction>,
    pub InputCombinationAxis: Vec<CombinationAxis>,
}

impl PbMessage for OneDeviceInputSetting {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Device {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.DeviceSubType.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceSubType);
        }
        for item in &self.InputAction {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.InputAxis {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.InputCombinationAction {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.InputCombinationAxis {
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
            5 => {
                self.Device = Some(
                    <InputSettingDevice as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => self.DeviceSubType = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.InputAction
                    .push(<InputAction as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.InputAxis
                    .push(<InputAxis as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.InputCombinationAction
                    .push(<CombinationAction as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.InputCombinationAxis
                    .push(<CombinationAxis as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneFishCage {
    pub EntityConfigId: i32,
    pub MaxCount: i32,
    pub Items: Vec<FishingItemInfo>,
    pub LastUpdateTime: i64,
    pub Id: i32,
    pub NextUpdateTime: i64,
    pub RefreshTime: i32,
}

impl PbMessage for OneFishCage {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.MaxCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCount);
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.LastUpdateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastUpdateTime);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.NextUpdateTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextUpdateTime);
        }
        if self.RefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshTime);
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
            2 => self.MaxCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.LastUpdateTime = r.read_varint_i64()?,
            5 => self.Id = r.read_varint_i32()?,
            6 => self.NextUpdateTime = r.read_varint_i64()?,
            7 => self.RefreshTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneFishPointInfo {
    pub EntityConfigId: i32,
    pub CurCount: i32,
    pub MaxCount: i32,
    pub LastUpdateTime: i64,
    pub Id: i32,
    pub NextUpdateTime: i64,
    pub RefreshTime: i32,
    pub GamePlayId: i32,
    pub Interacted: bool,
}

impl PbMessage for OneFishPointInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.CurCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCount);
        }
        if self.MaxCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCount);
        }
        if self.LastUpdateTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastUpdateTime);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.NextUpdateTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextUpdateTime);
        }
        if self.RefreshTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshTime);
        }
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayId);
        }
        if self.Interacted {
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
            1 => self.EntityConfigId = r.read_varint_i32()?,
            2 => self.CurCount = r.read_varint_i32()?,
            3 => self.MaxCount = r.read_varint_i32()?,
            4 => self.LastUpdateTime = r.read_varint_i64()?,
            5 => self.Id = r.read_varint_i32()?,
            6 => self.NextUpdateTime = r.read_varint_i64()?,
            7 => self.RefreshTime = r.read_varint_i32()?,
            8 => self.GamePlayId = r.read_varint_i32()?,
            9 => self.Interacted = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneFishingIllustratedInfo {
    pub Id: i32,
    pub MaxSize: i32,
    pub MinSize: i32,
}

impl PbMessage for OneFishingIllustratedInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.MaxSize != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxSize);
        }
        if self.MinSize != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinSize);
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
            2 => self.MaxSize = r.read_varint_i32()?,
            3 => self.MinSize = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneInfluenceInfo {
    pub InfluenceId: i32,
    pub RewardIndex: i32,
    pub Relation: i32,
}

impl PbMessage for OneInfluenceInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InfluenceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InfluenceId);
        }
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if self.Relation != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Relation);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.InfluenceId = r.read_varint_i32()?,
            2 => self.RewardIndex = r.read_varint_i32()?,
            3 => self.Relation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneItemFilterRule {
    pub FilterRuleId: i32,
    pub IdList: Vec<i32>,
}

impl PbMessage for OneItemFilterRule {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FilterRuleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FilterRuleId);
        }
        for item in &self.IdList {
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
            1 => self.FilterRuleId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnePhantomSetting {
    pub Index: i32,
    pub On: bool,
    pub Name: Vec<u8>,
    pub RuleList: Vec<OneItemFilterRule>,
}

impl PbMessage for OnePhantomSetting {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.On {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        for item in &self.RuleList {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => self.On = r.read_bool()?,
            3 => self.Name = r.read_len_delimited()?.to_vec(),
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RuleList
                    .push(<OneItemFilterRule as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneTimePointRewardActivityData {
    pub Id: i32,
    pub RewardTime: i64,
    pub Rewarded: bool,
    pub CanGetReward: bool,
    pub Task: Option<ConditionTask>,
}

impl PbMessage for OneTimePointRewardActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RewardTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RewardTime);
        }
        if self.Rewarded {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CanGetReward {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Task {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.RewardTime = r.read_varint_i64()?,
            3 => self.Rewarded = r.read_bool()?,
            4 => self.CanGetReward = r.read_bool()?,
            5 => {
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
pub struct OneTowerDefencePassInfo {
    pub ChallengeId: i32,
    pub IsSingle: bool,
    pub PassInfo: Option<TowerDefencePassInfo>,
}

impl PbMessage for OneTowerDefencePassInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.IsSingle {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.PassInfo {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.IsSingle = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PassInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneWeaponItemInfo {
    pub IncId: i32,
    pub WeaponLevel: i32,
    pub WeaponExp: i32,
    pub WeaponBreach: i32,
    pub WeaponResonLevel: i32,
    pub RoleId: i32,
}

impl PbMessage for OneWeaponItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.WeaponLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponLevel);
        }
        if self.WeaponExp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponExp);
        }
        if self.WeaponBreach != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBreach);
        }
        if self.WeaponResonLevel != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponResonLevel);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            1 => self.IncId = r.read_varint_i32()?,
            2 => self.WeaponLevel = r.read_varint_i32()?,
            3 => self.WeaponExp = r.read_varint_i32()?,
            4 => self.WeaponBreach = r.read_varint_i32()?,
            5 => self.WeaponResonLevel = r.read_varint_i32()?,
            6 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorActivityLevelRequest {
    pub RewardType: Option<OnlineMotorRewardType>,
    pub RewardIds: Vec<i32>,
}

impl OnlineMotorActivityLevelRequest {
    pub const MSG_ID: u16 = 24796;
}

impl PbMessage for OnlineMotorActivityLevelRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RewardIds {
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
            3 => {
                self.RewardType = Some(
                    <OnlineMotorRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorActivityLevelResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl OnlineMotorActivityLevelResponse {
    pub const MSG_ID: u16 = 18856;
}

impl PbMessage for OnlineMotorActivityLevelResponse {
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
pub struct OnlineMotorEndNotify {
    pub EndTime: i64,
    pub Rank: i32,
    pub PlayerId: i32,
}

impl OnlineMotorEndNotify {
    pub const MSG_ID: u16 = 28235;
}

impl PbMessage for OnlineMotorEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.Rank != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rank);
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
            2 => self.EndTime = r.read_varint_i64()?,
            3 => self.Rank = r.read_varint_i32()?,
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorSettleInfo {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub Role: i32,
    pub Ranking: i32,
    pub ConsumeTime: i32,
    pub IsFinished: bool,
    pub ChampionExtra: Option<ChampionExtra>,
    pub SkinId: i32,
}

impl PbMessage for OnlineMotorSettleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Role != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Role);
        }
        if self.Ranking != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ranking);
        }
        if self.ConsumeTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConsumeTime);
        }
        if self.IsFinished {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ChampionExtra {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => self.Role = r.read_varint_i32()?,
            4 => self.Ranking = r.read_varint_i32()?,
            5 => self.ConsumeTime = r.read_varint_i32()?,
            6 => self.IsFinished = r.read_bool()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ChampionExtra
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorSettleNotify {
    pub LevelId: i32,
    pub SettleInfos: Vec<OnlineMotorSettleInfo>,
}

impl OnlineMotorSettleNotify {
    pub const MSG_ID: u16 = 28720;
}

impl PbMessage for OnlineMotorSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.SettleInfos {
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
            10 => self.LevelId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.SettleInfos
                    .push(<OnlineMotorSettleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorStartNotify {
    pub BeginTime: i64,
}

impl OnlineMotorStartNotify {
    pub const MSG_ID: u16 = 20033;
}

impl PbMessage for OnlineMotorStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.BeginTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorTaskUpdate {
    pub LevelTasks: Vec<OnlineMotorTask>,
    pub GlobalTasks: Vec<OnlineMotorTask>,
}

impl PbMessage for OnlineMotorTaskUpdate {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.GlobalTasks {
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
                self.LevelTasks
                    .push(<OnlineMotorTask as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GlobalTasks
                    .push(<OnlineMotorTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OnlineMotorUpdateNotify {
    pub ActivityTasks: Option<OnlineMotorTaskUpdate>,
    pub LevelInfo: Option<OnlineMotorLevelInfo>,
}

impl OnlineMotorUpdateNotify {
    pub const MSG_ID: u16 = 20286;
}

impl PbMessage for OnlineMotorUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTasks {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.LevelInfo {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
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
pub struct OpenSystemBoardData {
    pub BoardId: i32,
}

impl PbMessage for OpenSystemBoardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoardId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BoardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpenSystemBoardResultRequest {
    pub IncId: i32,
    pub PlayerId: i32,
    pub Result: i32,
}

impl OpenSystemBoardResultRequest {
    pub const MSG_ID: u16 = 24936;
}

impl PbMessage for OpenSystemBoardResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Result != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Result);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.IncId = r.read_varint_i32()?,
            5 => self.PlayerId = r.read_varint_i32()?,
            6 => self.Result = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OpenSystemBoardResultResponse {
    pub Code: Option<ErrorCode>,
}

impl OpenSystemBoardResultResponse {
    pub const MSG_ID: u16 = 23490;
}

impl PbMessage for OpenSystemBoardResultResponse {
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
pub struct OperationRestrictionNotify {
    pub IsNull: bool,
    pub Params: Vec<u8>,
}

impl OperationRestrictionNotify {
    pub const MSG_ID: u16 = 15930;
}

impl PbMessage for OperationRestrictionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNull {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Params.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Params);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.IsNull = r.read_bool()?,
            9 => self.Params = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrnamentUnlockNotify {
    pub UnlockOrnamentIds: Vec<i32>,
}

impl OrnamentUnlockNotify {
    pub const MSG_ID: u16 = 27985;
}

impl PbMessage for OrnamentUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockOrnamentIds {
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
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockOrnamentIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OtherJoinSceneNotify {
    pub PlayerInfo: Option<ScenePlayerInformation>,
}

impl OtherJoinSceneNotify {
    pub const MSG_ID: u16 = 22182;
}

impl PbMessage for OtherJoinSceneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct ParkourChallenge {
    pub ChallengeId: i32,
    pub MaxScore: i32,
    pub MinDuration: i32,
    pub TakenScoreIndex: Vec<i32>,
}

impl PbMessage for ParkourChallenge {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.MinDuration != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinDuration);
        }
        for item in &self.TakenScoreIndex {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            3 => self.MaxScore = r.read_varint_i32()?,
            4 => self.MinDuration = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TakenScoreIndex,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeEndNotify {
    pub Duration: i32,
    pub IsComplete: bool,
    pub Score: i32,
    pub ChallengeId: i32,
}

impl ParkourChallengeEndNotify {
    pub const MSG_ID: u16 = 21009;
}

impl PbMessage for ParkourChallengeEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.IsComplete {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            2 => self.Duration = r.read_varint_i32()?,
            4 => self.IsComplete = r.read_bool()?,
            5 => self.Score = r.read_varint_i32()?,
            11 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeOpenNotify {
    pub ChallengeId: i32,
    pub IsOpen: bool,
}

impl ParkourChallengeOpenNotify {
    pub const MSG_ID: u16 = 21077;
}

impl PbMessage for ParkourChallengeOpenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.IsOpen {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            11 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeRequest {}

impl ParkourChallengeRequest {
    pub const MSG_ID: u16 = 21481;
}

impl PbMessage for ParkourChallengeRequest {
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
pub struct ParkourChallengeResponse {
    pub OpenIds: Vec<i32>,
    pub Challenges: Vec<ParkourChallenge>,
}

impl ParkourChallengeResponse {
    pub const MSG_ID: u16 = 28350;
}

impl PbMessage for ParkourChallengeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OpenIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Challenges {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.OpenIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Challenges
                    .push(<ParkourChallenge as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeTakeRequest {
    pub ChallengeId: i32,
    pub ScoreIndex: i32,
}

impl ParkourChallengeTakeRequest {
    pub const MSG_ID: u16 = 24399;
}

impl PbMessage for ParkourChallengeTakeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.ScoreIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScoreIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.ChallengeId = r.read_varint_i32()?,
            10 => self.ScoreIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeTakeResponse {
    pub Rewards: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ParkourChallengeTakeResponse {
    pub const MSG_ID: u16 = 21667;
}

impl PbMessage for ParkourChallengeTakeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Rewards.is_empty() {
            crate::wire::write_map(out, 3, &self.Rewards);
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
            3 => crate::wire::decode_map_entry(r, &mut self.Rewards)?,
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
pub struct ParkourChallengeTransRequest {
    pub ChallengeId: i32,
}

impl ParkourChallengeTransRequest {
    pub const MSG_ID: u16 = 27756;
}

impl PbMessage for ParkourChallengeTransRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParkourChallengeTransResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ParkourChallengeTransResponse {
    pub const MSG_ID: u16 = 16542;
}

impl PbMessage for ParkourChallengeTransResponse {
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
pub struct PartComponentInitRequest {
    pub PartComponent: Option<PartComponentPb>,
    pub EntityId: i64,
}

impl PartComponentInitRequest {
    pub const MSG_ID: u16 = 18536;
}

impl PbMessage for PartComponentInitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PartComponent {
            crate::wire::write_submessage(out, 4, v);
        }
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PartComponent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartComponentInitResponse {}

impl PartComponentInitResponse {
    pub const MSG_ID: u16 = 17051;
}

impl PbMessage for PartComponentInitResponse {
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
pub struct PassPortalRequest {
    pub EntityId: i64,
    pub Location: Option<Vector>,
}

impl PassPortalRequest {
    pub const MSG_ID: u16 = 20215;
}

impl PbMessage for PassPortalRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Location {
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
            4 => self.EntityId = r.read_varint_i64()?,
            9 => {
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
pub struct PassPortalResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PassPortalResponse {
    pub const MSG_ID: u16 = 27969;
}

impl PbMessage for PassPortalResponse {
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
pub struct PayGiftConditionFinishNotify {
    pub Gifts: Vec<PayGiftInfo>,
}

impl PayGiftConditionFinishNotify {
    pub const MSG_ID: u16 = 23030;
}

impl PbMessage for PayGiftConditionFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Gifts {
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
                self.Gifts.push(<PayGiftInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftInfoRequest {
    pub Version: Vec<u8>,
}

impl PayGiftInfoRequest {
    pub const MSG_ID: u16 = 21361;
}

impl PbMessage for PayGiftInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
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
            2 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftInfoResponse {
    pub Infos: Vec<PayGiftInfo>,
    pub Version: Vec<u8>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PayGiftInfoResponse {
    pub const MSG_ID: u16 = 15822;
}

impl PbMessage for PayGiftInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 3, item);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Infos.push(<PayGiftInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.Version = r.read_len_delimited()?.to_vec(),
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
pub struct PayGiftRequest {
    pub Id: i32,
    pub Version: Vec<u8>,
}

impl PayGiftRequest {
    pub const MSG_ID: u16 = 16617;
}

impl PbMessage for PayGiftRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
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
            4 => self.Id = r.read_varint_i32()?,
            15 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CallbackUrl: Vec<u8>,
    pub ReceiptId: Vec<u8>,
}

impl PayGiftResponse {
    pub const MSG_ID: u16 = 21886;
}

impl PbMessage for PayGiftResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.CallbackUrl.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CallbackUrl);
        }
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
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
            10 => self.CallbackUrl = r.read_len_delimited()?.to_vec(),
            14 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayGiftSuccessNotify {
    pub ReceiptId: Vec<u8>,
    pub ItemId: i32,
    pub ItemCount: i32,
    pub Id: i32,
    pub PayGiftInfo: Option<PayGiftInfo>,
}

impl PayGiftSuccessNotify {
    pub const MSG_ID: u16 = 23351;
}

impl PbMessage for PayGiftSuccessNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.PayGiftInfo {
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
            3 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            5 => self.ItemId = r.read_varint_i32()?,
            11 => self.ItemCount = r.read_varint_i32()?,
            14 => self.Id = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PayGiftInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayItemRequest {
    pub Id: i32,
    pub Version: Vec<u8>,
}

impl PayItemRequest {
    pub const MSG_ID: u16 = 16704;
}

impl PbMessage for PayItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
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
            1 => self.Id = r.read_varint_i32()?,
            3 => self.Version = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayItemResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CallbackUrl: Vec<u8>,
    pub ReceiptId: Vec<u8>,
}

impl PayItemResponse {
    pub const MSG_ID: u16 = 22536;
}

impl PbMessage for PayItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.CallbackUrl.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CallbackUrl);
        }
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
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
            6 => self.CallbackUrl = r.read_len_delimited()?.to_vec(),
            14 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayItemSuccessNotify {
    pub ReceiptId: Vec<u8>,
    pub Id: i32,
    pub ItemCount: i32,
    pub ItemId: i32,
}

impl PayItemSuccessNotify {
    pub const MSG_ID: u16 = 25610;
}

impl PbMessage for PayItemSuccessNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
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
            8 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            10 => self.Id = r.read_varint_i32()?,
            11 => self.ItemCount = r.read_varint_i32()?,
            13 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
