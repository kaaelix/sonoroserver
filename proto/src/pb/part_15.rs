use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestCmdGmParams {
    pub FuncName: Vec<u8>,
    pub Args: Vec<Vec<u8>>,
}

impl PbMessage for TestCmdGmParams {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FuncName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FuncName);
        }
        for item in &self.Args {
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
            1 => self.FuncName = r.read_len_delimited()?.to_vec(),
            2 => self.Args.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestCmdNotify {
    pub Complete: Option<TestCmdCompleteParams>,
    pub Request: Option<TestCmdRequestParams>,
    pub Gm: Option<TestCmdGmParams>,
}

impl TestCmdNotify {
    pub const MSG_ID: u16 = 20443;
}

impl PbMessage for TestCmdNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Complete {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Request {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Gm {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Complete
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Request
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Gm
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestCmdRequestParams {
    pub MessageId: i32,
    pub Data: Vec<u8>,
}

impl PbMessage for TestCmdRequestParams {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MessageId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MessageId);
        }
        if !self.Data.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Data);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MessageId = r.read_varint_i32()?,
            2 => self.Data = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestSpawnTemplateEntityPush {
    pub ConfigId: i32,
    pub Rot: Option<Rotator>,
    pub Pos: Option<Vector>,
    pub ExtraRatioConfId: i32,
    pub Id: i64,
    pub Level: i32,
}

impl TestSpawnTemplateEntityPush {
    pub const MSG_ID: u16 = 28168;
}

impl PbMessage for TestSpawnTemplateEntityPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.ExtraRatioConfId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraRatioConfId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            2 => self.ConfigId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ExtraRatioConfId = r.read_varint_i32()?,
            11 => self.Id = r.read_varint_i64()?,
            12 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TetrisFinishRequest {
    pub EntityId: i64,
    pub HostPlayerId: i32,
    pub Score: i32,
}

impl TetrisFinishRequest {
    pub const MSG_ID: u16 = 15095;
}

impl PbMessage for TetrisFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            7 => self.EntityId = r.read_varint_i64()?,
            9 => self.HostPlayerId = r.read_varint_i32()?,
            10 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TetrisFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TetrisFinishResponse {
    pub const MSG_ID: u16 = 16139;
}

impl PbMessage for TetrisFinishResponse {
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
pub struct TetrisUpdateRequest {
    pub ActivityId: i32,
    pub TetrisLevelInfo: Option<TetrisLevelInfo>,
}

impl TetrisUpdateRequest {
    pub const MSG_ID: u16 = 25007;
}

impl PbMessage for TetrisUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.TetrisLevelInfo {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.TetrisLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TetrisUpdateResponse {
    pub Items: BTreeMap<i32, i32>,
    pub ErrCode: Option<ErrorCode>,
}

impl TetrisUpdateResponse {
    pub const MSG_ID: u16 = 21709;
}

impl PbMessage for TetrisUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Items.is_empty() {
            crate::wire::write_map(out, 2, &self.Items);
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
            2 => crate::wire::decode_map_entry(r, &mut self.Items)?,
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
pub struct ThemeCeleWuWuTaskNotify {
    pub Task: Option<ConditionTask>,
}

impl ThemeCeleWuWuTaskNotify {
    pub const MSG_ID: u16 = 29613;
}

impl PbMessage for ThemeCeleWuWuTaskNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
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
pub struct ThemePersonalScoreRequest {
    pub Ids: Vec<i32>,
}

impl ThemePersonalScoreRequest {
    pub const MSG_ID: u16 = 20050;
}

impl PbMessage for ThemePersonalScoreRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThemePersonalScoreResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThemePersonalScoreResponse {
    pub const MSG_ID: u16 = 18903;
}

impl PbMessage for ThemePersonalScoreResponse {
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
pub struct ThemeWorldScoreRequest {
    pub Ids: Vec<i32>,
}

impl ThemeWorldScoreRequest {
    pub const MSG_ID: u16 = 27224;
}

impl PbMessage for ThemeWorldScoreRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThemeWorldScoreResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThemeWorldScoreResponse {
    pub const MSG_ID: u16 = 29997;
}

impl PbMessage for ThemeWorldScoreResponse {
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
pub struct ThirdPartySharedNotify {
    pub SharedIds: Vec<i32>,
}

impl ThirdPartySharedNotify {
    pub const MSG_ID: u16 = 17110;
}

impl PbMessage for ThirdPartySharedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SharedIds {
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SharedIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThroughTrainFinishViewRequest {
    pub ActivityId: i32,
}

impl ThroughTrainFinishViewRequest {
    pub const MSG_ID: u16 = 16804;
}

impl PbMessage for ThroughTrainFinishViewRequest {
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
pub struct ThroughTrainFinishViewResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThroughTrainFinishViewResponse {
    pub const MSG_ID: u16 = 19302;
}

impl PbMessage for ThroughTrainFinishViewResponse {
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
pub struct ThroughTrainForceRemindNotify {
    pub ActivityId: i32,
}

impl ThroughTrainForceRemindNotify {
    pub const MSG_ID: u16 = 29505;
}

impl PbMessage for ThroughTrainForceRemindNotify {
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
pub struct ThroughTrainRequest {
    pub ActivityId: i32,
}

impl ThroughTrainRequest {
    pub const MSG_ID: u16 = 26589;
}

impl PbMessage for ThroughTrainRequest {
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
pub struct ThroughTrainResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThroughTrainResponse {
    pub const MSG_ID: u16 = 29273;
}

impl PbMessage for ThroughTrainResponse {
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
pub struct ThrowDamageNotify {
    pub EntityId: i64,
    pub Durability: i32,
}

impl ThrowDamageNotify {
    pub const MSG_ID: u16 = 18961;
}

impl PbMessage for ThrowDamageNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Durability != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Durability);
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
            2 => self.Durability = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThrowDamageRecoveryNotify {
    pub Durability: i32,
    pub EntityId: i64,
}

impl ThrowDamageRecoveryNotify {
    pub const MSG_ID: u16 = 29960;
}

impl PbMessage for ThrowDamageRecoveryNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Durability != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Durability);
        }
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
            4 => self.Durability = r.read_varint_i32()?,
            9 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThrowDamageRequest {
    pub EntityId: i64,
    pub CalculateId: i64,
}

impl ThrowDamageRequest {
    pub const MSG_ID: u16 = 25936;
}

impl PbMessage for ThrowDamageRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.CalculateId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CalculateId);
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
            7 => self.CalculateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ThrowDamageResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ThrowDamageResponse {
    pub const MSG_ID: u16 = 26400;
}

impl PbMessage for ThrowDamageResponse {
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
pub struct TimeOccupationNotify {
    pub HandleType: Option<TimeHandleType>,
}

impl TimeOccupationNotify {
    pub const MSG_ID: u16 = 17295;
}

impl PbMessage for TimeOccupationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HandleType {
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
                self.HandleType = Some(
                    <TimeHandleType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TimePointActivityBatchRewardRequest {
    pub Ids: Vec<i32>,
    pub ActivityId: i32,
}

impl TimePointActivityBatchRewardRequest {
    pub const MSG_ID: u16 = 28173;
}

impl PbMessage for TimePointActivityBatchRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimePointActivityBatchRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TimePointActivityBatchRewardResponse {
    pub const MSG_ID: u16 = 15156;
}

impl PbMessage for TimePointActivityBatchRewardResponse {
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
pub struct TimePointActivityRewardRequest {
    pub Id: i32,
}

impl TimePointActivityRewardRequest {
    pub const MSG_ID: u16 = 25635;
}

impl PbMessage for TimePointActivityRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimePointActivityRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TimePointActivityRewardResponse {
    pub const MSG_ID: u16 = 18317;
}

impl PbMessage for TimePointActivityRewardResponse {
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
pub struct TimePointRewardActivityConditionNotify {
    pub Data: Option<TimePointRewardActivityData>,
    pub ActivityId: i32,
}

impl TimePointRewardActivityConditionNotify {
    pub const MSG_ID: u16 = 21826;
}

impl PbMessage for TimePointRewardActivityConditionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Data {
            crate::wire::write_submessage(out, 9, v);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimePointRewardActivityData {
    pub Rewards: Vec<OneTimePointRewardActivityData>,
}

impl PbMessage for TimePointRewardActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Rewards {
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
                self.Rewards
                    .push(<OneTimePointRewardActivityData as PbMessage>::decode(
                        bytes,
                    )?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeResetNotify {
    pub ResetId: i32,
}

impl TimeResetNotify {
    pub const MSG_ID: u16 = 19363;
}

impl PbMessage for TimeResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResetId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResetId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.ResetId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeScheduleIdleNotify {
    pub EntityIds: Vec<i64>,
}

impl TimeScheduleIdleNotify {
    pub const MSG_ID: u16 = 18933;
}

impl PbMessage for TimeScheduleIdleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeScheduleTriggerNotify {
    pub TimeScheduleId: i32,
    pub EntityIds: Vec<i64>,
}

impl TimeScheduleTriggerNotify {
    pub const MSG_ID: u16 = 26967;
}

impl PbMessage for TimeScheduleTriggerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeScheduleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeScheduleId);
        }
        for item in &self.EntityIds {
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
            10 => self.TimeScheduleId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimelineTraceControlRequest {
    pub Forward: bool,
    pub EntityId: i64,
}

impl TimelineTraceControlRequest {
    pub const MSG_ID: u16 = 26029;
}

impl PbMessage for TimelineTraceControlRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Forward {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            7 => self.Forward = r.read_bool()?,
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimelineTraceControlResponse {
    pub ControlPoint: i32,
    pub Code: Option<ErrorCode>,
}

impl TimelineTraceControlResponse {
    pub const MSG_ID: u16 = 23257;
}

impl PbMessage for TimelineTraceControlResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ControlPoint != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ControlPoint);
        }
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
            8 => self.ControlPoint = r.read_varint_i32()?,
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
pub struct TimelineTraceExitRequest {
    pub EntityId: i64,
}

impl TimelineTraceExitRequest {
    pub const MSG_ID: u16 = 28107;
}

impl PbMessage for TimelineTraceExitRequest {
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
pub struct TimelineTraceExitResponse {
    pub Code: Option<ErrorCode>,
}

impl TimelineTraceExitResponse {
    pub const MSG_ID: u16 = 16487;
}

impl PbMessage for TimelineTraceExitResponse {
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
pub struct TimelineTraceStartRequest {
    pub EntityId: i64,
    pub Index: i32,
}

impl TimelineTraceStartRequest {
    pub const MSG_ID: u16 = 29145;
}

impl PbMessage for TimelineTraceStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            5 => self.EntityId = r.read_varint_i64()?,
            13 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimelineTraceStartResponse {
    pub Index: i32,
    pub ControlPoint: i32,
    pub EntityIds: Vec<i64>,
    pub PointDatas: Vec<ControlPointData>,
    pub Code: Option<ErrorCode>,
}

impl TimelineTraceStartResponse {
    pub const MSG_ID: u16 = 21855;
}

impl PbMessage for TimelineTraceStartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.ControlPoint != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ControlPoint);
        }
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.PointDatas {
            crate::wire::write_submessage(out, 7, item);
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
            2 => self.Index = r.read_varint_i32()?,
            4 => self.ControlPoint = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PointDatas
                    .push(<ControlPointData as PbMessage>::decode(bytes)?);
            }
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
pub struct TokenAwardView {
    pub SurvivorsOption: Option<SurvivorsOption>,
}

impl PbMessage for TokenAwardView {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SurvivorsOption {
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
                self.SurvivorsOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TokenSelectView {
    pub SurvivorsOption: Option<SurvivorsOption>,
}

impl PbMessage for TokenSelectView {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SurvivorsOption {
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
                self.SurvivorsOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TopPhantomEquipGroupRequest {
    pub RoleId: i32,
    pub Index: i32,
}

impl TopPhantomEquipGroupRequest {
    pub const MSG_ID: u16 = 19922;
}

impl PbMessage for TopPhantomEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            1 => self.RoleId = r.read_varint_i32()?,
            4 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TopPhantomEquipGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
}

impl TopPhantomEquipGroupResponse {
    pub const MSG_ID: u16 = 24824;
}

impl PbMessage for TopPhantomEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomEquipGroupInfos {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpActivityInfoNotify {
    pub TotalTopUpActivityInfo: Option<TotalTopUpActivityInfo>,
}

impl TotalTopUpActivityInfoNotify {
    pub const MSG_ID: u16 = 24740;
}

impl PbMessage for TotalTopUpActivityInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TotalTopUpActivityInfo {
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
                self.TotalTopUpActivityInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpRewardNotify {
    pub TotalTopUpRewardInfo: Vec<TotalTopUpRewardInfo>,
}

impl TotalTopUpRewardNotify {
    pub const MSG_ID: u16 = 17958;
}

impl PbMessage for TotalTopUpRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TotalTopUpRewardInfo {
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
                self.TotalTopUpRewardInfo
                    .push(<TotalTopUpRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpRewardRequest {
    pub TotalTopUpRewardId: i32,
    pub SelectedId: Vec<i32>,
}

impl TotalTopUpRewardRequest {
    pub const MSG_ID: u16 = 27942;
}

impl PbMessage for TotalTopUpRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalTopUpRewardId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalTopUpRewardId);
        }
        for item in &self.SelectedId {
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
            8 => self.TotalTopUpRewardId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectedId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TotalTopUpRewardResponse {
    pub const MSG_ID: u16 = 22019;
}

impl PbMessage for TotalTopUpRewardResponse {
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
pub struct TotalTopUpScoreInfo {
    pub Id: i32,
    pub GoodsId: i32,
    pub Type: i32,
    pub Score: i32,
}

impl PbMessage for TotalTopUpScoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.GoodsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoodsId);
        }
        if self.Type != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.GoodsId = r.read_varint_i32()?,
            3 => self.Type = r.read_varint_i32()?,
            4 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TotalTopUpScoreInfoRequest {
    pub ActivityId: i32,
}

impl TotalTopUpScoreInfoRequest {
    pub const MSG_ID: u16 = 18368;
}

impl PbMessage for TotalTopUpScoreInfoRequest {
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
pub struct TotalTopUpScoreInfoResponse {
    pub Infos: Vec<TotalTopUpScoreInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl TotalTopUpScoreInfoResponse {
    pub const MSG_ID: u16 = 28081;
}

impl PbMessage for TotalTopUpScoreInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 1, item);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<TotalTopUpScoreInfo as PbMessage>::decode(bytes)?);
            }
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
pub struct TowerApplyFloorDataRequest {
    pub Apply: bool,
}

impl TowerApplyFloorDataRequest {
    pub const MSG_ID: u16 = 21733;
}

impl PbMessage for TowerApplyFloorDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Apply {
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
            3 => self.Apply = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerApplyFloorDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub NeedUpdateSeason: bool,
}

impl TowerApplyFloorDataResponse {
    pub const MSG_ID: u16 = 18513;
}

impl PbMessage for TowerApplyFloorDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NeedUpdateSeason {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.NeedUpdateSeason = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceActivityInfo {
    pub InstanceInfos: Vec<TowerDefenceInstanceInfo>,
    pub RewardedScoreIds: Vec<i32>,
    pub TotalScore: i32,
    pub ShowName: bool,
}

impl PbMessage for TowerDefenceActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstanceInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardedScoreIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
        if self.ShowName {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.InstanceInfos
                    .push(<TowerDefenceInstanceInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedScoreIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => self.TotalScore = r.read_varint_i32()?,
            4 => self.ShowName = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceBattleScoreInfo {
    pub Id: i32,
    pub Score: i32,
    pub RankScore: i32,
}

impl PbMessage for TowerDefenceBattleScoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.RankScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankScore);
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
            2 => self.Score = r.read_varint_i32()?,
            3 => self.RankScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceChallengeQuitRequest {}

impl TowerDefenceChallengeQuitRequest {
    pub const MSG_ID: u16 = 27718;
}

impl PbMessage for TowerDefenceChallengeQuitRequest {
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
pub struct TowerDefenceChallengeQuitResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Score: i32,
}

impl TowerDefenceChallengeQuitResponse {
    pub const MSG_ID: u16 = 16362;
}

impl PbMessage for TowerDefenceChallengeQuitResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceEndNotify {
    pub MaxScore: i32,
    pub Score: i32,
    pub PassTime: i32,
    pub InstId: i32,
    pub MinPassTime: i32,
    pub Success: bool,
    pub BattleInfo: Vec<TowerDefenceRoleScoreInfo>,
}

impl TowerDefenceEndNotify {
    pub const MSG_ID: u16 = 21135;
}

impl PbMessage for TowerDefenceEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.MinPassTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinPassTime);
        }
        if self.Success {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.BattleInfo {
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
            5 => self.MaxScore = r.read_varint_i32()?,
            6 => self.Score = r.read_varint_i32()?,
            8 => self.PassTime = r.read_varint_i32()?,
            10 => self.InstId = r.read_varint_i32()?,
            11 => self.MinPassTime = r.read_varint_i32()?,
            13 => self.Success = r.read_bool()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.BattleInfo
                    .push(<TowerDefenceRoleScoreInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceEnterInstNotify {
    pub RoleList: Vec<TowerDefenceRoleInfo>,
}

impl TowerDefenceEnterInstNotify {
    pub const MSG_ID: u16 = 29765;
}

impl PbMessage for TowerDefenceEnterInstNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
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
                self.RoleList
                    .push(<TowerDefenceRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceInfoUpdateNotify {
    pub ActivityInfo: Option<TowerDefenceActivityInfo>,
}

impl TowerDefenceInfoUpdateNotify {
    pub const MSG_ID: u16 = 20433;
}

impl PbMessage for TowerDefenceInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityInfo {
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
            14 => {
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
pub struct TowerDefenceInstanceInfoUpdateNotify {
    pub InstanceInfos: Vec<TowerDefenceInstanceInfo>,
}

impl TowerDefenceInstanceInfoUpdateNotify {
    pub const MSG_ID: u16 = 18881;
}

impl PbMessage for TowerDefenceInstanceInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstanceInfos {
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
                self.InstanceInfos
                    .push(<TowerDefenceInstanceInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceInstanceRewardRequest {
    pub Ids: Vec<i32>,
}

impl TowerDefenceInstanceRewardRequest {
    pub const MSG_ID: u16 = 23941;
}

impl PbMessage for TowerDefenceInstanceRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceInstanceRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerDefenceInstanceRewardResponse {
    pub const MSG_ID: u16 = 20681;
}

impl PbMessage for TowerDefenceInstanceRewardResponse {
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
pub struct TowerDefencePassInfo {
    pub PlayerPassInfo: Vec<TowerDefencePassPlayerInfo>,
    pub PassTime: i32,
    pub Score: i32,
    pub OwnerId: i32,
}

impl PbMessage for TowerDefencePassInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerPassInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerId);
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
                self.PlayerPassInfo
                    .push(<TowerDefencePassPlayerInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.PassTime = r.read_varint_i32()?,
            3 => self.Score = r.read_varint_i32()?,
            4 => self.OwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefencePassPlayerInfo {
    pub RoleList: Vec<TowerDefenceRolePassInfo>,
    pub Name: Vec<u8>,
    pub PlayerId: i32,
    pub HeadId: i32,
}

impl PbMessage for TowerDefencePassPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
            crate::wire::write_submessage(out, 1, item);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
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
                    .push(<TowerDefenceRolePassInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            4 => self.PlayerId = r.read_varint_i32()?,
            5 => self.HeadId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefencePhantomInfo {
    pub Id: i32,
    pub Level: i32,
    pub Exp: i32,
}

impl PbMessage for TowerDefencePhantomInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
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
            3 => self.Exp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefencePhantomInfoUpdateNotify {
    pub Phantoms: Vec<TowerDefencePhantomInfo>,
}

impl TowerDefencePhantomInfoUpdateNotify {
    pub const MSG_ID: u16 = 19510;
}

impl PbMessage for TowerDefencePhantomInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Phantoms {
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
                self.Phantoms
                    .push(<TowerDefencePhantomInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefencePlayerReviveNotify {
    pub ReviveTime: i64,
    pub PlayerId: i32,
}

impl TowerDefencePlayerReviveNotify {
    pub const MSG_ID: u16 = 29503;
}

impl PbMessage for TowerDefencePlayerReviveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReviveTime);
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
            2 => self.ReviveTime = r.read_varint_i64()?,
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRandListRequest {
    pub Id: i32,
}

impl TowerDefenceRandListRequest {
    pub const MSG_ID: u16 = 16075;
}

impl PbMessage for TowerDefenceRandListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRandListResponse {
    pub ShowName: bool,
    pub FriendPassData: Vec<OneTowerDefencePassInfo>,
    pub SelfPassData: Vec<OneTowerDefencePassInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerDefenceRandListResponse {
    pub const MSG_ID: u16 = 29510;
}

impl PbMessage for TowerDefenceRandListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowName {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.FriendPassData {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.SelfPassData {
            crate::wire::write_submessage(out, 9, item);
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
            3 => self.ShowName = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FriendPassData
                    .push(<OneTowerDefencePassInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.SelfPassData
                    .push(<OneTowerDefencePassInfo as PbMessage>::decode(bytes)?);
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
pub struct TowerDefenceRankShowNameRequest {
    pub ShowName: bool,
}

impl TowerDefenceRankShowNameRequest {
    pub const MSG_ID: u16 = 16285;
}

impl PbMessage for TowerDefenceRankShowNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowName {
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
            12 => self.ShowName = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRankShowNameResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerDefenceRankShowNameResponse {
    pub const MSG_ID: u16 = 17528;
}

impl PbMessage for TowerDefenceRankShowNameResponse {
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
pub struct TowerDefenceRoleInfo {
    pub RoleId: i32,
    pub Phantom: Option<TowerDefencePhantomInfo>,
}

impl PbMessage for TowerDefenceRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.Phantom {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Phantom
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRolePassInfo {
    pub RoleSkinId: i32,
    pub RoleLevel: i32,
    pub PhantomId: i32,
    pub Title: i32,
    pub ScoreList: Vec<TowerDefenceBattleScoreInfo>,
}

impl PbMessage for TowerDefenceRolePassInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.PhantomId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomId);
        }
        if self.Title != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Title);
        }
        for item in &self.ScoreList {
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
            3 => self.PhantomId = r.read_varint_i32()?,
            4 => self.Title = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ScoreList
                    .push(<TowerDefenceBattleScoreInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRoleReviveNotify {
    pub PlayerId: i32,
    pub ReviveTime: i64,
    pub RoleId: i32,
}

impl TowerDefenceRoleReviveNotify {
    pub const MSG_ID: u16 = 18859;
}

impl PbMessage for TowerDefenceRoleReviveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.ReviveTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ReviveTime);
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
            2 => self.PlayerId = r.read_varint_i32()?,
            8 => self.ReviveTime = r.read_varint_i64()?,
            9 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceRoleScoreInfo {
    pub ScoreList: Vec<TowerDefenceBattleScoreInfo>,
    pub RoleId: i32,
    pub PlayerId: i32,
    pub Title: i32,
}

impl PbMessage for TowerDefenceRoleScoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Title != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Title);
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
                self.ScoreList
                    .push(<TowerDefenceBattleScoreInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.RoleId = r.read_varint_i32()?,
            3 => self.PlayerId = r.read_varint_i32()?,
            5 => self.Title = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceScoreRewardRequest {
    pub Ids: Vec<i32>,
}

impl TowerDefenceScoreRewardRequest {
    pub const MSG_ID: u16 = 18725;
}

impl PbMessage for TowerDefenceScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerDefenceScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerDefenceScoreRewardResponse {
    pub const MSG_ID: u16 = 15411;
}

impl PbMessage for TowerDefenceScoreRewardResponse {
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
pub struct TowerDefenceSelfPassRequest {
    pub Id: i32,
}

impl TowerDefenceSelfPassRequest {
    pub const MSG_ID: u16 = 15513;
}

impl PbMessage for TowerDefenceSelfPassRequest {
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
pub struct TowerDefenceSelfPassResponse {
    pub SelfPassData: Vec<OneTowerDefencePassInfo>,
}

impl TowerDefenceSelfPassResponse {
    pub const MSG_ID: u16 = 28825;
}

impl PbMessage for TowerDefenceSelfPassResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelfPassData {
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
                self.SelfPassData
                    .push(<OneTowerDefencePassInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerFormationRecommendRequest {
    pub TowerConfigId: i32,
}

impl TowerFormationRecommendRequest {
    pub const MSG_ID: u16 = 16303;
}

impl PbMessage for TowerFormationRecommendRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.TowerConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerFormationRecommendResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TowerConfigId: i32,
    pub Formations: Vec<TowerRecommendFormation>,
    pub NeedUpdateSeason: bool,
}

impl TowerFormationRecommendResponse {
    pub const MSG_ID: u16 = 26594;
}

impl PbMessage for TowerFormationRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TowerConfigId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerConfigId);
        }
        for item in &self.Formations {
            crate::wire::write_submessage(out, 13, item);
        }
        if self.NeedUpdateSeason {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.TowerConfigId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Formations
                    .push(<TowerRecommendFormation as PbMessage>::decode(bytes)?);
            }
            15 => self.NeedUpdateSeason = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerGuideActivityInfoRequest {}

impl TowerGuideActivityInfoRequest {
    pub const MSG_ID: u16 = 19610;
}

impl PbMessage for TowerGuideActivityInfoRequest {
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
pub struct TowerGuideActivityInfoResponse {
    pub TowerGuideId: Vec<i32>,
}

impl TowerGuideActivityInfoResponse {
    pub const MSG_ID: u16 = 23078;
}

impl PbMessage for TowerGuideActivityInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TowerGuideId {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TowerGuideId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerGuideActivityRewardRequest {
    pub TowerGuideId: i32,
}

impl TowerGuideActivityRewardRequest {
    pub const MSG_ID: u16 = 16778;
}

impl PbMessage for TowerGuideActivityRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TowerGuideId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerGuideId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.TowerGuideId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerGuideActivityRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TowerGuideActivityRewardResponse {
    pub const MSG_ID: u16 = 22907;
}

impl PbMessage for TowerGuideActivityRewardResponse {
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
pub struct TowerRecommendFormation {
    pub Formation: Vec<TowerRecommendRole>,
    pub Usage: i32,
}

impl PbMessage for TowerRecommendFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Formation {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.Usage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Usage);
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
                self.Formation
                    .push(<TowerRecommendRole as PbMessage>::decode(bytes)?);
            }
            2 => self.Usage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TowerRecommendRole {
    pub RoleId: i32,
    pub LeaveSkillId: i32,
    pub Level: i32,
    pub SkillBranchId: i32,
}

impl PbMessage for TowerRecommendRole {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.LeaveSkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeaveSkillId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranchId);
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
            2 => self.LeaveSkillId = r.read_varint_i32()?,
            3 => self.Level = r.read_varint_i32()?,
            4 => self.SkillBranchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceEntityPosRequest {
    pub DungeonId: i32,
    pub ConfigId: i32,
}

impl TraceEntityPosRequest {
    pub const MSG_ID: u16 = 21153;
}

impl PbMessage for TraceEntityPosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DungeonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DungeonId);
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
            3 => self.DungeonId = r.read_varint_i32()?,
            9 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceEntityPosResponse {
    pub Pos: Option<Vector>,
    pub ErrorCode: Option<ErrorCode>,
}

impl TraceEntityPosResponse {
    pub const MSG_ID: u16 = 16641;
}

impl PbMessage for TraceEntityPosResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 5, v);
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
                self.Pos
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
pub struct TraceMoonPhaseData {
    pub Reward: bool,
}

impl PbMessage for TraceMoonPhaseData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Reward {
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
            1 => self.Reward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceMoonPhaseRewardRequest {
    pub ActivityId: i32,
}

impl TraceMoonPhaseRewardRequest {
    pub const MSG_ID: u16 = 23777;
}

impl PbMessage for TraceMoonPhaseRewardRequest {
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
pub struct TraceMoonPhaseRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TraceMoonPhaseRewardResponse {
    pub const MSG_ID: u16 = 17223;
}

impl PbMessage for TraceMoonPhaseRewardResponse {
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
pub struct TraceRecallQuestNotify {
    pub RecallQuestId: i32,
}

impl TraceRecallQuestNotify {
    pub const MSG_ID: u16 = 22097;
}

impl PbMessage for TraceRecallQuestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.RecallQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceRecallQuestRequest {
    pub RecallQuestId: i32,
    pub TraceType: i32,
    pub Operate: i32,
}

impl TraceRecallQuestRequest {
    pub const MSG_ID: u16 = 25878;
}

impl PbMessage for TraceRecallQuestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallQuestId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallQuestId);
        }
        if self.TraceType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceType);
        }
        if self.Operate != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Operate);
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
            7 => self.TraceType = r.read_varint_i32()?,
            11 => self.Operate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TraceRecallQuestResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl TraceRecallQuestResponse {
    pub const MSG_ID: u16 = 28806;
}

impl PbMessage for TraceRecallQuestResponse {
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
pub struct TrackMoonActivityRewardRequest {
    pub Id: i32,
    pub ActivityId: i32,
}

impl TrackMoonActivityRewardRequest {
    pub const MSG_ID: u16 = 28834;
}

impl PbMessage for TrackMoonActivityRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            10 => self.Id = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonActivityRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrackMoonActivityRewardResponse {
    pub const MSG_ID: u16 = 24303;
}

impl PbMessage for TrackMoonActivityRewardResponse {
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
pub struct TrackMoonBuildFlowRequest {
    pub BuildingId: i32,
}

impl TrackMoonBuildFlowRequest {
    pub const MSG_ID: u16 = 21892;
}

impl PbMessage for TrackMoonBuildFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuildingId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.BuildingId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonBuildFlowResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrackMoonBuildFlowResponse {
    pub const MSG_ID: u16 = 22345;
}

impl PbMessage for TrackMoonBuildFlowResponse {
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
pub struct TrackMoonBuildingDataRequest {}

impl TrackMoonBuildingDataRequest {
    pub const MSG_ID: u16 = 26292;
}

impl PbMessage for TrackMoonBuildingDataRequest {
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
pub struct TrackMoonBuildingDataResponse {
    pub BuildingInfos: Vec<BuildingInfo>,
}

impl TrackMoonBuildingDataResponse {
    pub const MSG_ID: u16 = 15600;
}

impl PbMessage for TrackMoonBuildingDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BuildingInfos {
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
                self.BuildingInfos
                    .push(<BuildingInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonConditionNotify {
    pub TrackMoonUpdateInfo: Option<TrackMoonUpdateInfo>,
    pub TrackMoonConditionType: Option<TrackMoonConditionType>,
}

impl TrackMoonConditionNotify {
    pub const MSG_ID: u16 = 23338;
}

impl PbMessage for TrackMoonConditionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TrackMoonUpdateInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.TrackMoonConditionType {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TrackMoonUpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                self.TrackMoonConditionType = Some(
                    <TrackMoonConditionType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TrackMoonEntrustDataRequest {}

impl TrackMoonEntrustDataRequest {
    pub const MSG_ID: u16 = 28535;
}

impl PbMessage for TrackMoonEntrustDataRequest {
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
pub struct TrackMoonEntrustDataResponse {
    pub EntrustInfos: Vec<EntrustInfo>,
}

impl TrackMoonEntrustDataResponse {
    pub const MSG_ID: u16 = 19470;
}

impl PbMessage for TrackMoonEntrustDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntrustInfos {
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
                self.EntrustInfos
                    .push(<EntrustInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonIllustratedNotify {
    pub Ids: Vec<i32>,
}

impl TrackMoonIllustratedNotify {
    pub const MSG_ID: u16 = 28408;
}

impl PbMessage for TrackMoonIllustratedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonIllustratedRequest {
    pub Ids: Vec<i32>,
}

impl TrackMoonIllustratedRequest {
    pub const MSG_ID: u16 = 21304;
}

impl PbMessage for TrackMoonIllustratedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonIllustratedResponse {
    pub Ids: Vec<i32>,
}

impl TrackMoonIllustratedResponse {
    pub const MSG_ID: u16 = 24278;
}

impl PbMessage for TrackMoonIllustratedResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonRoleDataRequest {}

impl TrackMoonRoleDataRequest {
    pub const MSG_ID: u16 = 20217;
}

impl PbMessage for TrackMoonRoleDataRequest {
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
pub struct TrackMoonRoleDataResponse {
    pub EntrustRoleInfos: Vec<EntrustRoleInfo>,
}

impl TrackMoonRoleDataResponse {
    pub const MSG_ID: u16 = 20611;
}

impl PbMessage for TrackMoonRoleDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntrustRoleInfos {
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
                self.EntrustRoleInfos
                    .push(<EntrustRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonRolePropChangeNotify {
    pub RoleProps: Vec<EntrustRoleInfo>,
}

impl TrackMoonRolePropChangeNotify {
    pub const MSG_ID: u16 = 27561;
}

impl PbMessage for TrackMoonRolePropChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleProps {
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
                self.RoleProps
                    .push(<EntrustRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonSettleRequest {}

impl TrackMoonSettleRequest {
    pub const MSG_ID: u16 = 15206;
}

impl PbMessage for TrackMoonSettleRequest {
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
pub struct TrackMoonSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MostLikeRole: Option<SettleInfo>,
    pub EntrustDoMost: Option<SettleInfo>,
    pub EntrustDo: i32,
    pub AlwaysRoleId: Option<SettleInfo>,
    pub WishTotal: i32,
    pub GoldTotal: i32,
    pub PopularTotal: i32,
}

impl TrackMoonSettleResponse {
    pub const MSG_ID: u16 = 27600;
}

impl PbMessage for TrackMoonSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MostLikeRole {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.EntrustDoMost {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.EntrustDo != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustDo);
        }
        if let Some(v) = &self.AlwaysRoleId {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.WishTotal != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WishTotal);
        }
        if self.GoldTotal != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoldTotal);
        }
        if self.PopularTotal != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PopularTotal);
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
                self.MostLikeRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.EntrustDoMost
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.EntrustDo = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AlwaysRoleId
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.WishTotal = r.read_varint_i32()?,
            11 => self.GoldTotal = r.read_varint_i32()?,
            15 => self.PopularTotal = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonTarget {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub Status: Option<TrackMoonTargetState>,
}

impl PbMessage for TrackMoonTarget {
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
        if let Some(v) = &self.Status {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Current = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
            4 => {
                self.Status = Some(
                    <TrackMoonTargetState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TrackMoonTargetRewardRequest {
    pub Id: i32,
}

impl TrackMoonTargetRewardRequest {
    pub const MSG_ID: u16 = 19837;
}

impl PbMessage for TrackMoonTargetRewardRequest {
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
pub struct TrackMoonTargetRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrackMoonTargetRewardResponse {
    pub const MSG_ID: u16 = 17100;
}

impl PbMessage for TrackMoonTargetRewardResponse {
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
pub struct TrackMoonTargetUpdateNotify {
    pub TrackMoonTargets: Vec<TrackMoonTarget>,
}

impl TrackMoonTargetUpdateNotify {
    pub const MSG_ID: u16 = 20094;
}

impl PbMessage for TrackMoonTargetUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrackMoonTargets {
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
                self.TrackMoonTargets
                    .push(<TrackMoonTarget as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonTokenCoinNotify {
    pub TokenCoin: i32,
}

impl TrackMoonTokenCoinNotify {
    pub const MSG_ID: u16 = 19882;
}

impl PbMessage for TrackMoonTokenCoinNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TokenCoin != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenCoin);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.TokenCoin = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrackMoonUpdateInfo {
    pub EntrustInfo: Option<EntrustInfo>,
    pub EntrustRoleInfo: Option<EntrustRoleInfo>,
    pub BuildingInfo: Option<BuildingInfo>,
}

impl PbMessage for TrackMoonUpdateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntrustInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.EntrustRoleInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.BuildingInfo {
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
                let bytes = r.read_len_delimited()?;
                self.EntrustInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.EntrustRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BuildingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrampleActiveCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for TrampleActiveCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
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
                self.EntityCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrampleGearTransitionNotify {
    pub TrampleGearTransition: Option<TrampleGearTransition>,
    pub TrampleGearEntiyId: i64,
}

impl TrampleGearTransitionNotify {
    pub const MSG_ID: u16 = 28029;
}

impl PbMessage for TrampleGearTransitionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TrampleGearTransition {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TrampleGearEntiyId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TrampleGearEntiyId);
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
                self.TrampleGearTransition = Some(
                    <TrampleGearTransition as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.TrampleGearEntiyId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseActivityInfo {
    pub UnlockTechNodes: Vec<i32>,
    pub SpecialRewards: Vec<TrapDefenseRewardInfo>,
    pub Rewards: Vec<TrapDefenseRewardInfo>,
    pub Auxiliaries: Vec<TrapDefenseAuxiliaryInfo>,
    pub Buildings: Vec<TrapDefenseBuildingInfo>,
    pub Challenges: Vec<TrapDefenseChallengeInfo>,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
    pub UnlockBds: Vec<i32>,
    pub TotalTechPoint: i32,
    pub LeftTechPoint: i32,
    pub LeftLevelUpPoint: i32,
    pub TotalLevelUpPoint: i32,
    pub UnlockBdGroups: Vec<i32>,
}

impl PbMessage for TrapDefenseActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockTechNodes {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialRewards {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Rewards {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.Auxiliaries {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.Buildings {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.Challenges {
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
        for item in &self.UnlockBds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TotalTechPoint != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalTechPoint);
        }
        if self.LeftTechPoint != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeftTechPoint);
        }
        if self.LeftLevelUpPoint != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeftLevelUpPoint);
        }
        if self.TotalLevelUpPoint != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalLevelUpPoint);
        }
        for item in &self.UnlockBdGroups {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockTechNodes,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SpecialRewards
                    .push(<TrapDefenseRewardInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Rewards
                    .push(<TrapDefenseRewardInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Auxiliaries
                    .push(<TrapDefenseAuxiliaryInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Buildings
                    .push(<TrapDefenseBuildingInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Challenges
                    .push(<TrapDefenseChallengeInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.LimitBeginTime = r.read_varint_i64()?,
            8 => self.LimitEndTime = r.read_varint_i64()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.TotalTechPoint = r.read_varint_i32()?,
            11 => self.LeftTechPoint = r.read_varint_i32()?,
            12 => self.LeftLevelUpPoint = r.read_varint_i32()?,
            13 => self.TotalLevelUpPoint = r.read_varint_i32()?,
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBdGroups, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryBranchRequest {
    pub Id: i32,
    pub Branch: i32,
}

impl TrapDefenseAuxiliaryBranchRequest {
    pub const MSG_ID: u16 = 16263;
}

impl PbMessage for TrapDefenseAuxiliaryBranchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
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
            14 => self.Branch = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryBranchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseAuxiliaryBranchResponse {
    pub const MSG_ID: u16 = 27327;
}

impl PbMessage for TrapDefenseAuxiliaryBranchResponse {
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
pub struct TrapDefenseAuxiliaryInfo {
    pub Id: i32,
    pub Level: i32,
    pub Branch: i32,
    pub CurMaxLevel: i32,
}

impl PbMessage for TrapDefenseAuxiliaryInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
        }
        if self.CurMaxLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurMaxLevel);
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
            3 => self.Branch = r.read_varint_i32()?,
            4 => self.CurMaxLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryLevelUpRequest {
    pub Id: i32,
}

impl TrapDefenseAuxiliaryLevelUpRequest {
    pub const MSG_ID: u16 = 20936;
}

impl PbMessage for TrapDefenseAuxiliaryLevelUpRequest {
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
pub struct TrapDefenseAuxiliaryLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseAuxiliaryLevelUpResponse {
    pub const MSG_ID: u16 = 24981;
}

impl PbMessage for TrapDefenseAuxiliaryLevelUpResponse {
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
pub struct TrapDefenseAuxiliaryResetRequest {
    pub Id: i32,
}

impl TrapDefenseAuxiliaryResetRequest {
    pub const MSG_ID: u16 = 26508;
}

impl PbMessage for TrapDefenseAuxiliaryResetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryResetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseAuxiliaryResetResponse {
    pub const MSG_ID: u16 = 29607;
}

impl PbMessage for TrapDefenseAuxiliaryResetResponse {
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
pub struct TrapDefenseBdGroupUnlockNotify {
    pub UnlockBdGroups: Vec<i32>,
}

impl TrapDefenseBdGroupUnlockNotify {
    pub const MSG_ID: u16 = 28407;
}

impl PbMessage for TrapDefenseBdGroupUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockBdGroups {
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
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBdGroups, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBdUpdateNotify {
    pub UnlockBds: Vec<i32>,
}

impl TrapDefenseBdUpdateNotify {
    pub const MSG_ID: u16 = 15259;
}

impl PbMessage for TrapDefenseBdUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockBds {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuildingBranchRequest {
    pub Id: i32,
    pub Branch: i32,
}

impl TrapDefenseBuildingBranchRequest {
    pub const MSG_ID: u16 = 17310;
}

impl PbMessage for TrapDefenseBuildingBranchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Id = r.read_varint_i32()?,
            6 => self.Branch = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuildingBranchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseBuildingBranchResponse {
    pub const MSG_ID: u16 = 16970;
}

impl PbMessage for TrapDefenseBuildingBranchResponse {
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
pub struct TrapDefenseBuildingInfo {
    pub Id: i32,
    pub Level: i32,
    pub Branch: i32,
    pub CurMaxLevel: i32,
    pub CellPrice: i32,
    pub OriginalConstructPrice: i32,
    pub DiscountConstructPrice: i32,
    pub DeconstructReturn: i32,
}

impl PbMessage for TrapDefenseBuildingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Branch != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Branch);
        }
        if self.CurMaxLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurMaxLevel);
        }
        if self.CellPrice != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CellPrice);
        }
        if self.OriginalConstructPrice != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginalConstructPrice);
        }
        if self.DiscountConstructPrice != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscountConstructPrice);
        }
        if self.DeconstructReturn != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DeconstructReturn);
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
            3 => self.Branch = r.read_varint_i32()?,
            4 => self.CurMaxLevel = r.read_varint_i32()?,
            5 => self.CellPrice = r.read_varint_i32()?,
            6 => self.OriginalConstructPrice = r.read_varint_i32()?,
            7 => self.DiscountConstructPrice = r.read_varint_i32()?,
            8 => self.DeconstructReturn = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuildingLevelUpRequest {
    pub Id: i32,
}

impl TrapDefenseBuildingLevelUpRequest {
    pub const MSG_ID: u16 = 18133;
}

impl PbMessage for TrapDefenseBuildingLevelUpRequest {
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
pub struct TrapDefenseBuildingLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseBuildingLevelUpResponse {
    pub const MSG_ID: u16 = 21393;
}

impl PbMessage for TrapDefenseBuildingLevelUpResponse {
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
pub struct TrapDefenseBuildingResetRequest {
    pub Id: i32,
}

impl TrapDefenseBuildingResetRequest {
    pub const MSG_ID: u16 = 18594;
}

impl PbMessage for TrapDefenseBuildingResetRequest {
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
pub struct TrapDefenseBuildingResetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseBuildingResetResponse {
    pub const MSG_ID: u16 = 20421;
}

impl PbMessage for TrapDefenseBuildingResetResponse {
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
pub struct TrapDefenseBuyShopProductRequest {
    pub BdGroupConfigId: i32,
    pub BuyCount: i32,
    pub ItemConfigId: i32,
}

impl TrapDefenseBuyShopProductRequest {
    pub const MSG_ID: u16 = 17521;
}

impl PbMessage for TrapDefenseBuyShopProductRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BdGroupConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BdGroupConfigId);
        }
        if self.BuyCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyCount);
        }
        if self.ItemConfigId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.BdGroupConfigId = r.read_varint_i32()?,
            7 => self.BuyCount = r.read_varint_i32()?,
            13 => self.ItemConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuyShopProductResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ShopInfo: Option<TrapDefenseShopPanelInfo>,
}

impl TrapDefenseBuyShopProductResponse {
    pub const MSG_ID: u16 = 29351;
}

impl PbMessage for TrapDefenseBuyShopProductResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ShopInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
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
pub struct TrapDefenseChallengeInfo {
    pub ChallengeId: i32,
    pub Unlock: bool,
    pub Target: Vec<i32>,
    pub Passed: bool,
    pub Rewarded: bool,
    pub UnlockTime: i64,
    pub Leaved: bool,
    pub MaxWave: i32,
}

impl PbMessage for TrapDefenseChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Target {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Passed {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Rewarded {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.Leaved {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxWave != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxWave);
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Target, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.Passed = r.read_bool()?,
            5 => self.Rewarded = r.read_bool()?,
            6 => self.UnlockTime = r.read_varint_i64()?,
            7 => self.Leaved = r.read_bool()?,
            8 => self.MaxWave = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseChallengeQuitRequest {
    pub Settle: bool,
}

impl TrapDefenseChallengeQuitRequest {
    pub const MSG_ID: u16 = 24031;
}

impl PbMessage for TrapDefenseChallengeQuitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Settle {
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
            11 => self.Settle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseChallengeQuitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseChallengeQuitResponse {
    pub const MSG_ID: u16 = 27534;
}

impl PbMessage for TrapDefenseChallengeQuitResponse {
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
pub struct TrapDefenseChallengeResultNotify {
    pub Result: Option<TrapDefenseResultInfo>,
}

impl TrapDefenseChallengeResultNotify {
    pub const MSG_ID: u16 = 27045;
}

impl PbMessage for TrapDefenseChallengeResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Result {
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
pub struct TrapDefenseChallengeUpdateNotify {
    pub ChallengeInfo: Vec<TrapDefenseChallengeInfo>,
}

impl TrapDefenseChallengeUpdateNotify {
    pub const MSG_ID: u16 = 16595;
}

impl PbMessage for TrapDefenseChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeInfo {
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
                self.ChallengeInfo
                    .push(<TrapDefenseChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseCtx {
    pub ConfigId: i32,
    pub NewChallenge: bool,
}

impl PbMessage for TrapDefenseCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.NewChallenge {
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
            2 => self.NewChallenge = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseCurChallengeInfoRequest {}

impl TrapDefenseCurChallengeInfoRequest {
    pub const MSG_ID: u16 = 19548;
}

impl PbMessage for TrapDefenseCurChallengeInfoRequest {
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
pub struct TrapDefenseCurChallengeInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Target: Vec<i32>,
}

impl TrapDefenseCurChallengeInfoResponse {
    pub const MSG_ID: u16 = 19452;
}

impl PbMessage for TrapDefenseCurChallengeInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Target {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Target, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseDevelopInfoUpdateNotify {
    pub Buildings: Vec<TrapDefenseBuildingInfo>,
    pub Auxiliaries: Vec<TrapDefenseAuxiliaryInfo>,
}

impl TrapDefenseDevelopInfoUpdateNotify {
    pub const MSG_ID: u16 = 16748;
}

impl PbMessage for TrapDefenseDevelopInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Buildings {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Auxiliaries {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Buildings
                    .push(<TrapDefenseBuildingInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Auxiliaries
                    .push(<TrapDefenseAuxiliaryInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseDevelopResetRequest {
    pub ActivityId: i32,
}

impl TrapDefenseDevelopResetRequest {
    pub const MSG_ID: u16 = 21084;
}

impl PbMessage for TrapDefenseDevelopResetRequest {
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
pub struct TrapDefenseDevelopResetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseDevelopResetResponse {
    pub const MSG_ID: u16 = 24446;
}

impl PbMessage for TrapDefenseDevelopResetResponse {
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
pub struct TrapDefenseEntityDataChangeNotify {
    pub EntityId: i64,
    pub NewComponentData: Option<TrapDefenseComponentPb>,
}

impl TrapDefenseEntityDataChangeNotify {
    pub const MSG_ID: u16 = 18208;
}

impl PbMessage for TrapDefenseEntityDataChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.NewComponentData {
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
            9 => self.EntityId = r.read_varint_i64()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.NewComponentData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseEnvironmentalKillCtxPb {
    pub TrapId: i64,
}

impl PbMessage for TrapDefenseEnvironmentalKillCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TrapId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TrapId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseGainingGroupUpdateNotify {
    pub Reason: Option<TrapDefenseGainingUpdateReason>,
    pub GainingGroupInfo: Option<TrapDefenseGainingInfo>,
}

impl TrapDefenseGainingGroupUpdateNotify {
    pub const MSG_ID: u16 = 21410;
}

impl PbMessage for TrapDefenseGainingGroupUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.GainingGroupInfo {
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
            4 => {
                self.Reason = Some(
                    <TrapDefenseGainingUpdateReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.GainingGroupInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseGainingInfo {
    pub GroupId: i32,
    pub LevelId: i32,
}

impl PbMessage for TrapDefenseGainingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.GroupId = r.read_varint_i32()?,
            2 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseGetRewardRequest {
    pub Ids: Vec<i32>,
}

impl TrapDefenseGetRewardRequest {
    pub const MSG_ID: u16 = 24481;
}

impl PbMessage for TrapDefenseGetRewardRequest {
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
pub struct TrapDefenseGetRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseGetRewardResponse {
    pub const MSG_ID: u16 = 26764;
}

impl PbMessage for TrapDefenseGetRewardResponse {
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
pub struct TrapDefenseGetSpecialRewardRequest {
    pub Ids: Vec<i32>,
}

impl TrapDefenseGetSpecialRewardRequest {
    pub const MSG_ID: u16 = 18758;
}

impl PbMessage for TrapDefenseGetSpecialRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseGetSpecialRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseGetSpecialRewardResponse {
    pub const MSG_ID: u16 = 27113;
}

impl PbMessage for TrapDefenseGetSpecialRewardResponse {
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
pub struct TrapDefenseGoldenCoinAbosorbedCtxPb {}

impl PbMessage for TrapDefenseGoldenCoinAbosorbedCtxPb {
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
pub struct TrapDefenseItemPbData {
    pub ConfigId: i32,
    pub CarryCount: i32,
    pub CarryLimit: i32,
}

impl PbMessage for TrapDefenseItemPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.CarryCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CarryCount);
        }
        if self.CarryLimit != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CarryLimit);
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
            2 => self.CarryCount = r.read_varint_i32()?,
            3 => self.CarryLimit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseItemUpdateNotify {
    pub ItemDatas: Vec<TrapDefenseItemPbData>,
}

impl TrapDefenseItemUpdateNotify {
    pub const MSG_ID: u16 = 26436;
}

impl PbMessage for TrapDefenseItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemDatas {
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
                self.ItemDatas
                    .push(<TrapDefenseItemPbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseMapChangeStepPbData {}

impl PbMessage for TrapDefenseMapChangeStepPbData {
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
pub struct TrapDefenseMonsterConsecutiveKillsNotify {
    pub ConsecutiveKillCount: i32,
}

impl TrapDefenseMonsterConsecutiveKillsNotify {
    pub const MSG_ID: u16 = 19577;
}

impl PbMessage for TrapDefenseMonsterConsecutiveKillsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConsecutiveKillCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConsecutiveKillCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.ConsecutiveKillCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseMonsterKilledCtxPb {
    pub EntityId: i64,
    pub PollutedGridCells: Vec<GridPlacementPbInfo>,
}

impl PbMessage for TrapDefenseMonsterKilledCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.PollutedGridCells {
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
                self.PollutedGridCells
                    .push(<GridPlacementPbInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseMonsterPbGroup {
    pub MonsterConfigIds: Vec<i32>,
    pub PreviewDelayTime: f32,
    pub PreviewIntervalTime: f32,
}

impl PbMessage for TrapDefenseMonsterPbGroup {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MonsterConfigIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PreviewDelayTime != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PreviewDelayTime).to_bits().to_le_bytes());
        }
        if self.PreviewIntervalTime != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PreviewIntervalTime).to_bits().to_le_bytes());
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
                &mut self.MonsterConfigIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => self.PreviewDelayTime = r.read_f32()?,
            3 => self.PreviewIntervalTime = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefensePreviewFinishRequest {}

impl TrapDefensePreviewFinishRequest {
    pub const MSG_ID: u16 = 16711;
}

impl PbMessage for TrapDefensePreviewFinishRequest {
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
pub struct TrapDefensePreviewFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefensePreviewFinishResponse {
    pub const MSG_ID: u16 = 26099;
}

impl PbMessage for TrapDefensePreviewFinishResponse {
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
pub struct TrapDefensePreviewStartNotify {
    pub MonsterEnhancedTipText: Vec<u8>,
    pub MonsterRoutesPreviewInfo: BTreeMap<i32, TrapDefenseMonsterPbGroup>,
    pub IsShopAppear: bool,
}

impl TrapDefensePreviewStartNotify {
    pub const MSG_ID: u16 = 23247;
}

impl PbMessage for TrapDefensePreviewStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.MonsterEnhancedTipText.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MonsterEnhancedTipText);
        }
        if !self.MonsterRoutesPreviewInfo.is_empty() {
            crate::wire::write_map(out, 7, &self.MonsterRoutesPreviewInfo);
        }
        if self.IsShopAppear {
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
            1 => self.MonsterEnhancedTipText = r.read_len_delimited()?.to_vec(),
            7 => crate::wire::decode_map_entry(r, &mut self.MonsterRoutesPreviewInfo)?,
            10 => self.IsShopAppear = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefensePreviewStepPbData {
    pub IsShopAppear: bool,
    pub MonsterEnhancedTipText: Vec<u8>,
    pub MonsterRoutesPreviewInfo: BTreeMap<i32, TrapDefenseMonsterPbGroup>,
    pub EndCountdownSecond: i32,
    pub IsWarningPrepareStyle: bool,
    pub ShopInfo: Option<TrapDefenseShopPanelInfo>,
}

impl PbMessage for TrapDefensePreviewStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShopAppear {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.MonsterEnhancedTipText.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MonsterEnhancedTipText);
        }
        if !self.MonsterRoutesPreviewInfo.is_empty() {
            crate::wire::write_map(out, 3, &self.MonsterRoutesPreviewInfo);
        }
        if self.EndCountdownSecond != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndCountdownSecond);
        }
        if self.IsWarningPrepareStyle {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ShopInfo {
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
            1 => self.IsShopAppear = r.read_bool()?,
            2 => self.MonsterEnhancedTipText = r.read_len_delimited()?.to_vec(),
            3 => crate::wire::decode_map_entry(r, &mut self.MonsterRoutesPreviewInfo)?,
            4 => self.EndCountdownSecond = r.read_varint_i32()?,
            5 => self.IsWarningPrepareStyle = r.read_bool()?,
            6 => {
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
pub struct TrapDefenseQuickEquipRequest {}

impl TrapDefenseQuickEquipRequest {
    pub const MSG_ID: u16 = 18555;
}

impl PbMessage for TrapDefenseQuickEquipRequest {
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
pub struct TrapDefenseQuickEquipResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseQuickEquipResponse {
    pub const MSG_ID: u16 = 20626;
}

impl PbMessage for TrapDefenseQuickEquipResponse {
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
pub struct TrapDefenseReachInstEndpointCtxPb {}

impl PbMessage for TrapDefenseReachInstEndpointCtxPb {
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
pub struct TrapDefenseRefreshGainingGroupRequest {}

impl TrapDefenseRefreshGainingGroupRequest {
    pub const MSG_ID: u16 = 25671;
}

impl PbMessage for TrapDefenseRefreshGainingGroupRequest {
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
pub struct TrapDefenseRefreshGainingGroupResponse {
    pub NewRewardInfo: Option<RewardGainingPanelInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseRefreshGainingGroupResponse {
    pub const MSG_ID: u16 = 20307;
}

impl PbMessage for TrapDefenseRefreshGainingGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewRewardInfo {
            crate::wire::write_submessage(out, 5, v);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.NewRewardInfo
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
pub struct TrapDefenseRefreshShopRequest {}

impl TrapDefenseRefreshShopRequest {
    pub const MSG_ID: u16 = 17409;
}

impl PbMessage for TrapDefenseRefreshShopRequest {
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
pub struct TrapDefenseRefreshShopResponse {
    pub NewShopInfo: Option<TrapDefenseShopPanelInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseRefreshShopResponse {
    pub const MSG_ID: u16 = 29329;
}

impl PbMessage for TrapDefenseRefreshShopResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewShopInfo {
            crate::wire::write_submessage(out, 11, v);
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.NewShopInfo
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
pub struct TrapDefenseResultInfo {
    pub Target: Vec<i32>,
    pub Success: bool,
    pub UnlockBuilding: Vec<i32>,
    pub UnlockAuxiliary: Vec<i32>,
    pub RewardItems: Vec<RewardItem>,
    pub UnlockBds: Vec<i32>,
    pub Quit: bool,
    pub RewardLevelUpPoint: i32,
    pub PreTarget: Vec<i32>,
    pub FinalTarget: Vec<i32>,
    pub Wave: i32,
    pub Health: i32,
    pub ChallengeId: i32,
    pub MaxWave: i32,
}

impl PbMessage for TrapDefenseResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Target {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Success {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.UnlockBuilding {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockAuxiliary {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardItems {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.UnlockBds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Quit {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardLevelUpPoint != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardLevelUpPoint);
        }
        for item in &self.PreTarget {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.FinalTarget {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Wave != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Wave);
        }
        if self.Health != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Health);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.MaxWave != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxWave);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Target, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.Success = r.read_bool()?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBuilding, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockAuxiliary,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItems
                    .push(<RewardItem as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockBds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => self.Quit = r.read_bool()?,
            8 => self.RewardLevelUpPoint = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreTarget, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FinalTarget, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.Wave = r.read_varint_i32()?,
            12 => self.Health = r.read_varint_i32()?,
            13 => self.ChallengeId = r.read_varint_i32()?,
            14 => self.MaxWave = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseRewardGainingGroupsNotify {
    pub RewardInfo: Option<RewardGainingPanelInfo>,
}

impl TrapDefenseRewardGainingGroupsNotify {
    pub const MSG_ID: u16 = 22298;
}

impl PbMessage for TrapDefenseRewardGainingGroupsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardInfo {
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
                self.RewardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseRewardInfo {
    pub TaskInfo: Option<ConditionTask>,
    pub LimitBeginTime: i64,
    pub LimitEndTime: i64,
}

impl PbMessage for TrapDefenseRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TaskInfo {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.TaskInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.LimitBeginTime = r.read_varint_i64()?,
            3 => self.LimitEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseRewardStepPbData {}

impl PbMessage for TrapDefenseRewardStepPbData {
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
pub struct TrapDefenseRewardUpdateNotify {
    pub Rewards: Vec<TrapDefenseRewardInfo>,
}

impl TrapDefenseRewardUpdateNotify {
    pub const MSG_ID: u16 = 20145;
}

impl PbMessage for TrapDefenseRewardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Rewards {
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
                self.Rewards
                    .push(<TrapDefenseRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSelectGainingGroupRequest {
    pub GainingGroupId: i32,
}

impl TrapDefenseSelectGainingGroupRequest {
    pub const MSG_ID: u16 = 18252;
}

impl PbMessage for TrapDefenseSelectGainingGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GainingGroupId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GainingGroupId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.GainingGroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSelectGainingGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseSelectGainingGroupResponse {
    pub const MSG_ID: u16 = 25867;
}

impl PbMessage for TrapDefenseSelectGainingGroupResponse {
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
pub struct TrapDefenseShopBdGroupPbInfo {
    pub BdGrougConfigId: i32,
    pub IsSold: bool,
}

impl PbMessage for TrapDefenseShopBdGroupPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BdGrougConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BdGrougConfigId);
        }
        if self.IsSold {
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
            1 => self.BdGrougConfigId = r.read_varint_i32()?,
            2 => self.IsSold = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseShopItemPbInfo {
    pub ItemConfigId: i32,
    pub StockCount: i32,
}

impl PbMessage for TrapDefenseShopItemPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemConfigId);
        }
        if self.StockCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StockCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemConfigId = r.read_varint_i32()?,
            2 => self.StockCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseShopPanelInfo {
    pub RemainRefreshTimes: i32,
    pub TotalRefreshTimes: i32,
    pub RefreshCost: i32,
    pub ShopProductPbInfos: Vec<TrapDefenseShopProductPbInfo>,
}

impl PbMessage for TrapDefenseShopPanelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RemainRefreshTimes != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainRefreshTimes);
        }
        if self.TotalRefreshTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalRefreshTimes);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCost);
        }
        for item in &self.ShopProductPbInfos {
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
            1 => self.RemainRefreshTimes = r.read_varint_i32()?,
            2 => self.TotalRefreshTimes = r.read_varint_i32()?,
            3 => self.RefreshCost = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ShopProductPbInfos
                    .push(<TrapDefenseShopProductPbInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseShopProductPbInfo {
    pub OriginalPrice: i32,
    pub DiscountPrice: i32,
    pub ItemPbInfo: Option<TrapDefenseShopItemPbInfo>,
    pub BdGroupPbInfo: Option<TrapDefenseShopBdGroupPbInfo>,
}

impl PbMessage for TrapDefenseShopProductPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OriginalPrice != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginalPrice);
        }
        if self.DiscountPrice != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscountPrice);
        }
        if let Some(v) = &self.ItemPbInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.BdGroupPbInfo {
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
            1 => self.OriginalPrice = r.read_varint_i32()?,
            2 => self.DiscountPrice = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ItemPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BdGroupPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSlot {
    pub Slot: i32,
    pub Building: Option<TrapDefenseBuildingInfo>,
    pub Auxiliary: Option<TrapDefenseAuxiliaryInfo>,
}

impl PbMessage for TrapDefenseSlot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Slot != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Slot);
        }
        if let Some(v) = &self.Building {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Auxiliary {
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
            1 => self.Slot = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Building
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Auxiliary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSlotUpdateRequest {
    pub Slot: Vec<TrapDefenseSlot>,
}

impl TrapDefenseSlotUpdateRequest {
    pub const MSG_ID: u16 = 28106;
}

impl PbMessage for TrapDefenseSlotUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Slot {
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
                self.Slot
                    .push(<TrapDefenseSlot as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSlotUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseSlotUpdateResponse {
    pub const MSG_ID: u16 = 20401;
}

impl PbMessage for TrapDefenseSlotUpdateResponse {
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
pub struct TrapDefenseSpawnMonsterStepPbData {
    pub EndCountdownSecond: i32,
    pub IsWarningPrepareStyle: bool,
}

impl PbMessage for TrapDefenseSpawnMonsterStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EndCountdownSecond != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndCountdownSecond);
        }
        if self.IsWarningPrepareStyle {
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
            1 => self.EndCountdownSecond = r.read_varint_i32()?,
            2 => self.IsWarningPrepareStyle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSpawnMonsterWaveEndNotify {
    pub CountdownSecond: i32,
}

impl TrapDefenseSpawnMonsterWaveEndNotify {
    pub const MSG_ID: u16 = 26815;
}

impl PbMessage for TrapDefenseSpawnMonsterWaveEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CountdownSecond != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CountdownSecond);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.CountdownSecond = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSpcecialCellDestructCtxPb {}

impl PbMessage for TrapDefenseSpcecialCellDestructCtxPb {
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
pub struct TrapDefenseSpecialRewardUpdateNotify {
    pub SpecialRewards: Vec<TrapDefenseRewardInfo>,
}

impl TrapDefenseSpecialRewardUpdateNotify {
    pub const MSG_ID: u16 = 21344;
}

impl PbMessage for TrapDefenseSpecialRewardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SpecialRewards {
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
                self.SpecialRewards
                    .push(<TrapDefenseRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseStepUpdateNotify {
    pub PreviewStepPbData: Option<TrapDefensePreviewStepPbData>,
    pub MapChangeStepPbData: Option<TrapDefenseMapChangeStepPbData>,
    pub RewardStepPbData: Option<TrapDefenseRewardStepPbData>,
    pub MonsterStepPbData: Option<TrapDefenseSpawnMonsterStepPbData>,
}

impl TrapDefenseStepUpdateNotify {
    pub const MSG_ID: u16 = 23425;
}

impl PbMessage for TrapDefenseStepUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PreviewStepPbData {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.MapChangeStepPbData {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.RewardStepPbData {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.MonsterStepPbData {
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
                self.PreviewStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MapChangeStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RewardStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.MonsterStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSyncGainingGroupsNotify {
    pub GainingInfos: Vec<TrapDefenseGainingInfo>,
}

impl TrapDefenseSyncGainingGroupsNotify {
    pub const MSG_ID: u16 = 25763;
}

impl PbMessage for TrapDefenseSyncGainingGroupsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GainingInfos {
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
                self.GainingInfos
                    .push(<TrapDefenseGainingInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSystemInfoNotify {
    pub TechParams: Option<TrapDefenseTechParam>,
    pub Buildings: Vec<TrapDefenseBuildingInfo>,
    pub Slots: Vec<TrapDefenseSlot>,
    pub VariableKeyNames: BTreeMap<Vec<u8>, Vec<u8>>,
    pub Auxiliaries: Vec<TrapDefenseAuxiliaryInfo>,
    pub ItemDatas: Vec<TrapDefenseItemPbData>,
    pub ChangedMiniMap: i32,
}

impl TrapDefenseSystemInfoNotify {
    pub const MSG_ID: u16 = 23974;
}

impl PbMessage for TrapDefenseSystemInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TechParams {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.Buildings {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.Slots {
            crate::wire::write_submessage(out, 4, item);
        }
        if !self.VariableKeyNames.is_empty() {
            crate::wire::write_map(out, 7, &self.VariableKeyNames);
        }
        for item in &self.Auxiliaries {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.ItemDatas {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.ChangedMiniMap != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangedMiniMap);
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
                self.TechParams
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Buildings
                    .push(<TrapDefenseBuildingInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Slots
                    .push(<TrapDefenseSlot as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_map_entry(r, &mut self.VariableKeyNames)?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Auxiliaries
                    .push(<TrapDefenseAuxiliaryInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ItemDatas
                    .push(<TrapDefenseItemPbData as PbMessage>::decode(bytes)?);
            }
            15 => self.ChangedMiniMap = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSystemInfoUpdateNotify {
    pub Slots: Vec<TrapDefenseSlot>,
}

impl TrapDefenseSystemInfoUpdateNotify {
    pub const MSG_ID: u16 = 18546;
}

impl PbMessage for TrapDefenseSystemInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Slots {
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
                self.Slots
                    .push(<TrapDefenseSlot as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTechParam {
    pub Params: BTreeMap<i32, i32>,
}

impl PbMessage for TrapDefenseTechParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Params.is_empty() {
            crate::wire::write_map(out, 1, &self.Params);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Params)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTechPointUpdateNotify {
    pub TotalLevelUpPoint: i32,
    pub TotalTechPoint: i32,
    pub LeftLevelUpPoint: i32,
    pub LeftTechPoint: i32,
}

impl TrapDefenseTechPointUpdateNotify {
    pub const MSG_ID: u16 = 15898;
}

impl PbMessage for TrapDefenseTechPointUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalLevelUpPoint != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalLevelUpPoint);
        }
        if self.TotalTechPoint != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalTechPoint);
        }
        if self.LeftLevelUpPoint != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeftLevelUpPoint);
        }
        if self.LeftTechPoint != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LeftTechPoint);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TotalLevelUpPoint = r.read_varint_i32()?,
            6 => self.TotalTechPoint = r.read_varint_i32()?,
            9 => self.LeftLevelUpPoint = r.read_varint_i32()?,
            14 => self.LeftTechPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTechUnlockRequest {
    pub Id: i32,
}

impl TrapDefenseTechUnlockRequest {
    pub const MSG_ID: u16 = 21102;
}

impl PbMessage for TrapDefenseTechUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTechUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseTechUnlockResponse {
    pub const MSG_ID: u16 = 19615;
}

impl PbMessage for TrapDefenseTechUnlockResponse {
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
pub struct TrapDefenseTechUpdateNotify {
    pub UnlockTechNodes: Vec<i32>,
}

impl TrapDefenseTechUpdateNotify {
    pub const MSG_ID: u16 = 29048;
}

impl PbMessage for TrapDefenseTechUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockTechNodes {
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
                &mut self.UnlockTechNodes,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTrapKillCtxPb {
    pub TrapId: i64,
}

impl PbMessage for TrapDefenseTrapKillCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TrapId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TrapId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseTrapSelfDestructCtxPb {
    pub DamagedGridCells: Vec<GridPlacementPbInfo>,
    pub FireCount: i32,
}

impl PbMessage for TrapDefenseTrapSelfDestructCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DamagedGridCells {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.FireCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FireCount);
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
                self.DamagedGridCells
                    .push(<GridPlacementPbInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.FireCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseUseItemRequest {
    pub ItemConfigId: i32,
}

impl TrapDefenseUseItemRequest {
    pub const MSG_ID: u16 = 16123;
}

impl PbMessage for TrapDefenseUseItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemConfigId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.ItemConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseUseItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrapDefenseUseItemResponse {
    pub const MSG_ID: u16 = 28819;
}

impl PbMessage for TrapDefenseUseItemResponse {
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
pub struct TreasureBoxSlotAllInfoNotify {
    pub AllTreasureBoxSlotInfo: Vec<TreasureBoxSlotInfo>,
}

impl TreasureBoxSlotAllInfoNotify {
    pub const MSG_ID: u16 = 27789;
}

impl PbMessage for TreasureBoxSlotAllInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AllTreasureBoxSlotInfo {
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
                self.AllTreasureBoxSlotInfo
                    .push(<TreasureBoxSlotInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreasureBoxSlotInfo {
    pub DetectionSlotId: i64,
    pub MarkId: i32,
}

impl PbMessage for TreasureBoxSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionSlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DetectionSlotId);
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
            1 => self.DetectionSlotId = r.read_varint_i64()?,
            2 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreasureChestCountData {
    pub TargetType: Option<TreasureChestCountTargetType>,
    pub TargetId: i32,
    pub StatType: Option<TreasureChestStatType>,
    pub CurrentCount: i32,
    pub TotalCount: i32,
}

impl PbMessage for TreasureChestCountData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TargetType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetId);
        }
        if let Some(v) = &self.StatType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurrentCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentCount);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
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
                self.TargetType = Some(
                    <TreasureChestCountTargetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.TargetId = r.read_varint_i32()?,
            3 => {
                self.StatType = Some(
                    <TreasureChestStatType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.CurrentCount = r.read_varint_i32()?,
            5 => self.TotalCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreasureChestCountIncrementNotify {
    pub AreaIdList: Vec<i32>,
    pub InstanceIdList: Vec<i32>,
    pub StatType: Option<TreasureChestStatType>,
}

impl TreasureChestCountIncrementNotify {
    pub const MSG_ID: u16 = 25587;
}

impl PbMessage for TreasureChestCountIncrementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaIdList {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.InstanceIdList {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.StatType {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AreaIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InstanceIdList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => {
                self.StatType = Some(
                    <TreasureChestStatType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TreasureChestCountNotify {
    pub Data: Vec<TreasureChestCountData>,
}

impl TreasureChestCountNotify {
    pub const MSG_ID: u16 = 23149;
}

impl PbMessage for TreasureChestCountNotify {
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
                    .push(<TreasureChestCountData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrialRoleRewardRequest {
    pub RoleId: i32,
}

impl TrialRoleRewardRequest {
    pub const MSG_ID: u16 = 22659;
}

impl PbMessage for TrialRoleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrialRoleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrialRoleRewardResponse {
    pub const MSG_ID: u16 = 26824;
}

impl PbMessage for TrialRoleRewardResponse {
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
pub struct TrialRoleSkinRewardRequest {
    pub RoleSkinTrialInfoId: i32,
}

impl TrialRoleSkinRewardRequest {
    pub const MSG_ID: u16 = 27840;
}

impl PbMessage for TrialRoleSkinRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinTrialInfoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinTrialInfoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoleSkinTrialInfoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrialRoleSkinRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TrialRoleSkinRewardResponse {
    pub const MSG_ID: u16 = 25081;
}

impl PbMessage for TrialRoleSkinRewardResponse {
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
pub struct TurntableActivityData {
    pub IsAllFinish: bool,
    pub CurRound: i32,
    pub RewardedItem: Vec<i32>,
    pub TurntableTasks: Vec<ActivityTask>,
}

impl PbMessage for TurntableActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAllFinish {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurRound != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRound);
        }
        for item in &self.RewardedItem {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TurntableTasks {
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
            1 => self.IsAllFinish = r.read_bool()?,
            2 => self.CurRound = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedItem, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TurntableTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TurntableCompleteRequest {
    pub EntityId: i64,
}

impl TurntableCompleteRequest {
    pub const MSG_ID: u16 = 20102;
}

impl PbMessage for TurntableCompleteRequest {
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
pub struct TurntableCompleteResponse {
    pub ErrCode: Option<ErrorCode>,
    pub EntityId: i64,
}

impl TurntableCompleteResponse {
    pub const MSG_ID: u16 = 18435;
}

impl PbMessage for TurntableCompleteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            12 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TurntableTaskUpdateNotify {
    pub TurntableTask: Option<ActivityTask>,
}

impl TurntableTaskUpdateNotify {
    pub const MSG_ID: u16 = 20220;
}

impl PbMessage for TurntableTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TurntableTask {
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
                self.TurntableTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TutorialUnlockNotify {
    pub UnLockList: Vec<TutorialInfo>,
}

impl TutorialUnlockNotify {
    pub const MSG_ID: u16 = 25322;
}

impl PbMessage for TutorialUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnLockList {
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
                self.UnLockList
                    .push(<TutorialInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UDPMovePackageNotify {
    pub MovingEntities: Vec<MovingEntityData>,
}

impl UDPMovePackageNotify {
    pub const MSG_ID: u16 = 21168;
}

impl PbMessage for UDPMovePackageNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MovingEntities {
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
                self.MovingEntities
                    .push(<MovingEntityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UDPMovePackagePush {
    pub SceneOwnerId: i32,
    pub MovingEntities: Vec<MovingEntityData>,
}

impl UDPMovePackagePush {
    pub const MSG_ID: u16 = 18826;
}

impl PbMessage for UDPMovePackagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
        }
        for item in &self.MovingEntities {
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
            6 => self.SceneOwnerId = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.MovingEntities
                    .push(<MovingEntityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UiGamePlayData {
    pub SheriffInferenceData: Option<UiGameSheriffInferenceData>,
}

impl PbMessage for UiGamePlayData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SheriffInferenceData {
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
                self.SheriffInferenceData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UiGamePlayRequest {
    pub GamePlayData: Option<UiGamePlayData>,
    pub Type: Option<UiGamePlayType>,
    pub GamePlayKey: Vec<u8>,
}

impl UiGamePlayRequest {
    pub const MSG_ID: u16 = 24744;
}

impl PbMessage for UiGamePlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GamePlayData {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.GamePlayKey.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GamePlayKey);
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
                self.GamePlayData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                self.Type = Some(
                    <UiGamePlayType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.GamePlayKey = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UiGamePlayResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl UiGamePlayResponse {
    pub const MSG_ID: u16 = 19566;
}

impl PbMessage for UiGamePlayResponse {
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
pub struct UiGameSheriffInferenceData {
    pub ProgressId: i32,
    pub EndingId: i32,
}

impl PbMessage for UiGameSheriffInferenceData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ProgressId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProgressId);
        }
        if self.EndingId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndingId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ProgressId = r.read_varint_i32()?,
            2 => self.EndingId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnOpenedAreaPullbackRequest {
    pub Reason: Option<TeleportReason>,
    pub TransferEffectId: i32,
}

impl UnOpenedAreaPullbackRequest {
    pub const MSG_ID: u16 = 17532;
}

impl PbMessage for UnOpenedAreaPullbackRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TransferEffectId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TransferEffectId);
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
                self.Reason = Some(
                    <TeleportReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.TransferEffectId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnOpenedAreaPullbackResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UnOpenedAreaPullbackResponse {
    pub const MSG_ID: u16 = 23277;
}

impl PbMessage for UnOpenedAreaPullbackResponse {
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
pub struct UnblockPlayerRequest {
    pub Id: i32,
}

impl UnblockPlayerRequest {
    pub const MSG_ID: u16 = 25581;
}

impl PbMessage for UnblockPlayerRequest {
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
pub struct UnblockPlayerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UnblockPlayerResponse {
    pub const MSG_ID: u16 = 26572;
}

impl PbMessage for UnblockPlayerResponse {
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
pub struct UnlockCardNotify {
    pub CardId: i32,
}

impl UnlockCardNotify {
    pub const MSG_ID: u16 = 22072;
}

impl PbMessage for UnlockCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockHeadFrameNotify {
    pub HeadFrameId: i32,
}

impl UnlockHeadFrameNotify {
    pub const MSG_ID: u16 = 19052;
}

impl PbMessage for UnlockHeadFrameNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HeadFrameId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadFrameId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.HeadFrameId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockHeadPhotoNotify {
    pub HeadPhotoId: i32,
}

impl UnlockHeadPhotoNotify {
    pub const MSG_ID: u16 = 23177;
}

impl PbMessage for UnlockHeadPhotoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HeadPhotoId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadPhotoId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.HeadPhotoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockInfo {
    pub ShopId: i32,
    pub Id: i32,
}

impl PbMessage for UnlockInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.ShopId = r.read_varint_i32()?,
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockInstEntranceRequest {
    pub EntranceId: i32,
}

impl UnlockInstEntranceRequest {
    pub const MSG_ID: u16 = 16204;
}

impl PbMessage for UnlockInstEntranceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntranceId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntranceId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.EntranceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockInstEntranceResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UnlockInstEntranceResponse {
    pub const MSG_ID: u16 = 24736;
}

impl PbMessage for UnlockInstEntranceResponse {
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
pub struct UnlockMapBlockNotify {
    pub UnlockBlockId: Vec<i32>,
}

impl UnlockMapBlockNotify {
    pub const MSG_ID: u16 = 15704;
}

impl PbMessage for UnlockMapBlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockBlockId {
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
            3 => {
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
pub struct UnlockMarkNotify {
    pub MarkId: i32,
}

impl UnlockMarkNotify {
    pub const MSG_ID: u16 = 25945;
}

impl PbMessage for UnlockMarkNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockMultiMapNotify {
    pub UnlockMultiMapId: Vec<i32>,
}

impl UnlockMultiMapNotify {
    pub const MSG_ID: u16 = 29531;
}

impl PbMessage for UnlockMultiMapNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockMultiMapId {
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
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockMultiMapId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockNpcSyncDataPb {
    pub NpcId: Vec<i32>,
}

impl PbMessage for UnlockNpcSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NpcId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NpcId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateEnableFunctionNotify {
    pub FuncInfo: Vec<EnableFunctionInfo>,
}

impl UpdateEnableFunctionNotify {
    pub const MSG_ID: u16 = 21861;
}

impl PbMessage for UpdateEnableFunctionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FuncInfo {
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
                self.FuncInfo
                    .push(<EnableFunctionInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateEnableSystemNotify {
    pub FuncInfo: Vec<EnableSystemFuncInfo>,
}

impl UpdateEnableSystemNotify {
    pub const MSG_ID: u16 = 17615;
}

impl PbMessage for UpdateEnableSystemNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FuncInfo {
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
                self.FuncInfo
                    .push(<EnableSystemFuncInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateEnterInfoNotify {
    pub EnterInfos: Vec<InstEnterInfoPb>,
}

impl UpdateEnterInfoNotify {
    pub const MSG_ID: u16 = 24015;
}

impl PbMessage for UpdateEnterInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EnterInfos {
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
                self.EnterInfos
                    .push(<InstEnterInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateFightRoleRequest {
    pub CurRoleId: i32,
    pub RoleIds: Vec<i32>,
}

impl UpdateFightRoleRequest {
    pub const MSG_ID: u16 = 24496;
}

impl PbMessage for UpdateFightRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurRoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRoleId);
        }
        for item in &self.RoleIds {
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
            6 => self.CurRoleId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateFightRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UpdateFightRoleResponse {
    pub const MSG_ID: u16 = 19583;
}

impl PbMessage for UpdateFightRoleResponse {
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
pub struct UpdateInstanceOwnerInfoNotify {
    pub InstanceOwnerInfo: Option<InstanceOwnerInfo>,
    pub Id: i64,
}

impl UpdateInstanceOwnerInfoNotify {
    pub const MSG_ID: u16 = 29689;
}

impl PbMessage for UpdateInstanceOwnerInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InstanceOwnerInfo {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
                let bytes = r.read_len_delimited()?;
                self.InstanceOwnerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateInstanceTimerNotify {
    pub InstanceTimers: BTreeMap<i32, InstanceTimer>,
}

impl UpdateInstanceTimerNotify {
    pub const MSG_ID: u16 = 17805;
}

impl PbMessage for UpdateInstanceTimerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.InstanceTimers.is_empty() {
            crate::wire::write_map(out, 15, &self.InstanceTimers);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_map_entry(r, &mut self.InstanceTimers)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateMonsterUnlockNotify {
    pub DetectionUnlocks: Option<DetectionUnlock>,
}

impl UpdateMonsterUnlockNotify {
    pub const MSG_ID: u16 = 17351;
}

impl PbMessage for UpdateMonsterUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.DetectionUnlocks {
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
                self.DetectionUnlocks
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateOwnerMingSuGenNotify {
    pub Id: i64,
}

impl UpdateOwnerMingSuGenNotify {
    pub const MSG_ID: u16 = 28808;
}

impl PbMessage for UpdateOwnerMingSuGenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePhantomEquipGroupRequest {
    pub RoleId: i32,
    pub PhantomIncrIds: Vec<i32>,
    pub Index: i32,
}

impl UpdatePhantomEquipGroupRequest {
    pub const MSG_ID: u16 = 22464;
}

impl PbMessage for UpdatePhantomEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.PhantomIncrIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            11 => self.RoleId = r.read_varint_i32()?,
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PhantomIncrIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePhantomEquipGroupResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
}

impl UpdatePhantomEquipGroupResponse {
    pub const MSG_ID: u16 = 27868;
}

impl PbMessage for UpdatePhantomEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomEquipGroupInfos {
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
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayerAllFightRoleNotify {
    pub FightRoleInfos: Vec<FightRoleInformation>,
    pub PlayerId: i32,
}

impl UpdatePlayerAllFightRoleNotify {
    pub const MSG_ID: u16 = 20791;
}

impl PbMessage for UpdatePlayerAllFightRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FightRoleInfos {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfos
                    .push(<FightRoleInformation as PbMessage>::decode(bytes)?);
            }
            15 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayerPassiveSkillRequest {
    pub PassiveSkillId: i64,
    pub RoleId: i32,
    pub CdEndTime: i64,
}

impl UpdatePlayerPassiveSkillRequest {
    pub const MSG_ID: u16 = 20221;
}

impl PbMessage for UpdatePlayerPassiveSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassiveSkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassiveSkillId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.CdEndTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CdEndTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PassiveSkillId = r.read_varint_i64()?,
            2 => self.RoleId = r.read_varint_i32()?,
            15 => self.CdEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayerPassiveSkillResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl UpdatePlayerPassiveSkillResponse {
    pub const MSG_ID: u16 = 22796;
}

impl PbMessage for UpdatePlayerPassiveSkillResponse {
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
pub struct UpdatePlayerSingleFightRoleNotify {
    pub FightRoleInfo: Option<FightRoleInformation>,
    pub PlayerId: i32,
}

impl UpdatePlayerSingleFightRoleNotify {
    pub const MSG_ID: u16 = 28925;
}

impl PbMessage for UpdatePlayerSingleFightRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FightRoleInfo {
            crate::wire::write_submessage(out, 2, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdatePlayersFightRolesInfoNotify {
    pub PlayersFightRoles: Vec<PlayerFightRolesInfo>,
}

impl UpdatePlayersFightRolesInfoNotify {
    pub const MSG_ID: u16 = 16745;
}

impl PbMessage for UpdatePlayersFightRolesInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayersFightRoles {
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
                self.PlayersFightRoles
                    .push(<PlayerFightRolesInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateSceneGroupDataRequest {
    pub IsReplace: bool,
    pub NeedReserve: bool,
    pub CurEntId: i64,
    pub ShowTips: bool,
    pub EntityIds: Vec<i64>,
    pub TargetGroup: Option<ESceneRolesGroup>,
}

impl UpdateSceneGroupDataRequest {
    pub const MSG_ID: u16 = 16945;
}

impl PbMessage for UpdateSceneGroupDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsReplace {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NeedReserve {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurEntId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurEntId);
        }
        if self.ShowTips {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.TargetGroup {
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
            3 => self.IsReplace = r.read_bool()?,
            4 => self.NeedReserve = r.read_bool()?,
            6 => self.CurEntId = r.read_varint_i64()?,
            8 => self.ShowTips = r.read_bool()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            10 => {
                self.TargetGroup = Some(
                    <ESceneRolesGroup as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct UpdateSceneGroupDataResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UpdateSceneGroupDataResponse {
    pub const MSG_ID: u16 = 19725;
}

impl PbMessage for UpdateSceneGroupDataResponse {
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
pub struct UpdateSceneItemBlackboardRequest {
    pub Params: Vec<SceneItemBlackboardParam>,
    pub EntityId: i64,
}

impl UpdateSceneItemBlackboardRequest {
    pub const MSG_ID: u16 = 15059;
}

impl PbMessage for UpdateSceneItemBlackboardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
            crate::wire::write_submessage(out, 1, item);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Params
                    .push(<SceneItemBlackboardParam as PbMessage>::decode(bytes)?);
            }
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateSceneItemBlackboardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl UpdateSceneItemBlackboardResponse {
    pub const MSG_ID: u16 = 19386;
}

impl PbMessage for UpdateSceneItemBlackboardResponse {
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
pub struct UpdateSysBuffNotify {
    pub EntityId: i64,
    pub BuffInfos: Vec<SysBuffInformation>,
}

impl UpdateSysBuffNotify {
    pub const MSG_ID: u16 = 23927;
}

impl PbMessage for UpdateSysBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.BuffInfos {
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
            6 => self.EntityId = r.read_varint_i64()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.BuffInfos
                    .push(<SysBuffInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateWorldTeamPlayerFightInfoNotify {
    pub PlayerId: i32,
    pub FightInfo: Option<WorldTeamPlayerFightInfo>,
}

impl UpdateWorldTeamPlayerFightInfoNotify {
    pub const MSG_ID: u16 = 20243;
}

impl PbMessage for UpdateWorldTeamPlayerFightInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.FightInfo {
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
            13 => self.PlayerId = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.FightInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseDetectionSkillRequest {
    pub Rot: Option<Rotator>,
    pub Pos: Option<Vector>,
    pub ConfirmUseDetectionSkill: bool,
    pub SkillId: i32,
}

impl UseDetectionSkillRequest {
    pub const MSG_ID: u16 = 29977;
}

impl PbMessage for UseDetectionSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.ConfirmUseDetectionSkill {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
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
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.ConfirmUseDetectionSkill = r.read_bool()?,
            13 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseDetectionSkillResponse {
    pub Code: Option<ErrorCode>,
    pub DetectionSlotId: i64,
    pub DetectionEntityPosInfo: Vec<DetectionEntityPosInfo>,
}

impl UseDetectionSkillResponse {
    pub const MSG_ID: u16 = 23849;
}

impl PbMessage for UseDetectionSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.DetectionSlotId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DetectionSlotId);
        }
        for item in &self.DetectionEntityPosInfo {
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
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.DetectionSlotId = r.read_varint_i64()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionEntityPosInfo
                    .push(<DetectionEntityPosInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsePhantomEquipGroupRequest {
    pub RoleId: i32,
    pub Index: i32,
}

impl UsePhantomEquipGroupRequest {
    pub const MSG_ID: u16 = 23641;
}

impl PbMessage for UsePhantomEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            7 => self.RoleId = r.read_varint_i32()?,
            12 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UsePhantomEquipGroupResponse {
    pub StackMergeInfos: Vec<PhantomStackMergeInfo>,
    pub EquipInfoList: Vec<RolePhantomEquipInfo>,
    pub StackSplitInfos: Vec<PhantomStackSplitInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl UsePhantomEquipGroupResponse {
    pub const MSG_ID: u16 = 19113;
}

impl PbMessage for UsePhantomEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StackMergeInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.EquipInfoList {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.StackSplitInfos {
            crate::wire::write_submessage(out, 13, item);
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.StackMergeInfos
                    .push(<PhantomStackMergeInfo as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.EquipInfoList
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.StackSplitInfos
                    .push(<PhantomStackSplitInfo as PbMessage>::decode(bytes)?);
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
pub struct UseSoundBoxSkillRequest {}

impl UseSoundBoxSkillRequest {
    pub const MSG_ID: u16 = 15919;
}

impl PbMessage for UseSoundBoxSkillRequest {
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
pub struct UseSoundBoxSkillResponse {
    pub Code: Option<ErrorCode>,
    pub DetectionEntityPosInfo: Option<DetectionEntityPosInfo>,
}

impl UseSoundBoxSkillResponse {
    pub const MSG_ID: u16 = 22512;
}

impl PbMessage for UseSoundBoxSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.DetectionEntityPosInfo {
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
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionEntityPosInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VacuumCleanerRequest {
    pub SkillId: i32,
    pub SceneOwnerId: i32,
    pub AdsorbEntityIncId: i64,
    pub EntityIncIds: Vec<i64>,
    pub Type: Option<AdsorbType>,
}

impl VacuumCleanerRequest {
    pub const MSG_ID: u16 = 22748;
}

impl PbMessage for VacuumCleanerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
        }
        if self.AdsorbEntityIncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AdsorbEntityIncId);
        }
        for item in &self.EntityIncIds {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
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
            5 => self.SkillId = r.read_varint_i32()?,
            6 => self.SceneOwnerId = r.read_varint_i32()?,
            9 => self.AdsorbEntityIncId = r.read_varint_i64()?,
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIncIds, |sr| {
                    Ok(sr.read_varint_i64()?)
                })?
            }
            14 => {
                self.Type = Some(<AdsorbType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VacuumCleanerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VacuumCleanerResponse {
    pub const MSG_ID: u16 = 22664;
}

impl PbMessage for VacuumCleanerResponse {
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
pub struct ValidTimeItemAddNotify {
    pub ItemList: Vec<ValidTimeItem>,
    pub Reason: i32,
}

impl ValidTimeItemAddNotify {
    pub const MSG_ID: u16 = 19758;
}

impl PbMessage for ValidTimeItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<ValidTimeItem as PbMessage>::decode(bytes)?);
            }
            12 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidTimeItemRemoveNotify {
    pub ItemList: Vec<RemoveValidTimeItem>,
}

impl ValidTimeItemRemoveNotify {
    pub const MSG_ID: u16 = 20989;
}

impl PbMessage for ValidTimeItemRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
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
                self.ItemList
                    .push(<RemoveValidTimeItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValidTimeItemUpdateNotify {
    pub ItemList: Vec<ValidTimeItem>,
}

impl ValidTimeItemUpdateNotify {
    pub const MSG_ID: u16 = 15855;
}

impl PbMessage for ValidTimeItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ItemList
                    .push(<ValidTimeItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VarComponentPb {
    pub Vars: BTreeMap<Vec<u8>, VarDefinePb>,
}

impl PbMessage for VarComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Vars.is_empty() {
            crate::wire::write_map(out, 1, &self.Vars);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Vars)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleChangePassengerRequest {
    pub SeatId: i32,
    pub RoleId: i32,
}

impl VehicleChangePassengerRequest {
    pub const MSG_ID: u16 = 22200;
}

impl PbMessage for VehicleChangePassengerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
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
            7 => self.SeatId = r.read_varint_i32()?,
            12 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleChangePassengerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleChangePassengerResponse {
    pub const MSG_ID: u16 = 21531;
}

impl PbMessage for VehicleChangePassengerResponse {
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
pub struct VehicleEnterSharingRequest {
    pub IsEnter: bool,
    pub RideMode: Option<ShareRideMode>,
}

impl VehicleEnterSharingRequest {
    pub const MSG_ID: u16 = 25279;
}

impl PbMessage for VehicleEnterSharingRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnter {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.RideMode {
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
            4 => self.IsEnter = r.read_bool()?,
            9 => {
                self.RideMode = Some(<ShareRideMode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleEnterSharingResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleEnterSharingResponse {
    pub const MSG_ID: u16 = 25137;
}

impl PbMessage for VehicleEnterSharingResponse {
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
pub struct VehicleInfoPb {
    pub EntityId: i64,
    pub SeatId: i32,
}

impl PbMessage for VehicleInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
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
            2 => self.SeatId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePassengerComponentPb {
    pub VehicleId: i64,
    pub SeatId: i32,
}

impl PbMessage for VehiclePassengerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VehicleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleId);
        }
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.VehicleId = r.read_varint_i64()?,
            2 => self.SeatId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePassengerUpdateNotify {
    pub PlayerId: i32,
    pub RoleId: i32,
    pub EntityId: i64,
    pub SeatId: i32,
}

impl VehiclePassengerUpdateNotify {
    pub const MSG_ID: u16 = 20875;
}

impl PbMessage for VehiclePassengerUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
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
            10 => self.RoleId = r.read_varint_i32()?,
            12 => self.EntityId = r.read_varint_i64()?,
            14 => self.SeatId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePlatformGetOnRequest {
    pub EntityId: i64,
    pub IsGetOn: bool,
}

impl VehiclePlatformGetOnRequest {
    pub const MSG_ID: u16 = 21411;
}

impl PbMessage for VehiclePlatformGetOnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsGetOn {
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
            4 => self.EntityId = r.read_varint_i64()?,
            8 => self.IsGetOn = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePlatformGetOnResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehiclePlatformGetOnResponse {
    pub const MSG_ID: u16 = 18298;
}

impl PbMessage for VehiclePlatformGetOnResponse {
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
pub struct VehicleRemovePassengerRequest {
    pub RoleId: i32,
}

impl VehicleRemovePassengerRequest {
    pub const MSG_ID: u16 = 26143;
}

impl PbMessage for VehicleRemovePassengerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleRemovePassengerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleRemovePassengerResponse {
    pub const MSG_ID: u16 = 20287;
}

impl PbMessage for VehicleRemovePassengerResponse {
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
pub struct VehicleRequest {
    pub EntityId: i64,
    pub ExitType: Option<ExitVehicleType>,
    pub ClientPredicted: bool,
    pub IsGetOn: bool,
    pub HostPlayerId: i32,
    pub SeatId: i32,
    pub ReasonMsg: Vec<u8>,
}

impl VehicleRequest {
    pub const MSG_ID: u16 = 21983;
}

impl PbMessage for VehicleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.ExitType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ClientPredicted {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsGetOn {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
        }
        if !self.ReasonMsg.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReasonMsg);
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
            4 => {
                self.ExitType = Some(
                    <ExitVehicleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.ClientPredicted = r.read_bool()?,
            8 => self.IsGetOn = r.read_bool()?,
            9 => self.HostPlayerId = r.read_varint_i32()?,
            11 => self.SeatId = r.read_varint_i32()?,
            14 => self.ReasonMsg = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleResponse {
    pub const MSG_ID: u16 = 23682;
}

impl PbMessage for VehicleResponse {
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
pub struct VehicleUpdateNotify {
    pub ExitType: Option<ExitVehicleType>,
    pub EntityId: i64,
    pub PlayerId: i32,
    pub SeatId: i32,
    pub ClientPredicted: bool,
}

impl VehicleUpdateNotify {
    pub const MSG_ID: u16 = 28018;
}

impl PbMessage for VehicleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ExitType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.SeatId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeatId);
        }
        if self.ClientPredicted {
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
            3 => {
                self.ExitType = Some(
                    <ExitVehicleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => self.EntityId = r.read_varint_i64()?,
            10 => self.PlayerId = r.read_varint_i32()?,
            12 => self.SeatId = r.read_varint_i32()?,
            15 => self.ClientPredicted = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VfxNpcPatrolFinishRequest {
    pub EntityId: i64,
}

impl VfxNpcPatrolFinishRequest {
    pub const MSG_ID: u16 = 26210;
}

impl PbMessage for VfxNpcPatrolFinishRequest {
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
pub struct VfxNpcPatrolFinishResponse {
    pub Code: Option<ErrorCode>,
}

impl VfxNpcPatrolFinishResponse {
    pub const MSG_ID: u16 = 16241;
}

impl PbMessage for VfxNpcPatrolFinishResponse {
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
pub struct VisionSkillCallNotify {
    pub EntityInfo: Option<DynamicEntityInformation>,
    pub PlayerId: i32,
}

impl VisionSkillCallNotify {
    pub const MSG_ID: u16 = 23392;
}

impl PbMessage for VisionSkillCallNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.EntityInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponAddNotify {
    pub WeaponInfo: Option<OneWeaponItemInfo>,
}

impl WeaponAddNotify {
    pub const MSG_ID: u16 = 20872;
}

impl PbMessage for WeaponAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.WeaponInfo {
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
                self.WeaponInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponAllInfoNotify {
    pub AllWeapon: Vec<OneWeaponItemInfo>,
}

impl WeaponAllInfoNotify {
    pub const MSG_ID: u16 = 16632;
}

impl PbMessage for WeaponAllInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AllWeapon {
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
                self.AllWeapon
                    .push(<OneWeaponItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponIllustratedInfo {
    pub WeaponId: i32,
    pub UnlockTime: i64,
}

impl PbMessage for WeaponIllustratedInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.WeaponId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponRemoveNotify {
    pub IncId: i32,
}

impl WeaponRemoveNotify {
    pub const MSG_ID: u16 = 15552;
}

impl PbMessage for WeaponRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSelectView {
    pub SurvivorsOption: Option<SurvivorsOption>,
}

impl PbMessage for WeaponSelectView {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SurvivorsOption {
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
                self.SurvivorsOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSkinAddNotify {
    pub SkinIds: Vec<i32>,
    pub IsLogin: bool,
}

impl WeaponSkinAddNotify {
    pub const MSG_ID: u16 = 17173;
}

impl PbMessage for WeaponSkinAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsLogin {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.IsLogin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSkinUnLoadNotify {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl WeaponSkinUnLoadNotify {
    pub const MSG_ID: u16 = 17893;
}

impl PbMessage for WeaponSkinUnLoadNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            2 => self.RoleId = r.read_varint_i32()?,
            10 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSkinUpdateNotify {
    pub SkinIds: Vec<i32>,
}

impl WeaponSkinUpdateNotify {
    pub const MSG_ID: u16 = 20115;
}

impl PbMessage for WeaponSkinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherControlInfoRequest {}

impl WeatherControlInfoRequest {
    pub const MSG_ID: u16 = 25430;
}

impl PbMessage for WeatherControlInfoRequest {
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
pub struct WeatherControlInfoResponse {
    pub UnlockedSwitchIds: Vec<i32>,
    pub IncId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl WeatherControlInfoResponse {
    pub const MSG_ID: u16 = 26543;
}

impl PbMessage for WeatherControlInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedSwitchIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.IncId);
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedSwitchIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.IncId = r.read_varint_i64()?,
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
pub struct WeatherControlRequest {
    pub WeatherSwitchId: i32,
}

impl WeatherControlRequest {
    pub const MSG_ID: u16 = 25604;
}

impl PbMessage for WeatherControlRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeatherSwitchId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.WeatherSwitchId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherControlResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub WeatherId: i32,
}

impl WeatherControlResponse {
    pub const MSG_ID: u16 = 28028;
}

impl PbMessage for WeatherControlResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.WeatherId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.WeatherId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherSwitchInfoRequest {}

impl WeatherSwitchInfoRequest {
    pub const MSG_ID: u16 = 27924;
}

impl PbMessage for WeatherSwitchInfoRequest {
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
pub struct WeatherSwitchInfoResponse {
    pub UnlockedSwitchIds: Vec<i32>,
}

impl WeatherSwitchInfoResponse {
    pub const MSG_ID: u16 = 27053;
}

impl PbMessage for WeatherSwitchInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedSwitchIds {
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
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockedSwitchIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeekCardContentInfo {
    pub WeekCardContentId: i32,
    pub UnlockDay: i32,
    pub RewardContent: BTreeMap<i32, i32>,
    pub UnlockTimeStamp: i64,
    pub Status: i32,
}

impl PbMessage for WeekCardContentInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeekCardContentId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekCardContentId);
        }
        if self.UnlockDay != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockDay);
        }
        if !self.RewardContent.is_empty() {
            crate::wire::write_map(out, 3, &self.RewardContent);
        }
        if self.UnlockTimeStamp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTimeStamp);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.WeekCardContentId = r.read_varint_i32()?,
            2 => self.UnlockDay = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.RewardContent)?,
            4 => self.UnlockTimeStamp = r.read_varint_i64()?,
            5 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeekCardInfo {
    pub WeekCardId: i32,
    pub Days: i32,
    pub BuyTimeStamp: i64,
    pub EndTimeStamp: i64,
    pub WeekCardContentInfos: Vec<WeekCardContentInfo>,
}

impl PbMessage for WeekCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeekCardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekCardId);
        }
        if self.Days != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Days);
        }
        if self.BuyTimeStamp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuyTimeStamp);
        }
        if self.EndTimeStamp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTimeStamp);
        }
        for item in &self.WeekCardContentInfos {
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
            1 => self.WeekCardId = r.read_varint_i32()?,
            2 => self.Days = r.read_varint_i32()?,
            3 => self.BuyTimeStamp = r.read_varint_i64()?,
            4 => self.EndTimeStamp = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.WeekCardContentInfos
                    .push(<WeekCardContentInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeekCardInfoRequest {
    pub WeekCardId: i32,
    pub WeekCardIds: Vec<i32>,
}

impl WeekCardInfoRequest {
    pub const MSG_ID: u16 = 26823;
}

impl PbMessage for WeekCardInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeekCardId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekCardId);
        }
        for item in &self.WeekCardIds {
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
            3 => self.WeekCardId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.WeekCardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeekCardInfoResponse {
    pub Infos: Option<WeekCardInfo>,
    pub WeekCardInfos: Vec<WeekCardInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl WeekCardInfoResponse {
    pub const MSG_ID: u16 = 16279;
}

impl PbMessage for WeekCardInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Infos {
            crate::wire::write_submessage(out, 6, v);
        }
        for item in &self.WeekCardInfos {
            crate::wire::write_submessage(out, 12, item);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.WeekCardInfos
                    .push(<WeekCardInfo as PbMessage>::decode(bytes)?);
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
pub struct WeekCardRewardRequest {
    pub WeekCardContentId: i32,
}

impl WeekCardRewardRequest {
    pub const MSG_ID: u16 = 18381;
}

impl PbMessage for WeekCardRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeekCardContentId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekCardContentId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.WeekCardContentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeekCardRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WeekCardRewardResponse {
    pub const MSG_ID: u16 = 15339;
}

impl PbMessage for WeekCardRewardResponse {
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
pub struct WeeklyFrameworkInfoUpdateNotify {
    pub FrameworkInfo: Option<WeeklyFrameworkInfo>,
}

impl WeeklyFrameworkInfoUpdateNotify {
    pub const MSG_ID: u16 = 24665;
}

impl PbMessage for WeeklyFrameworkInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FrameworkInfo {
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
                self.FrameworkInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeeklyFrameworkScoreRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl WeeklyFrameworkScoreRewardRequest {
    pub const MSG_ID: u16 = 27471;
}

impl PbMessage for WeeklyFrameworkScoreRewardRequest {
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
pub struct WeeklyFrameworkScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WeeklyFrameworkScoreRewardResponse {
    pub const MSG_ID: u16 = 23966;
}

impl PbMessage for WeeklyFrameworkScoreRewardResponse {
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
pub struct WellStreamingFinisRequest {
    pub EntityIncId: i64,
}

impl WellStreamingFinisRequest {
    pub const MSG_ID: u16 = 20746;
}

impl PbMessage for WellStreamingFinisRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WellStreamingFinisResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WellStreamingFinisResponse {
    pub const MSG_ID: u16 = 24801;
}

impl PbMessage for WellStreamingFinisResponse {
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
pub struct WhiteGreenPropData {
    pub AttributeType: i32,
    pub WhiteValue: i32,
    pub GreenValue: i32,
    pub CurrentValue: i32,
}

impl PbMessage for WhiteGreenPropData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttributeType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttributeType);
        }
        if self.WhiteValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WhiteValue);
        }
        if self.GreenValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GreenValue);
        }
        if self.CurrentValue != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.AttributeType = r.read_varint_i32()?,
            2 => self.WhiteValue = r.read_varint_i32()?,
            3 => self.GreenValue = r.read_varint_i32()?,
            4 => self.CurrentValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WillSwitchSceneGroupNotify {}

impl WillSwitchSceneGroupNotify {
    pub const MSG_ID: u16 = 24534;
}

impl PbMessage for WillSwitchSceneGroupNotify {
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
pub struct WorldBlackboardNotify {
    pub Params: Vec<BlackboardParam>,
}

impl WorldBlackboardNotify {
    pub const MSG_ID: u16 = 23585;
}

impl PbMessage for WorldBlackboardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
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
                self.Params
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldBlackboardRequest {
    pub Params: Vec<BlackboardParam>,
}

impl WorldBlackboardRequest {
    pub const MSG_ID: u16 = 22945;
}

impl PbMessage for WorldBlackboardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
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
                self.Params
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldBlackboardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WorldBlackboardResponse {
    pub const MSG_ID: u16 = 19301;
}

impl PbMessage for WorldBlackboardResponse {
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
pub struct WorldEnterPermissionsRequest {
    pub Type: Option<WorldEnterPermission>,
}

impl WorldEnterPermissionsRequest {
    pub const MSG_ID: u16 = 20021;
}

impl PbMessage for WorldEnterPermissionsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => {
                self.Type = Some(
                    <WorldEnterPermission as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct WorldEnterPermissionsResponse {
    pub Type: Option<WorldEnterPermission>,
    pub ErrorCode: Option<ErrorCode>,
}

impl WorldEnterPermissionsResponse {
    pub const MSG_ID: u16 = 24472;
}

impl PbMessage for WorldEnterPermissionsResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            7 => {
                self.Type = Some(
                    <WorldEnterPermission as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
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
pub struct WorldLevelDownRequest {}

impl WorldLevelDownRequest {
    pub const MSG_ID: u16 = 17014;
}

impl PbMessage for WorldLevelDownRequest {
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
pub struct WorldLevelDownResponse {
    pub CurWorldLevel: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub WorldLevelTimeStamp: i32,
    pub OriginWorldLevel: i32,
}

impl WorldLevelDownResponse {
    pub const MSG_ID: u16 = 28190;
}

impl PbMessage for WorldLevelDownResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurWorldLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWorldLevel);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.WorldLevelTimeStamp != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WorldLevelTimeStamp);
        }
        if self.OriginWorldLevel != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginWorldLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurWorldLevel = r.read_varint_i32()?,
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.WorldLevelTimeStamp = r.read_varint_i32()?,
            15 => self.OriginWorldLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldLevelRegainRequest {}

impl WorldLevelRegainRequest {
    pub const MSG_ID: u16 = 26635;
}

impl PbMessage for WorldLevelRegainRequest {
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
pub struct WorldLevelRegainResponse {
    pub WorldLevelTimeStamp: i32,
    pub OriginWorldLevel: i32,
    pub CurWorldLevel: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl WorldLevelRegainResponse {
    pub const MSG_ID: u16 = 23479;
}

impl PbMessage for WorldLevelRegainResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WorldLevelTimeStamp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WorldLevelTimeStamp);
        }
        if self.OriginWorldLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginWorldLevel);
        }
        if self.CurWorldLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWorldLevel);
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
            1 => self.WorldLevelTimeStamp = r.read_varint_i32()?,
            4 => self.OriginWorldLevel = r.read_varint_i32()?,
            8 => self.CurWorldLevel = r.read_varint_i32()?,
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
pub struct WorldTeamPlayerFightInfo {
    pub CurRoleId: i32,
    pub RoleInfos: Vec<WorldTeamRoleInfo>,
}

impl PbMessage for WorldTeamPlayerFightInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRoleId);
        }
        for item in &self.RoleInfos {
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
            2 => self.CurRoleId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfos
                    .push(<WorldTeamRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldTeamPlayerInfo {
    pub PlayerId: i32,
    pub PlayerName: Vec<u8>,
    pub HeadId: i32,
    pub Level: i32,
    pub Signature: Vec<u8>,
    pub FightInfo: Option<WorldTeamPlayerFightInfo>,
    pub PingState: Option<ENetPingState>,
    pub Item: Option<PlayerDetails>,
    pub PlayerTitleId: i32,
    pub PlayerTitleExtraParam: i32,
    pub Sex: i32,
    pub WorldLevel: i32,
}

impl PbMessage for WorldTeamPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Signature);
        }
        if let Some(v) = &self.FightInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.PingState {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Item {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
        }
        if self.PlayerTitleExtraParam != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleExtraParam);
        }
        if self.Sex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sex);
        }
        if self.WorldLevel != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WorldLevel);
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
            2 => self.PlayerName = r.read_len_delimited()?.to_vec(),
            3 => self.HeadId = r.read_varint_i32()?,
            4 => self.Level = r.read_varint_i32()?,
            5 => self.Signature = r.read_len_delimited()?.to_vec(),
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FightInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.PingState = Some(<ENetPingState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Item
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.PlayerTitleId = r.read_varint_i32()?,
            10 => self.PlayerTitleExtraParam = r.read_varint_i32()?,
            11 => self.Sex = r.read_varint_i32()?,
            12 => self.WorldLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldTeamPlayerInfoChangeNotify {
    pub IntValue: i32,
    pub ChangeType: Option<WorldTeamPlayerChangeType>,
    pub PlayerId: i32,
    pub StringValue: Vec<u8>,
}

impl WorldTeamPlayerInfoChangeNotify {
    pub const MSG_ID: u16 = 27539;
}

impl PbMessage for WorldTeamPlayerInfoChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IntValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IntValue);
        }
        if let Some(v) = &self.ChangeType {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.StringValue.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StringValue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.IntValue = r.read_varint_i32()?,
            10 => {
                self.ChangeType = Some(
                    <WorldTeamPlayerChangeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.PlayerId = r.read_varint_i32()?,
            14 => self.StringValue = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldTeamRoleInfo {
    pub RoleId: i32,
    pub RoleLevel: i32,
    pub RoleSkinId: i32,
}

impl PbMessage for WorldTeamRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
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
            3 => self.RoleSkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuKujiActivityPb {
    pub ActivityId: i32,
    pub KujiId: i32,
    pub KujiAwardGroupInfo: Vec<KujiAwardGroupPb>,
    pub CostItemId: i32,
    pub CostItemCount: i32,
    pub QuestFinishedCount: i32,
    pub QuestTotalCount: i32,
    pub QuestId: i32,
}

impl PbMessage for WuWuKujiActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.KujiId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KujiId);
        }
        for item in &self.KujiAwardGroupInfo {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.CostItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostItemId);
        }
        if self.CostItemCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostItemCount);
        }
        if self.QuestFinishedCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestFinishedCount);
        }
        if self.QuestTotalCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestTotalCount);
        }
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.KujiId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.KujiAwardGroupInfo
                    .push(<KujiAwardGroupPb as PbMessage>::decode(bytes)?);
            }
            4 => self.CostItemId = r.read_varint_i32()?,
            5 => self.CostItemCount = r.read_varint_i32()?,
            6 => self.QuestFinishedCount = r.read_varint_i32()?,
            7 => self.QuestTotalCount = r.read_varint_i32()?,
            8 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuKujiGachaRequest {
    pub ActivityId: i32,
}

impl WuWuKujiGachaRequest {
    pub const MSG_ID: u16 = 24138;
}

impl PbMessage for WuWuKujiGachaRequest {
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
pub struct WuWuKujiGachaResponse {
    pub TrickType: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub Awards: Vec<SingleGachaAwardPb>,
}

impl WuWuKujiGachaResponse {
    pub const MSG_ID: u16 = 17008;
}

impl PbMessage for WuWuKujiGachaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrickType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrickType);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Awards {
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
            3 => self.TrickType = r.read_varint_i32()?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Awards
                    .push(<SingleGachaAwardPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuPackageRewardRequest {
    pub WrapId: i32,
}

impl WuWuPackageRewardRequest {
    pub const MSG_ID: u16 = 18557;
}

impl PbMessage for WuWuPackageRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WrapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WrapId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.WrapId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WuWuPackageRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WuWuPackageRewardResponse {
    pub const MSG_ID: u16 = 17971;
}

impl PbMessage for WuWuPackageRewardResponse {
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
pub struct WuWuTaskRewardRequest {
    pub Ids: Vec<i32>,
}

impl WuWuTaskRewardRequest {
    pub const MSG_ID: u16 = 20996;
}

impl PbMessage for WuWuTaskRewardRequest {
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
pub struct WuWuTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl WuWuTaskRewardResponse {
    pub const MSG_ID: u16 = 20425;
}

impl PbMessage for WuWuTaskRewardResponse {
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
pub struct WuWuTaskUpdateNotify {
    pub Task: Option<ConditionTask>,
}

impl WuWuTaskUpdateNotify {
    pub const MSG_ID: u16 = 24180;
}

impl PbMessage for WuWuTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
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
pub struct XboxSettingUpdateRequest {
    pub XboxBlockUserCreatedContentViewing: bool,
    pub MatchXboxUser: bool,
}

impl XboxSettingUpdateRequest {
    pub const MSG_ID: u16 = 17883;
}

impl PbMessage for XboxSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.XboxBlockUserCreatedContentViewing {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MatchXboxUser {
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
            7 => self.XboxBlockUserCreatedContentViewing = r.read_bool()?,
            13 => self.MatchXboxUser = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct XboxSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl XboxSettingUpdateResponse {
    pub const MSG_ID: u16 = 16049;
}

impl PbMessage for XboxSettingUpdateResponse {
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
pub struct YuXueModeInfoNotify {
    pub GlobalSetting: i32,
    pub BtObjYuXueModeInfos: Vec<BtObjYuXueModeInfo>,
}

impl YuXueModeInfoNotify {
    pub const MSG_ID: u16 = 19246;
}

impl PbMessage for YuXueModeInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GlobalSetting != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GlobalSetting);
        }
        for item in &self.BtObjYuXueModeInfos {
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
            1 => self.GlobalSetting = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.BtObjYuXueModeInfos
                    .push(<BtObjYuXueModeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZitherPlayNoteNotify {
    pub Note: Vec<u8>,
    pub EntityId: i64,
}

impl ZitherPlayNoteNotify {
    pub const MSG_ID: u16 = 19034;
}

impl PbMessage for ZitherPlayNoteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Note.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Note);
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
            1 => self.Note = r.read_len_delimited()?.to_vec(),
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZitherPlayNotePush {
    pub Note: Vec<u8>,
}

impl ZitherPlayNotePush {
    pub const MSG_ID: u16 = 16749;
}

impl PbMessage for ZitherPlayNotePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Note.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Note);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Note = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZitherPlayStateRequest {
    pub State: Option<ZitherOpState>,
}

impl ZitherPlayStateRequest {
    pub const MSG_ID: u16 = 21137;
}

impl PbMessage for ZitherPlayStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.State {
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
                self.State = Some(<ZitherOpState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ZitherPlayStateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ZitherPlayStateResponse {
    pub const MSG_ID: u16 = 25209;
}

impl PbMessage for ZitherPlayStateResponse {
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
pub struct roleInfo {
    pub RoleId: i32,
    pub Name: Vec<u8>,
    pub Level: i32,
    pub Exp: i32,
    pub Breakthrough: i32,
    pub Skills: Vec<ArrayIntInt>,
    pub Phantom: Vec<ArrayIntInt>,
    pub Star: i32,
    pub Favor: i32,
    pub Reson: Vec<ResonInfo>,
    pub CurModel: i32,
    pub Models: Vec<i32>,
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
    pub CreateTime: u32,
    pub SkillNodeState: Vec<ArraySkillNode>,
    pub ResonantChainGroupIndex: i32,
    pub SkinId: i32,
    pub EnableSelfBgm: bool,
}

impl PbMessage for roleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        if self.Breakthrough != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Breakthrough);
        }
        for item in &self.Skills {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.Phantom {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.Star != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Star);
        }
        if self.Favor != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Favor);
        }
        for item in &self.Reson {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.CurModel != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurModel);
        }
        for item in &self.Models {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BaseProp {
            crate::wire::write_submessage(out, 14, item);
        }
        for item in &self.AddProp {
            crate::wire::write_submessage(out, 15, item);
        }
        if self.CreateTime != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.CreateTime));
        }
        for item in &self.SkillNodeState {
            crate::wire::write_submessage(out, 19, item);
        }
        if self.ResonantChainGroupIndex != 0 {
            crate::wire::write_tag(out, 20, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResonantChainGroupIndex);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.EnableSelfBgm {
            crate::wire::write_tag(out, 22, WireType::Varint);
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
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => self.Level = r.read_varint_i32()?,
            4 => self.Exp = r.read_varint_i32()?,
            5 => self.Breakthrough = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Skills.push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Phantom
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            8 => self.Star = r.read_varint_i32()?,
            10 => self.Favor = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Reson.push(<ResonInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.CurModel = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Models, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.BaseProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.AddProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            17 => self.CreateTime = r.read_varint_u32()?,
            19 => {
                let bytes = r.read_len_delimited()?;
                self.SkillNodeState
                    .push(<ArraySkillNode as PbMessage>::decode(bytes)?);
            }
            20 => self.ResonantChainGroupIndex = r.read_varint_i32()?,
            21 => self.SkinId = r.read_varint_i32()?,
            22 => self.EnableSelfBgm = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankScoreRange {
    pub PercentCode: u32,
    pub MaxScore: u64,
    pub MinScore: u64,
    pub HasScore: bool,
}

impl PbMessage for RoleTowerRankScoreRange {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PercentCode != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.PercentCode));
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.MaxScore as u64).to_le_bytes());
        }
        if self.MinScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed64);
            out.extend_from_slice(&(self.MinScore as u64).to_le_bytes());
        }
        if self.HasScore {
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
            1 => self.PercentCode = r.read_varint_u32()?,
            2 => self.MaxScore = r.read_fixed64()? as u64,
            3 => self.MinScore = r.read_fixed64()? as u64,
            4 => self.HasScore = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankRangeTable {
    pub LevelId: i32,
    pub TotalCount: i64,
    pub BuildUnixTime: i64,
    pub MaxPercentCode: u32,
    pub Ranges: Vec<RoleTowerRankScoreRange>,
}

impl PbMessage for RoleTowerRankRangeTable {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCount);
        }
        if self.BuildUnixTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuildUnixTime);
        }
        if self.MaxPercentCode != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.MaxPercentCode));
        }
        for item in &self.Ranges {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.TotalCount = r.read_varint_i64()?,
            3 => self.BuildUnixTime = r.read_varint_i64()?,
            4 => self.MaxPercentCode = r.read_varint_u32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Ranges
                    .push(<RoleTowerRankScoreRange as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiControlSwitch {
    pub EntityId: i64,
    pub AiInfo: Option<AiInformation>,
    pub PlayerId: i32,
}

impl PbMessage for AiControlSwitch {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.AiInfo {
            crate::wire::write_submessage(out, 2, v);
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AiInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AiControlSwitchRequest {
    pub EntityId: i64,
}

impl AiControlSwitchRequest {
    pub const MSG_ID: u16 = 27014;
}

impl PbMessage for AiControlSwitchRequest {
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
pub struct AiControlSwitchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AiControlSwitchResponse {
    pub const MSG_ID: u16 = 29374;
}

impl PbMessage for AiControlSwitchResponse {
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
pub struct AiControlSwitchNotify {
    pub AiControlSwitchInfos: Vec<AiControlSwitch>,
}

impl AiControlSwitchNotify {
    pub const MSG_ID: u16 = 16041;
}

impl PbMessage for AiControlSwitchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AiControlSwitchInfos {
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
                self.AiControlSwitchInfos
                    .push(<AiControlSwitch as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreAiControlSwitchNotify {
    pub EntityIds: Vec<i64>,
}

impl PreAiControlSwitchNotify {
    pub const MSG_ID: u16 = 29939;
}

impl PbMessage for PreAiControlSwitchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheatInputRequest {
    pub Type: Option<ECheatInput>,
    pub AoeDestroyEnemyRange: i32,
    pub EntityList: Vec<i64>,
}

impl CheatInputRequest {
    pub const MSG_ID: u16 = 16047;
}

impl PbMessage for CheatInputRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AoeDestroyEnemyRange != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AoeDestroyEnemyRange);
        }
        for item in &self.EntityList {
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
            5 => {
                self.Type = Some(<ECheatInput as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.AoeDestroyEnemyRange = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityList, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheatInputResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CheatInputResponse {
    pub const MSG_ID: u16 = 27253;
}

impl PbMessage for CheatInputResponse {
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
pub struct EnableDamageRecordRequest {
    pub Enable: bool,
    pub DamageRandomSeed: i32,
}

impl EnableDamageRecordRequest {
    pub const MSG_ID: u16 = 27554;
}

impl PbMessage for EnableDamageRecordRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Enable {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DamageRandomSeed != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DamageRandomSeed);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Enable = r.read_bool()?,
            7 => self.DamageRandomSeed = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnableDamageRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnableDamageRecordResponse {
    pub const MSG_ID: u16 = 27761;
}

impl PbMessage for EnableDamageRecordResponse {
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
pub struct VoxelSpan {
    pub X: f32,
    pub Y: f32,
    pub Smin: f32,
    pub Smax: f32,
}

impl PbMessage for VoxelSpan {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.Smin != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Smin).to_bits().to_le_bytes());
        }
        if self.Smax != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Smax).to_bits().to_le_bytes());
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
            2 => self.Y = r.read_f32()?,
            3 => self.Smin = r.read_f32()?,
            4 => self.Smax = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmVoxelInfoNotify {
    pub CellSize: f32,
    pub Spans: Vec<VoxelSpan>,
}

impl GmVoxelInfoNotify {
    pub const MSG_ID: u16 = 23572;
}

impl PbMessage for GmVoxelInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CellSize != 0.0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.CellSize).to_bits().to_le_bytes());
        }
        for item in &self.Spans {
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
            8 => self.CellSize = r.read_f32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Spans.push(<VoxelSpan as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmIsOverlapNotify {
    pub CellSize: f32,
    pub Box: Option<VoxelSpan>,
    pub IsOverlap: bool,
}

impl GmIsOverlapNotify {
    pub const MSG_ID: u16 = 26170;
}

impl PbMessage for GmIsOverlapNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CellSize != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.CellSize).to_bits().to_le_bytes());
        }
        if let Some(v) = &self.Box {
            crate::wire::write_submessage(out, 15, v);
        }
        if self.IsOverlap {
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
            1 => self.CellSize = r.read_f32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Box
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.IsOverlap = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OutputDebugInfoNotify {}

impl OutputDebugInfoNotify {
    pub const MSG_ID: u16 = 26742;
}

impl PbMessage for OutputDebugInfoNotify {
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
pub struct PlayerFollowerPb {
    pub Type: Option<EPlayerFollower>,
    pub EntityId: i64,
}

impl PbMessage for PlayerFollowerPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
                self.Type = Some(
                    <EPlayerFollower as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerFollowerNotify {
    pub PlayerId: i32,
    pub ListPlayerFollowerPb: Vec<PlayerFollowerPb>,
}

impl PlayerFollowerNotify {
    pub const MSG_ID: u16 = 27250;
}

impl PbMessage for PlayerFollowerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.ListPlayerFollowerPb {
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
            6 => self.PlayerId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ListPlayerFollowerPb
                    .push(<PlayerFollowerPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetDebugInfoNotify {
    pub LogId: Vec<u8>,
}

impl GetDebugInfoNotify {
    pub const MSG_ID: u16 = 20169;
}

impl PbMessage for GetDebugInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.LogId.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LogId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.LogId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DebugFightErrInfoNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub ExtraInfo: Vec<u8>,
}

impl DebugFightErrInfoNotify {
    pub const MSG_ID: u16 = 28626;
}

impl PbMessage for DebugFightErrInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ExtraInfo.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ExtraInfo);
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
            10 => self.ExtraInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DebugDrawPosNotify {
    pub PosList: Vec<Vector>,
    pub Duration: i32,
    pub Radius: i32,
    pub MapId: i32,
}

impl DebugDrawPosNotify {
    pub const MSG_ID: u16 = 20580;
}

impl PbMessage for DebugDrawPosNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PosList {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.Radius != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Radius);
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
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
                self.PosList.push(<Vector as PbMessage>::decode(bytes)?);
            }
            11 => self.Duration = r.read_varint_i32()?,
            2 => self.Radius = r.read_varint_i32()?,
            15 => self.MapId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
