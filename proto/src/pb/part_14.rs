use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemMoveSplineRequest {
    pub MoveStatus: Option<MoveStatus>,
    pub EntityId: i64,
    pub RuntimeData: Option<SceneItemSplineRuntimeData>,
    pub SplineEntityId: i32,
    pub MoveSplineConfig: Option<MoveSplineConfig>,
}

impl SceneItemMoveSplineRequest {
    pub const MSG_ID: u16 = 28835;
}

impl PbMessage for SceneItemMoveSplineRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MoveStatus {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.RuntimeData {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
        }
        if let Some(v) = &self.MoveSplineConfig {
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
            2 => {
                self.MoveStatus = Some(<MoveStatus as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.EntityId = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RuntimeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.SplineEntityId = r.read_varint_i32()?,
            13 => {
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
pub struct SceneItemMoveSplineResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneItemMoveSplineResponse {
    pub const MSG_ID: u16 = 24723;
}

impl PbMessage for SceneItemMoveSplineResponse {
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
pub struct SceneItemMoveTargetInfo {
    pub EntityId: i64,
    pub Location: Option<Vector>,
    pub MoveTime: f32,
    pub StayTime: f32,
    pub MaxSpeed: f32,
    pub Acceleration: f32,
}

impl PbMessage for SceneItemMoveTargetInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.MoveTime != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.MoveTime).to_bits().to_le_bytes());
        }
        if self.StayTime != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.StayTime).to_bits().to_le_bytes());
        }
        if self.MaxSpeed != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.MaxSpeed).to_bits().to_le_bytes());
        }
        if self.Acceleration != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Acceleration).to_bits().to_le_bytes());
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
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.MoveTime = r.read_f32()?,
            4 => self.StayTime = r.read_f32()?,
            5 => self.MaxSpeed = r.read_f32()?,
            6 => self.Acceleration = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemMoveTargetNotify {
    pub MoveInfo: Option<SceneItemMoveTargetInfo>,
}

impl SceneItemMoveTargetNotify {
    pub const MSG_ID: u16 = 22412;
}

impl PbMessage for SceneItemMoveTargetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MoveInfo {
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
                self.MoveInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemMoveTargetRequest {
    pub SceneOwnerId: i32,
    pub MoveInfo: Option<SceneItemMoveTargetInfo>,
}

impl SceneItemMoveTargetRequest {
    pub const MSG_ID: u16 = 24132;
}

impl PbMessage for SceneItemMoveTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
        }
        if let Some(v) = &self.MoveInfo {
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
            1 => self.SceneOwnerId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.MoveInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemMoveTargetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneItemMoveTargetResponse {
    pub const MSG_ID: u16 = 23917;
}

impl PbMessage for SceneItemMoveTargetResponse {
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
pub struct SceneItemPatrolInfoPb {
    pub PatrolPointIndex: i32,
}

impl PbMessage for SceneItemPatrolInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PatrolPointIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PatrolPointIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PatrolPointIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemSequenceFrameEndRequest {
    pub EntityId: i64,
    pub HostPlayerId: i32,
    pub EventName: Vec<u8>,
    pub SeqGuid: Vec<u8>,
}

impl SceneItemSequenceFrameEndRequest {
    pub const MSG_ID: u16 = 24870;
}

impl PbMessage for SceneItemSequenceFrameEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if !self.EventName.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.EventName);
        }
        if !self.SeqGuid.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SeqGuid);
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
            3 => self.HostPlayerId = r.read_varint_i32()?,
            5 => self.EventName = r.read_len_delimited()?.to_vec(),
            14 => self.SeqGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemSequenceFrameEndResponse {
    pub Code: Option<ErrorCode>,
}

impl SceneItemSequenceFrameEndResponse {
    pub const MSG_ID: u16 = 24461;
}

impl PbMessage for SceneItemSequenceFrameEndResponse {
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
pub struct SceneItemSequenceFrameStartRequest {
    pub SeqGuid: Vec<u8>,
    pub EventName: Vec<u8>,
    pub HostPlayerId: i32,
    pub EntityId: i64,
}

impl SceneItemSequenceFrameStartRequest {
    pub const MSG_ID: u16 = 20858;
}

impl PbMessage for SceneItemSequenceFrameStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SeqGuid.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SeqGuid);
        }
        if !self.EventName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.EventName);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
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
            1 => self.SeqGuid = r.read_len_delimited()?.to_vec(),
            3 => self.EventName = r.read_len_delimited()?.to_vec(),
            4 => self.HostPlayerId = r.read_varint_i32()?,
            9 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemSequenceFrameStartResponse {
    pub Code: Option<ErrorCode>,
}

impl SceneItemSequenceFrameStartResponse {
    pub const MSG_ID: u16 = 16917;
}

impl PbMessage for SceneItemSequenceFrameStartResponse {
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
pub struct SceneItemSetPosSenderNotify {
    pub EntityId: i64,
    pub PosSender: i32,
}

impl SceneItemSetPosSenderNotify {
    pub const MSG_ID: u16 = 15429;
}

impl PbMessage for SceneItemSetPosSenderNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PosSender != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PosSender);
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
            14 => self.PosSender = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemSetPosSenderRequest {
    pub EntityId: i64,
    pub IsControl: bool,
}

impl SceneItemSetPosSenderRequest {
    pub const MSG_ID: u16 = 26876;
}

impl PbMessage for SceneItemSetPosSenderRequest {
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
pub struct SceneItemSetPosSenderResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneItemSetPosSenderResponse {
    pub const MSG_ID: u16 = 19612;
}

impl PbMessage for SceneItemSetPosSenderResponse {
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
pub struct SceneItemStateRequest {
    pub EntityId: i64,
    pub OperateType: i32,
}

impl SceneItemStateRequest {
    pub const MSG_ID: u16 = 23046;
}

impl PbMessage for SceneItemStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.OperateType != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OperateType);
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
            10 => self.OperateType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneItemStateResponse {
    pub const MSG_ID: u16 = 17526;
}

impl PbMessage for SceneItemStateResponse {
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
pub struct SceneItemStopInfo {
    pub EntityId: i64,
    pub Location: Option<Vector>,
}

impl PbMessage for SceneItemStopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            1 => self.EntityId = r.read_varint_i64()?,
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
pub struct SceneItemStopRequest {
    pub StopInfos: Vec<SceneItemStopInfo>,
    pub SceneOwnerId: i32,
}

impl SceneItemStopRequest {
    pub const MSG_ID: u16 = 29926;
}

impl PbMessage for SceneItemStopRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StopInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneOwnerId);
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
                self.StopInfos
                    .push(<SceneItemStopInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.SceneOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemStopResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneItemStopResponse {
    pub const MSG_ID: u16 = 20130;
}

impl PbMessage for SceneItemStopResponse {
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
pub struct SceneLoadingFinishNotify {
    pub PlayerId: i32,
}

impl SceneLoadingFinishNotify {
    pub const MSG_ID: u16 = 28319;
}

impl PbMessage for SceneLoadingFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneLoadingTimeOutNotify {}

impl SceneLoadingTimeOutNotify {
    pub const MSG_ID: u16 = 15159;
}

impl PbMessage for SceneLoadingTimeOutNotify {
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
pub struct SceneModeChangeFinishRequest {}

impl SceneModeChangeFinishRequest {
    pub const MSG_ID: u16 = 16826;
}

impl PbMessage for SceneModeChangeFinishRequest {
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
pub struct SceneModeChangeFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneModeChangeFinishResponse {
    pub const MSG_ID: u16 = 20026;
}

impl PbMessage for SceneModeChangeFinishResponse {
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
pub struct ScenePlayerOfflineNotify {
    pub IsOffline: bool,
    pub PlayerId: i32,
    pub IsReLogin: bool,
}

impl ScenePlayerOfflineNotify {
    pub const MSG_ID: u16 = 21816;
}

impl PbMessage for ScenePlayerOfflineNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsOffline {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IsReLogin {
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
            10 => self.IsOffline = r.read_bool()?,
            11 => self.PlayerId = r.read_varint_i32()?,
            15 => self.IsReLogin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepConditionProgress {
    pub ConditionId: i32,
    pub ProgressNum: i32,
}

impl PbMessage for SceneStepConditionProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.ProgressNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ProgressNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConditionId = r.read_varint_i32()?,
            2 => self.ProgressNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepGroupInfo {
    pub StepGroupId: i32,
    pub Steps: Vec<SceneStepInfo>,
}

impl PbMessage for SceneStepGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StepGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StepGroupId);
        }
        for item in &self.Steps {
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
            1 => self.StepGroupId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Steps
                    .push(<SceneStepInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepGroupInfoNotify {
    pub StepGroupInfo: Vec<SceneStepGroupInfo>,
}

impl SceneStepGroupInfoNotify {
    pub const MSG_ID: u16 = 17210;
}

impl PbMessage for SceneStepGroupInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StepGroupInfo {
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
                self.StepGroupInfo
                    .push(<SceneStepGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepInfo {
    pub StepId: i32,
    pub Status: Option<SceneStepStatus>,
    pub Progress: Vec<SceneStepConditionProgress>,
    pub MultiPlayerProgress: Vec<SceneStepPlayerProgress>,
}

impl PbMessage for SceneStepInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StepId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StepId);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Progress {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.MultiPlayerProgress {
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
            1 => self.StepId = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <SceneStepStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Progress
                    .push(<SceneStepConditionProgress as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MultiPlayerProgress
                    .push(<SceneStepPlayerProgress as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepInfoNotify {
    pub StepGroupId: i32,
    pub StepInfo: Option<SceneStepInfo>,
}

impl SceneStepInfoNotify {
    pub const MSG_ID: u16 = 16336;
}

impl PbMessage for SceneStepInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StepGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StepGroupId);
        }
        if let Some(v) = &self.StepInfo {
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
            1 => self.StepGroupId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.StepInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneStepPlayerProgress {
    pub PlayerId: i32,
    pub Progress: Vec<SceneStepConditionProgress>,
}

impl PbMessage for SceneStepPlayerProgress {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.Progress {
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
                self.Progress
                    .push(<SceneStepConditionProgress as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneTempFishPointInfoUpdateNotify {
    pub SceneConfigId: i32,
    pub IsRemove: bool,
    pub FishPoint: Option<TempFishPointInfo>,
}

impl SceneTempFishPointInfoUpdateNotify {
    pub const MSG_ID: u16 = 16073;
}

impl PbMessage for SceneTempFishPointInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneConfigId);
        }
        if self.IsRemove {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.FishPoint {
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
            3 => self.SceneConfigId = r.read_varint_i32()?,
            11 => self.IsRemove = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.FishPoint
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneTestNotify {
    pub PlayerId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorMsg: Vec<u8>,
}

impl SceneTestNotify {
    pub const MSG_ID: u16 = 25522;
}

impl PbMessage for SceneTestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ErrorMsg.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ErrorMsg);
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.ErrorMsg = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScheduleNpcUpdatePosRequest {
    pub EntityId: i64,
    pub InstId: i32,
}

impl ScheduleNpcUpdatePosRequest {
    pub const MSG_ID: u16 = 21451;
}

impl PbMessage for ScheduleNpcUpdatePosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            10 => self.EntityId = r.read_varint_i64()?,
            14 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScheduleNpcUpdatePosResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ScheduleNpcUpdatePosResponse {
    pub const MSG_ID: u16 = 19698;
}

impl PbMessage for ScheduleNpcUpdatePosResponse {
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
pub struct ScheduleRewardRequest {
    pub RewardId: i32,
    pub ActivityId: i32,
}

impl ScheduleRewardRequest {
    pub const MSG_ID: u16 = 15291;
}

impl PbMessage for ScheduleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
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
            9 => self.RewardId = r.read_varint_i32()?,
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScheduleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ScheduleRewardResponse {
    pub const MSG_ID: u16 = 24763;
}

impl PbMessage for ScheduleRewardResponse {
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
pub struct ScheduleRewardUpdateNotify {
    pub ActivityId: i32,
    pub ScheduleRewardDatas: Vec<CiacconaScheduleRewardPbData>,
}

impl ScheduleRewardUpdateNotify {
    pub const MSG_ID: u16 = 18703;
}

impl PbMessage for ScheduleRewardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ScheduleRewardDatas {
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
            1 => self.ActivityId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ScheduleRewardDatas
                    .push(<CiacconaScheduleRewardPbData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardActivityInfo {
    pub RoundInfos: Vec<ScratchCardRoundInfo>,
    pub ScratchTimeInfos: Vec<ScratchCardTimeInfo>,
}

impl PbMessage for ScratchCardActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoundInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ScratchTimeInfos {
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
                self.RoundInfos
                    .push(<ScratchCardRoundInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ScratchTimeInfos
                    .push(<ScratchCardTimeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardActivityInfoRequest {
    pub ActivityId: i32,
}

impl ScratchCardActivityInfoRequest {
    pub const MSG_ID: u16 = 27153;
}

impl PbMessage for ScratchCardActivityInfoRequest {
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
pub struct ScratchCardActivityInfoResponse {
    pub Error: Option<ErrorCode>,
    pub Info: Option<ScratchCardActivityInfo>,
}

impl ScratchCardActivityInfoResponse {
    pub const MSG_ID: u16 = 24195;
}

impl PbMessage for ScratchCardActivityInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Info {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
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
pub struct ScratchCardCountInfoNotify {
    pub ScratchTimeInfos: Vec<ScratchCardTimeInfo>,
}

impl ScratchCardCountInfoNotify {
    pub const MSG_ID: u16 = 16808;
}

impl PbMessage for ScratchCardCountInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScratchTimeInfos {
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
                self.ScratchTimeInfos
                    .push(<ScratchCardTimeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardRewardRequest {
    pub Index: i32,
    pub RoundId: i32,
}

impl ScratchCardRewardRequest {
    pub const MSG_ID: u16 = 23653;
}

impl PbMessage for ScratchCardRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.RoundId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundId);
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
            5 => self.RoundId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardRewardResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub OpenType: i32,
    pub OpenIndex: BTreeMap<i32, ScratchCardRewardData>,
    pub Error: Option<ErrorCode>,
    pub LeftRewardItem: BTreeMap<i32, i32>,
}

impl ScratchCardRewardResponse {
    pub const MSG_ID: u16 = 15806;
}

impl PbMessage for ScratchCardRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 2, &self.ItemMap);
        }
        if self.OpenType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OpenType);
        }
        if !self.OpenIndex.is_empty() {
            crate::wire::write_map(out, 10, &self.OpenIndex);
        }
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.LeftRewardItem.is_empty() {
            crate::wire::write_map(out, 13, &self.LeftRewardItem);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            7 => self.OpenType = r.read_varint_i32()?,
            10 => crate::wire::decode_map_entry(r, &mut self.OpenIndex)?,
            12 => {
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => crate::wire::decode_map_entry(r, &mut self.LeftRewardItem)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardRoundInfo {
    pub RoundId: i32,
    pub UnlockTime: i64,
    pub RewardIds: BTreeMap<i32, ScratchCardRewardData>,
    pub LeftRewardItem: BTreeMap<i32, i32>,
}

impl PbMessage for ScratchCardRoundInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if !self.RewardIds.is_empty() {
            crate::wire::write_map(out, 3, &self.RewardIds);
        }
        if !self.LeftRewardItem.is_empty() {
            crate::wire::write_map(out, 4, &self.LeftRewardItem);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoundId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => crate::wire::decode_map_entry(r, &mut self.RewardIds)?,
            4 => crate::wire::decode_map_entry(r, &mut self.LeftRewardItem)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScratchCardTimeInfo {
    pub Id: i32,
    pub Progress: i32,
    pub FinishCount: i32,
}

impl PbMessage for ScratchCardTimeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.FinishCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Progress = r.read_varint_i32()?,
            3 => self.FinishCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeamlessTeleportArgs {
    pub ClientTriggerTeleportPb: Option<ClientTriggerTeleportPb>,
    pub NoRenderPortalCtx: Option<NoRenderPortalCtx>,
    pub LongPressInteractTeleportPb: Option<LongPressInteractTeleportPb>,
}

impl PbMessage for SeamlessTeleportArgs {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ClientTriggerTeleportPb {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.NoRenderPortalCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.LongPressInteractTeleportPb {
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
                self.ClientTriggerTeleportPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NoRenderPortalCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LongPressInteractTeleportPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeamlessTeleportRequest {
    pub Rotation: Option<Rotator>,
    pub TeleportArgs: Option<SeamlessTeleportArgs>,
    pub TeleportType: Option<SeamlessTeleportType>,
    pub VehicleId: i64,
    pub Location: Option<Vector>,
}

impl SeamlessTeleportRequest {
    pub const MSG_ID: u16 = 17907;
}

impl PbMessage for SeamlessTeleportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.TeleportArgs {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.TeleportType {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.VehicleId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleId);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TeleportArgs
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                self.TeleportType = Some(
                    <SeamlessTeleportType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.VehicleId = r.read_varint_i64()?,
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
pub struct SeamlessTeleportResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SeamlessTeleportResponse {
    pub const MSG_ID: u16 = 15205;
}

impl PbMessage for SeamlessTeleportResponse {
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
pub struct SeamlessTeleportVehicleRequest {
    pub VehicleEntityId: i64,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
}

impl SeamlessTeleportVehicleRequest {
    pub const MSG_ID: u16 = 16695;
}

impl PbMessage for SeamlessTeleportVehicleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VehicleEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleEntityId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Rotation {
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
            4 => self.VehicleEntityId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeamlessTeleportVehicleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SeamlessTeleportVehicleResponse {
    pub const MSG_ID: u16 = 24419;
}

impl PbMessage for SeamlessTeleportVehicleResponse {
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
pub struct SeasonData {
    pub SeasonId: i32,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RoguelikeTokenList: Vec<RoguelikeToken>,
    pub SeasonRewardList: Vec<SeasonReward>,
    pub TokenItemCount: i32,
}

impl PbMessage for SeasonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.RoguelikeTokenList {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.SeasonRewardList {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.TokenItemCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenItemCount);
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
            2 => self.StartTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeTokenList
                    .push(<RoguelikeToken as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SeasonRewardList
                    .push(<SeasonReward as PbMessage>::decode(bytes)?);
            }
            6 => self.TokenItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SeasonReward {
    pub Id: i32,
    pub IsReceive: bool,
}

impl PbMessage for SeasonReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.IsReceive {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.IsReceive = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecFBRoundEndFlowReportRequest {
    pub SprintMin: i32,
    pub PSCostTotal: i64,
    pub LogId: i64,
    pub RoundTimeUse: i64,
    pub PSReplySpeedMax: i32,
    pub SpeedMax: i32,
    pub PSUseCount: i32,
    pub SpeedMin: i32,
    pub PSReplyTotal: i64,
    pub ItemGetDistanceMax: i32,
    pub PSUseNumMin: i32,
    pub ItemGetCount: i32,
    pub SpeedAvg: i32,
    pub PSUseNumMax: i32,
    pub SprintMax: i32,
    pub ComboMax: i32,
    pub ItemGetDistanceMin: i32,
    pub PSReplySpeedMin: i32,
    pub ClientEndTime: Vec<u8>,
}

impl SecFBRoundEndFlowReportRequest {
    pub const MSG_ID: u16 = 17956;
}

impl PbMessage for SecFBRoundEndFlowReportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SprintMin != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMin);
        }
        if self.PSCostTotal != 0 {
            crate::wire::write_tag(out, 28, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PSCostTotal);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 29, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if self.RoundTimeUse != 0 {
            crate::wire::write_tag(out, 48, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RoundTimeUse);
        }
        if self.PSReplySpeedMax != 0 {
            crate::wire::write_tag(out, 66, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMax);
        }
        if self.SpeedMax != 0 {
            crate::wire::write_tag(out, 104, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMax);
        }
        if self.PSUseCount != 0 {
            crate::wire::write_tag(out, 136, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseCount);
        }
        if self.SpeedMin != 0 {
            crate::wire::write_tag(out, 146, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMin);
        }
        if self.PSReplyTotal != 0 {
            crate::wire::write_tag(out, 157, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PSReplyTotal);
        }
        if self.ItemGetDistanceMax != 0 {
            crate::wire::write_tag(out, 158, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMax);
        }
        if self.PSUseNumMin != 0 {
            crate::wire::write_tag(out, 184, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMin);
        }
        if self.ItemGetCount != 0 {
            crate::wire::write_tag(out, 190, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetCount);
        }
        if self.SpeedAvg != 0 {
            crate::wire::write_tag(out, 193, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedAvg);
        }
        if self.PSUseNumMax != 0 {
            crate::wire::write_tag(out, 201, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMax);
        }
        if self.SprintMax != 0 {
            crate::wire::write_tag(out, 223, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMax);
        }
        if self.ComboMax != 0 {
            crate::wire::write_tag(out, 235, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ComboMax);
        }
        if self.ItemGetDistanceMin != 0 {
            crate::wire::write_tag(out, 236, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMin);
        }
        if self.PSReplySpeedMin != 0 {
            crate::wire::write_tag(out, 246, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMin);
        }
        if !self.ClientEndTime.is_empty() {
            crate::wire::write_tag(out, 249, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientEndTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            17 => self.SprintMin = r.read_varint_i32()?,
            28 => self.PSCostTotal = r.read_varint_i64()?,
            29 => self.LogId = r.read_varint_i64()?,
            48 => self.RoundTimeUse = r.read_varint_i64()?,
            66 => self.PSReplySpeedMax = r.read_varint_i32()?,
            104 => self.SpeedMax = r.read_varint_i32()?,
            136 => self.PSUseCount = r.read_varint_i32()?,
            146 => self.SpeedMin = r.read_varint_i32()?,
            157 => self.PSReplyTotal = r.read_varint_i64()?,
            158 => self.ItemGetDistanceMax = r.read_varint_i32()?,
            184 => self.PSUseNumMin = r.read_varint_i32()?,
            190 => self.ItemGetCount = r.read_varint_i32()?,
            193 => self.SpeedAvg = r.read_varint_i32()?,
            201 => self.PSUseNumMax = r.read_varint_i32()?,
            223 => self.SprintMax = r.read_varint_i32()?,
            235 => self.ComboMax = r.read_varint_i32()?,
            236 => self.ItemGetDistanceMin = r.read_varint_i32()?,
            246 => self.PSReplySpeedMin = r.read_varint_i32()?,
            249 => self.ClientEndTime = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecFBRoundEndFlowReportResponse {
    pub Error: Option<DErrorResult>,
    pub LogId: i64,
}

impl SecFBRoundEndFlowReportResponse {
    pub const MSG_ID: u16 = 16776;
}

impl PbMessage for SecFBRoundEndFlowReportResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
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
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecFBRoundStartFlowRequest {
    pub LogId: i64,
    pub SrvFighter1Info: Option<SrvFighterInfo>,
    pub SrvFighter3Info: Option<SrvFighterInfo>,
    pub ClientStartTime: Vec<u8>,
    pub SrvFighter4Info: Option<SrvFighterInfo>,
    pub SrvFighter2Info: Option<SrvFighterInfo>,
}

impl SecFBRoundStartFlowRequest {
    pub const MSG_ID: u16 = 23993;
}

impl PbMessage for SecFBRoundStartFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LogId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if let Some(v) = &self.SrvFighter1Info {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.SrvFighter3Info {
            crate::wire::write_submessage(out, 6, v);
        }
        if !self.ClientStartTime.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientStartTime);
        }
        if let Some(v) = &self.SrvFighter4Info {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.SrvFighter2Info {
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
            2 => self.LogId = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter1Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter3Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.ClientStartTime = r.read_len_delimited()?.to_vec(),
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter4Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter2Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecFBRoundStartFlowResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LogId: i64,
}

impl SecFBRoundStartFlowResponse {
    pub const MSG_ID: u16 = 26201;
}

impl PbMessage for SecFBRoundStartFlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
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
            12 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecItemGetFlowData {
    pub ClientTime: Vec<u8>,
    pub ItemID: i32,
    pub ItemType: i32,
    pub ItemCount: i32,
    pub ItemGetType: i32,
    pub ItemFromType: i32,
    pub ItemToType: i32,
    pub ItemPositionX1: i32,
    pub ItemPositionY1: i32,
    pub ItemPositionZ1: i32,
    pub ItemPositionX2: i32,
    pub ItemPositionY2: i32,
    pub ItemPositionZ2: i32,
    pub ItemBornTime: i64,
    pub NearBornItemcount: i64,
}

impl PbMessage for SecItemGetFlowData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ClientTime.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientTime);
        }
        if self.ItemID != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemID);
        }
        if self.ItemType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemType);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCount);
        }
        if self.ItemGetType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetType);
        }
        if self.ItemFromType != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemFromType);
        }
        if self.ItemToType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemToType);
        }
        if self.ItemPositionX1 != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionX1);
        }
        if self.ItemPositionY1 != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionY1);
        }
        if self.ItemPositionZ1 != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionZ1);
        }
        if self.ItemPositionX2 != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionX2);
        }
        if self.ItemPositionY2 != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionY2);
        }
        if self.ItemPositionZ2 != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemPositionZ2);
        }
        if self.ItemBornTime != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ItemBornTime);
        }
        if self.NearBornItemcount != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NearBornItemcount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ClientTime = r.read_len_delimited()?.to_vec(),
            2 => self.ItemID = r.read_varint_i32()?,
            3 => self.ItemType = r.read_varint_i32()?,
            4 => self.ItemCount = r.read_varint_i32()?,
            5 => self.ItemGetType = r.read_varint_i32()?,
            6 => self.ItemFromType = r.read_varint_i32()?,
            7 => self.ItemToType = r.read_varint_i32()?,
            8 => self.ItemPositionX1 = r.read_varint_i32()?,
            9 => self.ItemPositionY1 = r.read_varint_i32()?,
            10 => self.ItemPositionZ1 = r.read_varint_i32()?,
            11 => self.ItemPositionX2 = r.read_varint_i32()?,
            12 => self.ItemPositionY2 = r.read_varint_i32()?,
            13 => self.ItemPositionZ2 = r.read_varint_i32()?,
            14 => self.ItemBornTime = r.read_varint_i64()?,
            15 => self.NearBornItemcount = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecRoleFightFlowData {
    pub SrvFighterID: i32,
    pub FighterInfoMax: Option<SrvFighterInfo>,
    pub ResidentTimeTotal: i64,
    pub RoundFighterDpsTotal: i64,
    pub RoundFighterDpsCount: i32,
    pub RoundFighterAtkMissTotal: i32,
    pub RoundFighterDamage1Max: i32,
    pub RoundFighterDamage1Min: i32,
    pub RoundFighterPlayerCritCount: i32,
    pub RoundFighterDamage2Max: i32,
    pub RoundFighterDamage2Min: i32,
    pub RoundFighterButtonClickCountATK: i32,
    pub RoundFighterButtonClickCount1: i32,
    pub RoundFighterButtonClickCount2: i32,
    pub RoundFighterButtonClickCount3: i32,
    pub RoundFighterButtonClickCountJump: i32,
    pub RoundFighterButtonClickCountDodge: i32,
    pub PosList: Vec<Vector>,
}

impl PbMessage for SecRoleFightFlowData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SrvFighterID != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SrvFighterID);
        }
        if let Some(v) = &self.FighterInfoMax {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.ResidentTimeTotal != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ResidentTimeTotal);
        }
        if self.RoundFighterDpsTotal != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RoundFighterDpsTotal);
        }
        if self.RoundFighterDpsCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterDpsCount);
        }
        if self.RoundFighterAtkMissTotal != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterAtkMissTotal);
        }
        if self.RoundFighterDamage1Max != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterDamage1Max);
        }
        if self.RoundFighterDamage1Min != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterDamage1Min);
        }
        if self.RoundFighterPlayerCritCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterPlayerCritCount);
        }
        if self.RoundFighterDamage2Max != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterDamage2Max);
        }
        if self.RoundFighterDamage2Min != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterDamage2Min);
        }
        if self.RoundFighterButtonClickCountATK != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCountATK);
        }
        if self.RoundFighterButtonClickCount1 != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCount1);
        }
        if self.RoundFighterButtonClickCount2 != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCount2);
        }
        if self.RoundFighterButtonClickCount3 != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCount3);
        }
        if self.RoundFighterButtonClickCountJump != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCountJump);
        }
        if self.RoundFighterButtonClickCountDodge != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundFighterButtonClickCountDodge);
        }
        for item in &self.PosList {
            crate::wire::write_submessage(out, 18, item);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SrvFighterID = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FighterInfoMax
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.ResidentTimeTotal = r.read_varint_i64()?,
            4 => self.RoundFighterDpsTotal = r.read_varint_i64()?,
            5 => self.RoundFighterDpsCount = r.read_varint_i32()?,
            6 => self.RoundFighterAtkMissTotal = r.read_varint_i32()?,
            7 => self.RoundFighterDamage1Max = r.read_varint_i32()?,
            8 => self.RoundFighterDamage1Min = r.read_varint_i32()?,
            9 => self.RoundFighterPlayerCritCount = r.read_varint_i32()?,
            10 => self.RoundFighterDamage2Max = r.read_varint_i32()?,
            11 => self.RoundFighterDamage2Min = r.read_varint_i32()?,
            12 => self.RoundFighterButtonClickCountATK = r.read_varint_i32()?,
            13 => self.RoundFighterButtonClickCount1 = r.read_varint_i32()?,
            14 => self.RoundFighterButtonClickCount2 = r.read_varint_i32()?,
            15 => self.RoundFighterButtonClickCount3 = r.read_varint_i32()?,
            16 => self.RoundFighterButtonClickCountJump = r.read_varint_i32()?,
            17 => self.RoundFighterButtonClickCountDodge = r.read_varint_i32()?,
            18 => {
                let bytes = r.read_len_delimited()?;
                self.PosList.push(<Vector as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecRoleFightFlowRequest {
    pub LogType: Option<AceBattleAntiCheatingLogType>,
    pub LogId: i64,
    pub SecRoleFightFlowDataList: Vec<SecRoleFightFlowData>,
    pub ClientTime: Vec<u8>,
}

impl SecRoleFightFlowRequest {
    pub const MSG_ID: u16 = 21319;
}

impl PbMessage for SecRoleFightFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LogType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        for item in &self.SecRoleFightFlowDataList {
            crate::wire::write_submessage(out, 13, item);
        }
        if !self.ClientTime.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientTime);
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
                self.LogType = Some(
                    <AceBattleAntiCheatingLogType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.LogId = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.SecRoleFightFlowDataList
                    .push(<SecRoleFightFlowData as PbMessage>::decode(bytes)?);
            }
            15 => self.ClientTime = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecRoleFightFlowResponse {
    pub Error: Option<DErrorResult>,
    pub LogId: i64,
}

impl SecRoleFightFlowResponse {
    pub const MSG_ID: u16 = 29975;
}

impl PbMessage for SecRoleFightFlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
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
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldInfoFlowRequest {
    pub ItemGetCount: i32,
    pub PSUseNumMax: i32,
    pub PSReplyTotal: i32,
    pub SprintMax: i32,
    pub SprintMin: i32,
    pub PSUseCount: i32,
    pub ItemGetDistanceMax: i32,
    pub PSReplySpeedMin: i32,
    pub SpeedMin: i32,
    pub ItemGetDistanceMin: i32,
    pub ClientTime: Vec<u8>,
    pub PSUseNumMin: i32,
    pub SpeedMax: i32,
    pub PSCostTotal: i32,
    pub PSReplySpeedMax: i32,
    pub SpeedAvg: i32,
    pub LogId: i64,
}

impl SecWorldInfoFlowRequest {
    pub const MSG_ID: u16 = 24782;
}

impl PbMessage for SecWorldInfoFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemGetCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetCount);
        }
        if self.PSUseNumMax != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMax);
        }
        if self.PSReplyTotal != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplyTotal);
        }
        if self.SprintMax != 0 {
            crate::wire::write_tag(out, 43, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMax);
        }
        if self.SprintMin != 0 {
            crate::wire::write_tag(out, 51, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMin);
        }
        if self.PSUseCount != 0 {
            crate::wire::write_tag(out, 62, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseCount);
        }
        if self.ItemGetDistanceMax != 0 {
            crate::wire::write_tag(out, 70, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMax);
        }
        if self.PSReplySpeedMin != 0 {
            crate::wire::write_tag(out, 73, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMin);
        }
        if self.SpeedMin != 0 {
            crate::wire::write_tag(out, 86, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMin);
        }
        if self.ItemGetDistanceMin != 0 {
            crate::wire::write_tag(out, 92, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMin);
        }
        if !self.ClientTime.is_empty() {
            crate::wire::write_tag(out, 93, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientTime);
        }
        if self.PSUseNumMin != 0 {
            crate::wire::write_tag(out, 99, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMin);
        }
        if self.SpeedMax != 0 {
            crate::wire::write_tag(out, 131, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMax);
        }
        if self.PSCostTotal != 0 {
            crate::wire::write_tag(out, 152, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSCostTotal);
        }
        if self.PSReplySpeedMax != 0 {
            crate::wire::write_tag(out, 159, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMax);
        }
        if self.SpeedAvg != 0 {
            crate::wire::write_tag(out, 175, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedAvg);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 247, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemGetCount = r.read_varint_i32()?,
            8 => self.PSUseNumMax = r.read_varint_i32()?,
            12 => self.PSReplyTotal = r.read_varint_i32()?,
            43 => self.SprintMax = r.read_varint_i32()?,
            51 => self.SprintMin = r.read_varint_i32()?,
            62 => self.PSUseCount = r.read_varint_i32()?,
            70 => self.ItemGetDistanceMax = r.read_varint_i32()?,
            73 => self.PSReplySpeedMin = r.read_varint_i32()?,
            86 => self.SpeedMin = r.read_varint_i32()?,
            92 => self.ItemGetDistanceMin = r.read_varint_i32()?,
            93 => self.ClientTime = r.read_len_delimited()?.to_vec(),
            99 => self.PSUseNumMin = r.read_varint_i32()?,
            131 => self.SpeedMax = r.read_varint_i32()?,
            152 => self.PSCostTotal = r.read_varint_i32()?,
            159 => self.PSReplySpeedMax = r.read_varint_i32()?,
            175 => self.SpeedAvg = r.read_varint_i32()?,
            247 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldInfoFlowResponse {
    pub LogId: i64,
    pub Error: Option<DErrorResult>,
}

impl SecWorldInfoFlowResponse {
    pub const MSG_ID: u16 = 21356;
}

impl PbMessage for SecWorldInfoFlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LogId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if let Some(v) = &self.Error {
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
            5 => self.LogId = r.read_varint_i64()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldSEndFlowRequest {
    pub SprintMax: i32,
    pub PSUseNumMin: i32,
    pub ComboMax: i32,
    pub RoundTimeUse: i32,
    pub ClientEndTime: Vec<u8>,
    pub PSUseNumMax: i32,
    pub ItemGetDistanceMin: i32,
    pub SpeedMin: i32,
    pub LogId: i64,
    pub PSUseCount: i32,
    pub ItemGetCount: i32,
    pub PSReplyTotal: i64,
    pub PSCostTotal: i64,
    pub SpeedAvg: i32,
    pub SpeedMax: i32,
    pub ItemGetDistanceMax: i32,
    pub PSReplySpeedMin: i32,
    pub PSReplySpeedMax: i32,
    pub SprintMin: i32,
}

impl SecWorldSEndFlowRequest {
    pub const MSG_ID: u16 = 16078;
}

impl PbMessage for SecWorldSEndFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SprintMax != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMax);
        }
        if self.PSUseNumMin != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMin);
        }
        if self.ComboMax != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ComboMax);
        }
        if self.RoundTimeUse != 0 {
            crate::wire::write_tag(out, 23, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundTimeUse);
        }
        if !self.ClientEndTime.is_empty() {
            crate::wire::write_tag(out, 58, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientEndTime);
        }
        if self.PSUseNumMax != 0 {
            crate::wire::write_tag(out, 93, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseNumMax);
        }
        if self.ItemGetDistanceMin != 0 {
            crate::wire::write_tag(out, 104, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMin);
        }
        if self.SpeedMin != 0 {
            crate::wire::write_tag(out, 110, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMin);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 121, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if self.PSUseCount != 0 {
            crate::wire::write_tag(out, 124, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSUseCount);
        }
        if self.ItemGetCount != 0 {
            crate::wire::write_tag(out, 130, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetCount);
        }
        if self.PSReplyTotal != 0 {
            crate::wire::write_tag(out, 135, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PSReplyTotal);
        }
        if self.PSCostTotal != 0 {
            crate::wire::write_tag(out, 147, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PSCostTotal);
        }
        if self.SpeedAvg != 0 {
            crate::wire::write_tag(out, 148, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedAvg);
        }
        if self.SpeedMax != 0 {
            crate::wire::write_tag(out, 163, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpeedMax);
        }
        if self.ItemGetDistanceMax != 0 {
            crate::wire::write_tag(out, 167, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemGetDistanceMax);
        }
        if self.PSReplySpeedMin != 0 {
            crate::wire::write_tag(out, 173, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMin);
        }
        if self.PSReplySpeedMax != 0 {
            crate::wire::write_tag(out, 180, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PSReplySpeedMax);
        }
        if self.SprintMin != 0 {
            crate::wire::write_tag(out, 207, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SprintMin);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SprintMax = r.read_varint_i32()?,
            5 => self.PSUseNumMin = r.read_varint_i32()?,
            7 => self.ComboMax = r.read_varint_i32()?,
            23 => self.RoundTimeUse = r.read_varint_i32()?,
            58 => self.ClientEndTime = r.read_len_delimited()?.to_vec(),
            93 => self.PSUseNumMax = r.read_varint_i32()?,
            104 => self.ItemGetDistanceMin = r.read_varint_i32()?,
            110 => self.SpeedMin = r.read_varint_i32()?,
            121 => self.LogId = r.read_varint_i64()?,
            124 => self.PSUseCount = r.read_varint_i32()?,
            130 => self.ItemGetCount = r.read_varint_i32()?,
            135 => self.PSReplyTotal = r.read_varint_i64()?,
            147 => self.PSCostTotal = r.read_varint_i64()?,
            148 => self.SpeedAvg = r.read_varint_i32()?,
            163 => self.SpeedMax = r.read_varint_i32()?,
            167 => self.ItemGetDistanceMax = r.read_varint_i32()?,
            173 => self.PSReplySpeedMin = r.read_varint_i32()?,
            180 => self.PSReplySpeedMax = r.read_varint_i32()?,
            207 => self.SprintMin = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldSEndFlowResponse {
    pub Error: Option<DErrorResult>,
    pub LogId: i64,
}

impl SecWorldSEndFlowResponse {
    pub const MSG_ID: u16 = 17703;
}

impl PbMessage for SecWorldSEndFlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
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
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldStartFlowRequest {
    pub SrvFighter4Info: Option<SrvFighterInfo>,
    pub SrvFighter3Info: Option<SrvFighterInfo>,
    pub ClientStartTime: Vec<u8>,
    pub SrvFighter1Info: Option<SrvFighterInfo>,
    pub LogId: i64,
    pub SrvFighter2Info: Option<SrvFighterInfo>,
}

impl SecWorldStartFlowRequest {
    pub const MSG_ID: u16 = 27803;
}

impl PbMessage for SecWorldStartFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SrvFighter4Info {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.SrvFighter3Info {
            crate::wire::write_submessage(out, 5, v);
        }
        if !self.ClientStartTime.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ClientStartTime);
        }
        if let Some(v) = &self.SrvFighter1Info {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
        }
        if let Some(v) = &self.SrvFighter2Info {
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
                let bytes = r.read_len_delimited()?;
                self.SrvFighter4Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter3Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ClientStartTime = r.read_len_delimited()?.to_vec(),
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter1Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.LogId = r.read_varint_i64()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighter2Info
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SecWorldStartFlowResponse {
    pub Error: Option<DErrorResult>,
    pub LogId: i64,
}

impl SecWorldStartFlowResponse {
    pub const MSG_ID: u16 = 17814;
}

impl PbMessage for SecWorldStartFlowResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.LogId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LogId);
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
            7 => self.LogId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectBabelActivityDeTermRequest {
    pub BabelTowerLevelId: i32,
    pub BabelDeTermId: Vec<i32>,
}

impl SelectBabelActivityDeTermRequest {
    pub const MSG_ID: u16 = 17576;
}

impl PbMessage for SelectBabelActivityDeTermRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BabelTowerLevelId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BabelTowerLevelId);
        }
        for item in &self.BabelDeTermId {
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
            7 => self.BabelTowerLevelId = r.read_varint_i32()?,
            12 => {
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
pub struct SelectBabelActivityDeTermResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SelectBabelActivityDeTermResponse {
    pub const MSG_ID: u16 = 17683;
}

impl PbMessage for SelectBabelActivityDeTermResponse {
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
pub struct SelectDetectionTargetRequest {
    pub SelectDetectionTarget: Option<SelectDetectionTarget>,
    pub IsCancelSelect: bool,
}

impl SelectDetectionTargetRequest {
    pub const MSG_ID: u16 = 21057;
}

impl PbMessage for SelectDetectionTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SelectDetectionTarget {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IsCancelSelect {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SelectDetectionTarget
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.IsCancelSelect = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SelectDetectionTargetResponse {
    pub SelectDetectionTarget: Option<SelectDetectionTarget>,
    pub Code: Option<ErrorCode>,
}

impl SelectDetectionTargetResponse {
    pub const MSG_ID: u16 = 15118;
}

impl PbMessage for SelectDetectionTargetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SelectDetectionTarget {
            crate::wire::write_submessage(out, 5, v);
        }
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SelectDetectionTarget
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct SelectViewOp {
    pub RogueResOption: Option<RogueResOption>,
}

impl PbMessage for SelectViewOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResOption {
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
                self.RogueResOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServerCommandNotify {
    pub Cmd: Vec<u8>,
}

impl ServerCommandNotify {
    pub const MSG_ID: u16 = 27430;
}

impl PbMessage for ServerCommandNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Cmd.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Cmd);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Cmd = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServerConfigVersionInfoNotify {
    pub PublicJson: i32,
    pub UniverseEditor: i32,
}

impl ServerConfigVersionInfoNotify {
    pub const MSG_ID: u16 = 28553;
}

impl PbMessage for ServerConfigVersionInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PublicJson != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PublicJson);
        }
        if self.UniverseEditor != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UniverseEditor);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.PublicJson = r.read_varint_i32()?,
            12 => self.UniverseEditor = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetAlertUiVisibleRequest {
    pub AreaId: i32,
    pub Visible: bool,
}

impl SetAlertUiVisibleRequest {
    pub const MSG_ID: u16 = 25636;
}

impl PbMessage for SetAlertUiVisibleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        if self.Visible {
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
            7 => self.AreaId = r.read_varint_i32()?,
            9 => self.Visible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetAlertUiVisibleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetAlertUiVisibleResponse {
    pub const MSG_ID: u16 = 26471;
}

impl PbMessage for SetAlertUiVisibleResponse {
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
pub struct SetBaoziStateRequest {
    pub FanEntityId: i64,
    pub BaoziEntityId: i64,
    pub IsActive: i32,
}

impl SetBaoziStateRequest {
    pub const MSG_ID: u16 = 28381;
}

impl PbMessage for SetBaoziStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FanEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FanEntityId);
        }
        if self.BaoziEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BaoziEntityId);
        }
        if self.IsActive != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IsActive);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.FanEntityId = r.read_varint_i64()?,
            10 => self.BaoziEntityId = r.read_varint_i64()?,
            14 => self.IsActive = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetBaoziStateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SetBaoziStateResponse {
    pub const MSG_ID: u16 = 29269;
}

impl PbMessage for SetBaoziStateResponse {
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
pub struct SetBtObjYuXueModeRequest {
    pub BtObjSetting: i32,
    pub BtType: Option<BtType>,
    pub BtObjId: i32,
}

impl SetBtObjYuXueModeRequest {
    pub const MSG_ID: u16 = 23639;
}

impl PbMessage for SetBtObjYuXueModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BtObjSetting != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjSetting);
        }
        if let Some(v) = &self.BtType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BtObjId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BtObjId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BtObjSetting = r.read_varint_i32()?,
            5 => {
                self.BtType = Some(<BtType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.BtObjId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetBtObjYuXueModeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetBtObjYuXueModeResponse {
    pub const MSG_ID: u16 = 23727;
}

impl PbMessage for SetBtObjYuXueModeResponse {
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
pub struct SetClientGlobalVarRequest {
    pub VarDefinePb: Option<VarDefinePb>,
    pub VarName: Vec<u8>,
}

impl SetClientGlobalVarRequest {
    pub const MSG_ID: u16 = 22272;
}

impl PbMessage for SetClientGlobalVarRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.VarDefinePb {
            crate::wire::write_submessage(out, 2, v);
        }
        if !self.VarName.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VarName);
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
                self.VarDefinePb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.VarName = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetClientGlobalVarResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl SetClientGlobalVarResponse {
    pub const MSG_ID: u16 = 17849;
}

impl PbMessage for SetClientGlobalVarResponse {
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
pub struct SetFanNumberOfTurnsRequest {
    pub EntityId: i64,
    pub NumberOfTurns: i32,
}

impl SetFanNumberOfTurnsRequest {
    pub const MSG_ID: u16 = 22814;
}

impl PbMessage for SetFanNumberOfTurnsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.NumberOfTurns != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NumberOfTurns);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.EntityId = r.read_varint_i64()?,
            14 => self.NumberOfTurns = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFanNumberOfTurnsResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SetFanNumberOfTurnsResponse {
    pub const MSG_ID: u16 = 27911;
}

impl PbMessage for SetFanNumberOfTurnsResponse {
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
pub struct SetFanStateRequest {
    pub FanEntityId: i64,
    pub IsActive: i32,
}

impl SetFanStateRequest {
    pub const MSG_ID: u16 = 16914;
}

impl PbMessage for SetFanStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FanEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FanEntityId);
        }
        if self.IsActive != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IsActive);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.FanEntityId = r.read_varint_i64()?,
            12 => self.IsActive = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetFanStateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SetFanStateResponse {
    pub const MSG_ID: u16 = 24361;
}

impl PbMessage for SetFanStateResponse {
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
pub struct SetInitTagRequest {
    pub EntityId: i64,
}

impl SetInitTagRequest {
    pub const MSG_ID: u16 = 19030;
}

impl PbMessage for SetInitTagRequest {
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
pub struct SetInitTagResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetInitTagResponse {
    pub const MSG_ID: u16 = 17305;
}

impl PbMessage for SetInitTagResponse {
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
pub struct SetMatchTeamMatchFlagRequest {
    pub IsMatch: bool,
}

impl SetMatchTeamMatchFlagRequest {
    pub const MSG_ID: u16 = 22339;
}

impl PbMessage for SetMatchTeamMatchFlagRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsMatch {
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
            14 => self.IsMatch = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetMatchTeamMatchFlagResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetMatchTeamMatchFlagResponse {
    pub const MSG_ID: u16 = 19772;
}

impl PbMessage for SetMatchTeamMatchFlagResponse {
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
pub struct SetNpcPerformStateNotify {
    pub State: Vec<u8>,
    pub EntityId: i64,
    pub Type: Vec<u8>,
}

impl SetNpcPerformStateNotify {
    pub const MSG_ID: u16 = 16291;
}

impl PbMessage for SetNpcPerformStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.State.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.State);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.Type.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Type);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.State = r.read_len_delimited()?.to_vec(),
            10 => self.EntityId = r.read_varint_i64()?,
            13 => self.Type = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetPlayerYuXueModeRequest {
    pub GlobalSetting: i32,
}

impl SetPlayerYuXueModeRequest {
    pub const MSG_ID: u16 = 18715;
}

impl PbMessage for SetPlayerYuXueModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GlobalSetting != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GlobalSetting);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.GlobalSetting = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SetPlayerYuXueModeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SetPlayerYuXueModeResponse {
    pub const MSG_ID: u16 = 22623;
}

impl PbMessage for SetPlayerYuXueModeResponse {
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
pub struct SettleInfo {
    pub Id: i32,
    pub Num: i32,
}

impl PbMessage for SettleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Num = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SettleStepPbData {
    pub ChallengeId: i32,
    pub StageId: i32,
    pub FloorId: i32,
    pub BranchIndex: i32,
    pub CostTimeMs: i64,
    pub GainTimeMs: i64,
    pub RemainTimeMs: i64,
    pub TeamStarInfo: BTreeMap<i32, i32>,
    pub PhantomInfo: Option<RoleTowerPhantomInfo>,
    pub GainTokens: Vec<RoleTowerTokenCount>,
}

impl PbMessage for SettleStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.StageId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StageId);
        }
        if self.FloorId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FloorId);
        }
        if self.BranchIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchIndex);
        }
        if self.CostTimeMs != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CostTimeMs);
        }
        if self.GainTimeMs != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GainTimeMs);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        if !self.TeamStarInfo.is_empty() {
            crate::wire::write_map(out, 9, &self.TeamStarInfo);
        }
        if let Some(v) = &self.PhantomInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        for item in &self.GainTokens {
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
            2 => self.ChallengeId = r.read_varint_i32()?,
            3 => self.StageId = r.read_varint_i32()?,
            4 => self.FloorId = r.read_varint_i32()?,
            5 => self.BranchIndex = r.read_varint_i32()?,
            6 => self.CostTimeMs = r.read_varint_i64()?,
            7 => self.GainTimeMs = r.read_varint_i64()?,
            8 => self.RemainTimeMs = r.read_varint_i64()?,
            9 => crate::wire::decode_map_entry(r, &mut self.TeamStarInfo)?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.GainTokens
                    .push(<RoleTowerTokenCount as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffAnomalyInfoUpdateNotify {
    pub AnomalyInfos: Vec<SheriffAnomalyInfo>,
}

impl SheriffAnomalyInfoUpdateNotify {
    pub const MSG_ID: u16 = 17753;
}

impl PbMessage for SheriffAnomalyInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AnomalyInfos {
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
                self.AnomalyInfos
                    .push(<SheriffAnomalyInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffCriminalUpdateNotify {
    pub CriminalInfos: Vec<SheriffCriminalInfo>,
}

impl SheriffCriminalUpdateNotify {
    pub const MSG_ID: u16 = 29606;
}

impl PbMessage for SheriffCriminalUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CriminalInfos {
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
                self.CriminalInfos
                    .push(<SheriffCriminalInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SheriffZoneItemUpdateNotify {
    pub ItemCount: i64,
    pub ZoneId: i32,
}

impl SheriffZoneItemUpdateNotify {
    pub const MSG_ID: u16 = 18326;
}

impl PbMessage for SheriffZoneItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ItemCount);
        }
        if self.ZoneId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ZoneId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemCount = r.read_varint_i64()?,
            4 => self.ZoneId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopBuyNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub EntityId: i32,
}

impl ShopBuyNotify {
    pub const MSG_ID: u16 = 16134;
}

impl PbMessage for ShopBuyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopBuySyncDataPb {
    pub ShopId: i32,
    pub GoodsId: i32,
    pub Count: i32,
}

impl PbMessage for ShopBuySyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.GoodsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoodsId);
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
            1 => self.ShopId = r.read_varint_i32()?,
            2 => self.GoodsId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopInfoNotify {
    pub ShopList: Vec<ShopInfo>,
    pub VersionStr: Vec<u8>,
}

impl ShopInfoNotify {
    pub const MSG_ID: u16 = 18614;
}

impl PbMessage for ShopInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShopList {
            crate::wire::write_submessage(out, 12, item);
        }
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ShopList.push(<ShopInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.VersionStr = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopUnlockNotify {
    pub UnlockList: Vec<UnlockInfo>,
}

impl ShopUnlockNotify {
    pub const MSG_ID: u16 = 23429;
}

impl PbMessage for ShopUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockList {
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
                self.UnlockList
                    .push(<UnlockInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopUpdateRequest {
    pub ShopId: i32,
}

impl ShopUpdateRequest {
    pub const MSG_ID: u16 = 17187;
}

impl PbMessage for ShopUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.ShopId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShopUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Info: Option<ShopInfo>,
}

impl ShopUpdateResponse {
    pub const MSG_ID: u16 = 27292;
}

impl PbMessage for ShopUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Info {
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
pub struct ShopView {
    pub SurvivorsOption: Option<SurvivorsOption>,
}

impl PbMessage for ShopView {
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
pub struct ShortMessageChatCharmChangeRequest {
    pub ChatCharmId: i32,
}

impl ShortMessageChatCharmChangeRequest {
    pub const MSG_ID: u16 = 23634;
}

impl PbMessage for ShortMessageChatCharmChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChatCharmId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatCharmId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.ChatCharmId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageChatCharmChangeResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageChatCharmChangeResponse {
    pub const MSG_ID: u16 = 15958;
}

impl PbMessage for ShortMessageChatCharmChangeResponse {
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
pub struct ShortMessageChatThemeChangeRequest {
    pub ChatThemeId: i32,
}

impl ShortMessageChatThemeChangeRequest {
    pub const MSG_ID: u16 = 29084;
}

impl PbMessage for ShortMessageChatThemeChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChatThemeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatThemeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ChatThemeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageChatThemeChangeResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageChatThemeChangeResponse {
    pub const MSG_ID: u16 = 21146;
}

impl PbMessage for ShortMessageChatThemeChangeResponse {
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
pub struct ShortMessageInfoRequest {}

impl ShortMessageInfoRequest {
    pub const MSG_ID: u16 = 19425;
}

impl PbMessage for ShortMessageInfoRequest {
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
pub struct ShortMessageInfoResponse {
    pub ChatBgIds: Vec<i32>,
    pub ChatBgId: i32,
    pub PartnerChange: Vec<i32>,
    pub ChatThemeId: i32,
    pub BubbleIds: Vec<i32>,
    pub ChatThemeIds: Vec<i32>,
    pub ChatCharmId: i32,
    pub BubbleId: i32,
    pub ShortMessageInfos: Vec<ShortMessageInfo>,
    pub ErrCode: Option<ErrorCode>,
    pub ChatCharmIds: Vec<i32>,
}

impl ShortMessageInfoResponse {
    pub const MSG_ID: u16 = 17966;
}

impl PbMessage for ShortMessageInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChatBgIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatBgId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatBgId);
        }
        for item in &self.PartnerChange {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatThemeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatThemeId);
        }
        for item in &self.BubbleIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ChatThemeIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChatCharmId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatCharmId);
        }
        if self.BubbleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BubbleId);
        }
        for item in &self.ShortMessageInfos {
            crate::wire::write_submessage(out, 13, item);
        }
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ChatCharmIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatBgIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.ChatBgId = r.read_varint_i32()?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PartnerChange, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            6 => self.ChatThemeId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BubbleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatThemeIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.ChatCharmId = r.read_varint_i32()?,
            12 => self.BubbleId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ShortMessageInfos
                    .push(<ShortMessageInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatCharmIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageItemUpdateNotify {
    pub ChatCharmIds: Vec<i32>,
    pub ChatThemeIds: Vec<i32>,
    pub BubbleIds: Vec<i32>,
    pub ChatBgIds: Vec<i32>,
}

impl ShortMessageItemUpdateNotify {
    pub const MSG_ID: u16 = 18965;
}

impl PbMessage for ShortMessageItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChatCharmIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ChatThemeIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.BubbleIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ChatBgIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatCharmIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatThemeIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BubbleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChatBgIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageReadRequest {
    pub ConfigId: i32,
}

impl ShortMessageReadRequest {
    pub const MSG_ID: u16 = 25131;
}

impl PbMessage for ShortMessageReadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageReadResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageReadResponse {
    pub const MSG_ID: u16 = 16177;
}

impl PbMessage for ShortMessageReadResponse {
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
pub struct ShortMessageReceivedRequest {
    pub ConfigId: i32,
}

impl ShortMessageReceivedRequest {
    pub const MSG_ID: u16 = 29328;
}

impl PbMessage for ShortMessageReceivedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageReceivedResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageReceivedResponse {
    pub const MSG_ID: u16 = 21328;
}

impl PbMessage for ShortMessageReceivedResponse {
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
pub struct ShortMessageReplyRequest {
    pub SelectId: i32,
    pub CurOptionId: i32,
    pub ConfigId: i32,
}

impl ShortMessageReplyRequest {
    pub const MSG_ID: u16 = 22894;
}

impl PbMessage for ShortMessageReplyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectId);
        }
        if self.CurOptionId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurOptionId);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            6 => self.SelectId = r.read_varint_i32()?,
            11 => self.CurOptionId = r.read_varint_i32()?,
            13 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageReplyResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageReplyResponse {
    pub const MSG_ID: u16 = 24310;
}

impl PbMessage for ShortMessageReplyResponse {
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
pub struct ShortMessageShowChangeRequest {
    pub ChatBgId: i32,
    pub BubbleId: i32,
}

impl ShortMessageShowChangeRequest {
    pub const MSG_ID: u16 = 27855;
}

impl PbMessage for ShortMessageShowChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChatBgId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChatBgId);
        }
        if self.BubbleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BubbleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ChatBgId = r.read_varint_i32()?,
            5 => self.BubbleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageShowChangeResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageShowChangeResponse {
    pub const MSG_ID: u16 = 27652;
}

impl PbMessage for ShortMessageShowChangeResponse {
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
pub struct ShortMessageUpdateNotify {
    pub Removes: Vec<i32>,
    pub Adds: Vec<ShortMessageInfo>,
    pub NewPartnerChange: Vec<i32>,
    pub Reason: Option<ChangeReason>,
}

impl ShortMessageUpdateNotify {
    pub const MSG_ID: u16 = 23082;
}

impl PbMessage for ShortMessageUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Removes {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Adds {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.NewPartnerChange {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Reason {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<ShortMessageInfo as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.NewPartnerChange,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            14 => {
                self.Reason = Some(<ChangeReason as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageUpdateRequest {
    pub ConfigId: i32,
    pub LastConfigId: i32,
    pub IsFinish: bool,
}

impl ShortMessageUpdateRequest {
    pub const MSG_ID: u16 = 22887;
}

impl PbMessage for ShortMessageUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.LastConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastConfigId);
        }
        if self.IsFinish {
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
            8 => self.ConfigId = r.read_varint_i32()?,
            9 => self.LastConfigId = r.read_varint_i32()?,
            15 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShortMessageUpdateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ShortMessageUpdateResponse {
    pub const MSG_ID: u16 = 19489;
}

impl PbMessage for ShortMessageUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct ShowPasserByNpcNotify {
    pub IsHide: bool,
    pub EntityIds: Vec<i64>,
}

impl ShowPasserByNpcNotify {
    pub const MSG_ID: u16 = 29077;
}

impl PbMessage for ShowPasserByNpcNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsHide {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            5 => self.IsHide = r.read_bool()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShowViewOp {
    pub GridTakeView: Option<GridTakeView>,
    pub AddRoleView: Option<AddRoleView>,
    pub RoleBuffUnlockView: Option<RoleBuffUnlockView>,
    pub InstResultView: Option<InstResultView>,
    pub RoleBondInfoView: Option<RoleBondInfoView>,
    pub AddTokenView: Option<AddTokenView>,
    pub ShowViewType: Option<ShowViewType>,
}

impl PbMessage for ShowViewOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GridTakeView {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.AddRoleView {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.RoleBuffUnlockView {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.InstResultView {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.RoleBondInfoView {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.AddTokenView {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.ShowViewType {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
                self.GridTakeView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AddRoleView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoleBuffUnlockView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.InstResultView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RoleBondInfoView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AddTokenView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1000 => {
                self.ShowViewType =
                    Some(<ShowViewType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignActivityFreeDropRequest {
    pub ActivityId: i32,
}

impl SignActivityFreeDropRequest {
    pub const MSG_ID: u16 = 27704;
}

impl PbMessage for SignActivityFreeDropRequest {
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
pub struct SignActivityFreeDropResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SignActivityFreeDropResponse {
    pub const MSG_ID: u16 = 15457;
}

impl PbMessage for SignActivityFreeDropResponse {
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
pub struct SignActivityKeepTimeNotify {
    pub ActivityId: i32,
    pub KeepTime: i64,
}

impl SignActivityKeepTimeNotify {
    pub const MSG_ID: u16 = 16823;
}

impl PbMessage for SignActivityKeepTimeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.KeepTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.KeepTime);
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
            11 => self.KeepTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignActivityRequest {
    pub ActivityId: i32,
    pub Index: i32,
}

impl SignActivityRequest {
    pub const MSG_ID: u16 = 29384;
}

impl PbMessage for SignActivityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            7 => self.ActivityId = r.read_varint_i32()?,
            10 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignActivityResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl SignActivityResponse {
    pub const MSG_ID: u16 = 18799;
}

impl PbMessage for SignActivityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 10, &self.ItemMap);
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
            10 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignActivitySignStateNotify {
    pub Index: i32,
    pub SignState: Option<SignState>,
    pub ActivityId: i32,
}

impl SignActivitySignStateNotify {
    pub const MSG_ID: u16 = 27234;
}

impl PbMessage for SignActivitySignStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.SignState {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            2 => self.Index = r.read_varint_i32()?,
            7 => {
                self.SignState = Some(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignatureUpdateNotify {
    pub Signature: Vec<u8>,
}

impl SignatureUpdateNotify {
    pub const MSG_ID: u16 = 28884;
}

impl PbMessage for SignatureUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Signature.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
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
            4 => self.Signature = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatBeginNotify {}

impl SimpleCombatBeginNotify {
    pub const MSG_ID: u16 = 26690;
}

impl PbMessage for SimpleCombatBeginNotify {
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
pub struct SimpleCombatEndNotify {}

impl SimpleCombatEndNotify {
    pub const MSG_ID: u16 = 26778;
}

impl PbMessage for SimpleCombatEndNotify {
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
pub struct SimpleCombatEntityAttributeUpdateNotify {
    pub EntityId: i64,
    pub AttributePbInfo: Option<SimpleCombatEntityAttributePbInfo>,
}

impl SimpleCombatEntityAttributeUpdateNotify {
    pub const MSG_ID: u16 = 16636;
}

impl PbMessage for SimpleCombatEntityAttributeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.AttributePbInfo {
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
            6 => self.EntityId = r.read_varint_i64()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.AttributePbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityBuffLayerCountNotify {
    pub LayerCount: i32,
    pub BuffId: i32,
    pub EntityId: i64,
}

impl SimpleCombatEntityBuffLayerCountNotify {
    pub const MSG_ID: u16 = 29187;
}

impl PbMessage for SimpleCombatEntityBuffLayerCountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LayerCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LayerCount);
        }
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
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
            1 => self.LayerCount = r.read_varint_i32()?,
            5 => self.BuffId = r.read_varint_i32()?,
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityBuffUpdateNotify {
    pub IsBuffAdd: bool,
    pub BuffId: i32,
    pub EntityId: i64,
}

impl SimpleCombatEntityBuffUpdateNotify {
    pub const MSG_ID: u16 = 18273;
}

impl PbMessage for SimpleCombatEntityBuffUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsBuffAdd {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffId);
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
            3 => self.IsBuffAdd = r.read_bool()?,
            11 => self.BuffId = r.read_varint_i32()?,
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityDieContext {
    pub DiePos: Option<Vector>,
    pub TrapDefenseTrapKillCtx: Option<TrapDefenseTrapKillCtxPb>,
    pub TrapDefenseTrapSelfDestructCtx: Option<TrapDefenseTrapSelfDestructCtxPb>,
    pub TrapDefenseGoldenCoinAbosorbedCtx: Option<TrapDefenseGoldenCoinAbosorbedCtxPb>,
    pub TrapDefenseReachInstEndpointCtx: Option<TrapDefenseReachInstEndpointCtxPb>,
    pub TrapDefenseEnvironmentalKillCtx: Option<TrapDefenseEnvironmentalKillCtxPb>,
    pub TrapDefenseMonsterKilledCtx: Option<TrapDefenseMonsterKilledCtxPb>,
    pub TrapDefenseSpcecialCellDestructCtx: Option<TrapDefenseSpcecialCellDestructCtxPb>,
    pub SurvivorsMonsterKilledCtx: Option<SurvivorsMonsterKilledCtxPb>,
    pub SurvivorsGoldenCoinAbosorbedCtx: Option<SurvivorsGoldenCoinAbosorbedCtxPb>,
    pub SurvivorsRoleKilledCtx: Option<SurvivorsRoleKilledCtxPb>,
    pub KurotatoMonsterKilledCtxPb: Option<KurotatoMonsterKilledCtxPb>,
    pub KurotatoDropAbsorbedCtxPb: Option<KurotatoDropAbsorbedCtxPb>,
    pub KurotatoRoleKilledCtxPb: Option<KurotatoRoleKilledCtxPb>,
    pub PinballMonsterKilledCtx: Option<PinballMonsterKilledCtxPb>,
    pub PinballEntityKilledCtx: Option<PinballEntityKilledCtxPb>,
    pub KurotatoStructureKilledCtxPb: Option<KurotatoStructureKilledCtxPb>,
    pub GPUMonsterKilledCtx: Option<GPUMonsterKilledCtxPb>,
    pub GPURoleKilledCtx: Option<GPURoleKilledCtxPb>,
}

impl PbMessage for SimpleCombatEntityDieContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.DiePos {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.TrapDefenseTrapKillCtx {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.TrapDefenseTrapSelfDestructCtx {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.TrapDefenseGoldenCoinAbosorbedCtx {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.TrapDefenseReachInstEndpointCtx {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.TrapDefenseEnvironmentalKillCtx {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.TrapDefenseMonsterKilledCtx {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.TrapDefenseSpcecialCellDestructCtx {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.SurvivorsMonsterKilledCtx {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.SurvivorsGoldenCoinAbosorbedCtx {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.SurvivorsRoleKilledCtx {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.KurotatoMonsterKilledCtxPb {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.KurotatoDropAbsorbedCtxPb {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.KurotatoRoleKilledCtxPb {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.PinballMonsterKilledCtx {
            crate::wire::write_submessage(out, 16, v);
        }
        if let Some(v) = &self.PinballEntityKilledCtx {
            crate::wire::write_submessage(out, 17, v);
        }
        if let Some(v) = &self.KurotatoStructureKilledCtxPb {
            crate::wire::write_submessage(out, 18, v);
        }
        if let Some(v) = &self.GPUMonsterKilledCtx {
            crate::wire::write_submessage(out, 19, v);
        }
        if let Some(v) = &self.GPURoleKilledCtx {
            crate::wire::write_submessage(out, 20, v);
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
                self.DiePos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseTrapKillCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseTrapSelfDestructCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseGoldenCoinAbosorbedCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseReachInstEndpointCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseEnvironmentalKillCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseMonsterKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.TrapDefenseSpcecialCellDestructCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsMonsterKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsGoldenCoinAbosorbedCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsRoleKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoMonsterKilledCtxPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoDropAbsorbedCtxPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoRoleKilledCtxPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                self.PinballMonsterKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                self.PinballEntityKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                self.KurotatoStructureKilledCtxPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                self.GPUMonsterKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                self.GPURoleKilledCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityDieRequest {
    pub EntityDieCtxMap: BTreeMap<i64, SimpleCombatEntityDieContext>,
}

impl SimpleCombatEntityDieRequest {
    pub const MSG_ID: u16 = 24565;
}

impl PbMessage for SimpleCombatEntityDieRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.EntityDieCtxMap.is_empty() {
            crate::wire::write_map(out, 13, &self.EntityDieCtxMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.EntityDieCtxMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntityDieResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SimpleCombatEntityDieResponse {
    pub const MSG_ID: u16 = 16558;
}

impl PbMessage for SimpleCombatEntityDieResponse {
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
pub struct SimpleCombatEntitySubTypeChangeNotify {
    pub NewComponentData: Option<SimpleCombatComponentPb>,
    pub EntityId: i64,
}

impl SimpleCombatEntitySubTypeChangeNotify {
    pub const MSG_ID: u16 = 25732;
}

impl PbMessage for SimpleCombatEntitySubTypeChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewComponentData {
            crate::wire::write_submessage(out, 3, v);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.NewComponentData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatEntitySyncAttributePush {
    pub EntityAttributeMap: BTreeMap<i64, SimpleCombatEntityAttributePbInfo>,
}

impl SimpleCombatEntitySyncAttributePush {
    pub const MSG_ID: u16 = 24434;
}

impl PbMessage for SimpleCombatEntitySyncAttributePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.EntityAttributeMap.is_empty() {
            crate::wire::write_map(out, 5, &self.EntityAttributeMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => crate::wire::decode_map_entry(r, &mut self.EntityAttributeMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleTrackReportRequest {}

impl SimpleTrackReportRequest {
    pub const MSG_ID: u16 = 15365;
}

impl PbMessage for SimpleTrackReportRequest {
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
pub struct SimpleTrackReportResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SimpleTrackReportMsgs: Vec<SimpleTrackReportMsg>,
}

impl SimpleTrackReportResponse {
    pub const MSG_ID: u16 = 23444;
}

impl PbMessage for SimpleTrackReportResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SimpleTrackReportMsgs {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SimpleTrackReportMsgs
                    .push(<SimpleTrackReportMsg as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleGachaAwardPb {
    pub GroupId: i32,
    pub Times: i32,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl PbMessage for SingleGachaAwardPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.Times != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Times);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 5, &self.ItemMap);
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
            2 => self.Times = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleInstRechallengeRequest {
    pub RechallengeInInst: bool,
}

impl SingleInstRechallengeRequest {
    pub const MSG_ID: u16 = 20682;
}

impl PbMessage for SingleInstRechallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RechallengeInInst {
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
            15 => self.RechallengeInInst = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SingleInstRechallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SingleInstRechallengeResponse {
    pub const MSG_ID: u16 = 23802;
}

impl PbMessage for SingleInstRechallengeResponse {
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
pub struct SitChairRequest {
    pub EntityId: i64,
    pub IsSitDown: bool,
    pub Request: Option<AnimationStateChangedRequest>,
    pub Data: Option<CombatCommon>,
}

impl SitChairRequest {
    pub const MSG_ID: u16 = 25917;
}

impl PbMessage for SitChairRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsSitDown {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Request {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.Data {
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
            2 => self.EntityId = r.read_varint_i64()?,
            3 => self.IsSitDown = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Request
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
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
pub struct SitChairResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SitChairResponse {
    pub const MSG_ID: u16 = 15985;
}

impl PbMessage for SitChairResponse {
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
pub struct SkillCdGroupInfo {
    pub ShareGroupId: i32,
    pub SkillCdInfo: Option<SkillCdInfo>,
}

impl PbMessage for SkillCdGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShareGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShareGroupId);
        }
        if let Some(v) = &self.SkillCdInfo {
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
            1 => self.ShareGroupId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SkillCdInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillCdInfo {
    pub SkillId: i64,
    pub CdEndTimeList: Vec<i64>,
}

impl PbMessage for SkillCdInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        for item in &self.CdEndTimeList {
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
            1 => self.SkillId = r.read_varint_i64()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CdEndTimeList, |sr| {
                    Ok(sr.read_varint_i64()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillExecuteInfoPb {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub MontageIndex: i32,
    pub MontagePlayTime: i32,
    pub Section: Vec<u8>,
    pub SpeedRatio: f32,
    pub MessageId: i64,
    pub MontageContext: i64,
}

impl PbMessage for SkillExecuteInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.MontagePlayTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontagePlayTime);
        }
        if !self.Section.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Section);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
        if self.MessageId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MessageId);
        }
        if self.MontageContext != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MontageContext);
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
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.MontageIndex = r.read_varint_i32()?,
            3 => self.MontagePlayTime = r.read_varint_i32()?,
            4 => self.Section = r.read_len_delimited()?.to_vec(),
            5 => self.SpeedRatio = r.read_f32()?,
            6 => self.MessageId = r.read_varint_i64()?,
            7 => self.MontageContext = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillInfoPb {
    pub SkillCdInfoList: Vec<SkillCdInfo>,
    pub SkillCdGroupInfoList: Vec<SkillCdGroupInfo>,
}

impl PbMessage for SkillInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkillCdInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.SkillCdGroupInfoList {
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
                self.SkillCdInfoList
                    .push(<SkillCdInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SkillCdGroupInfoList
                    .push(<SkillCdGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkinRewardInfoUpdateNotify {
    pub RewardInfo: Option<SkinRewardActivityRewardInfo>,
}

impl SkinRewardInfoUpdateNotify {
    pub const MSG_ID: u16 = 15316;
}

impl PbMessage for SkinRewardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardInfo {
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
pub struct SkinRewardRequest {
    pub RewardId: i32,
}

impl SkinRewardRequest {
    pub const MSG_ID: u16 = 21395;
}

impl PbMessage for SkinRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            14 => self.RewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkinRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SkinRewardResponse {
    pub const MSG_ID: u16 = 20013;
}

impl PbMessage for SkinRewardResponse {
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
pub struct SlashAndTowerCtx {
    pub LevelsId: i32,
    pub SelectBuff: Vec<i32>,
    pub SecondRoles: Vec<i32>,
    pub SkillBranchIds: Vec<i32>,
    pub ReChallenge: bool,
}

impl PbMessage for SlashAndTowerCtx {
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
        for item in &self.SkillBranchIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ReChallenge {
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
            1 => self.LevelsId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectBuff, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SecondRoles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => self.ReChallenge = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerLevelPlayNotify {
    pub SlashLevelPlayInfos: Vec<SlashLevelPlayInfo>,
}

impl SlashAndTowerLevelPlayNotify {
    pub const MSG_ID: u16 = 18154;
}

impl PbMessage for SlashAndTowerLevelPlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SlashLevelPlayInfos {
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
                self.SlashLevelPlayInfos
                    .push(<SlashLevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerOverNotify {}

impl SlashAndTowerOverNotify {
    pub const MSG_ID: u16 = 17309;
}

impl PbMessage for SlashAndTowerOverNotify {
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
pub struct SlashAndTowerRecommendRequest {
    pub Id: i32,
}

impl SlashAndTowerRecommendRequest {
    pub const MSG_ID: u16 = 29840;
}

impl PbMessage for SlashAndTowerRecommendRequest {
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
pub struct SlashAndTowerRecommendResponse {
    pub RecommendInfos: Vec<SlashLevelRecommend>,
    pub ErrorCode: Option<ErrorCode>,
}

impl SlashAndTowerRecommendResponse {
    pub const MSG_ID: u16 = 23623;
}

impl PbMessage for SlashAndTowerRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecommendInfos {
            crate::wire::write_submessage(out, 4, item);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RecommendInfos
                    .push(<SlashLevelRecommend as PbMessage>::decode(bytes)?);
            }
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
pub struct SlashAndTowerResetRequest {
    pub Id: i32,
}

impl SlashAndTowerResetRequest {
    pub const MSG_ID: u16 = 27317;
}

impl PbMessage for SlashAndTowerResetRequest {
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
pub struct SlashAndTowerResetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SlashAndTowerResetResponse {
    pub const MSG_ID: u16 = 29466;
}

impl PbMessage for SlashAndTowerResetResponse {
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
pub struct SlashAndTowerResultNotify {
    pub SecondKillMonsterScore: i32,
    pub FirstRoundScore: i32,
    pub SecondFormation: Option<BattleFormation>,
    pub SecondRoundScore: i32,
    pub LevelsId: i32,
    pub FirstKillMonsterScore: i32,
    pub FirstFormation: Option<BattleFormation>,
}

impl SlashAndTowerResultNotify {
    pub const MSG_ID: u16 = 24868;
}

impl PbMessage for SlashAndTowerResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SecondKillMonsterScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondKillMonsterScore);
        }
        if self.FirstRoundScore != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstRoundScore);
        }
        if let Some(v) = &self.SecondFormation {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.SecondRoundScore != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecondRoundScore);
        }
        if self.LevelsId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelsId);
        }
        if self.FirstKillMonsterScore != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstKillMonsterScore);
        }
        if let Some(v) = &self.FirstFormation {
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
            2 => self.SecondKillMonsterScore = r.read_varint_i32()?,
            4 => self.FirstRoundScore = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SecondFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.SecondRoundScore = r.read_varint_i32()?,
            8 => self.LevelsId = r.read_varint_i32()?,
            14 => self.FirstKillMonsterScore = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.FirstFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerSaveRecordRequest {
    pub Id: i32,
}

impl SlashAndTowerSaveRecordRequest {
    pub const MSG_ID: u16 = 19751;
}

impl PbMessage for SlashAndTowerSaveRecordRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerSaveRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SlashAndTowerSaveRecordResponse {
    pub const MSG_ID: u16 = 27817;
}

impl PbMessage for SlashAndTowerSaveRecordResponse {
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
pub struct SlashAndTowerScoreRewardRequest {
    pub Id: i32,
    pub RewardIds: Vec<i32>,
}

impl SlashAndTowerScoreRewardRequest {
    pub const MSG_ID: u16 = 25485;
}

impl PbMessage for SlashAndTowerScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.RewardIds {
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
            6 => self.Id = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashAndTowerScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RewardIds: Vec<i32>,
}

impl SlashAndTowerScoreRewardResponse {
    pub const MSG_ID: u16 = 18302;
}

impl PbMessage for SlashAndTowerScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RewardIds {
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
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlashLevelRecommend {
    pub FormationFirst: Option<BattleFormation>,
    pub FormationSecond: Option<BattleFormation>,
    pub Usage: i32,
}

impl PbMessage for SlashLevelRecommend {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FormationFirst {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.FormationSecond {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.Usage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
                self.FormationFirst
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FormationSecond
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.Usage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SlientFirstAwardNotify {
    pub Id: i32,
}

impl SlientFirstAwardNotify {
    pub const MSG_ID: u16 = 17464;
}

impl PbMessage for SlientFirstAwardNotify {
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
pub struct SlowTimePush {
    pub Duration: i32,
    pub Flag: bool,
    pub Dilation: f32,
}

impl SlowTimePush {
    pub const MSG_ID: u16 = 19838;
}

impl PbMessage for SlowTimePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.Flag {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Dilation != 0.0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Dilation).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Duration = r.read_varint_i32()?,
            9 => self.Flag = r.read_bool()?,
            14 => self.Dilation = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SneakFinishRequest {
    pub SneakId: i32,
    pub FinishType: Option<SneakFinishType>,
}

impl SneakFinishRequest {
    pub const MSG_ID: u16 = 20851;
}

impl PbMessage for SneakFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SneakId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SneakId);
        }
        if let Some(v) = &self.FinishType {
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
            2 => self.SneakId = r.read_varint_i32()?,
            14 => {
                self.FinishType = Some(
                    <SneakFinishType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct SneakFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SneakFinishResponse {
    pub const MSG_ID: u16 = 25091;
}

impl PbMessage for SneakFinishResponse {
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
pub struct SneakGameStateNotify {
    pub SneakId: i32,
    pub EntityIds: Vec<i64>,
    pub State: Option<SneakState>,
}

impl SneakGameStateNotify {
    pub const MSG_ID: u16 = 25864;
}

impl PbMessage for SneakGameStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SneakId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SneakId);
        }
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.State {
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
            1 => self.SneakId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            13 => {
                self.State = Some(<SneakState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SneakNotify {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub EndTime: i64,
    pub TreeIncId: i64,
}

impl SneakNotify {
    pub const MSG_ID: u16 = 20617;
}

impl PbMessage for SneakNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
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
            4 => self.NodeId = r.read_varint_i32()?,
            9 => self.EndTime = r.read_varint_i64()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SneakRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub IsStart: bool,
    pub TreeIncId: i64,
}

impl SneakRequest {
    pub const MSG_ID: u16 = 15909;
}

impl PbMessage for SneakRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.IsStart {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            7 => self.NodeId = r.read_varint_i32()?,
            11 => self.IsStart = r.read_bool()?,
            14 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SneakResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl SneakResponse {
    pub const MSG_ID: u16 = 27480;
}

impl PbMessage for SneakResponse {
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
pub struct SneakTimeRequest {
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
}

impl SneakTimeRequest {
    pub const MSG_ID: u16 = 27089;
}

impl PbMessage for SneakTimeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            11 => self.TreeIncId = r.read_varint_i64()?,
            13 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SneakTimeResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl SneakTimeResponse {
    pub const MSG_ID: u16 = 16525;
}

impl PbMessage for SneakTimeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorId {
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
pub struct SoaringChallengeData {
    pub RankS: i32,
    pub RankA: i32,
    pub RankB: i32,
    pub Score: i32,
}

impl PbMessage for SoaringChallengeData {
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
pub struct SolarSchoolAvgDataPb {
    pub AvgParamBit: Vec<i64>,
}

impl PbMessage for SolarSchoolAvgDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AvgParamBit {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AvgParamBit, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolBatchSyncData {
    pub AttrChange: Option<AttrChangeSyncDataPb>,
    pub UnlockNpc: Option<UnlockNpcSyncDataPb>,
    pub NpcFavorChange: Option<NpcFavorChangeSyncDataPb>,
    pub JoinInfluence: Option<JoinInfluenceSyncDataPb>,
    pub Behave: Option<BehaveSyncDataPb>,
    pub ItemChange: Option<ItemChangeSyncDataPb>,
    pub ExamFinish: Option<ExamFinishSyncDataPb>,
    pub ShopBuy: Option<ShopBuySyncDataPb>,
    pub CompleteQuest: Option<CompleteQuestSyncDataPb>,
}

impl PbMessage for SolarSchoolBatchSyncData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AttrChange {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.UnlockNpc {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.NpcFavorChange {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.JoinInfluence {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Behave {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.ItemChange {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.ExamFinish {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.ShopBuy {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.CompleteQuest {
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
                self.AttrChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockNpc
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.NpcFavorChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.JoinInfluence
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Behave
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ItemChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ExamFinish
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ShopBuy
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CompleteQuest
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolBatchSyncRequest {
    pub ActivityId: i32,
    pub GameDayNow: i32,
    pub TraceId: i32,
    pub SyncData: Vec<SolarSchoolBatchSyncData>,
}

impl SolarSchoolBatchSyncRequest {
    pub const MSG_ID: u16 = 28917;
}

impl PbMessage for SolarSchoolBatchSyncRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.GameDayNow != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GameDayNow);
        }
        if self.TraceId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceId);
        }
        for item in &self.SyncData {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            4 => self.GameDayNow = r.read_varint_i32()?,
            5 => self.TraceId = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SyncData
                    .push(<SolarSchoolBatchSyncData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolBatchSyncResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FailedIndex: i32,
}

impl SolarSchoolBatchSyncResponse {
    pub const MSG_ID: u16 = 26100;
}

impl PbMessage for SolarSchoolBatchSyncResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FailedIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailedIndex);
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
            8 => self.FailedIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolBehaviorCountPb {
    pub BehaviorId: i32,
    pub TotalCount: i32,
    pub SuccessCount: i32,
    pub SpecialState: i32,
}

impl PbMessage for SolarSchoolBehaviorCountPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BehaviorId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BehaviorId);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
        }
        if self.SuccessCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SuccessCount);
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
            2 => self.TotalCount = r.read_varint_i32()?,
            3 => self.SuccessCount = r.read_varint_i32()?,
            4 => self.SpecialState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolCharacterDataPb {
    pub Npc: Vec<SolarSchoolNpcPb>,
}

impl PbMessage for SolarSchoolCharacterDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Npc {
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
                self.Npc
                    .push(<SolarSchoolNpcPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolExamDataPb {
    pub Exam: Vec<SolarSchoolExamResult>,
}

impl PbMessage for SolarSchoolExamDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Exam {
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
                self.Exam
                    .push(<SolarSchoolExamResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolExamResult {
    pub Id: i32,
    pub Score: i32,
    pub Rank: i32,
    pub Published: bool,
}

impl PbMessage for SolarSchoolExamResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Rank != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rank);
        }
        if self.Published {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            3 => self.Rank = r.read_varint_i32()?,
            4 => self.Published = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolFlowPb {
    pub SolarSchoolFlowId: i32,
    pub ReadPlotFlatBit: Vec<i64>,
}

impl PbMessage for SolarSchoolFlowPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SolarSchoolFlowId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SolarSchoolFlowId);
        }
        for item in &self.ReadPlotFlatBit {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.SolarSchoolFlowId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ReadPlotFlatBit,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolGalleryUpdateNotify {
    pub ActivityId: i32,
    pub GalleryEntryUnlocked: bool,
    pub Gallery: Vec<SolarSchoolGalleryPb>,
}

impl SolarSchoolGalleryUpdateNotify {
    pub const MSG_ID: u16 = 15173;
}

impl PbMessage for SolarSchoolGalleryUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.GalleryEntryUnlocked {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Gallery {
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
            5 => self.ActivityId = r.read_varint_i32()?,
            6 => self.GalleryEntryUnlocked = r.read_bool()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Gallery
                    .push(<SolarSchoolGalleryPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolIdCountPb {
    pub Id: i32,
    pub Count: i32,
}

impl PbMessage for SolarSchoolIdCountPb {
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
pub struct SolarSchoolInfluenceLevelPb {
    pub InfluenceId: i32,
    pub Level: i32,
}

impl PbMessage for SolarSchoolInfluenceLevelPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InfluenceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InfluenceId);
        }
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
            1 => self.InfluenceId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolInventoryDataPb {
    pub Items: Vec<SolarSchoolIdCountPb>,
    pub GotItemBit: Vec<i64>,
}

impl PbMessage for SolarSchoolInventoryDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.GotItemBit {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<SolarSchoolIdCountPb as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GotItemBit, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolLoadRequest {
    pub SolarSchoolClientResVersion: i32,
    pub ActivityId: i32,
    pub SlotId: i32,
}

impl SolarSchoolLoadRequest {
    pub const MSG_ID: u16 = 15073;
}

impl PbMessage for SolarSchoolLoadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SolarSchoolClientResVersion != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SolarSchoolClientResVersion);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            1 => self.SolarSchoolClientResVersion = r.read_varint_i32()?,
            5 => self.ActivityId = r.read_varint_i32()?,
            7 => self.SlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolLoadResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Flow: Vec<SolarSchoolFlowPb>,
    pub SavePb: Option<SolarSchoolSavePb>,
}

impl SolarSchoolLoadResponse {
    pub const MSG_ID: u16 = 18031;
}

impl PbMessage for SolarSchoolLoadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Flow {
            crate::wire::write_submessage(out, 10, item);
        }
        if let Some(v) = &self.SavePb {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Flow
                    .push(<SolarSchoolFlowPb as PbMessage>::decode(bytes)?);
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SavePb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolLocationDataPb {
    pub ViewedBit: Vec<i64>,
}

impl PbMessage for SolarSchoolLocationDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ViewedBit {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ViewedBit, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolNpcPb {
    pub NpcId: i32,
    pub FavorLevel: i32,
    pub Favor: i32,
    pub MaxFavorReachedOrder: i32,
    pub GiftReceivedCount: i32,
}

impl PbMessage for SolarSchoolNpcPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NpcId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NpcId);
        }
        if self.FavorLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FavorLevel);
        }
        if self.Favor != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Favor);
        }
        if self.MaxFavorReachedOrder != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFavorReachedOrder);
        }
        if self.GiftReceivedCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GiftReceivedCount);
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
            2 => self.FavorLevel = r.read_varint_i32()?,
            3 => self.Favor = r.read_varint_i32()?,
            4 => self.MaxFavorReachedOrder = r.read_varint_i32()?,
            5 => self.GiftReceivedCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolOneKeyGalleryRewardRequest {
    pub Id: Vec<i32>,
}

impl SolarSchoolOneKeyGalleryRewardRequest {
    pub const MSG_ID: u16 = 26247;
}

impl PbMessage for SolarSchoolOneKeyGalleryRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Id {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Id, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolOneKeyGalleryRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolOneKeyGalleryRewardResponse {
    pub const MSG_ID: u16 = 19859;
}

impl PbMessage for SolarSchoolOneKeyGalleryRewardResponse {
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
pub struct SolarSchoolOneKeyRewardRequest {
    pub TaskType: Option<SolarSchoolTaskType>,
    pub TaskId: Vec<i32>,
}

impl SolarSchoolOneKeyRewardRequest {
    pub const MSG_ID: u16 = 22187;
}

impl PbMessage for SolarSchoolOneKeyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TaskType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TaskId {
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
            5 => {
                self.TaskType = Some(
                    <SolarSchoolTaskType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolOneKeyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolOneKeyRewardResponse {
    pub const MSG_ID: u16 = 26713;
}

impl PbMessage for SolarSchoolOneKeyRewardResponse {
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
pub struct SolarSchoolOneQuestPb {
    pub QuestId: i32,
    pub QuestReward: Vec<SolarSchoolOneQuestRewardPb>,
}

impl PbMessage for SolarSchoolOneQuestPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestId);
        }
        for item in &self.QuestReward {
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
            1 => self.QuestId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.QuestReward
                    .push(<SolarSchoolOneQuestRewardPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolOneQuestRewardPb {
    pub QuestRewardId: i32,
    pub Progress: i32,
    pub Rewarded: bool,
}

impl PbMessage for SolarSchoolOneQuestRewardPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestRewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuestRewardId);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.Rewarded {
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
            1 => self.QuestRewardId = r.read_varint_i32()?,
            2 => self.Progress = r.read_varint_i32()?,
            3 => self.Rewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolPlayerDataPb {
    pub Attr: Vec<SolarSchoolAttrPb>,
    pub InfluenceLevels: Vec<SolarSchoolInfluenceLevelPb>,
    pub Behavior: Vec<SolarSchoolBehaviorCountPb>,
    pub Buff: Vec<SolarSchoolIdCountPb>,
}

impl PbMessage for SolarSchoolPlayerDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attr {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.InfluenceLevels {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.Behavior {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.Buff {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Attr
                    .push(<SolarSchoolAttrPb as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.InfluenceLevels
                    .push(<SolarSchoolInfluenceLevelPb as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Behavior
                    .push(<SolarSchoolBehaviorCountPb as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Buff
                    .push(<SolarSchoolIdCountPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolQuestDataPb {
    pub QuestList: Vec<SolarSchoolOneQuestPb>,
    pub CompletedNodeBit: Vec<i64>,
    pub PendingNodeId: Vec<i32>,
    pub RewardedSolarSchoolQuestBit: i64,
}

impl PbMessage for SolarSchoolQuestDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.CompletedNodeBit {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.PendingNodeId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RewardedSolarSchoolQuestBit != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RewardedSolarSchoolQuestBit);
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
                self.QuestList
                    .push(<SolarSchoolOneQuestPb as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompletedNodeBit,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PendingNodeId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => self.RewardedSolarSchoolQuestBit = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolReachEndingRequest {
    pub Id: Vec<i32>,
}

impl SolarSchoolReachEndingRequest {
    pub const MSG_ID: u16 = 27408;
}

impl PbMessage for SolarSchoolReachEndingRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Id {
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
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Id, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolReachEndingResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolReachEndingResponse {
    pub const MSG_ID: u16 = 23961;
}

impl PbMessage for SolarSchoolReachEndingResponse {
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
pub struct SolarSchoolSaveGamePb {
    pub Player: Option<SolarSchoolPlayerDataPb>,
    pub Time: Option<SolarSchoolTimeDataPb>,
    pub Character: Option<SolarSchoolCharacterDataPb>,
    pub Quest: Option<SolarSchoolQuestDataPb>,
    pub Inventory: Option<SolarSchoolInventoryDataPb>,
    pub Schedule: Option<SolarSchoolScheduleDataPb>,
    pub Exam: Option<SolarSchoolExamDataPb>,
    pub Shop: Option<SolarSchoolShopDataPb>,
    pub Avg: Option<SolarSchoolAvgDataPb>,
    pub Summary: Option<SolarSchoolSummaryRecordDataPb>,
    pub Location: Option<SolarSchoolLocationDataPb>,
}

impl PbMessage for SolarSchoolSaveGamePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Player {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Time {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Character {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Quest {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Inventory {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Schedule {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Exam {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.Avg {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.Summary {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.Location {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Player
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Time
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Character
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Quest
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Inventory
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Schedule
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Exam
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Avg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Summary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
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
pub struct SolarSchoolSavePb {
    pub Preview: Option<SolarSchoolSavePreviewPb>,
    pub ActivityId: i32,
    pub Game: Option<SolarSchoolSaveGamePb>,
    pub Version: i32,
    pub TraceId: i32,
}

impl PbMessage for SolarSchoolSavePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Preview {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.Game {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if self.TraceId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceId);
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
                self.Preview
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.ActivityId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Game
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.Version = r.read_varint_i32()?,
            5 => self.TraceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSaveRequest {
    pub SavePb: Option<SolarSchoolSavePb>,
    pub Reason: Option<SolarSchoolSaveReason>,
    pub ActivityId: i32,
    pub LoadSourceSlotId: i32,
}

impl SolarSchoolSaveRequest {
    pub const MSG_ID: u16 = 18477;
}

impl PbMessage for SolarSchoolSaveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SavePb {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LoadSourceSlotId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoadSourceSlotId);
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
                self.SavePb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.Reason = Some(
                    <SolarSchoolSaveReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => self.ActivityId = r.read_varint_i32()?,
            13 => self.LoadSourceSlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSaveResponse {
    pub Flow: Vec<SolarSchoolFlowPb>,
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolSaveResponse {
    pub const MSG_ID: u16 = 21936;
}

impl PbMessage for SolarSchoolSaveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Flow {
            crate::wire::write_submessage(out, 11, item);
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Flow
                    .push(<SolarSchoolFlowPb as PbMessage>::decode(bytes)?);
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
pub struct SolarSchoolScheduleDataPb {
    pub History: Vec<SolarSchoolWeekSchedulePb>,
}

impl PbMessage for SolarSchoolScheduleDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.History {
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
                self.History
                    .push(<SolarSchoolWeekSchedulePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSetFlowRequest {
    pub SolarSchoolFlowId: i32,
    pub ReadPlot: Vec<i32>,
}

impl SolarSchoolSetFlowRequest {
    pub const MSG_ID: u16 = 21219;
}

impl PbMessage for SolarSchoolSetFlowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SolarSchoolFlowId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SolarSchoolFlowId);
        }
        for item in &self.ReadPlot {
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
            5 => self.SolarSchoolFlowId = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadPlot, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSetFlowResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolSetFlowResponse {
    pub const MSG_ID: u16 = 27809;
}

impl PbMessage for SolarSchoolSetFlowResponse {
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
pub struct SolarSchoolShopDataPb {
    pub ShopData: Vec<SolarSchoolShopRuntimeDataPb>,
}

impl PbMessage for SolarSchoolShopDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShopData {
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
                self.ShopData
                    .push(<SolarSchoolShopRuntimeDataPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolShopGoodsBuyDataPb {
    pub GoodsId: i32,
    pub BoughtCount: i32,
}

impl PbMessage for SolarSchoolShopGoodsBuyDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GoodsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoodsId);
        }
        if self.BoughtCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.GoodsId = r.read_varint_i32()?,
            2 => self.BoughtCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolShopRuntimeDataPb {
    pub ShopId: i32,
    pub RandomSeed: i32,
    pub RefreshCycleIndex: i32,
    pub GoodsBuyData: Vec<SolarSchoolShopGoodsBuyDataPb>,
}

impl PbMessage for SolarSchoolShopRuntimeDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.RandomSeed != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RandomSeed);
        }
        if self.RefreshCycleIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCycleIndex);
        }
        for item in &self.GoodsBuyData {
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
            1 => self.ShopId = r.read_varint_i32()?,
            2 => self.RandomSeed = r.read_varint_i32()?,
            3 => self.RefreshCycleIndex = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GoodsBuyData
                    .push(<SolarSchoolShopGoodsBuyDataPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSlotCellPb {
    pub Weekday: i32,
    pub Slot: i32,
    pub BehaviorId: i32,
    pub ReplacedLowBehaviorId: i32,
    pub FirstWeekday: i32,
}

impl PbMessage for SolarSchoolSlotCellPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Weekday != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Weekday);
        }
        if self.Slot != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Slot);
        }
        if self.BehaviorId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BehaviorId);
        }
        if self.ReplacedLowBehaviorId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReplacedLowBehaviorId);
        }
        if self.FirstWeekday != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstWeekday);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Weekday = r.read_varint_i32()?,
            2 => self.Slot = r.read_varint_i32()?,
            3 => self.BehaviorId = r.read_varint_i32()?,
            4 => self.ReplacedLowBehaviorId = r.read_varint_i32()?,
            5 => self.FirstWeekday = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSummaryDataPb {
    pub Attr: Vec<SolarSchoolAttrPb>,
    pub CompletedNoticeCount: i32,
    pub ManualPurchaseCount: i32,
    pub GainedMoney: i32,
}

impl PbMessage for SolarSchoolSummaryDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attr {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CompletedNoticeCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CompletedNoticeCount);
        }
        if self.ManualPurchaseCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ManualPurchaseCount);
        }
        if self.GainedMoney != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GainedMoney);
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
                self.Attr
                    .push(<SolarSchoolAttrPb as PbMessage>::decode(bytes)?);
            }
            2 => self.CompletedNoticeCount = r.read_varint_i32()?,
            3 => self.ManualPurchaseCount = r.read_varint_i32()?,
            4 => self.GainedMoney = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolSummaryRecordDataPb {
    pub SpanId: i32,
    pub MonthSummary: Option<SolarSchoolSummaryDataPb>,
    pub TotalSummary: Option<SolarSchoolSummaryDataPb>,
    pub MonthSettled: bool,
}

impl PbMessage for SolarSchoolSummaryRecordDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SpanId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpanId);
        }
        if let Some(v) = &self.MonthSummary {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.TotalSummary {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.MonthSettled {
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
            1 => self.SpanId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MonthSummary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TotalSummary
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.MonthSettled = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolTaskProgressRewardRequest {
    pub ActivityId: i32,
}

impl SolarSchoolTaskProgressRewardRequest {
    pub const MSG_ID: u16 = 22424;
}

impl PbMessage for SolarSchoolTaskProgressRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolTaskProgressRewardResponse {
    pub Activity: Option<SolarSchoolActivityPb>,
    pub ErrorCode: Option<ErrorCode>,
}

impl SolarSchoolTaskProgressRewardResponse {
    pub const MSG_ID: u16 = 18486;
}

impl PbMessage for SolarSchoolTaskProgressRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Activity {
            crate::wire::write_submessage(out, 9, v);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Activity
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
pub struct SolarSchoolTaskUpdateNotify {
    pub Task: Vec<SolarSchoolTaskTypePb>,
}

impl SolarSchoolTaskUpdateNotify {
    pub const MSG_ID: u16 = 16887;
}

impl PbMessage for SolarSchoolTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
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
                self.Task
                    .push(<SolarSchoolTaskTypePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolTimeDataPb {
    pub DayIndex: i32,
}

impl PbMessage for SolarSchoolTimeDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DayIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DayIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DayIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SolarSchoolWeekSchedulePb {
    pub Cell: Vec<SolarSchoolSlotCellPb>,
    pub InstanceId: Vec<i32>,
    pub SkippedWeekday: Vec<i32>,
}

impl PbMessage for SolarSchoolWeekSchedulePb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Cell {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.InstanceId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkippedWeekday {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Cell
                    .push(<SolarSchoolSlotCellPb as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InstanceId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkippedWeekday, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpecialEffectInfo {}

impl PbMessage for SpecialEffectInfo {
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
pub struct SpecialItemEquipRequest {
    pub ItemId: i32,
}

impl SpecialItemEquipRequest {
    pub const MSG_ID: u16 = 17718;
}

impl PbMessage for SpecialItemEquipRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpecialItemEquipResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SpecialItemEquipResponse {
    pub const MSG_ID: u16 = 17256;
}

impl PbMessage for SpecialItemEquipResponse {
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
pub struct SpecialItemNotify {
    pub EquipItemId: i32,
}

impl SpecialItemNotify {
    pub const MSG_ID: u16 = 20874;
}

impl PbMessage for SpecialItemNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EquipItemId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipItemId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.EquipItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpecialItemUnEquipRequest {}

impl SpecialItemUnEquipRequest {
    pub const MSG_ID: u16 = 19843;
}

impl PbMessage for SpecialItemUnEquipRequest {
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
pub struct SpecialItemUnEquipResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SpecialItemUnEquipResponse {
    pub const MSG_ID: u16 = 24255;
}

impl PbMessage for SpecialItemUnEquipResponse {
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
pub struct SplineMoveNotify {
    pub Pos: Option<Vector>,
    pub PathRatio: i32,
    pub EntityId: i64,
    pub PathId: i32,
}

impl SplineMoveNotify {
    pub const MSG_ID: u16 = 26842;
}

impl PbMessage for SplineMoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PathRatio != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PathRatio);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PathId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PathId);
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.PathRatio = r.read_varint_i32()?,
            9 => self.EntityId = r.read_varint_i64()?,
            10 => self.PathId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SplineMoveRequest {
    pub Pos: Option<Vector>,
    pub PathId: i32,
    pub EntityId: i64,
    pub PathRatio: i32,
}

impl SplineMoveRequest {
    pub const MSG_ID: u16 = 26613;
}

impl PbMessage for SplineMoveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PathId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PathId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.PathRatio != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PathRatio);
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.PathId = r.read_varint_i32()?,
            14 => self.EntityId = r.read_varint_i64()?,
            15 => self.PathRatio = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SplineMoveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SplineMoveResponse {
    pub const MSG_ID: u16 = 26766;
}

impl PbMessage for SplineMoveResponse {
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
pub struct SplinePositionNotify {
    pub SplineIdx: i32,
    pub EntityId: i64,
    pub Dir: bool,
    pub SplineId: i32,
    pub TimeScheduleId: i32,
}

impl SplinePositionNotify {
    pub const MSG_ID: u16 = 28425;
}

impl PbMessage for SplinePositionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SplineIdx != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineIdx);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Dir {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SplineId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineId);
        }
        if self.TimeScheduleId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeScheduleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SplineIdx = r.read_varint_i32()?,
            3 => self.EntityId = r.read_varint_i64()?,
            10 => self.Dir = r.read_bool()?,
            14 => self.SplineId = r.read_varint_i32()?,
            15 => self.TimeScheduleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SplinePositionUpdateRequest {
    pub SplineId: i32,
    pub EntityId: i64,
    pub Dir: bool,
    pub SplineType: Option<NpcTimeScheduleSplineType>,
    pub SplineIdx: i32,
    pub InstId: i32,
}

impl SplinePositionUpdateRequest {
    pub const MSG_ID: u16 = 24134;
}

impl PbMessage for SplinePositionUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SplineId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Dir {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.SplineType {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SplineIdx != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineIdx);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            2 => self.SplineId = r.read_varint_i32()?,
            3 => self.EntityId = r.read_varint_i64()?,
            4 => self.Dir = r.read_bool()?,
            6 => {
                self.SplineType = Some(
                    <NpcTimeScheduleSplineType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.SplineIdx = r.read_varint_i32()?,
            12 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SplinePositionUpdateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SplinePositionUpdateResponse {
    pub const MSG_ID: u16 = 29976;
}

impl PbMessage for SplinePositionUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct SpringFestivalAreaUnlockNotify {
    pub IsFullLoad: bool,
    pub AreaInfos: Vec<AreaInfo>,
    pub ActivityId: i32,
}

impl SpringFestivalAreaUnlockNotify {
    pub const MSG_ID: u16 = 16024;
}

impl PbMessage for SpringFestivalAreaUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFullLoad {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.AreaInfos {
            crate::wire::write_submessage(out, 8, item);
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
            5 => self.IsFullLoad = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AreaInfos.push(<AreaInfo as PbMessage>::decode(bytes)?);
            }
            9 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalAtmosphereRewardRequest {
    pub ActivityId: i32,
    pub LevelIds: Vec<i32>,
}

impl SpringFestivalAtmosphereRewardRequest {
    pub const MSG_ID: u16 = 24192;
}

impl PbMessage for SpringFestivalAtmosphereRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.LevelIds {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LevelIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalAtmosphereRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SpringFestivalAtmosphereRewardResponse {
    pub const MSG_ID: u16 = 17452;
}

impl PbMessage for SpringFestivalAtmosphereRewardResponse {
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
pub struct SpringFestivalAtmosphereUpdateNotify {
    pub AtmosphereLevel: i32,
    pub ActivityId: i32,
    pub Atmosphere: i32,
}

impl SpringFestivalAtmosphereUpdateNotify {
    pub const MSG_ID: u16 = 26608;
}

impl PbMessage for SpringFestivalAtmosphereUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AtmosphereLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AtmosphereLevel);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Atmosphere != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Atmosphere);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.AtmosphereLevel = r.read_varint_i32()?,
            10 => self.ActivityId = r.read_varint_i32()?,
            11 => self.Atmosphere = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalCtx {
    pub ActivityId: i32,
}

impl PbMessage for SpringFestivalCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalScoreRewardRequest {
    pub ActivityId: i32,
    pub ScoreIds: Vec<i32>,
}

impl SpringFestivalScoreRewardRequest {
    pub const MSG_ID: u16 = 17619;
}

impl PbMessage for SpringFestivalScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ScoreIds {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalScoreRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SpringFestivalScoreRewardResponse {
    pub const MSG_ID: u16 = 20918;
}

impl PbMessage for SpringFestivalScoreRewardResponse {
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
pub struct SpringFestivalTaskRewardRequest {
    pub TaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl SpringFestivalTaskRewardRequest {
    pub const MSG_ID: u16 = 27247;
}

impl PbMessage for SpringFestivalTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringFestivalTaskRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SpringFestivalTaskRewardResponse {
    pub const MSG_ID: u16 = 19632;
}

impl PbMessage for SpringFestivalTaskRewardResponse {
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
pub struct SpringFestivalUpdateNotify {
    pub Task: Option<ConditionTask>,
    pub SkipEntry: Option<SpringSkipEntry>,
    pub JokerLevelInfos: Option<GuessJokerLevelInfoArray>,
    pub FunctionId: i32,
}

impl SpringFestivalUpdateNotify {
    pub const MSG_ID: u16 = 16164;
}

impl PbMessage for SpringFestivalUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.SkipEntry {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.JokerLevelInfos {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.FunctionId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FunctionId);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkipEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.JokerLevelInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.FunctionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringSignDrawRewardRequest {
    pub Id: i32,
}

impl SpringSignDrawRewardRequest {
    pub const MSG_ID: u16 = 17001;
}

impl PbMessage for SpringSignDrawRewardRequest {
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
pub struct SpringSignDrawRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SpringSignDrawRewardResponse {
    pub const MSG_ID: u16 = 23619;
}

impl PbMessage for SpringSignDrawRewardResponse {
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
pub struct SpringSignDrawRoleRequest {}

impl SpringSignDrawRoleRequest {
    pub const MSG_ID: u16 = 18204;
}

impl PbMessage for SpringSignDrawRoleRequest {
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
pub struct SpringSignDrawRoleResponse {
    pub DrawId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl SpringSignDrawRoleResponse {
    pub const MSG_ID: u16 = 25860;
}

impl PbMessage for SpringSignDrawRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DrawId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DrawId);
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
            7 => self.DrawId = r.read_varint_i32()?,
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
pub struct SpringSignInfo {
    pub SpringSignActivityTasks: Vec<ActivityTask>,
    pub CanInvite: bool,
    pub DrawRoles: Vec<i32>,
    pub SkinReward: bool,
}

impl PbMessage for SpringSignInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SpringSignActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CanInvite {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.DrawRoles {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SkinReward {
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
                self.SpringSignActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => self.CanInvite = r.read_bool()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DrawRoles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.SkinReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpringSignPhotoRewardRequest {}

impl SpringSignPhotoRewardRequest {
    pub const MSG_ID: u16 = 26463;
}

impl PbMessage for SpringSignPhotoRewardRequest {
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
pub struct SpringSignPhotoRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SpringSignPhotoRewardResponse {
    pub const MSG_ID: u16 = 20917;
}

impl PbMessage for SpringSignPhotoRewardResponse {
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
pub struct SpringSignSkinRewardRequest {}

impl SpringSignSkinRewardRequest {
    pub const MSG_ID: u16 = 20317;
}

impl PbMessage for SpringSignSkinRewardRequest {
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
pub struct SpringSignSkinRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SpringSignSkinRewardResponse {
    pub const MSG_ID: u16 = 26418;
}

impl PbMessage for SpringSignSkinRewardResponse {
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
pub struct SrvFighterInfo {
    pub RoleId: i32,
    pub Level: i32,
    pub Exp: i32,
    pub Breakthrough: i32,
    pub PropDataList: Vec<PropConfig>,
    pub SrvFighterSkillInfo: Vec<SrvFighterSkillInfo>,
}

impl PbMessage for SrvFighterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        if self.Breakthrough != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Breakthrough);
        }
        for item in &self.PropDataList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.SrvFighterSkillInfo {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.Exp = r.read_varint_i32()?,
            4 => self.Breakthrough = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PropDataList
                    .push(<PropConfig as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SrvFighterSkillInfo
                    .push(<SrvFighterSkillInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SrvFighterSkillInfo {
    pub SkillId: i32,
    pub Level: i32,
}

impl PbMessage for SrvFighterSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
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
            1 => self.SkillId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartMatchNotify {
    pub InstId: i32,
}

impl StartMatchNotify {
    pub const MSG_ID: u16 = 23110;
}

impl PbMessage for StartMatchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartMatchRequest {
    pub InstEnterId: i32,
    pub IsInviteTeammate: bool,
    pub InstId: i32,
}

impl StartMatchRequest {
    pub const MSG_ID: u16 = 23965;
}

impl PbMessage for StartMatchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstEnterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstEnterId);
        }
        if self.IsInviteTeammate {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            1 => self.InstEnterId = r.read_varint_i32()?,
            3 => self.IsInviteTeammate = r.read_bool()?,
            12 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartMatchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl StartMatchResponse {
    pub const MSG_ID: u16 = 24408;
}

impl PbMessage for StartMatchResponse {
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
pub struct StartRecallRequest {
    pub RecallId: i32,
    pub Continue: bool,
}

impl StartRecallRequest {
    pub const MSG_ID: u16 = 18032;
}

impl PbMessage for StartRecallRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RecallId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecallId);
        }
        if self.Continue {
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
            2 => self.RecallId = r.read_varint_i32()?,
            7 => self.Continue = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartRecallResponse {
    pub CanContinue: bool,
    pub ErrorCode: Option<ErrorCode>,
    pub EnterInstId: i32,
}

impl StartRecallResponse {
    pub const MSG_ID: u16 = 22108;
}

impl PbMessage for StartRecallResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanContinue {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EnterInstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnterInstId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.CanContinue = r.read_bool()?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.EnterInstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StartSetQuestFocusNotify {
    pub Reason: Option<QuestFocusReason>,
    pub FocusQuestId: i32,
}

impl StartSetQuestFocusNotify {
    pub const MSG_ID: u16 = 28873;
}

impl PbMessage for StartSetQuestFocusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FocusQuestId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            6 => {
                self.Reason = Some(
                    <QuestFocusReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.FocusQuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateChangeConditionActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
    pub StateIndex: i32,
    pub ConditionIndex: i32,
}

impl PbMessage for StateChangeConditionActionCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.StateIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateIndex);
        }
        if self.ConditionIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionIndex);
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
            2 => self.StateIndex = r.read_varint_i32()?,
            3 => self.ConditionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StopHoldHandNotify {
    pub EntityId: i64,
    pub ActionType: i32,
    pub TargetEntityId: i64,
}

impl StopHoldHandNotify {
    pub const MSG_ID: u16 = 16932;
}

impl PbMessage for StopHoldHandNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ActionType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionType);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            2 => self.EntityId = r.read_varint_i64()?,
            3 => self.ActionType = r.read_varint_i32()?,
            14 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StopMoveSplineRequest {
    pub EntityId: i64,
    pub IsInterrupt: bool,
    pub SplineEntityId: i32,
}

impl StopMoveSplineRequest {
    pub const MSG_ID: u16 = 28704;
}

impl PbMessage for StopMoveSplineRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsInterrupt {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
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
            10 => self.IsInterrupt = r.read_bool()?,
            13 => self.SplineEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StopMoveSplineResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl StopMoveSplineResponse {
    pub const MSG_ID: u16 = 24675;
}

impl PbMessage for StopMoveSplineResponse {
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
pub struct SubmitAwakeAndLoadEntityNodeRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl SubmitAwakeAndLoadEntityNodeRequest {
    pub const MSG_ID: u16 = 17608;
}

impl PbMessage for SubmitAwakeAndLoadEntityNodeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            9 => self.NodeId = r.read_varint_i32()?,
            13 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubmitAwakeAndLoadEntityNodeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SubmitAwakeAndLoadEntityNodeResponse {
    pub const MSG_ID: u16 = 16025;
}

impl PbMessage for SubmitAwakeAndLoadEntityNodeResponse {
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
pub struct SubmitItemInfoNotify {
    pub GatherTaskSubmitInfos: Vec<GatherTaskSubmitInfo>,
}

impl SubmitItemInfoNotify {
    pub const MSG_ID: u16 = 22681;
}

impl PbMessage for SubmitItemInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GatherTaskSubmitInfos {
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
                self.GatherTaskSubmitInfos
                    .push(<GatherTaskSubmitInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubmitWalkingPatternPointRequest {
    pub NodeId: i32,
    pub TreeOwnerId: i32,
    pub TreeIncId: i64,
    pub Point: i32,
}

impl SubmitWalkingPatternPointRequest {
    pub const MSG_ID: u16 = 17328;
}

impl PbMessage for SubmitWalkingPatternPointRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Point);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.NodeId = r.read_varint_i32()?,
            9 => self.TreeOwnerId = r.read_varint_i32()?,
            14 => self.TreeIncId = r.read_varint_i64()?,
            15 => self.Point = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubmitWalkingPatternPointResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SubmitWalkingPatternPointResponse {
    pub const MSG_ID: u16 = 25667;
}

impl PbMessage for SubmitWalkingPatternPointResponse {
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
pub struct Summon_PlayerFollowerPb {
    pub Type: Option<Summon_EPlayerFollower>,
    pub EntityId: i64,
}

impl PbMessage for Summon_PlayerFollowerPb {
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
                    <Summon_EPlayerFollower as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct Summon_PlayerFollowerSelfComponentPb {
    pub PlayerEntityId: i64,
    pub SummonConfigId: i32,
}

impl PbMessage for Summon_PlayerFollowerSelfComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlayerEntityId);
        }
        if self.SummonConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PlayerEntityId = r.read_varint_i64()?,
            2 => self.SummonConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summon_PlayerFollowersComponentPb {
    pub ListPlayerFollowerPb: Vec<Summon_PlayerFollowerPb>,
}

impl PbMessage for Summon_PlayerFollowersComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ListPlayerFollowerPb {
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
                self.ListPlayerFollowerPb
                    .push(<Summon_PlayerFollowerPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summon_SummonerComponentPb {
    pub SummonerId: i64,
    pub SummonCfgId: i32,
    pub SummonSkillId: i32,
    pub PlayerId: i32,
    pub Type: Option<Summon_ESummonType>,
    pub AvatarCfgId: i32,
}

impl PbMessage for Summon_SummonerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
        }
        if self.SummonCfgId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonCfgId);
        }
        if self.SummonSkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonSkillId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AvatarCfgId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AvatarCfgId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SummonerId = r.read_varint_i64()?,
            2 => self.SummonCfgId = r.read_varint_i32()?,
            3 => self.SummonSkillId = r.read_varint_i32()?,
            4 => self.PlayerId = r.read_varint_i32()?,
            5 => {
                self.Type = Some(
                    <Summon_ESummonType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => self.AvatarCfgId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summon_SummonsComponentPb {
    pub Version: i32,
}

impl PbMessage for Summon_SummonsComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Version != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Version = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summon2Request {
    pub SummonerEntityId: i64,
    pub Version: i32,
    pub SummonInfo: Option<SummonRequestInfo>,
}

impl Summon2Request {
    pub const MSG_ID: u16 = 24593;
}

impl PbMessage for Summon2Request {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerEntityId);
        }
        if self.Version != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
        }
        if let Some(v) = &self.SummonInfo {
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
            4 => self.SummonerEntityId = r.read_varint_i64()?,
            7 => self.Version = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.SummonInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summon2Response {
    pub Version: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl Summon2Response {
    pub const MSG_ID: u16 = 20104;
}

impl PbMessage for Summon2Response {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Version != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Version);
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
            2 => self.Version = r.read_varint_i32()?,
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
pub struct SummonRandomNotify {
    pub SummonerId: i64,
    pub SummonIds: Vec<i64>,
}

impl SummonRandomNotify {
    pub const MSG_ID: u16 = 28895;
}

impl PbMessage for SummonRandomNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
        }
        for item in &self.SummonIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            9 => self.SummonerId = r.read_varint_i64()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SummonIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonRandomRequest {
    pub SummonInfo: Option<RandomSummonRequestInfo>,
    pub SummonerId: i64,
}

impl SummonRandomRequest {
    pub const MSG_ID: u16 = 19590;
}

impl PbMessage for SummonRandomRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SummonInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
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
                self.SummonInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.SummonerId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonRandomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SummonRandomResponse {
    pub const MSG_ID: u16 = 17070;
}

impl PbMessage for SummonRandomResponse {
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
pub struct SunSpiritActionOperationEndPush {
    pub PlayerId: i32,
    pub IncId: i32,
    pub Index: i32,
}

impl SunSpiritActionOperationEndPush {
    pub const MSG_ID: u16 = 22214;
}

impl PbMessage for SunSpiritActionOperationEndPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            7 => self.PlayerId = r.read_varint_i32()?,
            9 => self.IncId = r.read_varint_i32()?,
            14 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritActionOperationNotify {
    pub PlayerId: i32,
    pub EntityId: i64,
    pub TakeUpInfo: Vec<SunSpiritPb>,
    pub OperationType: Option<SunSpiritOperationType>,
    pub Index: i32,
    pub IncId: i32,
}

impl SunSpiritActionOperationNotify {
    pub const MSG_ID: u16 = 15928;
}

impl PbMessage for SunSpiritActionOperationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.TakeUpInfo {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.OperationType {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            3 => self.EntityId = r.read_varint_i64()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.TakeUpInfo
                    .push(<SunSpiritPb as PbMessage>::decode(bytes)?);
            }
            9 => {
                self.OperationType = Some(
                    <SunSpiritOperationType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => self.Index = r.read_varint_i32()?,
            11 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritCollectNotify {
    pub SpiritInfo: Option<SunSpiritPb>,
}

impl SunSpiritCollectNotify {
    pub const MSG_ID: u16 = 27484;
}

impl PbMessage for SunSpiritCollectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SpiritInfo {
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
                self.SpiritInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritListNotify {
    pub SpiritInfo: Vec<SunSpiritPb>,
}

impl SunSpiritListNotify {
    pub const MSG_ID: u16 = 15581;
}

impl PbMessage for SunSpiritListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SpiritInfo {
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
                self.SpiritInfo
                    .push(<SunSpiritPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritUpdateNotify {
    pub SpiritInfo: Vec<SunSpiritPb>,
}

impl SunSpiritUpdateNotify {
    pub const MSG_ID: u16 = 28796;
}

impl PbMessage for SunSpiritUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SpiritInfo {
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
                self.SpiritInfo
                    .push(<SunSpiritPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorLastInfo {
    pub LevelId: i32,
    pub InfiniteMode: bool,
    pub RoleId: i32,
    pub WaveId: i32,
    pub MaxWaveId: i32,
}

impl PbMessage for SurvivorLastInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.InfiniteMode {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveId);
        }
        if self.MaxWaveId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxWaveId);
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
            2 => self.InfiniteMode = r.read_bool()?,
            3 => self.RoleId = r.read_varint_i32()?,
            4 => self.WaveId = r.read_varint_i32()?,
            5 => self.MaxWaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsChallengeInfo {
    pub LevelId: i32,
    pub OpenTime: i64,
    pub NormalModeInfos: Option<ModeInfo>,
    pub InfiniteModeInfos: Option<ModeInfo>,
}

impl PbMessage for SurvivorsChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if let Some(v) = &self.NormalModeInfos {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.InfiniteModeInfos {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.OpenTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.NormalModeInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InfiniteModeInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsChallengeInfoUpdateNotify {
    pub SurvivorsChallengeInfos: Vec<SurvivorsChallengeInfo>,
}

impl SurvivorsChallengeInfoUpdateNotify {
    pub const MSG_ID: u16 = 17917;
}

impl PbMessage for SurvivorsChallengeInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SurvivorsChallengeInfos {
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
                self.SurvivorsChallengeInfos
                    .push(<SurvivorsChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsCombatStepPbData {
    pub DurationTime: i32,
}

impl PbMessage for SurvivorsCombatStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DurationTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DurationTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DurationTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsComboParamUpdateNotify {
    pub ComboParamInfos: Vec<ComboParamInfo>,
}

impl SurvivorsComboParamUpdateNotify {
    pub const MSG_ID: u16 = 20928;
}

impl PbMessage for SurvivorsComboParamUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ComboParamInfos {
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
                self.ComboParamInfos
                    .push(<ComboParamInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsCtx {
    pub Continue: bool,
    pub LevelId: i32,
    pub InfiniteMode: bool,
    pub WeaponId: i32,
    pub RoleId: i32,
}

impl PbMessage for SurvivorsCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Continue {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.InfiniteMode {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            3 => self.InfiniteMode = r.read_bool()?,
            4 => self.WeaponId = r.read_varint_i32()?,
            5 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsEndStepPbData {}

impl PbMessage for SurvivorsEndStepPbData {
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
pub struct SurvivorsGainData {
    pub SurvivorsRole: Option<SurvivorsRole>,
    pub SurvivorsWeapon: Option<SurvivorsWeapon>,
    pub SurvivorsToken: Option<SurvivorsToken>,
    pub SurvivorsRoleEvolve: Option<SurvivorsRoleEvolve>,
    pub SurvivorsRoleLv: Option<SurvivorsRoleLv>,
    pub SurvivorsWeaponEvolve: Option<SurvivorsWeaponEvolve>,
    pub SurvivorsWeaponLv: Option<SurvivorsWeaponLv>,
    pub IncId: i32,
    pub ConfigId: i32,
}

impl PbMessage for SurvivorsGainData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SurvivorsRole {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.SurvivorsWeapon {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.SurvivorsToken {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.SurvivorsRoleEvolve {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.SurvivorsRoleLv {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.SurvivorsWeaponEvolve {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.SurvivorsWeaponLv {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1001, crate::wire::WireType::Varint);
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
                let bytes = r.read_len_delimited()?;
                self.SurvivorsRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsWeapon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsToken
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsRoleEvolve
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsRoleLv
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsWeaponEvolve
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsWeaponLv
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1000 => self.IncId = r.read_varint_i32()?,
            1001 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsGainDataUpdateNotify {
    pub Updates: Vec<SurvivorsGainData>,
    pub Removes: Vec<i32>,
    pub Adds: Vec<SurvivorsGainData>,
}

impl SurvivorsGainDataUpdateNotify {
    pub const MSG_ID: u16 = 28759;
}

impl PbMessage for SurvivorsGainDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Updates {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.Removes {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Adds {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<SurvivorsGainData as PbMessage>::decode(bytes)?);
            }
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<SurvivorsGainData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsGoldenCoinAbosorbedCtxPb {}

impl PbMessage for SurvivorsGoldenCoinAbosorbedCtxPb {
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
pub struct SurvivorsHandBooskUnlockNotify {
    pub UnlockWeaponHandBooks: Vec<i32>,
    pub UnlockItemHandBooks: Vec<i32>,
    pub ActivityId: i32,
    pub UnlockRoleHandBooks: Vec<i32>,
}

impl SurvivorsHandBooskUnlockNotify {
    pub const MSG_ID: u16 = 25363;
}

impl PbMessage for SurvivorsHandBooskUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockWeaponHandBooks {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockItemHandBooks {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.UnlockRoleHandBooks {
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
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockWeaponHandBooks,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockItemHandBooks,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => self.ActivityId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockRoleHandBooks,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsInstGlobalDataNotify {
    pub ForceIncId: i32,
    pub WeaponMaxCount: i32,
    pub SurvivorsOpDatas: Vec<SurvivorsOpData>,
    pub LevelId: i32,
    pub SurvivorsGainDatas: Vec<SurvivorsGainData>,
    pub UnlockWeaponWave: Vec<i32>,
    pub ComboParamInfos: Vec<ComboParamInfo>,
    pub GlobalParams: BTreeMap<i32, i32>,
}

impl SurvivorsInstGlobalDataNotify {
    pub const MSG_ID: u16 = 28285;
}

impl PbMessage for SurvivorsInstGlobalDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ForceIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ForceIncId);
        }
        if self.WeaponMaxCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponMaxCount);
        }
        for item in &self.SurvivorsOpDatas {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.SurvivorsGainDatas {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.UnlockWeaponWave {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ComboParamInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        if !self.GlobalParams.is_empty() {
            crate::wire::write_map(out, 12, &self.GlobalParams);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.ForceIncId = r.read_varint_i32()?,
            3 => self.WeaponMaxCount = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsOpDatas
                    .push(<SurvivorsOpData as PbMessage>::decode(bytes)?);
            }
            7 => self.LevelId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsGainDatas
                    .push(<SurvivorsGainData as PbMessage>::decode(bytes)?);
            }
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockWeaponWave,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ComboParamInfos
                    .push(<ComboParamInfo as PbMessage>::decode(bytes)?);
            }
            12 => crate::wire::decode_map_entry(r, &mut self.GlobalParams)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsLastInfoRequest {}

impl SurvivorsLastInfoRequest {
    pub const MSG_ID: u16 = 23667;
}

impl PbMessage for SurvivorsLastInfoRequest {
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
pub struct SurvivorsLastInfoResponse {
    pub SurvivorLastInfo: Option<SurvivorLastInfo>,
}

impl SurvivorsLastInfoResponse {
    pub const MSG_ID: u16 = 28545;
}

impl PbMessage for SurvivorsLastInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SurvivorLastInfo {
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
                self.SurvivorLastInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsLockRequest {
    pub ClientSelectId: i32,
    pub IncId: i32,
    pub IsLock: bool,
}

impl SurvivorsLockRequest {
    pub const MSG_ID: u16 = 19978;
}

impl PbMessage for SurvivorsLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ClientSelectId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClientSelectId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.IsLock {
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
            4 => self.ClientSelectId = r.read_varint_i32()?,
            5 => self.IncId = r.read_varint_i32()?,
            8 => self.IsLock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsLockResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SurvivorsLockResponse {
    pub const MSG_ID: u16 = 24428;
}

impl PbMessage for SurvivorsLockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct SurvivorsMonsterConsecutiveKillsNotify {
    pub ConsecutiveKillCount: i32,
}

impl SurvivorsMonsterConsecutiveKillsNotify {
    pub const MSG_ID: u16 = 26546;
}

impl PbMessage for SurvivorsMonsterConsecutiveKillsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConsecutiveKillCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.ConsecutiveKillCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsMonsterKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for SurvivorsMonsterKilledCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsNormalAwardRequest {
    pub Configs: Vec<i32>,
}

impl SurvivorsNormalAwardRequest {
    pub const MSG_ID: u16 = 25147;
}

impl PbMessage for SurvivorsNormalAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Configs {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Configs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsNormalAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SurvivorsNormalAwardResponse {
    pub const MSG_ID: u16 = 24955;
}

impl PbMessage for SurvivorsNormalAwardResponse {
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
pub struct SurvivorsOpData {
    pub TokenAwardView: Option<TokenAwardView>,
    pub TokenSelectView: Option<TokenSelectView>,
    pub WeaponSelectView: Option<WeaponSelectView>,
    pub ShopView: Option<ShopView>,
    pub EvolveView: Option<EvolveView>,
    pub ResultView: Option<ResultView>,
    pub RoleOrWeaponUpView: Option<RoleOrWeaponUpView>,
    pub IncId: i32,
}

impl PbMessage for SurvivorsOpData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TokenAwardView {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.TokenSelectView {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.WeaponSelectView {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.ShopView {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.EvolveView {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.ResultView {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.RoleOrWeaponUpView {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1000, crate::wire::WireType::Varint);
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
                self.TokenAwardView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TokenSelectView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponSelectView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ShopView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.EvolveView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ResultView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RoleOrWeaponUpView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1000 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsOpDataForceChangeNotify {
    pub ForceIncId: i32,
}

impl SurvivorsOpDataForceChangeNotify {
    pub const MSG_ID: u16 = 20232;
}

impl PbMessage for SurvivorsOpDataForceChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ForceIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ForceIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ForceIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsOpDataUpdateNotify {
    pub ForceIncId: i32,
    pub Updates: Vec<SurvivorsOpData>,
    pub Adds: Vec<SurvivorsOpData>,
    pub Removes: Vec<i32>,
}

impl SurvivorsOpDataUpdateNotify {
    pub const MSG_ID: u16 = 20390;
}

impl PbMessage for SurvivorsOpDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ForceIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ForceIncId);
        }
        for item in &self.Updates {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.Adds {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.Removes {
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
            1 => self.ForceIncId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<SurvivorsOpData as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<SurvivorsOpData as PbMessage>::decode(bytes)?);
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Removes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsOpDoneRequest {
    pub IncId: i32,
    pub ClientSelectIds: Vec<i32>,
}

impl SurvivorsOpDoneRequest {
    pub const MSG_ID: u16 = 27513;
}

impl PbMessage for SurvivorsOpDoneRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        for item in &self.ClientSelectIds {
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
            6 => self.IncId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(
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
pub struct SurvivorsOpDoneResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SurvivorsOpDoneResponse {
    pub const MSG_ID: u16 = 18127;
}

impl PbMessage for SurvivorsOpDoneResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct SurvivorsOption {
    pub GoodsDetails: Vec<GoodsDetail>,
    pub IsSelect: bool,
    pub CanGiveUp: bool,
    pub CanMultiSelect: bool,
    pub MaxSelectCount: i32,
    pub UseSelectCount: i32,
    pub UseRefreshCount: i32,
    pub RefreshItem: i32,
    pub RefreshCost: i32,
}

impl PbMessage for SurvivorsOption {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GoodsDetails {
            crate::wire::write_submessage(out, 1, item);
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
                self.GoodsDetails
                    .push(<GoodsDetail as PbMessage>::decode(bytes)?);
            }
            1001 => self.IsSelect = r.read_bool()?,
            1002 => self.CanGiveUp = r.read_bool()?,
            1003 => self.CanMultiSelect = r.read_bool()?,
            1004 => self.MaxSelectCount = r.read_varint_i32()?,
            1005 => self.UseSelectCount = r.read_varint_i32()?,
            1007 => self.UseRefreshCount = r.read_varint_i32()?,
            1008 => self.RefreshItem = r.read_varint_i32()?,
            1009 => self.RefreshCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsPrepareStepPbData {}

impl PbMessage for SurvivorsPrepareStepPbData {
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
pub struct SurvivorsResultRequest {}

impl SurvivorsResultRequest {
    pub const MSG_ID: u16 = 17323;
}

impl PbMessage for SurvivorsResultRequest {
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
pub struct SurvivorsResultResponse {
    pub ErrCode: Option<ErrorCode>,
    pub ResultView: Option<ResultView>,
}

impl SurvivorsResultResponse {
    pub const MSG_ID: u16 = 24240;
}

impl PbMessage for SurvivorsResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ResultView {
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
                self.ErrCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ResultView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsRewardStepPbData {}

impl PbMessage for SurvivorsRewardStepPbData {
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
pub struct SurvivorsRole {
    pub Level: i32,
    pub MaxLevel: i32,
    pub Affixs: Vec<i32>,
    pub Evolves: Vec<i32>,
}

impl PbMessage for SurvivorsRole {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.MaxLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLevel);
        }
        for item in &self.Affixs {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Evolves {
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
            1 => self.Level = r.read_varint_i32()?,
            2 => self.MaxLevel = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Affixs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Evolves, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsRoleEvolve {
    pub RoleId: i32,
}

impl PbMessage for SurvivorsRoleEvolve {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsRoleKilledCtxPb {}

impl PbMessage for SurvivorsRoleKilledCtxPb {
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
pub struct SurvivorsRoleLv {
    pub RoleId: i32,
    pub SurvivorsRole: Option<SurvivorsRole>,
}

impl PbMessage for SurvivorsRoleLv {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.SurvivorsRole {
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
                self.SurvivorsRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsScoreAwardRequest {
    pub Configs: Vec<i32>,
}

impl SurvivorsScoreAwardRequest {
    pub const MSG_ID: u16 = 19499;
}

impl PbMessage for SurvivorsScoreAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Configs {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Configs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsScoreAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SurvivorsScoreAwardResponse {
    pub const MSG_ID: u16 = 23924;
}

impl PbMessage for SurvivorsScoreAwardResponse {
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
pub struct SurvivorsSpawnMonsterWaveEndNotify {
    pub CountdownSecond: i32,
}

impl SurvivorsSpawnMonsterWaveEndNotify {
    pub const MSG_ID: u16 = 21452;
}

impl PbMessage for SurvivorsSpawnMonsterWaveEndNotify {
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
pub struct SurvivorsStepAdvanceRequest {
    pub PrepareStepPbData: Option<SurvivorsPrepareStepPbData>,
    pub CombatStepPbData: Option<SurvivorsCombatStepPbData>,
    pub EndStepPbData: Option<SurvivorsEndStepPbData>,
    pub RewardStepPbData: Option<SurvivorsRewardStepPbData>,
}

impl SurvivorsStepAdvanceRequest {
    pub const MSG_ID: u16 = 28159;
}

impl PbMessage for SurvivorsStepAdvanceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PrepareStepPbData {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.CombatStepPbData {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.EndStepPbData {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.RewardStepPbData {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PrepareStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.CombatStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.EndStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RewardStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsStepAdvanceResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl SurvivorsStepAdvanceResponse {
    pub const MSG_ID: u16 = 23962;
}

impl PbMessage for SurvivorsStepAdvanceResponse {
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
pub struct SurvivorsStepUpdateNotify {
    pub EndStepPbData: Option<SurvivorsEndStepPbData>,
    pub RewardStepPbData: Option<SurvivorsRewardStepPbData>,
    pub PrepareStepPbData: Option<SurvivorsPrepareStepPbData>,
    pub CombatStepPbData: Option<SurvivorsCombatStepPbData>,
}

impl SurvivorsStepUpdateNotify {
    pub const MSG_ID: u16 = 24499;
}

impl PbMessage for SurvivorsStepUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EndStepPbData {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.RewardStepPbData {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.PrepareStepPbData {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.CombatStepPbData {
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
                self.EndStepPbData
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
                self.PrepareStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.CombatStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsTalentLevelUpRequest {
    pub SkillId: i32,
}

impl SurvivorsTalentLevelUpRequest {
    pub const MSG_ID: u16 = 24431;
}

impl PbMessage for SurvivorsTalentLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            13 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsTalentLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Level: i32,
}

impl SurvivorsTalentLevelUpResponse {
    pub const MSG_ID: u16 = 23700;
}

impl PbMessage for SurvivorsTalentLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsTalentUnlockNotify {
    pub SkillId: i32,
}

impl SurvivorsTalentUnlockNotify {
    pub const MSG_ID: u16 = 26156;
}

impl PbMessage for SurvivorsTalentUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsTaskDataUpdateNotify {
    pub ActivityTaskData: Option<ActivityTaskData>,
}

impl SurvivorsTaskDataUpdateNotify {
    pub const MSG_ID: u16 = 26512;
}

impl PbMessage for SurvivorsTaskDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTaskData {
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
pub struct SurvivorsToken {}

impl PbMessage for SurvivorsToken {
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
pub struct SurvivorsVarRecordNotify {
    pub VariableKeyNames: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl SurvivorsVarRecordNotify {
    pub const MSG_ID: u16 = 20648;
}

impl PbMessage for SurvivorsVarRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.VariableKeyNames.is_empty() {
            crate::wire::write_map(out, 15, &self.VariableKeyNames);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_map_entry(r, &mut self.VariableKeyNames)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsWeapon {
    pub Level: i32,
    pub MaxLevel: i32,
    pub Affixs: Vec<i32>,
    pub Evolves: Vec<i32>,
    pub KillMonsterCount: i32,
    pub WeaponBondId: i32,
}

impl PbMessage for SurvivorsWeapon {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.MaxLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLevel);
        }
        for item in &self.Affixs {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Evolves {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.KillMonsterCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillMonsterCount);
        }
        if self.WeaponBondId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBondId);
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
            2 => self.MaxLevel = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Affixs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Evolves, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => self.KillMonsterCount = r.read_varint_i32()?,
            6 => self.WeaponBondId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsWeaponEvolve {
    pub WeaponId: i32,
    pub WeaponBondId: i32,
    pub WeaponIncId: i32,
}

impl PbMessage for SurvivorsWeaponEvolve {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.WeaponBondId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBondId);
        }
        if self.WeaponIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponIncId);
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
            2 => self.WeaponBondId = r.read_varint_i32()?,
            3 => self.WeaponIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsWeaponInfoRequest {}

impl SurvivorsWeaponInfoRequest {
    pub const MSG_ID: u16 = 19901;
}

impl PbMessage for SurvivorsWeaponInfoRequest {
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
pub struct SurvivorsWeaponInfoResponse {
    pub WeaponKillDict: BTreeMap<i32, i32>,
}

impl SurvivorsWeaponInfoResponse {
    pub const MSG_ID: u16 = 23080;
}

impl PbMessage for SurvivorsWeaponInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.WeaponKillDict.is_empty() {
            crate::wire::write_map(out, 15, &self.WeaponKillDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_map_entry(r, &mut self.WeaponKillDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsWeaponLv {
    pub WeaponId: i32,
    pub SurvivorsWeapon: Option<SurvivorsWeapon>,
}

impl PbMessage for SurvivorsWeaponLv {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if let Some(v) = &self.SurvivorsWeapon {
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
            1 => self.WeaponId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsWeapon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchBattleModeRequest {
    pub Client: bool,
    pub ClientControllerModule: Option<BattleModule>,
}

impl SwitchBattleModeRequest {
    pub const MSG_ID: u16 = 18672;
}

impl PbMessage for SwitchBattleModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Client {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ClientControllerModule {
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
            5 => self.Client = r.read_bool()?,
            15 => {
                self.ClientControllerModule =
                    Some(<BattleModule as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchBattleModeResponse {
    pub Client: bool,
}

impl SwitchBattleModeResponse {
    pub const MSG_ID: u16 = 19436;
}

impl PbMessage for SwitchBattleModeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Client {
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
            9 => self.Client = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchNodeNotify {}

impl SwitchNodeNotify {
    pub const MSG_ID: u16 = 115;
}

impl PbMessage for SwitchNodeNotify {
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
pub struct SyncPlayerLocationNotify {
    pub LocationList: Vec<PlayerLocation>,
}

impl SyncPlayerLocationNotify {
    pub const MSG_ID: u16 = 24978;
}

impl PbMessage for SyncPlayerLocationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LocationList {
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
                self.LocationList
                    .push(<PlayerLocation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SynthesisInfoUpdateNotify {
    pub HideSynthesisIdList: Vec<i32>,
    pub SynthesisInfoList: Vec<OneSynthesisInfo>,
}

impl SynthesisInfoUpdateNotify {
    pub const MSG_ID: u16 = 26909;
}

impl PbMessage for SynthesisInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HideSynthesisIdList {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SynthesisInfoList {
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
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.HideSynthesisIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => {
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
pub struct SysErrorNotify {
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
}

impl SysErrorNotify {
    pub const MSG_ID: u16 = 27956;
}

impl PbMessage for SysErrorNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 6, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            6 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
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
pub struct SysInfoNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub IsShowErrorCodeTips: bool,
}

impl SysInfoNotify {
    pub const MSG_ID: u16 = 23221;
}

impl PbMessage for SysInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 11, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.IsShowErrorCodeTips {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            14 => self.IsShowErrorCodeTips = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SystemMarkHideInfoNotify {
    pub SystemMarkHideInfo: Vec<SystemMarkHideInfoPb>,
}

impl SystemMarkHideInfoNotify {
    pub const MSG_ID: u16 = 19492;
}

impl PbMessage for SystemMarkHideInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SystemMarkHideInfo {
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
                self.SystemMarkHideInfo
                    .push(<SystemMarkHideInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetGearHitRequest {
    pub BulletId: i32,
    pub EntityId: i64,
    pub BulletTypes: Vec<i32>,
    pub HitParts: Vec<Vec<u8>>,
}

impl TargetGearHitRequest {
    pub const MSG_ID: u16 = 20478;
}

impl PbMessage for TargetGearHitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.BulletTypes {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.HitParts {
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
            6 => self.BulletId = r.read_varint_i32()?,
            7 => self.EntityId = r.read_varint_i64()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BulletTypes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => self.HitParts.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TargetGearHitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TargetGearHitResponse {
    pub const MSG_ID: u16 = 15616;
}

impl PbMessage for TargetGearHitResponse {
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
pub struct TaskDoneInfo {
    pub TaskId: i32,
    pub State: Option<GatherTaskState>,
}

impl PbMessage for TaskDoneInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if let Some(v) = &self.State {
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
            1 => self.TaskId = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <GatherTaskState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TeamChallengeRequest {
    pub InstId: i32,
    pub InstEnterId: i32,
    pub IsInviteTeammate: bool,
}

impl TeamChallengeRequest {
    pub const MSG_ID: u16 = 16714;
}

impl PbMessage for TeamChallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.InstEnterId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstEnterId);
        }
        if self.IsInviteTeammate {
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
            2 => self.InstId = r.read_varint_i32()?,
            5 => self.InstEnterId = r.read_varint_i32()?,
            10 => self.IsInviteTeammate = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamChallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TeamChallengeResponse {
    pub const MSG_ID: u16 = 21288;
}

impl PbMessage for TeamChallengeResponse {
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
pub struct TeamMatchAcceptInviteNotify {
    pub PlayerId: i32,
    pub AcceptResult: Option<EMatchAcceptInviteResult>,
}

impl TeamMatchAcceptInviteNotify {
    pub const MSG_ID: u16 = 24254;
}

impl PbMessage for TeamMatchAcceptInviteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.AcceptResult {
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
            8 => self.PlayerId = r.read_varint_i32()?,
            11 => {
                self.AcceptResult = Some(
                    <EMatchAcceptInviteResult as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct TeamMatchAcceptInviteRequest {
    pub AcceptResult: Option<EMatchAcceptInviteResult>,
    pub HostId: i32,
    pub InstId: i32,
}

impl TeamMatchAcceptInviteRequest {
    pub const MSG_ID: u16 = 22419;
}

impl PbMessage for TeamMatchAcceptInviteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AcceptResult {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HostId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            3 => {
                self.AcceptResult = Some(
                    <EMatchAcceptInviteResult as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => self.HostId = r.read_varint_i32()?,
            13 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamMatchAcceptInviteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TeamMatchAcceptInviteResponse {
    pub const MSG_ID: u16 = 26064;
}

impl PbMessage for TeamMatchAcceptInviteResponse {
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
pub struct TeamMatchFlagNotify {
    pub MatchFlag: bool,
    pub InstId: i32,
}

impl TeamMatchFlagNotify {
    pub const MSG_ID: u16 = 15636;
}

impl PbMessage for TeamMatchFlagNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MatchFlag {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            13 => self.MatchFlag = r.read_bool()?,
            14 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamMatchInviteNotify {
    pub InstId: i32,
    pub HostId: i32,
}

impl TeamMatchInviteNotify {
    pub const MSG_ID: u16 = 24663;
}

impl PbMessage for TeamMatchInviteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.HostId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.InstId = r.read_varint_i32()?,
            14 => self.HostId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamMatchInviteRequest {
    pub InstId: i32,
}

impl TeamMatchInviteRequest {
    pub const MSG_ID: u16 = 18437;
}

impl PbMessage for TeamMatchInviteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamMatchInviteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TeamMatchInviteResponse {
    pub const MSG_ID: u16 = 17491;
}

impl PbMessage for TeamMatchInviteResponse {
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
pub struct TeamMatchInviteRetToHostNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub InstId: i32,
}

impl TeamMatchInviteRetToHostNotify {
    pub const MSG_ID: u16 = 15606;
}

impl PbMessage for TeamMatchInviteRetToHostNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamParkourLevelActivityInfo {
    pub TeamParkourLevelPlayInfos: Vec<TeamParkourLevelPlayInfo>,
    pub ActivityTaskDatas: Option<ActivityTaskData>,
}

impl PbMessage for TeamParkourLevelActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TeamParkourLevelPlayInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.ActivityTaskDatas {
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
                self.TeamParkourLevelPlayInfos
                    .push(<TeamParkourLevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTaskDatas
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamParkourLevelPlayInfo {
    pub LevelId: i32,
    pub Score: i32,
    pub Ranking: i32,
    pub StartTime: i32,
    pub TimeCost: i32,
}

impl PbMessage for TeamParkourLevelPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.Ranking != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ranking);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartTime);
        }
        if self.TimeCost != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeCost);
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
            2 => self.Score = r.read_varint_i32()?,
            3 => self.Ranking = r.read_varint_i32()?,
            4 => self.StartTime = r.read_varint_i32()?,
            5 => self.TimeCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamParkourRewardRequest {
    pub RewardId: i32,
    pub ActivityId: i32,
}

impl TeamParkourRewardRequest {
    pub const MSG_ID: u16 = 19943;
}

impl PbMessage for TeamParkourRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
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
            9 => self.RewardId = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamParkourRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TeamParkourRewardResponse {
    pub const MSG_ID: u16 = 28854;
}

impl PbMessage for TeamParkourRewardResponse {
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
pub struct TeamParkourSettleNotify {
    pub LevelId: i32,
    pub PlayerSettleInfos: Vec<PlayerSettleInfo>,
}

impl TeamParkourSettleNotify {
    pub const MSG_ID: u16 = 16281;
}

impl PbMessage for TeamParkourSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.PlayerSettleInfos {
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
            11 => self.LevelId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerSettleInfos
                    .push(<PlayerSettleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeamParkourTaskNotify {
    pub ActivityTask: Option<ActivityTask>,
}

impl TeamParkourTaskNotify {
    pub const MSG_ID: u16 = 18720;
}

impl PbMessage for TeamParkourTaskNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityTask {
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
pub struct TeamTeleportStartNotify {
    pub PlayerId: i32,
    pub TimeoutSec: i32,
}

impl TeamTeleportStartNotify {
    pub const MSG_ID: u16 = 25696;
}

impl PbMessage for TeamTeleportStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.TimeoutSec != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeoutSec);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.PlayerId = r.read_varint_i32()?,
            11 => self.TimeoutSec = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportDungeonForbidNotify {
    pub DungeonId: i32,
}

impl TeleportDungeonForbidNotify {
    pub const MSG_ID: u16 = 28830;
}

impl PbMessage for TeleportDungeonForbidNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DungeonId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DungeonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.DungeonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TeleportVehicleNotify {
    pub Reason: Option<TeleportReason>,
    pub Vehicle: i64,
    pub TransitionOption: Option<TransitionOptionPb>,
    pub NoLoading: bool,
    pub Rotation: Option<Rotator>,
    pub PsgEntityIds: Vec<i64>,
    pub Location: Option<Vector>,
    pub PsgPlayerIds: Vec<i32>,
}

impl TeleportVehicleNotify {
    pub const MSG_ID: u16 = 22711;
}

impl PbMessage for TeleportVehicleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Vehicle != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Vehicle);
        }
        if let Some(v) = &self.TransitionOption {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.NoLoading {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 8, v);
        }
        for item in &self.PsgEntityIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 13, v);
        }
        for item in &self.PsgPlayerIds {
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
                self.Reason = Some(
                    <TeleportReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.Vehicle = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TransitionOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.NoLoading = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PsgEntityIds, |sr| {
                    Ok(sr.read_varint_i64()?)
                })?
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PsgPlayerIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateEntitySpawnerContentNotify {
    pub EntityId: i64,
    pub SpawnerType: Option<TemplateSpawnerType>,
    pub CreateEntityInfos: Vec<SpawnerEntityInfo>,
}

impl TemplateEntitySpawnerContentNotify {
    pub const MSG_ID: u16 = 18989;
}

impl PbMessage for TemplateEntitySpawnerContentNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.SpawnerType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.CreateEntityInfos {
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
            1 => self.EntityId = r.read_varint_i64()?,
            4 => {
                self.SpawnerType = Some(
                    <TemplateSpawnerType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CreateEntityInfos
                    .push(<SpawnerEntityInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestCmdCompleteParams {
    pub Result: bool,
}

impl PbMessage for TestCmdCompleteParams {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Result {
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
            1 => self.Result = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
