use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovePackageNotify {
    pub MovingEntities: Vec<MovingEntityData>,
}

impl MovePackageNotify {
    pub const MSG_ID: u16 = 17980;
}

impl PbMessage for MovePackageNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MovingEntities {
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
                self.MovingEntities
                    .push(<MovingEntityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntitySimplyMoveInfoPackagePush {
    pub MoveInfos: Vec<EntitySimplyMoveInfo>,
    pub SceneOwnerId: i32,
}

impl EntitySimplyMoveInfoPackagePush {
    pub const MSG_ID: u16 = 23337;
}

impl PbMessage for EntitySimplyMoveInfoPackagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MoveInfos {
            crate::wire::write_submessage(out, 15, item);
        }
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            15 => {
                let bytes = r.read_len_delimited()?;
                self.MoveInfos
                    .push(<EntitySimplyMoveInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.SceneOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveSummonEntityRequest {
    pub SummonerId: i64,
    pub SkillId: i32,
    pub RemoveType: Option<ERemoveEntityType>,
    pub RemoveENtityIds: Vec<i64>,
}

impl RemoveSummonEntityRequest {
    pub const MSG_ID: u16 = 17165;
}

impl PbMessage for RemoveSummonEntityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if let Some(v) = &self.RemoveType {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RemoveENtityIds {
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
            1 => self.SummonerId = r.read_varint_i64()?,
            5 => self.SkillId = r.read_varint_i32()?,
            6 => {
                self.RemoveType = Some(
                    <ERemoveEntityType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RemoveENtityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveSummonEntityResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RemoveSummonEntityResponse {
    pub const MSG_ID: u16 = 28927;
}

impl PbMessage for RemoveSummonEntityResponse {
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
pub struct EntityOnLandedRequest {
    pub EntityId: i64,
}

impl EntityOnLandedRequest {
    pub const MSG_ID: u16 = 21482;
}

impl PbMessage for EntityOnLandedRequest {
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
pub struct EntityOnLandedResponse {}

impl EntityOnLandedResponse {
    pub const MSG_ID: u16 = 27404;
}

impl PbMessage for EntityOnLandedResponse {
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
pub struct AttributeChangedRequest {
    pub Id: i64,
    pub Attributes: Vec<GameplayAttributeData>,
}

impl AttributeChangedRequest {
    pub const MSG_ID: u16 = 19018;
}

impl PbMessage for AttributeChangedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.Attributes {
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
            11 => self.Id = r.read_varint_i64()?,
            1 => {
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
pub struct AttributeChangedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AttributeChangedResponse {
    pub const MSG_ID: u16 = 24670;
}

impl PbMessage for AttributeChangedResponse {
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
pub struct AttributeChangedNotify {
    pub Attributes: Vec<GameplayAttributeData>,
}

impl AttributeChangedNotify {
    pub const MSG_ID: u16 = 26859;
}

impl PbMessage for AttributeChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attributes {
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
                self.Attributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateInitRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateInitRequest {
    pub const MSG_ID: u16 = 29999;
}

impl PbMessage for AnimationStateInitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Id = r.read_varint_i64()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateInitPush {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateInitPush {
    pub const MSG_ID: u16 = 22251;
}

impl PbMessage for AnimationStateInitPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.Id = r.read_varint_i64()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            15 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateInitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimationStateInitResponse {
    pub const MSG_ID: u16 = 27644;
}

impl PbMessage for AnimationStateInitResponse {
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
pub struct AnimationStateInitNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub TimeStamp: f32,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateInitNotify {
    pub const MSG_ID: u16 = 23601;
}

impl PbMessage for AnimationStateInitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 12, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Id = r.read_varint_i64()?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            1 => self.TimeStamp = r.read_f32()?,
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateChangedRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateChangedRequest {
    pub const MSG_ID: u16 = 17015;
}

impl PbMessage for AnimationStateChangedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.Id = r.read_varint_i64()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            1 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateChangedPush {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateChangedPush {
    pub const MSG_ID: u16 = 24970;
}

impl PbMessage for AnimationStateChangedPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.Id = r.read_varint_i64()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            1 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            9 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationStateChangedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimationStateChangedResponse {
    pub const MSG_ID: u16 = 23977;
}

impl PbMessage for AnimationStateChangedResponse {
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
pub struct AnimationStateChangedNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub States: Vec<i32>,
    pub TimeStamp: f32,
    pub SpecialStates: Vec<i32>,
    pub ModelId: i32,
}

impl AnimationStateChangedNotify {
    pub const MSG_ID: u16 = 24252;
}

impl PbMessage for AnimationStateChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.States {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        for item in &self.SpecialStates {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.Id = r.read_varint_i64()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.States, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.TimeStamp = r.read_f32()?,
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SpecialStates, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimStateChangeInfo {
    pub AnimationStates: Vec<i32>,
    pub SpecialAnimationStates: Vec<i32>,
    pub ModelId: i32,
}

impl PbMessage for AnimStateChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AnimationStates {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SpecialAnimationStates {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ModelId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModelId);
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
                &mut self.AnimationStates,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SpecialAnimationStates,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => self.ModelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimStateChangeInfoList {
    pub EntityId: i64,
    pub AnimStateChangeInfo: Vec<AnimStateChangeInfo>,
}

impl PbMessage for AnimStateChangeInfoList {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.AnimStateChangeInfo {
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
                self.AnimStateChangeInfo
                    .push(<AnimStateChangeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PackAnimChangedNotify {
    pub EntityAnimState: Vec<AnimStateChangeInfoList>,
}

impl PackAnimChangedNotify {
    pub const MSG_ID: u16 = 23124;
}

impl PbMessage for PackAnimChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityAnimState {
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
                self.EntityAnimState
                    .push(<AnimStateChangeInfoList as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityCommonTagNotify {
    pub Id: i64,
    pub Tags: Vec<CommonTagData>,
}

impl EntityCommonTagNotify {
    pub const MSG_ID: u16 = 20031;
}

impl PbMessage for EntityCommonTagNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.Tags {
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
            13 => self.Id = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Tags.push(<CommonTagData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonRequestInfo {
    pub SummonEntityId: i64,
    pub SkillId: i32,
    pub SummonConfigId: i32,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub IsVisible: bool,
}

impl PbMessage for SummonRequestInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonEntityId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.SummonConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonConfigId);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.IsVisible {
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
            1 => self.SummonEntityId = r.read_varint_i64()?,
            2 => self.SkillId = r.read_varint_i32()?,
            3 => self.SummonConfigId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.IsVisible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonRequest {
    pub SummonerEntityId: i64,
    pub SummonInfo: Option<SummonRequestInfo>,
}

impl SummonRequest {
    pub const MSG_ID: u16 = 26598;
}

impl PbMessage for SummonRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerEntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerEntityId);
        }
        if let Some(v) = &self.SummonInfo {
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
            9 => self.SummonerEntityId = r.read_varint_i64()?,
            7 => {
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
pub struct SummonResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SummonResponse {
    pub const MSG_ID: u16 = 15829;
}

impl PbMessage for SummonResponse {
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
pub struct Summon3Request {
    pub SummonerEntityId: i64,
    pub SummonInfo: Option<SummonRequestInfo>,
}

impl Summon3Request {
    pub const MSG_ID: u16 = 29752;
}

impl PbMessage for Summon3Request {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerEntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerEntityId);
        }
        if let Some(v) = &self.SummonInfo {
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
            6 => self.SummonerEntityId = r.read_varint_i64()?,
            10 => {
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
pub struct Summon3Response {
    pub ErrorCode: Option<ErrorCode>,
}

impl Summon3Response {
    pub const MSG_ID: u16 = 25946;
}

impl PbMessage for Summon3Response {
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
pub struct SummonEntityNotify {
    pub SummonerId: i64,
    pub SummonIds: Vec<i64>,
}

impl SummonEntityNotify {
    pub const MSG_ID: u16 = 19653;
}

impl PbMessage for SummonEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
        }
        for item in &self.SummonIds {
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
            3 => self.SummonerId = r.read_varint_i64()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SummonIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateSceneDateRequest {
    pub AddDays: u32,
    pub Hour: i32,
    pub Minute: i32,
    pub Reason: Option<SceneDateUpdateReason>,
}

impl UpdateSceneDateRequest {
    pub const MSG_ID: u16 = 25350;
}

impl PbMessage for UpdateSceneDateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AddDays != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.AddDays));
        }
        if self.Hour != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Hour);
        }
        if self.Minute != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Minute);
        }
        if let Some(v) = &self.Reason {
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
            4 => self.AddDays = r.read_varint_u32()?,
            3 => self.Hour = r.read_varint_i32()?,
            9 => self.Minute = r.read_varint_i32()?,
            15 => {
                self.Reason = Some(
                    <SceneDateUpdateReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct UpdateSceneDateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CurrDate: u32,
}

impl UpdateSceneDateResponse {
    pub const MSG_ID: u16 = 24289;
}

impl PbMessage for UpdateSceneDateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurrDate != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.CurrDate));
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
            11 => self.CurrDate = r.read_varint_u32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSceneAoiData {
    pub DynamicEntityList: Vec<DynamicEntityInformation>,
    pub GenIds: Vec<i64>,
    pub Entities: Vec<EntityPb>,
}

impl PbMessage for PlayerSceneAoiData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DynamicEntityList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.GenIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.Entities {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.DynamicEntityList
                    .push(<DynamicEntityInformation as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GenIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Entities.push(<EntityPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonInfo {
    pub SummonCfgId: i32,
    pub SummonerId: i64,
    pub SummonSkillId: i32,
}

impl PbMessage for SummonInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonCfgId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonCfgId);
        }
        if self.SummonerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonerId);
        }
        if self.SummonSkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SummonSkillId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SummonCfgId = r.read_varint_i32()?,
            2 => self.SummonerId = r.read_varint_i64()?,
            3 => self.SummonSkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeatherControlInfoWithoutCheckAsyncRequest {}

impl WeatherControlInfoWithoutCheckAsyncRequest {
    pub const MSG_ID: u16 = 27924;
}

impl PbMessage for WeatherControlInfoWithoutCheckAsyncRequest {
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
pub struct WeatherControlInfoWithoutCheckAsyncResponse {
    pub UnlockedWeatherSwitchConfigIdList: Vec<i32>,
}

impl WeatherControlInfoWithoutCheckAsyncResponse {
    pub const MSG_ID: u16 = 27053;
}

impl PbMessage for WeatherControlInfoWithoutCheckAsyncResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockedWeatherSwitchConfigIdList {
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
                &mut self.UnlockedWeatherSwitchConfigIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFollowTrackRequest {
    pub EntityId: i64,
}

impl EntityFollowTrackRequest {
    pub const MSG_ID: u16 = 25969;
}

impl PbMessage for EntityFollowTrackRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFollowTrackResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityFollowTrackResponse {
    pub const MSG_ID: u16 = 21101;
}

impl PbMessage for EntityFollowTrackResponse {
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
pub struct PlayerRebackSceneNotify {
    pub EntityId: i64,
}

impl PlayerRebackSceneNotify {
    pub const MSG_ID: u16 = 26270;
}

impl PbMessage for PlayerRebackSceneNotify {
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
pub struct DrownRequest {}

impl DrownRequest {
    pub const MSG_ID: u16 = 25778;
}

impl PbMessage for DrownRequest {
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
pub struct DrownPush {}

impl DrownPush {
    pub const MSG_ID: u16 = 29058;
}

impl PbMessage for DrownPush {
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
pub struct DrownResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DrownResponse {
    pub const MSG_ID: u16 = 21329;
}

impl PbMessage for DrownResponse {
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
pub struct DrownNotify {}

impl DrownNotify {
    pub const MSG_ID: u16 = 16564;
}

impl PbMessage for DrownNotify {
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
pub struct DrownEndTeleportRequest {}

impl DrownEndTeleportRequest {
    pub const MSG_ID: u16 = 21245;
}

impl PbMessage for DrownEndTeleportRequest {
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

#[derive(Debug, Clone, PartialEq)]
pub enum DrownEndTeleportPushYcuOneof {
    TeleportPos(Vector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrownEndTeleportPush {
    pub ycu: Option<DrownEndTeleportPushYcuOneof>,
}

impl DrownEndTeleportPush {
    pub const MSG_ID: u16 = 23918;
}

impl PbMessage for DrownEndTeleportPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ycu {
            match oneof {
                DrownEndTeleportPushYcuOneof::TeleportPos(v) => {
                    crate::wire::write_submessage(out, 7, v);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.ycu = Some(DrownEndTeleportPushYcuOneof::TeleportPos(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DrownEndTeleportResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DrownEndTeleportResponse {
    pub const MSG_ID: u16 = 20620;
}

impl PbMessage for DrownEndTeleportResponse {
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
pub struct MonsterDrownRequest {
    pub Pos: Option<Vector>,
}

impl MonsterDrownRequest {
    pub const MSG_ID: u16 = 20753;
}

impl PbMessage for MonsterDrownRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterDrownPush {
    pub Pos: Option<Vector>,
}

impl MonsterDrownPush {
    pub const MSG_ID: u16 = 26858;
}

impl PbMessage for MonsterDrownPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterDrownResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MonsterDrownResponse {
    pub const MSG_ID: u16 = 22186;
}

impl PbMessage for MonsterDrownResponse {
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
pub struct SceneLoadingFinishRequest {
    pub SceneId: Vec<u8>,
}

impl SceneLoadingFinishRequest {
    pub const MSG_ID: u16 = 27192;
}

impl PbMessage for SceneLoadingFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
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
            2 => self.SceneId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneLoadingFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SceneLoadingFinishResponse {
    pub const MSG_ID: u16 = 28792;
}

impl PbMessage for SceneLoadingFinishResponse {
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
pub struct UpdateVoxelEnvRequest {
    pub ServerCaveMode: i32,
}

impl UpdateVoxelEnvRequest {
    pub const MSG_ID: u16 = 27177;
}

impl PbMessage for UpdateVoxelEnvRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ServerCaveMode != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerCaveMode);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.ServerCaveMode = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateVoxelEnvResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ServerCaveMode: i32,
}

impl UpdateVoxelEnvResponse {
    pub const MSG_ID: u16 = 15604;
}

impl PbMessage for UpdateVoxelEnvResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerCaveMode != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerCaveMode);
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
            6 => self.ServerCaveMode = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneRoadSyncNotify {
    pub SceneId: Vec<u8>,
    pub InstanceId: i32,
    pub EnabledRoads: Vec<i32>,
}

impl SceneRoadSyncNotify {
    pub const MSG_ID: u16 = 17096;
}

impl PbMessage for SceneRoadSyncNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SceneId);
        }
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
        }
        for item in &self.EnabledRoads {
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
            11 => self.SceneId = r.read_len_delimited()?.to_vec(),
            1 => self.InstanceId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EnabledRoads, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BlackScreenStatePush {
    pub IsOpen: bool,
    pub Source: Option<BlackScreenSourceType>,
    pub BlackScreenReason: Vec<u8>,
}

impl BlackScreenStatePush {
    pub const MSG_ID: u16 = 15727;
}

impl PbMessage for BlackScreenStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsOpen {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Source {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.BlackScreenReason.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.BlackScreenReason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.IsOpen = r.read_bool()?,
            14 => {
                self.Source = Some(
                    <BlackScreenSourceType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.BlackScreenReason = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MontageContext {
    pub SkillId: i64,
    pub MontageIndex: i32,
    pub ConstateId: i64,
}

impl PbMessage for MontageContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ConstateId);
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
            2 => self.MontageIndex = r.read_varint_i32()?,
            3 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitInformation {
    pub Originator: i64,
    pub Id: i64,
    pub TargetId: i64,
    pub BulletId: i64,
    pub HasBeHitData: bool,
    pub HitEffectPos: Option<Vector>,
    pub HitEffectRotate: Option<Rotator>,
    pub IsShake: bool,
    pub HitPos: Option<Vector>,
    pub EnterFk: bool,
    pub IsHitWeakness: bool,
    pub IsTriggerCounterattack: bool,
    pub VictimRotation: Option<Rotator>,
    pub IsChangeVictimRotation: bool,
    pub HitPart: Vec<u8>,
    pub IsTriggerVisionCounterAttack: bool,
    pub SkillId: i64,
    pub FightState: i32,
    pub BeHitAnim: i32,
    pub Source: Option<EBulletCreateSource>,
    pub PhantomSkillIdentify: i32,
}

impl PbMessage for HitInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Originator != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Originator);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if self.HasBeHitData {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.HitEffectPos {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.HitEffectRotate {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IsShake {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.HitPos {
            crate::wire::write_submessage(out, 9, v);
        }
        if self.EnterFk {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsHitWeakness {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsTriggerCounterattack {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.VictimRotation {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.IsChangeVictimRotation {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.HitPart.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.HitPart);
        }
        if self.IsTriggerVisionCounterAttack {
            crate::wire::write_tag(out, 16, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.FightState != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightState);
        }
        if self.BeHitAnim != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BeHitAnim);
        }
        if let Some(v) = &self.Source {
            crate::wire::write_tag(out, 20, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PhantomSkillIdentify != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomSkillIdentify);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Originator = r.read_varint_i64()?,
            2 => self.Id = r.read_varint_i64()?,
            3 => self.TargetId = r.read_varint_i64()?,
            4 => self.BulletId = r.read_varint_i64()?,
            5 => self.HasBeHitData = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.HitEffectPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.HitEffectRotate
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.IsShake = r.read_bool()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.HitPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.EnterFk = r.read_bool()?,
            11 => self.IsHitWeakness = r.read_bool()?,
            12 => self.IsTriggerCounterattack = r.read_bool()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.VictimRotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.IsChangeVictimRotation = r.read_bool()?,
            15 => self.HitPart = r.read_len_delimited()?.to_vec(),
            16 => self.IsTriggerVisionCounterAttack = r.read_bool()?,
            17 => self.SkillId = r.read_varint_i64()?,
            18 => self.FightState = r.read_varint_i32()?,
            19 => self.BeHitAnim = r.read_varint_i32()?,
            20 => {
                self.Source = Some(
                    <EBulletCreateSource as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            21 => self.PhantomSkillIdentify = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CounterAttackInfo {
    pub Id: i64,
    pub FightState: i32,
    pub TriggerCounterType: i32,
    pub CounterAnIndex: i32,
}

impl PbMessage for CounterAttackInfo {
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
pub struct SkillNodeInfo {
    pub SubProtocol: i32,
    pub MontageIndex: i32,
    pub SpeedRatio: f32,
    pub SkillSingleId: i32,
    pub SkillIndex: i32,
    pub StartSection: Vec<u8>,
    pub StartTimeSeconds: f32,
}

impl PbMessage for SkillNodeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubProtocol != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubProtocol);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
        }
        if self.SkillIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillIndex);
        }
        if !self.StartSection.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StartSection);
        }
        if self.StartTimeSeconds != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.StartTimeSeconds).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SubProtocol = r.read_varint_i32()?,
            2 => self.MontageIndex = r.read_varint_i32()?,
            3 => self.SpeedRatio = r.read_f32()?,
            4 => self.SkillSingleId = r.read_varint_i32()?,
            5 => self.SkillIndex = r.read_varint_i32()?,
            6 => self.StartSection = r.read_len_delimited()?.to_vec(),
            7 => self.StartTimeSeconds = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillRequest {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillNodeInfos: Option<SkillNodeInfo>,
}

impl SkillRequest {
    pub const MSG_ID: u16 = 28732;
}

impl PbMessage for SkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.SkillNodeInfos {
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SkillNodeInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SkillResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SkillResponse {
    pub const MSG_ID: u16 = 17453;
}

impl PbMessage for SkillResponse {
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
pub struct SkillNotify {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillNodeInfos: Option<SkillNodeInfo>,
}

impl SkillNotify {
    pub const MSG_ID: u16 = 19915;
}

impl PbMessage for SkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.SkillNodeInfos {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkillNodeInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseSkillRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
    pub BattleFlags: Vec<i32>,
}

impl UseSkillRequest {
    pub const MSG_ID: u16 = 15873;
}

impl PbMessage for UseSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
        }
        for item in &self.BattleFlags {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.SkillSingleId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BattleFlags, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseSkillResponse {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl UseSkillResponse {
    pub const MSG_ID: u16 = 21045;
}

impl PbMessage for UseSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.SkillSingleId = r.read_varint_i32()?,
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
pub struct UseSkillNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
}

impl UseSkillNotify {
    pub const MSG_ID: u16 = 19813;
}

impl PbMessage for UseSkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 15, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.SkillSingleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InterruptSkillInfo {
    pub EntityId: i64,
    pub SkillId: i64,
    pub BulletId: i64,
}

impl PbMessage for InterruptSkillInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
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
            2 => self.SkillId = r.read_varint_i64()?,
            3 => self.BulletId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EndSkillRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
    pub Reason: Option<EEndSkillReason>,
    pub InterruptSkillInfo: Option<InterruptSkillInfo>,
}

impl EndSkillRequest {
    pub const MSG_ID: u16 = 26657;
}

impl PbMessage for EndSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InterruptSkillInfo {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.SkillSingleId = r.read_varint_i32()?,
            8 => {
                self.Reason = Some(
                    <EEndSkillReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.InterruptSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EndSkillPush {
    pub CombatCommon: Option<CombatCommon>,
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
    pub Reason: Option<EEndSkillReason>,
    pub InterruptSkillInfo: Option<InterruptSkillInfo>,
}

impl EndSkillPush {
    pub const MSG_ID: u16 = 18120;
}

impl PbMessage for EndSkillPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InterruptSkillInfo {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.SkillSingleId = r.read_varint_i32()?,
            1 => {
                self.Reason = Some(
                    <EEndSkillReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.InterruptSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EndSkillResponse {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl EndSkillResponse {
    pub const MSG_ID: u16 = 24932;
}

impl PbMessage for EndSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1 => self.SkillSingleId = r.read_varint_i32()?,
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
pub struct EndSkillNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub SkillSingleId: i32,
}

impl EndSkillNotify {
    pub const MSG_ID: u16 = 27580;
}

impl PbMessage for EndSkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.UseSkillInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.SkillSingleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillSingleId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.UseSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.SkillSingleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InterruptSkillInDelayRequest {
    pub SkillId: i64,
}

impl InterruptSkillInDelayRequest {
    pub const MSG_ID: u16 = 24013;
}

impl PbMessage for InterruptSkillInDelayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InterruptSkillInDelayPush {
    pub SkillId: i64,
}

impl InterruptSkillInDelayPush {
    pub const MSG_ID: u16 = 25432;
}

impl PbMessage for InterruptSkillInDelayPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InterruptSkillInDelayResponse {
    pub SkillId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl InterruptSkillInDelayResponse {
    pub const MSG_ID: u16 = 28034;
}

impl PbMessage for InterruptSkillInDelayResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
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
            3 => self.SkillId = r.read_varint_i64()?,
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
pub struct UseSkillFailRequest {
    pub SkillId: i64,
}

impl UseSkillFailRequest {
    pub const MSG_ID: u16 = 15861;
}

impl PbMessage for UseSkillFailRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseSkillFailPush {
    pub SkillId: i64,
}

impl UseSkillFailPush {
    pub const MSG_ID: u16 = 15749;
}

impl PbMessage for UseSkillFailPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseSkillFailResponse {
    pub SkillId: i64,
    pub Error: Option<DErrorResult>,
}

impl UseSkillFailResponse {
    pub const MSG_ID: u16 = 22556;
}

impl PbMessage for UseSkillFailResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if let Some(v) = &self.Error {
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
            11 => self.SkillId = r.read_varint_i64()?,
            3 => {
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
pub struct CounterAttackPush {
    pub CounterAttackInfo: Option<CounterAttackInfo>,
}

impl CounterAttackPush {
    pub const MSG_ID: u16 = 21161;
}

impl PbMessage for CounterAttackPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CounterAttackInfo {
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
                self.CounterAttackInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub HitInfo: Option<HitInformation>,
    pub SkillMessageId: i64,
}

impl HitRequest {
    pub const MSG_ID: u16 = 17489;
}

impl PbMessage for HitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.HitInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.SkillMessageId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillMessageId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.HitInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.SkillMessageId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitResponse {
    pub HitInfo: Option<HitInformation>,
    pub ErrorCode: Option<ErrorCode>,
}

impl HitResponse {
    pub const MSG_ID: u16 = 27996;
}

impl PbMessage for HitResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HitInfo {
            crate::wire::write_submessage(out, 13, v);
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.HitInfo
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
pub struct HitNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub HitInfo: Option<HitInformation>,
}

impl HitNotify {
    pub const MSG_ID: u16 = 20656;
}

impl PbMessage for HitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.HitInfo {
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.HitInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitEndRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub TargetId: i64,
}

impl HitEndRequest {
    pub const MSG_ID: u16 = 19130;
}

impl PbMessage for HitEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.TargetId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitEndPush {
    pub CombatCommon: Option<CombatCommon>,
    pub TargetId: i64,
}

impl HitEndPush {
    pub const MSG_ID: u16 = 15924;
}

impl PbMessage for HitEndPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.TargetId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HitEndResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HitEndResponse {
    pub const MSG_ID: u16 = 27143;
}

impl PbMessage for HitEndResponse {
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
pub struct CaughtInfo {
    pub Attacker: i64,
    pub CaughtInfoId: i64,
    pub IsEnd: bool,
    pub FightState: i32,
}

impl PbMessage for CaughtInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Attacker != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Attacker);
        }
        if self.CaughtInfoId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CaughtInfoId);
        }
        if self.IsEnd {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FightState != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Attacker = r.read_varint_i64()?,
            2 => self.CaughtInfoId = r.read_varint_i64()?,
            3 => self.IsEnd = r.read_bool()?,
            4 => self.FightState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CaughtRequest {
    pub Info: Option<CaughtInfo>,
}

impl CaughtRequest {
    pub const MSG_ID: u16 = 17706;
}

impl PbMessage for CaughtRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct CaughtPush {
    pub Info: Option<CaughtInfo>,
}

impl CaughtPush {
    pub const MSG_ID: u16 = 16870;
}

impl PbMessage for CaughtPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct CaughtResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CaughtResponse {
    pub const MSG_ID: u16 = 22560;
}

impl PbMessage for CaughtResponse {
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
pub struct CaughtNotify {
    pub Info: Option<CaughtInfo>,
}

impl CaughtNotify {
    pub const MSG_ID: u16 = 29446;
}

impl PbMessage for CaughtNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct ActiveBulletHandle {
    pub PlayerId: i32,
    pub HandleId: i32,
}

impl PbMessage for ActiveBulletHandle {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
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
            2 => self.HandleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreateBulletRequestParentHandleOneof {
    BulletHandle(ActiveBulletHandle),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateBulletRequest {
    pub ParentHandle: Option<CreateBulletRequestParentHandleOneof>,
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
    pub OwnerEntityId: i64,
    pub BulletId: i64,
    pub SkillId: i64,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub TargetId: i64,
    pub SpawnEntityId: i64,
    pub SpawnVelocityEntityId: i64,
    pub IsLocal: bool,
    pub DtType: i32,
    pub RandomPosOffset: Option<Vector>,
    pub RandomInitSpeedOffset: Option<Vector>,
}

impl CreateBulletRequest {
    pub const MSG_ID: u16 = 18246;
}

impl PbMessage for CreateBulletRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ParentHandle {
            match oneof {
                CreateBulletRequestParentHandleOneof::BulletHandle(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.OwnerEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerEntityId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.SpawnEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnEntityId);
        }
        if self.SpawnVelocityEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnVelocityEntityId);
        }
        if self.IsLocal {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DtType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DtType);
        }
        if let Some(v) = &self.RandomPosOffset {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.RandomInitSpeedOffset {
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
                let val = <ActiveBulletHandle as PbMessage>::decode(bytes)?;
                self.ParentHandle = Some(CreateBulletRequestParentHandleOneof::BulletHandle(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.OwnerEntityId = r.read_varint_i64()?,
            12 => self.BulletId = r.read_varint_i64()?,
            15 => self.SkillId = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.TargetId = r.read_varint_i64()?,
            8 => self.SpawnEntityId = r.read_varint_i64()?,
            10 => self.SpawnVelocityEntityId = r.read_varint_i64()?,
            7 => self.IsLocal = r.read_bool()?,
            5 => self.DtType = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RandomPosOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.RandomInitSpeedOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreateBulletResponsePushParentHandleOneof {
    BulletHandle(ActiveBulletHandle),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateBulletResponsePush {
    pub ParentHandle: Option<CreateBulletResponsePushParentHandleOneof>,
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
    pub OwnerEntityId: i64,
    pub BulletId: i64,
    pub SkillId: i64,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub TargetId: i64,
    pub SpawnEntityId: i64,
    pub SpawnVelocityEntityId: i64,
    pub IsLocal: bool,
    pub DtType: i32,
    pub RandomPosOffset: Option<Vector>,
    pub RandomInitSpeedOffset: Option<Vector>,
}

impl CreateBulletResponsePush {
    pub const MSG_ID: u16 = 26882;
}

impl PbMessage for CreateBulletResponsePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ParentHandle {
            match oneof {
                CreateBulletResponsePushParentHandleOneof::BulletHandle(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.OwnerEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerEntityId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.SpawnEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnEntityId);
        }
        if self.SpawnVelocityEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnVelocityEntityId);
        }
        if self.IsLocal {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DtType != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DtType);
        }
        if let Some(v) = &self.RandomPosOffset {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.RandomInitSpeedOffset {
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
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActiveBulletHandle as PbMessage>::decode(bytes)?;
                self.ParentHandle =
                    Some(CreateBulletResponsePushParentHandleOneof::BulletHandle(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.OwnerEntityId = r.read_varint_i64()?,
            12 => self.BulletId = r.read_varint_i64()?,
            3 => self.SkillId = r.read_varint_i64()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.TargetId = r.read_varint_i64()?,
            13 => self.SpawnEntityId = r.read_varint_i64()?,
            4 => self.SpawnVelocityEntityId = r.read_varint_i64()?,
            10 => self.IsLocal = r.read_bool()?,
            11 => self.DtType = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RandomPosOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RandomInitSpeedOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateBulletResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CreateBulletResponse {
    pub const MSG_ID: u16 = 19384;
}

impl PbMessage for CreateBulletResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum CreateBulletNotifyParentHandleOneof {
    BulletHandle(ActiveBulletHandle),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreateBulletNotifyLocationIdOneof {
    LocationEntityId(i64),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateBulletNotify {
    pub ParentHandle: Option<CreateBulletNotifyParentHandleOneof>,
    pub LocationId: Option<CreateBulletNotifyLocationIdOneof>,
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
    pub OwnerEntityId: i64,
    pub BulletId: i64,
    pub SkillId: i64,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub TargetId: i64,
    pub SpawnEntityId: i64,
    pub SpawnVelocityEntityId: i64,
    pub TarLocation: Option<Vector>,
    pub DtType: i32,
    pub Size: Option<Vector>,
    pub RandomPosOffset: Option<Vector>,
    pub RandomInitSpeedOffset: Option<Vector>,
    pub HitCase: Vec<u8>,
}

impl CreateBulletNotify {
    pub const MSG_ID: u16 = 15915;
}

impl PbMessage for CreateBulletNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.ParentHandle {
            match oneof {
                CreateBulletNotifyParentHandleOneof::BulletHandle(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
            }
        }
        if let Some(oneof) = &self.LocationId {
            match oneof {
                CreateBulletNotifyLocationIdOneof::LocationEntityId(v) => {
                    crate::wire::write_tag(out, 207, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 240, v);
        }
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 59, v);
        }
        if self.OwnerEntityId != 0 {
            crate::wire::write_tag(out, 125, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerEntityId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 165, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 31, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 170, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 101, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 184, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.SpawnEntityId != 0 {
            crate::wire::write_tag(out, 186, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnEntityId);
        }
        if self.SpawnVelocityEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SpawnVelocityEntityId);
        }
        if let Some(v) = &self.TarLocation {
            crate::wire::write_submessage(out, 214, v);
        }
        if self.DtType != 0 {
            crate::wire::write_tag(out, 155, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DtType);
        }
        if let Some(v) = &self.Size {
            crate::wire::write_submessage(out, 195, v);
        }
        if let Some(v) = &self.RandomPosOffset {
            crate::wire::write_submessage(out, 226, v);
        }
        if let Some(v) = &self.RandomInitSpeedOffset {
            crate::wire::write_submessage(out, 103, v);
        }
        if !self.HitCase.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.HitCase);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActiveBulletHandle as PbMessage>::decode(bytes)?;
                self.ParentHandle = Some(CreateBulletNotifyParentHandleOneof::BulletHandle(val));
            }
            207 => {
                self.LocationId = Some(CreateBulletNotifyLocationIdOneof::LocationEntityId(
                    r.read_varint_i64()?,
                ))
            }
            240 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            125 => self.OwnerEntityId = r.read_varint_i64()?,
            165 => self.BulletId = r.read_varint_i64()?,
            31 => self.SkillId = r.read_varint_i64()?,
            170 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            101 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            184 => self.TargetId = r.read_varint_i64()?,
            186 => self.SpawnEntityId = r.read_varint_i64()?,
            3 => self.SpawnVelocityEntityId = r.read_varint_i64()?,
            214 => {
                let bytes = r.read_len_delimited()?;
                self.TarLocation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            155 => self.DtType = r.read_varint_i32()?,
            195 => {
                let bytes = r.read_len_delimited()?;
                self.Size
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            226 => {
                let bytes = r.read_len_delimited()?;
                self.RandomPosOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            103 => {
                let bytes = r.read_len_delimited()?;
                self.RandomInitSpeedOffset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.HitCase = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DestroyBulletRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
}

impl DestroyBulletRequest {
    pub const MSG_ID: u16 = 27103;
}

impl PbMessage for DestroyBulletRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.Handle {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DestroyBulletResponsePush {
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
}

impl DestroyBulletResponsePush {
    pub const MSG_ID: u16 = 28662;
}

impl PbMessage for DestroyBulletResponsePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Handle {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DestroyBulletResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DestroyBulletResponse {
    pub const MSG_ID: u16 = 29062;
}

impl PbMessage for DestroyBulletResponse {
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
pub struct DestroyBulletNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
    pub IsCreateSubBullet: bool,
}

impl DestroyBulletNotify {
    pub const MSG_ID: u16 = 16659;
}

impl PbMessage for DestroyBulletNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IsCreateSubBullet {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.IsCreateSubBullet = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyBulletParams {
    pub CombatCommon: Option<CombatCommon>,
    pub Handle: Option<ActiveBulletHandle>,
    pub TargetId: i64,
}

impl PbMessage for ModifyBulletParams {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.TargetId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyBulletParamsRequest {
    pub ModifyBulletParams: Option<ModifyBulletParams>,
}

impl ModifyBulletParamsRequest {
    pub const MSG_ID: u16 = 29414;
}

impl PbMessage for ModifyBulletParamsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ModifyBulletParams {
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
                self.ModifyBulletParams
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyBulletParamsPush {
    pub ModifyBulletParams: Option<ModifyBulletParams>,
}

impl ModifyBulletParamsPush {
    pub const MSG_ID: u16 = 16856;
}

impl PbMessage for ModifyBulletParamsPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ModifyBulletParams {
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
                self.ModifyBulletParams
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModifyBulletParamsResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ModifyBulletParamsResponse {
    pub const MSG_ID: u16 = 17302;
}

impl PbMessage for ModifyBulletParamsResponse {
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
pub struct ModifyBulletParamsNotify {
    pub ModifyBulletParams: Option<ModifyBulletParams>,
}

impl ModifyBulletParamsNotify {
    pub const MSG_ID: u16 = 28690;
}

impl PbMessage for ModifyBulletParamsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ModifyBulletParams {
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
                self.ModifyBulletParams
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DamageContextSourceOneof {
    SourceType(DamageSourceType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DamageContextBulletOneof {
    BulletId(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DamageContextSkillOneof {
    SkillId(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DamageContextSkillMessageOneof {
    SkillMessageId(i64),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageContext {
    pub Source: Option<DamageContextSourceOneof>,
    pub Bullet: Option<DamageContextBulletOneof>,
    pub Skill: Option<DamageContextSkillOneof>,
    pub SkillMessage: Option<DamageContextSkillMessageOneof>,
    pub BulletTags: Vec<i32>,
}

impl PbMessage for DamageContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Source {
            match oneof {
                DamageContextSourceOneof::SourceType(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if let Some(oneof) = &self.Bullet {
            match oneof {
                DamageContextBulletOneof::BulletId(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.Skill {
            match oneof {
                DamageContextSkillOneof::SkillId(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.SkillMessage {
            match oneof {
                DamageContextSkillMessageOneof::SkillMessageId(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        for item in &self.BulletTags {
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
                self.Source = Some(DamageContextSourceOneof::SourceType(
                    <DamageSourceType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            2 => self.Bullet = Some(DamageContextBulletOneof::BulletId(r.read_varint_i64()?)),
            4 => self.Skill = Some(DamageContextSkillOneof::SkillId(r.read_varint_i64()?)),
            5 => {
                self.SkillMessage = Some(DamageContextSkillMessageOneof::SkillMessageId(
                    r.read_varint_i64()?,
                ))
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BulletTags, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageExecuteRequest {
    pub DamageId: i64,
    pub SkillLevel: i32,
    pub AttackerEntityId: i64,
    pub TargetEntityId: i64,
    pub IsAddEnergy: bool,
    pub IsCounterAttack: bool,
    pub ForceCritical: bool,
    pub IsBlocked: bool,
    pub PartIndex: i32,
    pub CounterSkillMessageId: i64,
    pub DamageContext: Option<DamageContext>,
    pub RandomSeed: i32,
    pub IsBreakWeakness: bool,
}

impl DamageExecuteRequest {
    pub const MSG_ID: u16 = 18654;
}

impl PbMessage for DamageExecuteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DamageId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DamageId);
        }
        if self.SkillLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillLevel);
        }
        if self.AttackerEntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AttackerEntityId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.IsAddEnergy {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsCounterAttack {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ForceCritical {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsBlocked {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
        }
        if self.CounterSkillMessageId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CounterSkillMessageId);
        }
        if let Some(v) = &self.DamageContext {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.RandomSeed != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RandomSeed);
        }
        if self.IsBreakWeakness {
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
            14 => self.DamageId = r.read_varint_i64()?,
            7 => self.SkillLevel = r.read_varint_i32()?,
            5 => self.AttackerEntityId = r.read_varint_i64()?,
            1 => self.TargetEntityId = r.read_varint_i64()?,
            11 => self.IsAddEnergy = r.read_bool()?,
            13 => self.IsCounterAttack = r.read_bool()?,
            6 => self.ForceCritical = r.read_bool()?,
            12 => self.IsBlocked = r.read_bool()?,
            10 => self.PartIndex = r.read_varint_i32()?,
            4 => self.CounterSkillMessageId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DamageContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.RandomSeed = r.read_varint_i32()?,
            3 => self.IsBreakWeakness = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageExecuteResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub AttackerEntityId: i64,
    pub TargetEntityId: i64,
    pub Damage: i32,
    pub PartIndex: i32,
    pub IsCrit: bool,
    pub KilledTarget: bool,
    pub ShieldCoverDamage: i32,
    pub ImmuneType: Option<EDamageImmune>,
    pub ElementType: i32,
    pub ChangeLife: i32,
    pub ChangeWeakness: i32,
}

impl DamageExecuteResponse {
    pub const MSG_ID: u16 = 16478;
}

impl PbMessage for DamageExecuteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AttackerEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AttackerEntityId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Damage);
        }
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
        }
        if self.IsCrit {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.KilledTarget {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ShieldCoverDamage != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShieldCoverDamage);
        }
        if let Some(v) = &self.ImmuneType {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ElementType != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementType);
        }
        if self.ChangeLife != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeLife);
        }
        if self.ChangeWeakness != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeWeakness);
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
            1 => self.AttackerEntityId = r.read_varint_i64()?,
            4 => self.TargetEntityId = r.read_varint_i64()?,
            9 => self.Damage = r.read_varint_i32()?,
            13 => self.PartIndex = r.read_varint_i32()?,
            3 => self.IsCrit = r.read_bool()?,
            2 => self.KilledTarget = r.read_bool()?,
            8 => self.ShieldCoverDamage = r.read_varint_i32()?,
            7 => {
                self.ImmuneType = Some(<EDamageImmune as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.ElementType = r.read_varint_i32()?,
            6 => self.ChangeLife = r.read_varint_i32()?,
            12 => self.ChangeWeakness = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageExecuteNotify {
    pub DamageId: i64,
    pub AttackerEntityId: i64,
    pub TargetEntityId: i64,
    pub Damage: i32,
    pub PartIndex: i32,
    pub IsCrit: bool,
    pub KilledTarget: bool,
    pub ShieldCoverDamage: i32,
    pub SkillLevel: i32,
    pub DamageContext: Option<DamageContext>,
    pub ImmuneType: Option<EDamageImmune>,
    pub ElementType: i32,
    pub ChangeLife: i32,
    pub ChangeWeakness: i32,
    pub Bop: i64,
}

impl DamageExecuteNotify {
    pub const MSG_ID: u16 = 28829;
}

impl PbMessage for DamageExecuteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DamageId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DamageId);
        }
        if self.AttackerEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AttackerEntityId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Damage);
        }
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
        }
        if self.IsCrit {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.KilledTarget {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ShieldCoverDamage != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShieldCoverDamage);
        }
        if self.SkillLevel != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillLevel);
        }
        if let Some(v) = &self.DamageContext {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.ImmuneType {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ElementType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementType);
        }
        if self.ChangeLife != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeLife);
        }
        if self.ChangeWeakness != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeWeakness);
        }
        if self.Bop != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Bop);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.DamageId = r.read_varint_i64()?,
            10 => self.AttackerEntityId = r.read_varint_i64()?,
            2 => self.TargetEntityId = r.read_varint_i64()?,
            7 => self.Damage = r.read_varint_i32()?,
            11 => self.PartIndex = r.read_varint_i32()?,
            3 => self.IsCrit = r.read_bool()?,
            14 => self.KilledTarget = r.read_bool()?,
            6 => self.ShieldCoverDamage = r.read_varint_i32()?,
            13 => self.SkillLevel = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DamageContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                self.ImmuneType = Some(<EDamageImmune as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.ElementType = r.read_varint_i32()?,
            1 => self.ChangeLife = r.read_varint_i32()?,
            9 => self.ChangeWeakness = r.read_varint_i32()?,
            8 => self.Bop = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillAddRequest {
    pub PassiveSkillId: i64,
    pub TargetEntityId: i64,
}

impl PassiveSkillAddRequest {
    pub const MSG_ID: u16 = 17475;
}

impl PbMessage for PassiveSkillAddRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassiveSkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassiveSkillId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.PassiveSkillId = r.read_varint_i64()?,
            5 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillAddPush {
    pub PassiveSkillId: i64,
    pub TargetEntityId: i64,
}

impl PassiveSkillAddPush {
    pub const MSG_ID: u16 = 19744;
}

impl PbMessage for PassiveSkillAddPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassiveSkillId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassiveSkillId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            13 => self.PassiveSkillId = r.read_varint_i64()?,
            8 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillAddResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PassiveSkillAddResponse {
    pub const MSG_ID: u16 = 26311;
}

impl PbMessage for PassiveSkillAddResponse {
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
pub struct PassiveSkillRemoveRequest {
    pub PassiveSkillId: i64,
    pub TargetEntityId: i64,
}

impl PassiveSkillRemoveRequest {
    pub const MSG_ID: u16 = 20977;
}

impl PbMessage for PassiveSkillRemoveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassiveSkillId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassiveSkillId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            5 => self.PassiveSkillId = r.read_varint_i64()?,
            8 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillRemovePush {
    pub PassiveSkillId: i64,
    pub TargetEntityId: i64,
}

impl PassiveSkillRemovePush {
    pub const MSG_ID: u16 = 21698;
}

impl PbMessage for PassiveSkillRemovePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PassiveSkillId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PassiveSkillId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            8 => self.PassiveSkillId = r.read_varint_i64()?,
            9 => self.TargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillRemoveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PassiveSkillRemoveResponse {
    pub const MSG_ID: u16 = 22073;
}

impl PbMessage for PassiveSkillRemoveResponse {
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
pub struct PassiveSkillAddNotify {
    pub EntityId: i64,
    pub PassiveSkillItemPbList: Vec<PassiveSkillItemPb>,
}

impl PassiveSkillAddNotify {
    pub const MSG_ID: u16 = 25025;
}

impl PbMessage for PassiveSkillAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.PassiveSkillItemPbList {
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
            8 => self.EntityId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PassiveSkillItemPbList
                    .push(<PassiveSkillItemPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillRemoveNotify {
    pub EntityId: i64,
    pub SkillIdList: Vec<i64>,
}

impl PassiveSkillRemoveNotify {
    pub const MSG_ID: u16 = 21248;
}

impl PbMessage for PassiveSkillRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.SkillIdList {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            2 => self.EntityId = r.read_varint_i64()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillIdList, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterViewDirectionRequest {}

impl EnterViewDirectionRequest {
    pub const MSG_ID: u16 = 25151;
}

impl PbMessage for EnterViewDirectionRequest {
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
pub struct EnterViewDirectionPush {}

impl EnterViewDirectionPush {
    pub const MSG_ID: u16 = 19916;
}

impl PbMessage for EnterViewDirectionPush {
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
pub struct EnterViewDirectionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnterViewDirectionResponse {
    pub const MSG_ID: u16 = 28371;
}

impl PbMessage for EnterViewDirectionResponse {
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
pub struct ExitViewDirectionRequest {}

impl ExitViewDirectionRequest {
    pub const MSG_ID: u16 = 16245;
}

impl PbMessage for ExitViewDirectionRequest {
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
pub struct ExitViewDirectionPush {}

impl ExitViewDirectionPush {
    pub const MSG_ID: u16 = 16646;
}

impl PbMessage for ExitViewDirectionPush {
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
pub struct ExitViewDirectionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ExitViewDirectionResponse {
    pub const MSG_ID: u16 = 28401;
}

impl PbMessage for ExitViewDirectionResponse {
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
pub struct TriggerExitSkillRequest {
    pub EnterEntityId: i64,
    pub LeaveEntityId: i64,
}

impl TriggerExitSkillRequest {
    pub const MSG_ID: u16 = 23172;
}

impl PbMessage for TriggerExitSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EnterEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EnterEntityId);
        }
        if self.LeaveEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LeaveEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.EnterEntityId = r.read_varint_i64()?,
            1 => self.LeaveEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TriggerExitSkillPush {
    pub EnterEntityId: i64,
    pub LeaveEntityId: i64,
}

impl TriggerExitSkillPush {
    pub const MSG_ID: u16 = 24805;
}

impl PbMessage for TriggerExitSkillPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EnterEntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EnterEntityId);
        }
        if self.LeaveEntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LeaveEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.EnterEntityId = r.read_varint_i64()?,
            7 => self.LeaveEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TriggerExitSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TriggerExitSkillResponse {
    pub const MSG_ID: u16 = 18531;
}

impl PbMessage for TriggerExitSkillResponse {
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
pub struct MonsterInfo {
    pub MonsterId: i32,
    pub Count: i32,
    pub GenId: i64,
}

impl PbMessage for MonsterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.GenId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GenId);
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
            2 => self.Count = r.read_varint_i32()?,
            3 => self.GenId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchBattleModeNotify {
    pub ServerControllerModules: Vec<BattleModule>,
    pub ClientControllerModules: Vec<BattleModule>,
}

impl SwitchBattleModeNotify {
    pub const MSG_ID: u16 = 24840;
}

impl PbMessage for SwitchBattleModeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ServerControllerModules {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        for item in &self.ClientControllerModules {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ServerControllerModules,
                |sr| {
                    Ok(<BattleModule as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                },
            )?,
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ClientControllerModules,
                |sr| {
                    Ok(<BattleModule as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                },
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityIsVisibleRequest {
    pub Id: i64,
    pub IsVisible: bool,
    pub CombatCommon: Option<CombatCommon>,
}

impl EntityIsVisibleRequest {
    pub const MSG_ID: u16 = 26500;
}

impl PbMessage for EntityIsVisibleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CombatCommon {
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
            2 => self.Id = r.read_varint_i64()?,
            9 => self.IsVisible = r.read_bool()?,
            4 => {
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
pub struct EntityIsVisiblePush {
    pub Id: i64,
    pub IsVisible: bool,
    pub CombatCommon: Option<CombatCommon>,
}

impl EntityIsVisiblePush {
    pub const MSG_ID: u16 = 29093;
}

impl PbMessage for EntityIsVisiblePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CombatCommon {
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
            12 => self.Id = r.read_varint_i64()?,
            4 => self.IsVisible = r.read_bool()?,
            13 => {
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
pub struct EntityIsVisibleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityIsVisibleResponse {
    pub const MSG_ID: u16 = 27947;
}

impl PbMessage for EntityIsVisibleResponse {
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
pub struct EntityIsVisibleNotify {
    pub Id: i64,
    pub IsVisible: bool,
    pub CombatCommon: Option<CombatCommon>,
}

impl EntityIsVisibleNotify {
    pub const MSG_ID: u16 = 27390;
}

impl PbMessage for EntityIsVisibleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CombatCommon {
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
            1 => self.Id = r.read_varint_i64()?,
            10 => self.IsVisible = r.read_bool()?,
            8 => {
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
pub struct MotorIsEnablePush {
    pub Id: i64,
    pub IsEnable: bool,
    pub CombatCommon: Option<CombatCommon>,
}

impl MotorIsEnablePush {
    pub const MSG_ID: u16 = 16848;
}

impl PbMessage for MotorIsEnablePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsEnable {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CombatCommon {
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
            8 => self.Id = r.read_varint_i64()?,
            14 => self.IsEnable = r.read_bool()?,
            9 => {
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
pub struct ActorVisibleRequest {
    pub Id: i64,
    pub IsActorVisible: bool,
}

impl ActorVisibleRequest {
    pub const MSG_ID: u16 = 17594;
}

impl PbMessage for ActorVisibleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsActorVisible {
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
            9 => self.Id = r.read_varint_i64()?,
            7 => self.IsActorVisible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActorVisiblePush {
    pub Id: i64,
    pub IsActorVisible: bool,
}

impl ActorVisiblePush {
    pub const MSG_ID: u16 = 16875;
}

impl PbMessage for ActorVisiblePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsActorVisible {
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
            2 => self.Id = r.read_varint_i64()?,
            3 => self.IsActorVisible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActorVisibleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ActorVisibleResponse {
    pub const MSG_ID: u16 = 23563;
}

impl PbMessage for ActorVisibleResponse {
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
pub struct ActorVisibleNotify {
    pub Id: i64,
    pub IsActorVisible: bool,
}

impl ActorVisibleNotify {
    pub const MSG_ID: u16 = 29002;
}

impl PbMessage for ActorVisibleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.IsActorVisible {
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
            15 => self.Id = r.read_varint_i64()?,
            11 => self.IsActorVisible = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchCharacterStateRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub OldState: Option<EntityState>,
    pub NewState: Option<EntityState>,
}

impl SwitchCharacterStateRequest {
    pub const MSG_ID: u16 = 26359;
}

impl PbMessage for SwitchCharacterStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.OldState {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.NewState {
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.Id = r.read_varint_i64()?,
            5 => {
                self.OldState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                self.NewState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchCharacterStatePush {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub OldState: Option<EntityState>,
    pub NewState: Option<EntityState>,
}

impl SwitchCharacterStatePush {
    pub const MSG_ID: u16 = 17009;
}

impl PbMessage for SwitchCharacterStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.OldState {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.NewState {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.Id = r.read_varint_i64()?,
            1 => {
                self.OldState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => {
                self.NewState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchCharacterStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SwitchCharacterStateResponse {
    pub const MSG_ID: u16 = 26238;
}

impl PbMessage for SwitchCharacterStateResponse {
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
pub struct SwitchCharacterStateNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub OldState: Option<EntityState>,
    pub NewState: Option<EntityState>,
}

impl SwitchCharacterStateNotify {
    pub const MSG_ID: u16 = 17711;
}

impl PbMessage for SwitchCharacterStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.OldState {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.NewState {
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.Id = r.read_varint_i64()?,
            14 => {
                self.OldState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                self.NewState = Some(<EntityState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleStateChangeRequest {
    pub EntityId: i64,
    pub InBattle: bool,
}

impl BattleStateChangeRequest {
    pub const MSG_ID: u16 = 19982;
}

impl PbMessage for BattleStateChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InBattle {
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
            14 => self.EntityId = r.read_varint_i64()?,
            8 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleStateChangePush {
    pub EntityId: i64,
    pub InBattle: bool,
}

impl BattleStateChangePush {
    pub const MSG_ID: u16 = 19385;
}

impl PbMessage for BattleStateChangePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InBattle {
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
            3 => self.EntityId = r.read_varint_i64()?,
            4 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BattleStateChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BattleStateChangeResponse {
    pub const MSG_ID: u16 = 22078;
}

impl PbMessage for BattleStateChangeResponse {
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
pub struct BattleStateChangeNotify {
    pub EntityId: i64,
    pub InBattle: bool,
}

impl BattleStateChangeNotify {
    pub const MSG_ID: u16 = 17671;
}

impl PbMessage for BattleStateChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InBattle {
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
            7 => self.EntityId = r.read_varint_i64()?,
            9 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ToughCalcExtraRatioChangeRequest {
    pub Id: i64,
    pub Duration: i32,
}

impl ToughCalcExtraRatioChangeRequest {
    pub const MSG_ID: u16 = 25451;
}

impl PbMessage for ToughCalcExtraRatioChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.Id = r.read_varint_i64()?,
            11 => self.Duration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ToughCalcExtraRatioChangePush {
    pub Id: i64,
    pub Duration: i32,
}

impl ToughCalcExtraRatioChangePush {
    pub const MSG_ID: u16 = 24735;
}

impl PbMessage for ToughCalcExtraRatioChangePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Id = r.read_varint_i64()?,
            9 => self.Duration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ToughCalcExtraRatioChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ToughCalcExtraRatioChangeResponse {
    pub const MSG_ID: u16 = 29306;
}

impl PbMessage for ToughCalcExtraRatioChangeResponse {
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
pub struct MonsterBoomRequest {
    pub Delay: i32,
}

impl MonsterBoomRequest {
    pub const MSG_ID: u16 = 17163;
}

impl PbMessage for MonsterBoomRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Delay != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.Delay = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterBoomPush {
    pub Delay: i32,
}

impl MonsterBoomPush {
    pub const MSG_ID: u16 = 25242;
}

impl PbMessage for MonsterBoomPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Delay != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.Delay = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterBoomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MonsterBoomResponse {
    pub const MSG_ID: u16 = 28184;
}

impl PbMessage for MonsterBoomResponse {
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
pub struct MontagePlayNotify {
    pub SkillId: i64,
    pub MontageIndex: i32,
}

impl MontagePlayNotify {
    pub const MSG_ID: u16 = 19227;
}

impl PbMessage for MontagePlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
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
            2 => self.SkillId = r.read_varint_i64()?,
            9 => self.MontageIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ANStartRequest {
    pub SkillId: i64,
    pub MontageIndex: i32,
    pub AnIndex: i32,
}

impl ANStartRequest {
    pub const MSG_ID: u16 = 25509;
}

impl PbMessage for ANStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.AnIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AnIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.SkillId = r.read_varint_i64()?,
            5 => self.MontageIndex = r.read_varint_i32()?,
            10 => self.AnIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ANStartPush {
    pub SkillId: i64,
    pub MontageIndex: i32,
    pub AnIndex: i32,
}

impl ANStartPush {
    pub const MSG_ID: u16 = 19776;
}

impl PbMessage for ANStartPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.AnIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AnIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SkillId = r.read_varint_i64()?,
            14 => self.MontageIndex = r.read_varint_i32()?,
            13 => self.AnIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ANStartResponse {
    pub SkillId: i64,
    pub MontageIndex: i32,
    pub AnIndex: i32,
    pub Error: Option<DErrorResult>,
}

impl ANStartResponse {
    pub const MSG_ID: u16 = 25684;
}

impl PbMessage for ANStartResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.AnIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AnIndex);
        }
        if let Some(v) = &self.Error {
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
            15 => self.SkillId = r.read_varint_i64()?,
            7 => self.MontageIndex = r.read_varint_i32()?,
            13 => self.AnIndex = r.read_varint_i32()?,
            4 => {
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
pub struct ANStartNotify {
    pub SkillId: i64,
    pub MontageIndex: i32,
    pub AnIndex: i32,
}

impl ANStartNotify {
    pub const MSG_ID: u16 = 16944;
}

impl PbMessage for ANStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.MontageIndex != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageIndex);
        }
        if self.AnIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AnIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.SkillId = r.read_varint_i64()?,
            15 => self.MontageIndex = r.read_varint_i32()?,
            2 => self.AnIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterBattleStateInfo {
    pub EntityId: i64,
    pub InBattle: bool,
}

impl PbMessage for CharacterBattleStateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InBattle {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PushContextIdNotify {
    pub Id: i64,
}

impl PushContextIdNotify {
    pub const MSG_ID: u16 = 17290;
}

impl PbMessage for PushContextIdNotify {
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
pub struct CharacterBattleStateChangeNotify {
    pub CharacterBattleStateInfo: Vec<CharacterBattleStateInfo>,
}

impl CharacterBattleStateChangeNotify {
    pub const MSG_ID: u16 = 16199;
}

impl PbMessage for CharacterBattleStateChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CharacterBattleStateInfo {
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
                self.CharacterBattleStateInfo
                    .push(<CharacterBattleStateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerBattleStateChangeNotify {
    pub PlayerId: i32,
    pub InBattle: bool,
}

impl PlayerBattleStateChangeNotify {
    pub const MSG_ID: u16 = 15313;
}

impl PbMessage for PlayerBattleStateChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.InBattle {
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
            3 => self.PlayerId = r.read_varint_i32()?,
            14 => self.InBattle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShieldUpdateInfo {
    pub Handle: i32,
    pub ConfigId: i32,
    pub ShieldValue: i32,
    pub UpdateType: Option<EShieldUpdateType>,
}

impl PbMessage for ShieldUpdateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.ShieldValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShieldValue);
        }
        if let Some(v) = &self.UpdateType {
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
            1 => self.Handle = r.read_varint_i32()?,
            2 => self.ConfigId = r.read_varint_i32()?,
            3 => self.ShieldValue = r.read_varint_i32()?,
            4 => {
                self.UpdateType = Some(
                    <EShieldUpdateType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct ShieldUpdateNotify {
    pub Shields: Vec<ShieldUpdateInfo>,
}

impl ShieldUpdateNotify {
    pub const MSG_ID: u16 = 27866;
}

impl PbMessage for ShieldUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Shields {
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
                self.Shields
                    .push(<ShieldUpdateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTagChangeRequest {
    pub TagId: i32,
    pub TagCount: i32,
}

impl RoleTagChangeRequest {
    pub const MSG_ID: u16 = 17502;
}

impl PbMessage for RoleTagChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TagId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        if self.TagCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.TagId = r.read_varint_i32()?,
            10 => self.TagCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTagChangePush {
    pub TagId: i32,
    pub TagCount: i32,
}

impl RoleTagChangePush {
    pub const MSG_ID: u16 = 24743;
}

impl PbMessage for RoleTagChangePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TagId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        if self.TagCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.TagId = r.read_varint_i32()?,
            3 => self.TagCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTagChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTagChangeResponse {
    pub const MSG_ID: u16 = 16302;
}

impl PbMessage for RoleTagChangeResponse {
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
pub struct GameplayCueRequest {
    pub GameplayCueId: i64,
}

impl GameplayCueRequest {
    pub const MSG_ID: u16 = 20852;
}

impl PbMessage for GameplayCueRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GameplayCueId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GameplayCueId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.GameplayCueId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayCuePush {
    pub GameplayCueId: i64,
}

impl GameplayCuePush {
    pub const MSG_ID: u16 = 21343;
}

impl PbMessage for GameplayCuePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GameplayCueId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GameplayCueId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.GameplayCueId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayCueResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GameplayCueResponse {
    pub const MSG_ID: u16 = 15031;
}

impl PbMessage for GameplayCueResponse {
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
pub struct GameplayCueNotify {
    pub GameplayCueId: i64,
}

impl GameplayCueNotify {
    pub const MSG_ID: u16 = 21307;
}

impl PbMessage for GameplayCueNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GameplayCueId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GameplayCueId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.GameplayCueId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecuteQteNotify {
    pub DownEntityId: i64,
    pub UpEntityId: i64,
    pub FnvHash: i32,
}

impl ExecuteQteNotify {
    pub const MSG_ID: u16 = 24112;
}

impl PbMessage for ExecuteQteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DownEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DownEntityId);
        }
        if self.UpEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpEntityId);
        }
        if self.FnvHash != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FnvHash);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.DownEntityId = r.read_varint_i64()?,
            1 => self.UpEntityId = r.read_varint_i64()?,
            2 => self.FnvHash = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecuteQteRequest {
    pub DownEntityId: i64,
    pub UpEntityId: i64,
    pub FnvHash: i32,
}

impl ExecuteQteRequest {
    pub const MSG_ID: u16 = 25206;
}

impl PbMessage for ExecuteQteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DownEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DownEntityId);
        }
        if self.UpEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpEntityId);
        }
        if self.FnvHash != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FnvHash);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DownEntityId = r.read_varint_i64()?,
            13 => self.UpEntityId = r.read_varint_i64()?,
            11 => self.FnvHash = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecuteQtePush {
    pub DownEntityId: i64,
    pub UpEntityId: i64,
    pub FnvHash: i32,
}

impl ExecuteQtePush {
    pub const MSG_ID: u16 = 16952;
}

impl PbMessage for ExecuteQtePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DownEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DownEntityId);
        }
        if self.UpEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpEntityId);
        }
        if self.FnvHash != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FnvHash);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.DownEntityId = r.read_varint_i64()?,
            14 => self.UpEntityId = r.read_varint_i64()?,
            13 => self.FnvHash = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExecuteQteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ExecuteQteResponse {
    pub const MSG_ID: u16 = 20100;
}

impl PbMessage for ExecuteQteResponse {
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
pub struct NewLinkStateNotify {
    pub LinkConfigId: i32,
    pub Current: Option<ENewLinkStage>,
    pub PlayerId: i32,
}

impl NewLinkStateNotify {
    pub const MSG_ID: u16 = 21875;
}

impl PbMessage for NewLinkStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LinkConfigId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LinkConfigId);
        }
        if let Some(v) = &self.Current {
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
            10 => self.LinkConfigId = r.read_varint_i32()?,
            1 => {
                self.Current = Some(<ENewLinkStage as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct NewLinkBurstPush {}

impl NewLinkBurstPush {
    pub const MSG_ID: u16 = 28450;
}

impl PbMessage for NewLinkBurstPush {
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
pub struct CharacterAttachRequest {
    pub CharacterAttachInfo: Option<CharacterAttachInfo>,
    pub TargetEntity: i64,
}

impl CharacterAttachRequest {
    pub const MSG_ID: u16 = 28910;
}

impl PbMessage for CharacterAttachRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CharacterAttachInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.TargetEntity != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntity);
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
                self.CharacterAttachInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.TargetEntity = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddCombineEntitiesRelationNotify {
    pub CharacterAttachInfo: Option<CharacterAttachInfo>,
    pub TargetEntity: i64,
}

impl AddCombineEntitiesRelationNotify {
    pub const MSG_ID: u16 = 24720;
}

impl PbMessage for AddCombineEntitiesRelationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CharacterAttachInfo {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.TargetEntity != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntity);
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
                self.CharacterAttachInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.TargetEntity = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveCombineRelationNotify {
    pub CombineEntity: i64,
    pub TargetEntity: i64,
}

impl RemoveCombineRelationNotify {
    pub const MSG_ID: u16 = 18044;
}

impl PbMessage for RemoveCombineRelationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CombineEntity != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CombineEntity);
        }
        if self.TargetEntity != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntity);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.CombineEntity = r.read_varint_i64()?,
            14 => self.TargetEntity = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterAttachResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CharacterAttachResponse {
    pub const MSG_ID: u16 = 16600;
}

impl PbMessage for CharacterAttachResponse {
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
pub struct CharacterDetachRequest {
    pub EntityA: i64,
    pub EntityB: i64,
}

impl CharacterDetachRequest {
    pub const MSG_ID: u16 = 20585;
}

impl PbMessage for CharacterDetachRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityA != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityA);
        }
        if self.EntityB != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityB);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.EntityA = r.read_varint_i64()?,
            3 => self.EntityB = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterDetachResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CharacterDetachResponse {
    pub const MSG_ID: u16 = 20084;
}

impl PbMessage for CharacterDetachResponse {
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
pub struct ModifyEntityCampNotify {
    pub TargetEntityId: i64,
    pub Camp: i32,
}

impl ModifyEntityCampNotify {
    pub const MSG_ID: u16 = 21724;
}

impl PbMessage for ModifyEntityCampNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.Camp != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Camp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.TargetEntityId = r.read_varint_i64()?,
            9 => self.Camp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MontagePlayPush {
    pub Name: Vec<u8>,
    pub Path: i32,
    pub SpeedRatio: f32,
    pub StartSection: Vec<u8>,
    pub StartTimeSeconds: f32,
}

impl MontagePlayPush {
    pub const MSG_ID: u16 = 24591;
}

impl PbMessage for MontagePlayPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Path != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Path);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
        if !self.StartSection.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StartSection);
        }
        if self.StartTimeSeconds != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.StartTimeSeconds).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Name = r.read_len_delimited()?.to_vec(),
            10 => self.Path = r.read_varint_i32()?,
            13 => self.SpeedRatio = r.read_f32()?,
            5 => self.StartSection = r.read_len_delimited()?.to_vec(),
            2 => self.StartTimeSeconds = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionTriggerPush {
    pub VisionId: i32,
}

impl VisionTriggerPush {
    pub const MSG_ID: u16 = 28446;
}

impl PbMessage for VisionTriggerPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VisionId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VisionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.VisionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionTriggerNotify {
    pub VisionId: i32,
}

impl VisionTriggerNotify {
    pub const MSG_ID: u16 = 24819;
}

impl PbMessage for VisionTriggerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VisionId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VisionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.VisionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransformBuffStackNotify {
    pub BuffHandle: i64,
    pub BuffId: i64,
    pub BuffStackModifier: i32,
}

impl TransformBuffStackNotify {
    pub const MSG_ID: u16 = 21805;
}

impl PbMessage for TransformBuffStackNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffHandle != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffHandle);
        }
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.BuffStackModifier != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffStackModifier);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.BuffHandle = r.read_varint_i64()?,
            12 => self.BuffId = r.read_varint_i64()?,
            6 => self.BuffStackModifier = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorSummonAndRidePush {
    pub EntityId: i64,
    pub VehicleIncId: i64,
    pub Transform: Option<Transform>,
}

impl MotorSummonAndRidePush {
    pub const MSG_ID: u16 = 18805;
}

impl PbMessage for MotorSummonAndRidePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.VehicleIncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleIncId);
        }
        if let Some(v) = &self.Transform {
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
            13 => self.EntityId = r.read_varint_i64()?,
            10 => self.VehicleIncId = r.read_varint_i64()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Transform
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorSummonAndRideNotify {
    pub PlayerId: i32,
    pub EntityId: i64,
    pub VehicleIncId: i64,
    pub Transform: Option<Transform>,
}

impl MotorSummonAndRideNotify {
    pub const MSG_ID: u16 = 21704;
}

impl PbMessage for MotorSummonAndRideNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.VehicleIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleIncId);
        }
        if let Some(v) = &self.Transform {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            11 => self.EntityId = r.read_varint_i64()?,
            14 => self.VehicleIncId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Transform
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GaSwitchCommonEnemyProCampRequest {}

impl GaSwitchCommonEnemyProCampRequest {
    pub const MSG_ID: u16 = 18387;
}

impl PbMessage for GaSwitchCommonEnemyProCampRequest {
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
pub struct GaSwitchCommonEnemyProCampResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GaSwitchCommonEnemyProCampResponse {
    pub const MSG_ID: u16 = 20488;
}

impl PbMessage for GaSwitchCommonEnemyProCampResponse {
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
pub struct BulletPatternPush {
    pub BulletPatternHandleId: i64,
    pub BulletPatternId: i32,
}

impl BulletPatternPush {
    pub const MSG_ID: u16 = 15914;
}

impl PbMessage for BulletPatternPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletPatternHandleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletPatternHandleId);
        }
        if self.BulletPatternId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletPatternId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.BulletPatternHandleId = r.read_varint_i64()?,
            3 => self.BulletPatternId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BulletPatternNotify {
    pub BulletPatternHandleId: i64,
    pub BulletPatternId: i32,
}

impl BulletPatternNotify {
    pub const MSG_ID: u16 = 24460;
}

impl PbMessage for BulletPatternNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletPatternHandleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletPatternHandleId);
        }
        if self.BulletPatternId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BulletPatternId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.BulletPatternHandleId = r.read_varint_i64()?,
            6 => self.BulletPatternId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuickHackOpenPush {
    pub DeviceId: i32,
    pub OwnerEntityId: i64,
}

impl QuickHackOpenPush {
    pub const MSG_ID: u16 = 25808;
}

impl PbMessage for QuickHackOpenPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DeviceId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DeviceId);
        }
        if self.OwnerEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.DeviceId = r.read_varint_i32()?,
            12 => self.OwnerEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct QuickHackRamVerifyPush {
    pub DeviceId: i32,
    pub QuickHackSkillIdList: Vec<i32>,
    pub OpenQuickHackPreMessageId: i64,
}

impl QuickHackRamVerifyPush {
    pub const MSG_ID: u16 = 22375;
}

impl PbMessage for QuickHackRamVerifyPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DeviceId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DeviceId);
        }
        for item in &self.QuickHackSkillIdList {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.OpenQuickHackPreMessageId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenQuickHackPreMessageId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.DeviceId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.QuickHackSkillIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            10 => self.OpenQuickHackPreMessageId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DodgeInfoPush {
    pub BulletOwnerId: i64,
    pub BulletId: i64,
}

impl DodgeInfoPush {
    pub const MSG_ID: u16 = 17078;
}

impl PbMessage for DodgeInfoPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BulletOwnerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletOwnerId);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.BulletOwnerId = r.read_varint_i64()?,
            12 => self.BulletId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeSummonEntityStateRequest {
    pub SummonEntityId: i64,
    pub TagId: i32,
}

impl ChangeSummonEntityStateRequest {
    pub const MSG_ID: u16 = 23569;
}

impl PbMessage for ChangeSummonEntityStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SummonEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SummonEntityId);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SummonEntityId = r.read_varint_i64()?,
            7 => self.TagId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeSummonEntityStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeSummonEntityStateResponse {
    pub const MSG_ID: u16 = 23698;
}

impl PbMessage for ChangeSummonEntityStateResponse {
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
pub struct RoleTrialSceneRollbackPush {}

impl RoleTrialSceneRollbackPush {
    pub const MSG_ID: u16 = 22790;
}

impl PbMessage for RoleTrialSceneRollbackPush {
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
pub struct DamageShowNotify {
    pub Id: i32,
    pub Damage: i32,
}

impl DamageShowNotify {
    pub const MSG_ID: u16 = 16964;
}

impl PbMessage for DamageShowNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, self.Damage as i32);
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
            2 => self.Damage = r.read_sint32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationAttr {
    pub AttrId: i32,
    pub Ratio: i32,
    pub BaseMaxValue: i32,
    pub MaxValue: i32,
    pub CurrentValue: i32,
}

impl PbMessage for FormationAttr {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrId);
        }
        if self.Ratio != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ratio);
        }
        if self.BaseMaxValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BaseMaxValue);
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
            3 => self.BaseMaxValue = r.read_varint_i32()?,
            4 => self.MaxValue = r.read_varint_i32()?,
            5 => self.CurrentValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationAttrNotify {
    pub Duration: i64,
    pub FormationAttrs: Vec<FormationAttr>,
}

impl FormationAttrNotify {
    pub const MSG_ID: u16 = 16879;
}

impl PbMessage for FormationAttrNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Duration);
        }
        for item in &self.FormationAttrs {
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
            5 => self.Duration = r.read_varint_i64()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.FormationAttrs
                    .push(<FormationAttr as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationAttrRequest {
    pub Duration: i64,
    pub FormationAttrs: Vec<FormationAttr>,
}

impl FormationAttrRequest {
    pub const MSG_ID: u16 = 19501;
}

impl PbMessage for FormationAttrRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Duration != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Duration);
        }
        for item in &self.FormationAttrs {
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
            11 => self.Duration = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FormationAttrs
                    .push(<FormationAttr as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationAttrResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FormationAttrResponse {
    pub const MSG_ID: u16 = 28542;
}

impl PbMessage for FormationAttrResponse {
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
pub struct RecoverPropFromServer {
    pub AttrId: i32,
    pub Ratio: i32,
    pub MaxValue: i32,
    pub ValueIncrement: i32,
}

impl PbMessage for RecoverPropFromServer {
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
        if self.ValueIncrement != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ValueIncrement);
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
            5 => self.ValueIncrement = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RecoverPropChangedNotify {
    pub Attributes: Vec<RecoverPropFromServer>,
    pub Duration: i64,
}

impl RecoverPropChangedNotify {
    pub const MSG_ID: u16 = 17789;
}

impl PbMessage for RecoverPropChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attributes {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Duration);
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
                self.Attributes
                    .push(<RecoverPropFromServer as PbMessage>::decode(bytes)?);
            }
            6 => self.Duration = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FragileChangeRequest {
    pub EntityId: i64,
    pub Flag: bool,
}

impl FragileChangeRequest {
    pub const MSG_ID: u16 = 22818;
}

impl PbMessage for FragileChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Flag {
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
            10 => self.EntityId = r.read_varint_i64()?,
            14 => self.Flag = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FragileChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FragileChangeResponse {
    pub const MSG_ID: u16 = 21843;
}

impl PbMessage for FragileChangeResponse {
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
pub struct DamageRecordEntity {
    pub EntityId: i64,
    pub BuffIds: Vec<i64>,
    pub Attr: Vec<GameplayAttributeData>,
    pub AttrSnapshot: Vec<GameplayAttributeData>,
}

impl PbMessage for DamageRecordEntity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.Attr {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.AttrSnapshot {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Attr
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AttrSnapshot
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageCalculationDetails {
    pub ABaseAttackValue: i64,
    pub VEffectiveDefense: f32,
    pub ADamageFactor: f32,
    pub ADamageBonusRate: f32,
    pub ACritChance: i64,
    pub AWeaknessMasteryCoefficient: f32,
    pub VMonsterTypeRate: f32,
    pub ARate: f32,
    pub VDefFactor: f32,
    pub VResistanceFactor: f32,
    pub VbDamageReduce: f32,
    pub VbElementReduce: f32,
    pub AEnergyChange: i64,
    pub WeaknessLvValue: f32,
    pub VWeaknessBuffStack: i32,
    pub HitDamageBonusRate: f32,
    pub WeakDamageBonusRate: f32,
    pub ExceptedDamageValue: f32,
    pub DarkTideAttackValue: f32,
    pub ADamageBonusRate2: f32,
}

impl PbMessage for DamageCalculationDetails {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ABaseAttackValue != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ABaseAttackValue);
        }
        if self.VEffectiveDefense != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VEffectiveDefense).to_bits().to_le_bytes());
        }
        if self.ADamageFactor != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ADamageFactor).to_bits().to_le_bytes());
        }
        if self.ADamageBonusRate != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ADamageBonusRate).to_bits().to_le_bytes());
        }
        if self.ACritChance != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ACritChance);
        }
        if self.AWeaknessMasteryCoefficient != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.AWeaknessMasteryCoefficient).to_bits().to_le_bytes());
        }
        if self.VMonsterTypeRate != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VMonsterTypeRate).to_bits().to_le_bytes());
        }
        if self.ARate != 0.0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ARate).to_bits().to_le_bytes());
        }
        if self.VDefFactor != 0.0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VDefFactor).to_bits().to_le_bytes());
        }
        if self.VResistanceFactor != 0.0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VResistanceFactor).to_bits().to_le_bytes());
        }
        if self.VbDamageReduce != 0.0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VbDamageReduce).to_bits().to_le_bytes());
        }
        if self.VbElementReduce != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.VbElementReduce).to_bits().to_le_bytes());
        }
        if self.AEnergyChange != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AEnergyChange);
        }
        if self.WeaknessLvValue != 0.0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.WeaknessLvValue).to_bits().to_le_bytes());
        }
        if self.VWeaknessBuffStack != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VWeaknessBuffStack);
        }
        if self.HitDamageBonusRate != 0.0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.HitDamageBonusRate).to_bits().to_le_bytes());
        }
        if self.WeakDamageBonusRate != 0.0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.WeakDamageBonusRate).to_bits().to_le_bytes());
        }
        if self.ExceptedDamageValue != 0.0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ExceptedDamageValue).to_bits().to_le_bytes());
        }
        if self.DarkTideAttackValue != 0.0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DarkTideAttackValue).to_bits().to_le_bytes());
        }
        if self.ADamageBonusRate2 != 0.0 {
            crate::wire::write_tag(out, 20, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ADamageBonusRate2).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ABaseAttackValue = r.read_varint_i64()?,
            2 => self.VEffectiveDefense = r.read_f32()?,
            3 => self.ADamageFactor = r.read_f32()?,
            4 => self.ADamageBonusRate = r.read_f32()?,
            5 => self.ACritChance = r.read_varint_i64()?,
            6 => self.AWeaknessMasteryCoefficient = r.read_f32()?,
            7 => self.VMonsterTypeRate = r.read_f32()?,
            8 => self.ARate = r.read_f32()?,
            9 => self.VDefFactor = r.read_f32()?,
            10 => self.VResistanceFactor = r.read_f32()?,
            11 => self.VbDamageReduce = r.read_f32()?,
            12 => self.VbElementReduce = r.read_f32()?,
            13 => self.AEnergyChange = r.read_varint_i64()?,
            14 => self.WeaknessLvValue = r.read_f32()?,
            15 => self.VWeaknessBuffStack = r.read_varint_i32()?,
            16 => self.HitDamageBonusRate = r.read_f32()?,
            17 => self.WeakDamageBonusRate = r.read_f32()?,
            18 => self.ExceptedDamageValue = r.read_f32()?,
            19 => self.DarkTideAttackValue = r.read_f32()?,
            20 => self.ADamageBonusRate2 = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamageRecordNotify {
    pub TimestampMs: i64,
    pub DamageConfId: i64,
    pub DamageValue: i32,
    pub SkillId: i64,
    pub SkillLevel: i32,
    pub BulletId: i64,
    pub DamageSourceType: Option<DamageSourceType>,
    pub IsCritical: bool,
    pub Attacker: Option<DamageRecordEntity>,
    pub Victim: Option<DamageRecordEntity>,
    pub DamageCalculationDetails: Option<DamageCalculationDetails>,
    pub IsWeakness: bool,
}

impl DamageRecordNotify {
    pub const MSG_ID: u16 = 15583;
}

impl PbMessage for DamageRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimestampMs != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TimestampMs);
        }
        if self.DamageConfId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DamageConfId);
        }
        if self.DamageValue != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DamageValue);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if self.SkillLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillLevel);
        }
        if self.BulletId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BulletId);
        }
        if let Some(v) = &self.DamageSourceType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsCritical {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Attacker {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.Victim {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.DamageCalculationDetails {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.IsWeakness {
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
            13 => self.TimestampMs = r.read_varint_i64()?,
            12 => self.DamageConfId = r.read_varint_i64()?,
            5 => self.DamageValue = r.read_varint_i32()?,
            3 => self.SkillId = r.read_varint_i64()?,
            6 => self.SkillLevel = r.read_varint_i32()?,
            8 => self.BulletId = r.read_varint_i64()?,
            4 => {
                self.DamageSourceType = Some(
                    <DamageSourceType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.IsCritical = r.read_bool()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Attacker
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Victim
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.DamageCalculationDetails
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.IsWeakness = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestDamageRecordEntity {
    pub EntityId: i64,
    pub ConfigId: i32,
    pub BuffIds: Vec<i64>,
    pub Attr: Vec<GameplayAttributeData>,
}

impl PbMessage for TestDamageRecordEntity {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.Attr {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.ConfigId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Attr
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TestDamageRecordNotify {
    pub TimestampMs: i64,
    pub Entities: Vec<TestDamageRecordEntity>,
}

impl TestDamageRecordNotify {
    pub const MSG_ID: u16 = 25211;
}

impl PbMessage for TestDamageRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimestampMs != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TimestampMs);
        }
        for item in &self.Entities {
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
            1 => self.TimestampMs = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Entities
                    .push(<TestDamageRecordEntity as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeStopPush {
    pub TimeDilation: f32,
}

impl TimeStopPush {
    pub const MSG_ID: u16 = 16707;
}

impl PbMessage for TimeStopPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeDilation != 0.0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeDilation).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.TimeDilation = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TsAnimNotifyStateAbsoluteTimeStopRequest {
    pub Duration: i32,
    pub Dilation: i32,
}

impl TsAnimNotifyStateAbsoluteTimeStopRequest {
    pub const MSG_ID: u16 = 28674;
}

impl PbMessage for TsAnimNotifyStateAbsoluteTimeStopRequest {
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
pub struct TsAnimNotifyStateAbsoluteTimeStopPush {
    pub Duration: i32,
    pub Dilation: i32,
}

impl TsAnimNotifyStateAbsoluteTimeStopPush {
    pub const MSG_ID: u16 = 28729;
}

impl PbMessage for TsAnimNotifyStateAbsoluteTimeStopPush {
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
pub struct TsAnimNotifyStateAbsoluteTimeStopResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl TsAnimNotifyStateAbsoluteTimeStopResponse {
    pub const MSG_ID: u16 = 29834;
}

impl PbMessage for TsAnimNotifyStateAbsoluteTimeStopResponse {
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
pub struct RTimeStopRequest {
    pub IsStopCharacter: bool,
    pub Duration: i32,
}

impl RTimeStopRequest {
    pub const MSG_ID: u16 = 16580;
}

impl PbMessage for RTimeStopRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsStopCharacter {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.IsStopCharacter = r.read_bool()?,
            12 => self.Duration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopPush {
    pub IsStopCharacter: bool,
    pub Duration: i32,
    pub Dilation: i32,
}

impl RTimeStopPush {
    pub const MSG_ID: u16 = 21080;
}

impl PbMessage for RTimeStopPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsStopCharacter {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.Dilation != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            5 => self.IsStopCharacter = r.read_bool()?,
            1 => self.Duration = r.read_varint_i32()?,
            12 => self.Dilation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RTimeStopResponse {
    pub const MSG_ID: u16 = 24814;
}

impl PbMessage for RTimeStopResponse {
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
pub struct EntityTimeDilationPush {
    pub EntityId: i64,
    pub TimeDilation: f32,
}

impl EntityTimeDilationPush {
    pub const MSG_ID: u16 = 25621;
}

impl PbMessage for EntityTimeDilationPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.TimeDilation != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeDilation).to_bits().to_le_bytes());
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
            7 => self.TimeDilation = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopInstRequest {
    pub Flag: bool,
    pub Duration: i32,
}

impl RTimeStopInstRequest {
    pub const MSG_ID: u16 = 18157;
}

impl PbMessage for RTimeStopInstRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Flag {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.Flag = r.read_bool()?,
            10 => self.Duration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopInstPush {
    pub Flag: bool,
    pub Duration: i32,
}

impl RTimeStopInstPush {
    pub const MSG_ID: u16 = 20960;
}

impl PbMessage for RTimeStopInstPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Flag {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.Flag = r.read_bool()?,
            3 => self.Duration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RTimeStopInstResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RTimeStopInstResponse {
    pub const MSG_ID: u16 = 15227;
}

impl PbMessage for RTimeStopInstResponse {
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
pub struct TimeCheckRequest {
    pub ClientTime: i64,
    pub TimeDilation: f32,
    pub FlowTimeDilation: f32,
}

impl TimeCheckRequest {
    pub const MSG_ID: u16 = 28799;
}

impl PbMessage for TimeCheckRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ClientTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientTime);
        }
        if self.TimeDilation != 0.0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeDilation).to_bits().to_le_bytes());
        }
        if self.FlowTimeDilation != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.FlowTimeDilation).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.ClientTime = r.read_varint_i64()?,
            15 => self.TimeDilation = r.read_f32()?,
            7 => self.FlowTimeDilation = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeCheckNotify {
    pub ClientTime: i64,
    pub ServerTime: i64,
    pub ServerCombatTime: i64,
    pub ServerStopTime: i64,
    pub ServerFlowTimestamp: i64,
}

impl TimeCheckNotify {
    pub const MSG_ID: u16 = 18938;
}

impl PbMessage for TimeCheckNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ClientTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientTime);
        }
        if self.ServerTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerTime);
        }
        if self.ServerCombatTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerCombatTime);
        }
        if self.ServerStopTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerStopTime);
        }
        if self.ServerFlowTimestamp != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerFlowTimestamp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.ClientTime = r.read_varint_i64()?,
            9 => self.ServerTime = r.read_varint_i64()?,
            10 => self.ServerCombatTime = r.read_varint_i64()?,
            4 => self.ServerStopTime = r.read_varint_i64()?,
            15 => self.ServerFlowTimestamp = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimeCheckResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ClientTime: i64,
    pub ServerTime: i64,
    pub ServerCombatTime: i64,
    pub ServerStopTime: i64,
    pub ServerFlowTimestamp: i64,
}

impl TimeCheckResponse {
    pub const MSG_ID: u16 = 28031;
}

impl PbMessage for TimeCheckResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ClientTime != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientTime);
        }
        if self.ServerTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerTime);
        }
        if self.ServerCombatTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerCombatTime);
        }
        if self.ServerStopTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerStopTime);
        }
        if self.ServerFlowTimestamp != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ServerFlowTimestamp);
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
            15 => self.ClientTime = r.read_varint_i64()?,
            1 => self.ServerTime = r.read_varint_i64()?,
            13 => self.ServerCombatTime = r.read_varint_i64()?,
            4 => self.ServerStopTime = r.read_varint_i64()?,
            7 => self.ServerFlowTimestamp = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationRoleInfo {
    pub roleId: i32,
    pub MaxHp: i32,
    pub CurHp: i32,
    pub Level: i32,
    pub RoleSkinId: i32,
    pub SkillBranchId: i32,
    pub WeaponId: i32,
    pub WeaponBreachLevel: i32,
    pub WeaponSkinId: i32,
    pub DressList: Vec<i32>,
}

impl PbMessage for FormationRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.roleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.roleId);
        }
        if self.MaxHp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxHp);
        }
        if self.CurHp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurHp);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranchId);
        }
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.WeaponBreachLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBreachLevel);
        }
        if self.WeaponSkinId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponSkinId);
        }
        for item in &self.DressList {
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
            1 => self.roleId = r.read_varint_i32()?,
            2 => self.MaxHp = r.read_varint_i32()?,
            3 => self.CurHp = r.read_varint_i32()?,
            4 => self.Level = r.read_varint_i32()?,
            5 => self.RoleSkinId = r.read_varint_i32()?,
            6 => self.SkillBranchId = r.read_varint_i32()?,
            7 => self.WeaponId = r.read_varint_i32()?,
            8 => self.WeaponBreachLevel = r.read_varint_i32()?,
            9 => self.WeaponSkinId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DressList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightFormationNotifyInfo {
    pub FormationId: i32,
    pub CurRole: i32,
    pub RoleInfos: Vec<FormationRoleInfo>,
    pub IsCurrent: bool,
    pub Name: Vec<u8>,
}

impl PbMessage for FightFormationNotifyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        if self.CurRole != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRole);
        }
        for item in &self.RoleInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsCurrent {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
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
            1 => self.FormationId = r.read_varint_i32()?,
            2 => self.CurRole = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfos
                    .push(<FormationRoleInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.IsCurrent = r.read_bool()?,
            5 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightFormation {
    pub FormationId: i32,
    pub CurRole: i32,
    pub RoleIds: Vec<i32>,
    pub IsCurrent: bool,
    pub Name: Vec<u8>,
}

impl PbMessage for FightFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        if self.CurRole != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRole);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsCurrent {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
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
            1 => self.FormationId = r.read_varint_i32()?,
            2 => self.CurRole = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.IsCurrent = r.read_bool()?,
            5 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwitchRoleRequestTransformOneof {
    Transform(Transform),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchRoleRequest {
    pub transform: Option<SwitchRoleRequestTransformOneof>,
    pub RoleId: i32,
    pub SwitchType: Option<SwitchRoleType>,
    pub OnStageWithoutControl: bool,
}

impl SwitchRoleRequest {
    pub const MSG_ID: u16 = 27457;
}

impl PbMessage for SwitchRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.transform {
            match oneof {
                SwitchRoleRequestTransformOneof::Transform(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
            }
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.SwitchType {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.OnStageWithoutControl {
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
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <Transform as PbMessage>::decode(bytes)?;
                self.transform = Some(SwitchRoleRequestTransformOneof::Transform(val));
            }
            3 => self.RoleId = r.read_varint_i32()?,
            8 => {
                self.SwitchType = Some(
                    <SwitchRoleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.OnStageWithoutControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoleId: i32,
}

impl SwitchRoleResponse {
    pub const MSG_ID: u16 = 16820;
}

impl PbMessage for SwitchRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwitchRoleNotifyTransformOneof {
    Transform(Transform),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchRoleNotify {
    pub transform: Option<SwitchRoleNotifyTransformOneof>,
    pub PlayerId: i32,
    pub UpEntityId: i64,
    pub DownEntityId: i64,
    pub OnStageWithoutControl: bool,
}

impl SwitchRoleNotify {
    pub const MSG_ID: u16 = 26835;
}

impl PbMessage for SwitchRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.transform {
            match oneof {
                SwitchRoleNotifyTransformOneof::Transform(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
            }
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.UpEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UpEntityId);
        }
        if self.DownEntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DownEntityId);
        }
        if self.OnStageWithoutControl {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                let val = <Transform as PbMessage>::decode(bytes)?;
                self.transform = Some(SwitchRoleNotifyTransformOneof::Transform(val));
            }
            6 => self.PlayerId = r.read_varint_i32()?,
            12 => self.UpEntityId = r.read_varint_i64()?,
            11 => self.DownEntityId = r.read_varint_i64()?,
            14 => self.OnStageWithoutControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleGoDownPush {}

impl RoleGoDownPush {
    pub const MSG_ID: u16 = 16452;
}

impl PbMessage for RoleGoDownPush {
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
pub struct UpdateFormationRequest {
    pub Formations: Vec<FightFormation>,
}

impl UpdateFormationRequest {
    pub const MSG_ID: u16 = 23144;
}

impl PbMessage for UpdateFormationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Formations {
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
                self.Formations
                    .push(<FightFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateFormationResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Formation: Option<FightFormation>,
}

impl UpdateFormationResponse {
    pub const MSG_ID: u16 = 24546;
}

impl PbMessage for UpdateFormationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Formation {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenameFormationRequest {
    pub FormationId: i32,
    pub Name: Vec<u8>,
}

impl RenameFormationRequest {
    pub const MSG_ID: u16 = 26314;
}

impl PbMessage for RenameFormationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
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
            8 => self.FormationId = r.read_varint_i32()?,
            1 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenameFormationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RenameFormationResponse {
    pub const MSG_ID: u16 = 18743;
}

impl PbMessage for RenameFormationResponse {
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
pub struct PlayerFightFormations {
    pub PlayerId: i32,
    pub Formations: Vec<FightFormationNotifyInfo>,
}

impl PbMessage for PlayerFightFormations {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.Formations {
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
                self.Formations
                    .push(<FightFormationNotifyInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateFormationNotify {
    pub PlayersFormations: Vec<PlayerFightFormations>,
}

impl UpdateFormationNotify {
    pub const MSG_ID: u16 = 20593;
}

impl PbMessage for UpdateFormationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayersFormations {
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
                self.PlayersFormations
                    .push(<PlayerFightFormations as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetFormationDataRequest {}

impl GetFormationDataRequest {
    pub const MSG_ID: u16 = 20343;
}

impl PbMessage for GetFormationDataRequest {
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
pub struct GetFormationDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Formations: Vec<FightFormation>,
}

impl GetFormationDataResponse {
    pub const MSG_ID: u16 = 18668;
}

impl PbMessage for GetFormationDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Formations {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Formations
                    .push(<FightFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientCurrentRoleReportRequest {
    pub PlayerId: i32,
    pub CurrentRoleId: i32,
    pub CurrentEntityId: i64,
}

impl ClientCurrentRoleReportRequest {
    pub const MSG_ID: u16 = 19037;
}

impl PbMessage for ClientCurrentRoleReportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.CurrentRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentRoleId);
        }
        if self.CurrentEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurrentEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.PlayerId = r.read_varint_i32()?,
            5 => self.CurrentRoleId = r.read_varint_i32()?,
            13 => self.CurrentEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientCurrentRoleReportPush {
    pub PlayerId: i32,
    pub CurrentRoleId: i32,
    pub CurrentEntityId: i64,
}

impl ClientCurrentRoleReportPush {
    pub const MSG_ID: u16 = 29219;
}

impl PbMessage for ClientCurrentRoleReportPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.CurrentRoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentRoleId);
        }
        if self.CurrentEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurrentEntityId);
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
            4 => self.CurrentRoleId = r.read_varint_i32()?,
            10 => self.CurrentEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientCurrentRoleReportResponse {
    pub PlayerId: i32,
    pub CurrentEntityId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl ClientCurrentRoleReportResponse {
    pub const MSG_ID: u16 = 15392;
}

impl PbMessage for ClientCurrentRoleReportResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.CurrentEntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurrentEntityId);
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
            11 => self.PlayerId = r.read_varint_i32()?,
            9 => self.CurrentEntityId = r.read_varint_i64()?,
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
pub struct CombatValNotify {
    pub Type: Option<CombatValType>,
    pub Value: i32,
}

impl CombatValNotify {
    pub const MSG_ID: u16 = 18757;
}

impl PbMessage for CombatValNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Value);
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
                self.Type = Some(<CombatValType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.Value = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatValDecayBeginNotify {
    pub Type: Option<CombatValType>,
    pub Value: i32,
    pub DecayRemain: i32,
}

impl CombatValDecayBeginNotify {
    pub const MSG_ID: u16 = 27363;
}

impl PbMessage for CombatValDecayBeginNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Value);
        }
        if self.DecayRemain != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DecayRemain);
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
                self.Type = Some(<CombatValType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.Value = r.read_varint_i32()?,
            1 => self.DecayRemain = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SquadPropChangedNotify {
    pub Attributes: Vec<GameplayAttributeData>,
}

impl SquadPropChangedNotify {
    pub const MSG_ID: u16 = 20340;
}

impl PbMessage for SquadPropChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Attributes {
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
                self.Attributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScreenShotTakenPush {}

impl ScreenShotTakenPush {
    pub const MSG_ID: u16 = 27304;
}

impl PbMessage for ScreenShotTakenPush {
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
pub struct ControlTemporaryTeleportParam {
    pub TemporaryTeleportIds: Vec<i64>,
}

impl PbMessage for ControlTemporaryTeleportParam {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TemporaryTeleportIds {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TemporaryTeleportIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlParamParamOneof {
    TemporaryTeleportParam(ControlTemporaryTeleportParam),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ControlParam {
    pub Param: Option<ControlParamParamOneof>,
    pub ControlType: Option<ControlType>,
}

impl PbMessage for ControlParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Param {
            match oneof {
                ControlParamParamOneof::TemporaryTeleportParam(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
            }
        }
        if let Some(v) = &self.ControlType {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <ControlTemporaryTeleportParam as PbMessage>::decode(bytes)?;
                self.Param = Some(ControlParamParamOneof::TemporaryTeleportParam(val));
            }
            1 => {
                self.ControlType = Some(<ControlType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ControlInfoNotify {
    pub ForbidList: Vec<ControlParam>,
}

impl ControlInfoNotify {
    pub const MSG_ID: u16 = 22309;
}

impl PbMessage for ControlInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ForbidList {
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
                self.ForbidList
                    .push(<ControlParam as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainGetPlayerIdRequest {}

impl DirectTrainGetPlayerIdRequest {
    pub const MSG_ID: u16 = 29272;
}

impl PbMessage for DirectTrainGetPlayerIdRequest {
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

#[derive(Debug, Clone, PartialEq)]
pub enum DirectTrainGetPlayerIdResponseMU1Oneof {
    Activities(ActivityData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainGetPlayerIdResponse {
    pub MU1: Option<DirectTrainGetPlayerIdResponseMU1Oneof>,
    pub Activitys: Vec<ActivityData>,
}

impl DirectTrainGetPlayerIdResponse {
    pub const MSG_ID: u16 = 19087;
}

impl PbMessage for DirectTrainGetPlayerIdResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.MU1 {
            match oneof {
                DirectTrainGetPlayerIdResponseMU1Oneof::Activities(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
            }
        }
        for item in &self.Activitys {
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
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivityData as PbMessage>::decode(bytes)?;
                self.MU1 = Some(DirectTrainGetPlayerIdResponseMU1Oneof::Activities(val));
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Activitys
                    .push(<ActivityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnergyInfo {
    pub EnergyCount: i32,
    pub LastRenewEnergyTime: i32,
    pub EnergyType: i32,
}

impl PbMessage for EnergyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EnergyCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnergyCount);
        }
        if self.LastRenewEnergyTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRenewEnergyTime);
        }
        if self.EnergyType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnergyType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EnergyCount = r.read_varint_i32()?,
            2 => self.LastRenewEnergyTime = r.read_varint_i32()?,
            3 => self.EnergyType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnergyUpdateNotify {
    pub UpdateInfo: Vec<EnergyInfo>,
}

impl EnergyUpdateNotify {
    pub const MSG_ID: u16 = 24514;
}

impl PbMessage for EnergyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UpdateInfo {
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
                self.UpdateInfo
                    .push(<EnergyInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnergySyncRequest {
    pub EnergyTypes: Vec<i32>,
}

impl EnergySyncRequest {
    pub const MSG_ID: u16 = 17150;
}

impl PbMessage for EnergySyncRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EnergyTypes {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EnergyTypes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnergySyncResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SyncInfo: Vec<EnergyInfo>,
}

impl EnergySyncResponse {
    pub const MSG_ID: u16 = 16637;
}

impl PbMessage for EnergySyncResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SyncInfo {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SyncInfo
                    .push(<EnergyInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreProgressRequest {
    pub AreaIds: Vec<i32>,
}

impl ExploreProgressRequest {
    pub const MSG_ID: u16 = 19630;
}

impl PbMessage for ExploreProgressRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaIds {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AreaIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AreaExploreInfo {
    pub AreaId: i32,
    pub ExploreProgress: Vec<OneExploreItem>,
    pub ExplorePercent: i32,
}

impl PbMessage for AreaExploreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        for item in &self.ExploreProgress {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.ExplorePercent != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExplorePercent);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ExploreProgress
                    .push(<OneExploreItem as PbMessage>::decode(bytes)?);
            }
            3 => self.ExplorePercent = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneExploreItem {
    pub ExploreProgressId: i32,
    pub ExplorePercent: i32,
    pub CurCount: i32,
    pub TotalCount: i32,
    pub IsLocked: bool,
}

impl PbMessage for OneExploreItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExploreProgressId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreProgressId);
        }
        if self.ExplorePercent != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExplorePercent);
        }
        if self.CurCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCount);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
        }
        if self.IsLocked {
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
            1 => self.ExploreProgressId = r.read_varint_i32()?,
            2 => self.ExplorePercent = r.read_varint_i32()?,
            3 => self.CurCount = r.read_varint_i32()?,
            4 => self.TotalCount = r.read_varint_i32()?,
            5 => self.IsLocked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreProgressResponse {
    pub AreaProgress: Vec<AreaExploreInfo>,
}

impl ExploreProgressResponse {
    pub const MSG_ID: u16 = 29853;
}

impl PbMessage for ExploreProgressResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaProgress {
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
                self.AreaProgress
                    .push(<AreaExploreInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiExploreScoreRewardRequest {
    pub nBs: BTreeMap<i32, i32>,
}

impl MultiExploreScoreRewardRequest {
    pub const MSG_ID: u16 = 20889;
}

impl PbMessage for MultiExploreScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.nBs.is_empty() {
            crate::wire::write_map(out, 13, &self.nBs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => crate::wire::decode_map_entry(r, &mut self.nBs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiExploreScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MultiExploreScoreRewardResponse {
    pub const MSG_ID: u16 = 27486;
}

impl PbMessage for MultiExploreScoreRewardResponse {
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
pub struct CountryExploreScoreInfoRequest {
    pub CountryId: i32,
}

impl CountryExploreScoreInfoRequest {
    pub const MSG_ID: u16 = 16184;
}

impl PbMessage for CountryExploreScoreInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CountryId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CountryId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.CountryId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CountryExploreScoreInfoResponse {
    pub ExploreScore: i32,
    pub CountryExploreScoreReceived: Vec<CountryExploreScoreReceived>,
}

impl CountryExploreScoreInfoResponse {
    pub const MSG_ID: u16 = 24983;
}

impl PbMessage for CountryExploreScoreInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExploreScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreScore);
        }
        for item in &self.CountryExploreScoreReceived {
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
            8 => self.ExploreScore = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.CountryExploreScoreReceived
                    .push(<CountryExploreScoreReceived as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CountryExploreScoreReceived {
    pub AreaId: i32,
    pub ExploreProgress: Vec<i32>,
}

impl PbMessage for CountryExploreScoreReceived {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        for item in &self.ExploreProgress {
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
            1 => self.AreaId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ExploreProgress,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CountryExploreLevel {
    pub CountryId: i32,
    pub ExploreLevel: i32,
}

impl PbMessage for CountryExploreLevel {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CountryId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CountryId);
        }
        if self.ExploreLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CountryId = r.read_varint_i32()?,
            2 => self.ExploreLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreLevelNotify {
    pub CountryExploreLevel: Vec<CountryExploreLevel>,
}

impl ExploreLevelNotify {
    pub const MSG_ID: u16 = 24649;
}

impl PbMessage for ExploreLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CountryExploreLevel {
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
                self.CountryExploreLevel
                    .push(<CountryExploreLevel as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveAreaStageRewardAsyncRequest {
    pub AreaStageRewardDataList: Vec<i32>,
}

impl ReceiveAreaStageRewardAsyncRequest {
    pub const MSG_ID: u16 = 29451;
}

impl PbMessage for ReceiveAreaStageRewardAsyncRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaStageRewardDataList {
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
                &mut self.AreaStageRewardDataList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReceiveAreaStageRewardAsyncResponse {
    pub AreaStageRewardDataList: Vec<i32>,
}

impl ReceiveAreaStageRewardAsyncResponse {
    pub const MSG_ID: u16 = 28094;
}

impl PbMessage for ReceiveAreaStageRewardAsyncResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaStageRewardDataList {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AreaStageRewardDataList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreProgressRewardIdsNotify {
    pub AreaStageRewardDataList: Vec<i32>,
}

impl ExploreProgressRewardIdsNotify {
    pub const MSG_ID: u16 = 25441;
}

impl PbMessage for ExploreProgressRewardIdsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AreaStageRewardDataList {
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
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AreaStageRewardDataList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreToolAllNotify {
    pub SkillList: Vec<i32>,
    pub ExploreSkill: i32,
    pub NewUnlock: Vec<i32>,
}

impl ExploreToolAllNotify {
    pub const MSG_ID: u16 = 23555;
}

impl PbMessage for ExploreToolAllNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkillList {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ExploreSkill != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreSkill);
        }
        for item in &self.NewUnlock {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => self.ExploreSkill = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NewUnlock, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreSkillRouletteUpdateNotify {
    pub RouletteInfo: Vec<ExploreSkillRoulette>,
}

impl ExploreSkillRouletteUpdateNotify {
    pub const MSG_ID: u16 = 25728;
}

impl PbMessage for ExploreSkillRouletteUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RouletteInfo {
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
                self.RouletteInfo
                    .push(<ExploreSkillRoulette as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionExploreSkillSetRequest {
    pub SkillId: i32,
    pub IsAutoChange: bool,
    pub RouletteType: Option<RouletteType>,
}

impl VisionExploreSkillSetRequest {
    pub const MSG_ID: u16 = 16974;
}

impl PbMessage for VisionExploreSkillSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.IsAutoChange {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.RouletteType {
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
            12 => self.SkillId = r.read_varint_i32()?,
            4 => self.IsAutoChange = r.read_bool()?,
            9 => {
                self.RouletteType =
                    Some(<RouletteType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionExploreSkillSetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkillId: i32,
}

impl VisionExploreSkillSetResponse {
    pub const MSG_ID: u16 = 18378;
}

impl PbMessage for VisionExploreSkillSetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionExploreSkillNotify {
    pub ExploreSkill: i32,
}

impl VisionExploreSkillNotify {
    pub const MSG_ID: u16 = 17772;
}

impl PbMessage for VisionExploreSkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExploreSkill != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreSkill);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.ExploreSkill = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreSkillRoulette {
    pub SkillIds: Vec<i32>,
    pub ExtraItemId: i32,
    pub ExploreSkill: i32,
}

impl PbMessage for ExploreSkillRoulette {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkillIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ExtraItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraItemId);
        }
        if self.ExploreSkill != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreSkill);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.ExtraItemId = r.read_varint_i32()?,
            3 => self.ExploreSkill = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreSkillRouletteSetRequest {
    pub SkillRoulette: Option<ExploreSkillRoulette>,
    pub RouletteType: Option<RouletteType>,
    pub SkillRoulettes: Vec<ExploreSkillRoulette>,
}

impl ExploreSkillRouletteSetRequest {
    pub const MSG_ID: u16 = 15609;
}

impl PbMessage for ExploreSkillRouletteSetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SkillRoulette {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.RouletteType {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SkillRoulettes {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkillRoulette
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                self.RouletteType =
                    Some(<RouletteType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SkillRoulettes
                    .push(<ExploreSkillRoulette as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreSkillRouletteSetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkillRoulette: Option<ExploreSkillRoulette>,
    pub RouletteType: Option<RouletteType>,
    pub SkillRoulettes: Vec<ExploreSkillRoulette>,
}

impl ExploreSkillRouletteSetResponse {
    pub const MSG_ID: u16 = 27182;
}

impl PbMessage for ExploreSkillRouletteSetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.SkillRoulette {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.RouletteType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SkillRoulettes {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SkillRoulette
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            1 => {
                self.RouletteType =
                    Some(<RouletteType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SkillRoulettes
                    .push(<ExploreSkillRoulette as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlockIllustratedPhantom {
    pub MonsterId: i32,
    pub SkinIds: Vec<i32>,
    pub EqupiedSkin: i32,
    pub IsSpecial: bool,
}

impl PbMessage for UnlockIllustratedPhantom {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        for item in &self.SkinIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EqupiedSkin != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EqupiedSkin);
        }
        if self.IsSpecial {
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
            1 => self.MonsterId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.EqupiedSkin = r.read_varint_i32()?,
            4 => self.IsSpecial = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionUnlockNotify {
    pub UnlockIllustratedPhantoms: Vec<UnlockIllustratedPhantom>,
    pub EquipedMonsterIds: Vec<i32>,
}

impl PhantomInteractionUnlockNotify {
    pub const MSG_ID: u16 = 23244;
}

impl PbMessage for PhantomInteractionUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockIllustratedPhantoms {
            crate::wire::write_submessage(out, 15, item);
        }
        for item in &self.EquipedMonsterIds {
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
            15 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockIllustratedPhantoms
                    .push(<UnlockIllustratedPhantom as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EquipedMonsterIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionInfoUpdateNotify {
    pub UnlockIllustratedPhantom: Option<UnlockIllustratedPhantom>,
}

impl PhantomInteractionInfoUpdateNotify {
    pub const MSG_ID: u16 = 29193;
}

impl PbMessage for PhantomInteractionInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UnlockIllustratedPhantom {
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
                self.UnlockIllustratedPhantom
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionEquipRequest {
    pub EquipedMonsterIds: Vec<i32>,
}

impl PhantomInteractionEquipRequest {
    pub const MSG_ID: u16 = 21354;
}

impl PbMessage for PhantomInteractionEquipRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipedMonsterIds {
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
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EquipedMonsterIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionEquipResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomInteractionEquipResponse {
    pub const MSG_ID: u16 = 21997;
}

impl PbMessage for PhantomInteractionEquipResponse {
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
pub struct SkinChangeInfo {
    pub MonsterId: i32,
    pub SkinId: i32,
}

impl PbMessage for SkinChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.MonsterId = r.read_varint_i32()?,
            2 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionSkinChangeRequest {
    pub SkinChangeInfos: Vec<SkinChangeInfo>,
}

impl PhantomInteractionSkinChangeRequest {
    pub const MSG_ID: u16 = 21448;
}

impl PbMessage for PhantomInteractionSkinChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinChangeInfos {
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
                self.SkinChangeInfos
                    .push(<SkinChangeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomInteractionSkinChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomInteractionSkinChangeResponse {
    pub const MSG_ID: u16 = 18194;
}

impl PbMessage for PhantomInteractionSkinChangeResponse {
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
pub struct FavorQuest {
    pub Chapter: i32,
    pub Status: Option<FavorQuestStatus>,
}

impl PbMessage for FavorQuest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Chapter != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Chapter);
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
            1 => self.Chapter = r.read_varint_i32()?,
            2 => {
                self.Status = Some(
                    <FavorQuestStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FavorItem {
    pub Id: i32,
    pub Status: Option<FavorItemStatus>,
}

impl PbMessage for FavorItem {
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
                    <FavorItemStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoleFavor {
    pub RoleId: i32,
    pub Level: i32,
    pub Exp: i32,
    pub WordIds: Vec<FavorItem>,
    pub StoryIds: Vec<FavorItem>,
    pub GoodsIds: Vec<FavorItem>,
    pub FavorQuest: Option<FavorQuest>,
}

impl PbMessage for RoleFavor {
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
        for item in &self.WordIds {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.StoryIds {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.GoodsIds {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.FavorQuest {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.Exp = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.WordIds.push(<FavorItem as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.StoryIds.push(<FavorItem as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.GoodsIds.push(<FavorItem as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.FavorQuest
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorListRequest {}

impl RoleFavorListRequest {
    pub const MSG_ID: u16 = 22639;
}

impl PbMessage for RoleFavorListRequest {
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
pub struct RoleFavorListResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FavorList: Vec<RoleFavor>,
}

impl RoleFavorListResponse {
    pub const MSG_ID: u16 = 28736;
}

impl PbMessage for RoleFavorListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FavorList {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FavorList
                    .push(<RoleFavor as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorListNotify {
    pub FavorList: Vec<RoleFavor>,
    pub RoleConditionInfoMap: BTreeMap<i32, ConditionInfo>,
}

impl RoleFavorListNotify {
    pub const MSG_ID: u16 = 29381;
}

impl PbMessage for RoleFavorListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FavorList {
            crate::wire::write_submessage(out, 10, item);
        }
        if !self.RoleConditionInfoMap.is_empty() {
            crate::wire::write_map(out, 2, &self.RoleConditionInfoMap);
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
                self.FavorList
                    .push(<RoleFavor as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_map_entry(r, &mut self.RoleConditionInfoMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemFinishList {
    pub ConditionIdList: Vec<i32>,
}

impl PbMessage for ItemFinishList {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionIdList {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ConditionIdList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionItem {
    pub ItemFinishMap: BTreeMap<i32, ItemFinishList>,
}

impl PbMessage for ConditionItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemFinishMap.is_empty() {
            crate::wire::write_map(out, 1, &self.ItemFinishMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.ItemFinishMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionInfo {
    pub FinishConditionMap: BTreeMap<i32, ConditionItem>,
}

impl PbMessage for ConditionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FinishConditionMap.is_empty() {
            crate::wire::write_map(out, 2, &self.FinishConditionMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_map_entry(r, &mut self.FinishConditionMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotion {
    pub RoleId: i32,
    pub MotionIds: Vec<FavorItem>,
}

impl PbMessage for RoleMotion {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.MotionIds {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MotionIds
                    .push(<FavorItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionListNotify {
    pub MotionList: Vec<RoleMotion>,
    pub RoleConditionInfoMap: BTreeMap<i32, ConditionInfo>,
}

impl RoleMotionListNotify {
    pub const MSG_ID: u16 = 16046;
}

impl PbMessage for RoleMotionListNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotionList {
            crate::wire::write_submessage(out, 2, item);
        }
        if !self.RoleConditionInfoMap.is_empty() {
            crate::wire::write_map(out, 7, &self.RoleConditionInfoMap);
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
                self.MotionList
                    .push(<RoleMotion as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_map_entry(r, &mut self.RoleConditionInfoMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingDataRequest {}

impl FishingDataRequest {
    pub const MSG_ID: u16 = 18438;
}

impl PbMessage for FishingDataRequest {
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
pub struct FishingDataResponse {
    pub FishingData: Option<FishingData>,
}

impl FishingDataResponse {
    pub const MSG_ID: u16 = 22673;
}

impl PbMessage for FishingDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishingData {
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
                self.FishingData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingData {
    pub Entrusts: BTreeMap<i32, FishingEntrustStatus>,
    pub TraceEntrusts: i32,
    pub FishingTech: Vec<FishingTechInfo>,
    pub ShipInfo: Option<FishingShipInfo>,
    pub IllustratedInfo: Option<FishingIllustratedInfo>,
    pub SceneCages: BTreeMap<i32, SceneFishCageInfo>,
    pub SceneFishPoints: BTreeMap<i32, SceneFishPointInfo>,
    pub NoticeIds: Vec<i32>,
    pub HandInInfo: Vec<HandInInfo>,
    pub UnlockPortId: Vec<i32>,
    pub PhantomSkinList: Vec<i32>,
    pub EntrustRefreshRatio: i32,
}

impl PbMessage for FishingData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Entrusts.is_empty() {
            crate::wire::write_map(out, 3, &self.Entrusts);
        }
        if self.TraceEntrusts != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceEntrusts);
        }
        for item in &self.FishingTech {
            crate::wire::write_submessage(out, 5, item);
        }
        if let Some(v) = &self.ShipInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.IllustratedInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if !self.SceneCages.is_empty() {
            crate::wire::write_map(out, 8, &self.SceneCages);
        }
        if !self.SceneFishPoints.is_empty() {
            crate::wire::write_map(out, 9, &self.SceneFishPoints);
        }
        for item in &self.NoticeIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.HandInInfo {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.UnlockPortId {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.PhantomSkinList {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EntrustRefreshRatio != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustRefreshRatio);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_map_entry(r, &mut self.Entrusts)?,
            4 => self.TraceEntrusts = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FishingTech
                    .push(<FishingTechInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ShipInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.IllustratedInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => crate::wire::decode_map_entry(r, &mut self.SceneCages)?,
            9 => crate::wire::decode_map_entry(r, &mut self.SceneFishPoints)?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NoticeIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.HandInInfo
                    .push(<HandInInfo as PbMessage>::decode(bytes)?);
            }
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockPortId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PhantomSkinList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            14 => self.EntrustRefreshRatio = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingTechInfo {
    pub NodeId: i32,
    pub Level: i32,
    pub CanUnlock: bool,
}

impl PbMessage for FishingTechInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.CanUnlock {
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
            1 => self.NodeId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.CanUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingShipInfo {
    pub SkinId: i32,
    pub SailingTime: i64,
    pub IsSailing: bool,
    pub CabinInfo: Option<CabinInfo>,
    pub EntityId: i64,
    pub IsInPort: bool,
    pub PortId: i32,
    pub LastPortId: i32,
}

impl PbMessage for FishingShipInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.SailingTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SailingTime);
        }
        if self.IsSailing {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CabinInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsInPort {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PortId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PortId);
        }
        if self.LastPortId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastPortId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SkinId = r.read_varint_i32()?,
            6 => self.SailingTime = r.read_varint_i64()?,
            7 => self.IsSailing = r.read_bool()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.CabinInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.EntityId = r.read_varint_i64()?,
            11 => self.IsInPort = r.read_bool()?,
            12 => self.PortId = r.read_varint_i32()?,
            13 => self.LastPortId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingIllustratedInfo {
    pub IllustratedList: Vec<OneFishingIllustratedData>,
    pub RewardedId: Vec<FishingIllustratedRewardInfo>,
    pub UnlockDetections: Vec<i32>,
}

impl PbMessage for FishingIllustratedInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.IllustratedList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.RewardedId {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.UnlockDetections {
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
                self.IllustratedList
                    .push(<OneFishingIllustratedData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RewardedId
                    .push(<FishingIllustratedRewardInfo as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockDetections,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneFishingIllustratedData {
    pub Id: i32,
    pub MaxSize: i32,
    pub MinSize: i32,
}

impl PbMessage for OneFishingIllustratedData {
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
pub struct FishingItemInfo {
    pub ItemId: i32,
    pub IncrId: i32,
    pub Rotate: Option<FishingItemRotate>,
    pub Pos: Option<IntVector2D>,
    pub Size: i32,
    pub Cup: Option<FishCup>,
    pub Quality: i32,
    pub Price: i32,
}

impl PbMessage for FishingItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
        }
        if let Some(v) = &self.Rotate {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.Size != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Size);
        }
        if let Some(v) = &self.Cup {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Quality != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Quality);
        }
        if self.Price != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Price);
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
            4 => {
                self.Rotate = Some(
                    <FishingItemRotate as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.Size = r.read_varint_i32()?,
            7 => {
                self.Cup = Some(<FishCup as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.Quality = r.read_varint_i32()?,
            9 => self.Price = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IntVector2D {
    pub X: i32,
    pub Y: i32,
}

impl PbMessage for IntVector2D {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.X);
        }
        if self.Y != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Y);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.X = r.read_varint_i32()?,
            2 => self.Y = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CabinInfo {
    pub FishingItem: Vec<FishingItemInfo>,
    pub CabinShape: i32,
    pub QuickSellShape: i32,
    pub NetCabinItems: Vec<FishingItemInfo>,
    pub TempCabinItems: Vec<FishingItemInfo>,
    pub QuickSellRatio: i32,
}

impl PbMessage for CabinInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FishingItem {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CabinShape != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CabinShape);
        }
        if self.QuickSellShape != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuickSellShape);
        }
        for item in &self.NetCabinItems {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.TempCabinItems {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.QuickSellRatio != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.QuickSellRatio);
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
                self.FishingItem
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.CabinShape = r.read_varint_i32()?,
            3 => self.QuickSellShape = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.NetCabinItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TempCabinItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            6 => self.QuickSellRatio = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneFishCageInfo {
    pub Cages: Vec<SceneFishCageData>,
}

impl PbMessage for SceneFishCageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Cages {
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
                self.Cages
                    .push(<SceneFishCageData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneFishCageData {
    pub Id: i32,
    pub EntityConfigId: i32,
    pub MaxCount: i32,
    pub Items: Vec<FishingItemInfo>,
    pub LastUpdateTime: i64,
    pub NextUpdateTime: i64,
    pub RefreshTime: i32,
}

impl PbMessage for SceneFishCageData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
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
            5 => self.Id = r.read_varint_i32()?,
            1 => self.EntityConfigId = r.read_varint_i32()?,
            2 => self.MaxCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.LastUpdateTime = r.read_varint_i64()?,
            6 => self.NextUpdateTime = r.read_varint_i64()?,
            7 => self.RefreshTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneFishPointInfo {
    pub FishPoints: Vec<SceneFishPointData>,
    pub TempFishPoints: Vec<TempFishPointInfo>,
}

impl PbMessage for SceneFishPointInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FishPoints {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.TempFishPoints {
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
                self.FishPoints
                    .push(<SceneFishPointData as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TempFishPoints
                    .push(<TempFishPointInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneFishPointData {
    pub Id: i32,
    pub EntityConfigId: i32,
    pub CurCount: i32,
    pub MaxCount: i32,
    pub LastUpdateTime: i64,
    pub NextUpdateTime: i64,
    pub RefreshTime: i32,
    pub GamePlayId: i32,
    pub Interacted: bool,
}

impl PbMessage for SceneFishPointData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
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
            5 => self.Id = r.read_varint_i32()?,
            1 => self.EntityConfigId = r.read_varint_i32()?,
            2 => self.CurCount = r.read_varint_i32()?,
            3 => self.MaxCount = r.read_varint_i32()?,
            4 => self.LastUpdateTime = r.read_varint_i64()?,
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
pub struct TempFishPointInfo {
    pub EntityId: i64,
    pub CurCount: i32,
    pub MaxCount: i32,
    pub ConfigId: i32,
    pub GamePlayId: i32,
}

impl PbMessage for TempFishPointInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.CurCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCount);
        }
        if self.MaxCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCount);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayId);
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
            2 => self.CurCount = r.read_varint_i32()?,
            3 => self.MaxCount = r.read_varint_i32()?,
            4 => self.ConfigId = r.read_varint_i32()?,
            5 => self.GamePlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingIllustratedRewardInfo {
    pub Id: i32,
    pub CurrentProgress: i32,
    pub TargetProgress: i32,
    pub HasPassed: bool,
    pub IsTaken: bool,
}

impl PbMessage for FishingIllustratedRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CurrentProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentProgress);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetProgress);
        }
        if self.HasPassed {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsTaken {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.CurrentProgress = r.read_varint_i32()?,
            3 => self.TargetProgress = r.read_varint_i32()?,
            4 => self.HasPassed = r.read_bool()?,
            5 => self.IsTaken = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInInfo {
    pub Id: i32,
    pub FishingItem: Vec<FishingItemInfo>,
}

impl PbMessage for HandInInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.FishingItem {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FishingItem
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowStartNotify {
    pub FlowIncId: i64,
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
    pub GameCtx: Option<GameCtxPb>,
    pub PlotMode: Vec<u8>,
    pub aAsync: bool,
    pub IsSkip: bool,
    pub HasPlotPos: bool,
    pub PlotCoordinates: Option<Vector>,
}

impl FlowStartNotify {
    pub const MSG_ID: u16 = 25049;
}

impl PbMessage for FlowStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
        if !self.FlowListName.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowListName);
        }
        if self.FlowId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlowId);
        }
        if self.StateId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateId);
        }
        if let Some(v) = &self.GameCtx {
            crate::wire::write_submessage(out, 14, v);
        }
        if !self.PlotMode.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlotMode);
        }
        if self.aAsync {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSkip {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HasPlotPos {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.PlotCoordinates {
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
            6 => self.FlowIncId = r.read_varint_i64()?,
            8 => self.FlowListName = r.read_len_delimited()?.to_vec(),
            12 => self.FlowId = r.read_varint_i32()?,
            4 => self.StateId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.GameCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.PlotMode = r.read_len_delimited()?.to_vec(),
            10 => self.aAsync = r.read_bool()?,
            15 => self.IsSkip = r.read_bool()?,
            3 => self.HasPlotPos = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PlotCoordinates
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowEndRequest {
    pub FlowIncId: i64,
    pub IsSkip: bool,
    pub OptionInfos: BTreeMap<i32, FlowOptionInfoList>,
    pub ClientError: bool,
}

impl FlowEndRequest {
    pub const MSG_ID: u16 = 26672;
}

impl PbMessage for FlowEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
        if self.IsSkip {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.OptionInfos.is_empty() {
            crate::wire::write_map(out, 3, &self.OptionInfos);
        }
        if self.ClientError {
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
            9 => self.FlowIncId = r.read_varint_i64()?,
            12 => self.IsSkip = r.read_bool()?,
            3 => crate::wire::decode_map_entry(r, &mut self.OptionInfos)?,
            14 => self.ClientError = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowOptionInfoList {
    pub OptionIndexList: Vec<FlowOptionInfo>,
}

impl PbMessage for FlowOptionInfoList {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OptionIndexList {
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
                self.OptionIndexList
                    .push(<FlowOptionInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowOptionInfo {
    pub TalkId: i32,
    pub OptionIndex: i32,
}

impl PbMessage for FlowOptionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalkId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalkId);
        }
        if self.OptionIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OptionIndex);
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
            2 => self.OptionIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowEndResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlowEndResponse {
    pub const MSG_ID: u16 = 29872;
}

impl PbMessage for FlowEndResponse {
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
pub struct FlowActionRequest {
    pub FlowIncId: i64,
    pub ActionId: i32,
}

impl FlowActionRequest {
    pub const MSG_ID: u16 = 22962;
}

impl PbMessage for FlowActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FlowIncId);
        }
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            14 => self.FlowIncId = r.read_varint_i64()?,
            11 => self.ActionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowActionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlowActionResponse {
    pub const MSG_ID: u16 = 29226;
}

impl PbMessage for FlowActionResponse {
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
pub struct FlySkinEquipData {
    pub SkinId: i32,
    pub RoleIds: Vec<i32>,
}

impl PbMessage for FlySkinEquipData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        for item in &self.RoleIds {
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
            1 => self.SkinId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFlyEquipNotify {
    pub FlySkinEquipData: Vec<FlySkinEquipData>,
}

impl RoleFlyEquipNotify {
    pub const MSG_ID: u16 = 21114;
}

impl PbMessage for RoleFlyEquipNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlySkinEquipData {
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
                self.FlySkinEquipData
                    .push(<FlySkinEquipData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinWearRequest {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl FlySkinWearRequest {
    pub const MSG_ID: u16 = 22343;
}

impl PbMessage for FlySkinWearRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            8 => self.RoleId = r.read_varint_i32()?,
            1 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinWearResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlySkinWearResponse {
    pub const MSG_ID: u16 = 25179;
}

impl PbMessage for FlySkinWearResponse {
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
pub struct EquipFlySkinData {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl PbMessage for EquipFlySkinData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinWearAllRoleRequest {
    pub SkinId: i32,
}

impl FlySkinWearAllRoleRequest {
    pub const MSG_ID: u16 = 17655;
}

impl PbMessage for FlySkinWearAllRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinWearAllRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FlySkinData: Vec<EquipFlySkinData>,
}

impl FlySkinWearAllRoleResponse {
    pub const MSG_ID: u16 = 15632;
}

impl PbMessage for FlySkinWearAllRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FlySkinData {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FlySkinData
                    .push(<EquipFlySkinData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFlyEquipChangeNotify {
    pub FlySkinData: Vec<EquipFlySkinData>,
}

impl RoleFlyEquipChangeNotify {
    pub const MSG_ID: u16 = 19754;
}

impl PbMessage for RoleFlyEquipChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlySkinData {
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
                self.FlySkinData
                    .push(<EquipFlySkinData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinConfigData {
    pub SkinId: i32,
    pub FlySkinId: i32,
}

impl PbMessage for FlySkinConfigData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.FlySkinId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlySkinId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SkinId = r.read_varint_i32()?,
            2 => self.FlySkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFlySkinChangeData {
    pub EntityId: i64,
    pub FlySkinConfigData: Vec<FlySkinConfigData>,
}

impl PbMessage for EntityFlySkinChangeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.FlySkinConfigData {
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
                self.FlySkinConfigData
                    .push(<FlySkinConfigData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SoarWingOrParaglidingSkinChangeNotify {
    pub FlySkinData: Vec<EntityFlySkinChangeData>,
}

impl SoarWingOrParaglidingSkinChangeNotify {
    pub const MSG_ID: u16 = 16196;
}

impl PbMessage for SoarWingOrParaglidingSkinChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlySkinData {
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
                self.FlySkinData
                    .push(<EntityFlySkinChangeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlyEquipAddNotify {
    pub UnlockFlySkinIds: Vec<i32>,
}

impl FlyEquipAddNotify {
    pub const MSG_ID: u16 = 21836;
}

impl PbMessage for FlyEquipAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockFlySkinIds {
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
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockFlySkinIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForgeInfoRequest {}

impl ForgeInfoRequest {
    pub const MSG_ID: u16 = 16907;
}

impl PbMessage for ForgeInfoRequest {
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
pub struct ForgeInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ForgeInfoList: Vec<OneForgeInfo>,
    pub ForgeConfigs: Vec<OneForgeConfig>,
    pub LimitRefreshTime: i64,
}

impl ForgeInfoResponse {
    pub const MSG_ID: u16 = 18747;
}

impl PbMessage for ForgeInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ForgeInfoList {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.ForgeConfigs {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.LimitRefreshTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitRefreshTime);
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
                self.ForgeInfoList
                    .push(<OneForgeInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ForgeConfigs
                    .push(<OneForgeConfig as PbMessage>::decode(bytes)?);
            }
            2 => self.LimitRefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneForgeInfo {
    pub Id: i32,
    pub LastRoleId: i32,
    pub LimitTotalCount: i32,
    pub LimitForgeCount: i32,
    pub StartTime: i64,
    pub EndTime: i64,
}

impl PbMessage for OneForgeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.LastRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastRoleId);
        }
        if self.LimitTotalCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitTotalCount);
        }
        if self.LimitForgeCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitForgeCount);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            3 => self.LastRoleId = r.read_varint_i32()?,
            4 => self.LimitTotalCount = r.read_varint_i32()?,
            5 => self.LimitForgeCount = r.read_varint_i32()?,
            6 => self.StartTime = r.read_varint_i64()?,
            7 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneForgeConfig {
    pub Id: i32,
    pub StartTime: i64,
    pub EndTime: i64,
}

impl PbMessage for OneForgeConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
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
            2 => self.StartTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Formation {
    pub Id: i32,
    pub RoleIndex: i32,
    pub Role: Vec<i32>,
    pub IsCurrent: bool,
}

impl PbMessage for Formation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RoleIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIndex);
        }
        for item in &self.Role {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsCurrent {
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
            2 => self.RoleIndex = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Role, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.IsCurrent = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendInfo {
    pub Info: Option<PlayerDetails>,
    pub Remark: Vec<u8>,
}

impl PbMessage for FriendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.Remark.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Remark);
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
            2 => self.Remark = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendApply {
    pub Info: Option<PlayerDetails>,
    pub CreatedTime: i64,
}

impl PbMessage for FriendApply {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.CreatedTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CreatedTime);
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
            2 => self.CreatedTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendAllRequest {}

impl FriendAllRequest {
    pub const MSG_ID: u16 = 29479;
}

impl PbMessage for FriendAllRequest {
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
pub struct FriendAllResponse {
    pub FriendInfoList: Vec<FriendInfo>,
    pub FriendApplyList: Vec<FriendApply>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FriendAllResponse {
    pub const MSG_ID: u16 = 20502;
}

impl PbMessage for FriendAllResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FriendInfoList {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.FriendApplyList {
            crate::wire::write_submessage(out, 3, item);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.FriendInfoList
                    .push(<FriendInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FriendApplyList
                    .push(<FriendApply as PbMessage>::decode(bytes)?);
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
pub struct Function {
    pub Id: i32,
    pub Flag: i32,
}

impl PbMessage for Function {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Flag != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Flag);
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
            5 => self.Flag = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncOpenNotify {
    pub Func: Vec<Function>,
}

impl FuncOpenNotify {
    pub const MSG_ID: u16 = 19093;
}

impl PbMessage for FuncOpenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Func {
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
                self.Func.push(<Function as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaConsume {
    pub Times: i32,
    pub Consume: i32,
}

impl PbMessage for GachaConsume {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Times != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Times);
        }
        if self.Consume != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Consume);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Times = r.read_varint_i32()?,
            2 => self.Consume = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaDiscountInfo {
    pub Times: i32,
    pub LimitTimes: i32,
    pub DiscountConsume: i32,
    pub UsedTimes: i32,
}

impl PbMessage for GachaDiscountInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Times != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Times);
        }
        if self.LimitTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitTimes);
        }
        if self.DiscountConsume != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscountConsume);
        }
        if self.UsedTimes != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UsedTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Times = r.read_varint_i32()?,
            2 => self.LimitTimes = r.read_varint_i32()?,
            3 => self.DiscountConsume = r.read_varint_i32()?,
            4 => self.UsedTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaPoolInfo {
    pub Id: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub Title: Vec<u8>,
    pub Description: Vec<u8>,
    pub UiType: i32,
    pub ThemeColor: Vec<u8>,
    pub ShowIdList: Vec<i32>,
    pub UpList: Vec<i32>,
    pub PreviewIdList: Vec<i32>,
    pub ComplianceDetail: Vec<u8>,
}

impl PbMessage for GachaPoolInfo {
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
        if !self.Title.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Title);
        }
        if !self.Description.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Description);
        }
        if self.UiType != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UiType);
        }
        if !self.ThemeColor.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ThemeColor);
        }
        for item in &self.ShowIdList {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UpList {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.PreviewIdList {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.ComplianceDetail.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ComplianceDetail);
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
            4 => self.Title = r.read_len_delimited()?.to_vec(),
            5 => self.Description = r.read_len_delimited()?.to_vec(),
            6 => self.UiType = r.read_varint_i32()?,
            7 => self.ThemeColor = r.read_len_delimited()?.to_vec(),
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ShowIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UpList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreviewIdList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.ComplianceDetail = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaInfo {
    pub Id: i32,
    pub TodayTimes: i32,
    pub TotalTimes: i32,
    pub ItemId: i32,
    pub GachaConsumes: Vec<GachaConsume>,
    pub UsePoolId: i32,
    pub Pools: Vec<GachaPoolInfo>,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub DailyLimitTimes: i32,
    pub TotalLimitTimes: i32,
    pub ResourcesId: Vec<u8>,
    pub GachaAccumulateId: i32,
    pub GachaDiscountInfos: Vec<GachaDiscountInfo>,
    pub OnlyViewDiscount: bool,
    pub IsShowProgress: bool,
    pub DiscountTagDetails: BTreeMap<i32, Vec<u8>>,
}

impl PbMessage for GachaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.TodayTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TodayTimes);
        }
        if self.TotalTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalTimes);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        for item in &self.GachaConsumes {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.UsePoolId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UsePoolId);
        }
        for item in &self.Pools {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.DailyLimitTimes != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DailyLimitTimes);
        }
        if self.TotalLimitTimes != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalLimitTimes);
        }
        if !self.ResourcesId.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ResourcesId);
        }
        if self.GachaAccumulateId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaAccumulateId);
        }
        for item in &self.GachaDiscountInfos {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.OnlyViewDiscount {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsShowProgress {
            crate::wire::write_tag(out, 16, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.DiscountTagDetails.is_empty() {
            crate::wire::write_map(out, 17, &self.DiscountTagDetails);
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
            2 => self.TodayTimes = r.read_varint_i32()?,
            3 => self.TotalTimes = r.read_varint_i32()?,
            4 => self.ItemId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.GachaConsumes
                    .push(<GachaConsume as PbMessage>::decode(bytes)?);
            }
            6 => self.UsePoolId = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Pools
                    .push(<GachaPoolInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.BeginTime = r.read_varint_i64()?,
            9 => self.EndTime = r.read_varint_i64()?,
            10 => self.DailyLimitTimes = r.read_varint_i32()?,
            11 => self.TotalLimitTimes = r.read_varint_i32()?,
            12 => self.ResourcesId = r.read_len_delimited()?.to_vec(),
            13 => self.GachaAccumulateId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.GachaDiscountInfos
                    .push(<GachaDiscountInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.OnlyViewDiscount = r.read_bool()?,
            16 => self.IsShowProgress = r.read_bool()?,
            17 => crate::wire::decode_map_entry(r, &mut self.DiscountTagDetails)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaReward {
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for GachaReward {
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

#[derive(Debug, Clone, PartialEq)]
pub enum GachaResultBottomOneof {
    BottomExtraReward(GachaReward),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaResult {
    pub Bottom: Option<GachaResultBottomOneof>,
    pub GachaReward: Option<GachaReward>,
    pub ExtraRewards: Vec<GachaReward>,
    pub TransformRewards: Vec<GachaReward>,
}

impl PbMessage for GachaResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Bottom {
            match oneof {
                GachaResultBottomOneof::BottomExtraReward(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if let Some(v) = &self.GachaReward {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ExtraRewards {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.TransformRewards {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <GachaReward as PbMessage>::decode(bytes)?;
                self.Bottom = Some(GachaResultBottomOneof::BottomExtraReward(val));
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.GachaReward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraRewards
                    .push(<GachaReward as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TransformRewards
                    .push(<GachaReward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaInfoRequest {
    pub Language: i32,
}

impl GachaInfoRequest {
    pub const MSG_ID: u16 = 25069;
}

impl PbMessage for GachaInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Language != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Language);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Language = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GachaInfos: Vec<GachaInfo>,
    pub DailyTotalLeftTimes: i32,
    pub RecordId: Vec<u8>,
}

impl GachaInfoResponse {
    pub const MSG_ID: u16 = 22256;
}

impl PbMessage for GachaInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GachaInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.DailyTotalLeftTimes != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DailyTotalLeftTimes);
        }
        if !self.RecordId.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.RecordId);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.GachaInfos
                    .push(<GachaInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.DailyTotalLeftTimes = r.read_varint_i32()?,
            9 => self.RecordId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaRequest {
    pub GachaId: i32,
    pub GachaTimes: i32,
}

impl GachaRequest {
    pub const MSG_ID: u16 = 19232;
}

impl PbMessage for GachaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaId);
        }
        if self.GachaTimes != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.GachaId = r.read_varint_i32()?,
            14 => self.GachaTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GachaResults: Vec<GachaResult>,
}

impl GachaResponse {
    pub const MSG_ID: u16 = 28271;
}

impl PbMessage for GachaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GachaResults {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.GachaResults
                    .push(<GachaResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaUsePoolRequest {
    pub GachaId: i32,
    pub PoolId: i32,
}

impl GachaUsePoolRequest {
    pub const MSG_ID: u16 = 29160;
}

impl PbMessage for GachaUsePoolRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaId);
        }
        if self.PoolId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PoolId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.GachaId = r.read_varint_i32()?,
            1 => self.PoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaUsePoolResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GachaUsePoolResponse {
    pub const MSG_ID: u16 = 19735;
}

impl PbMessage for GachaUsePoolResponse {
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
pub struct GachaItem {
    pub ItemId: i32,
    pub IsUp: bool,
}

impl PbMessage for GachaItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IsUp {
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.IsUp = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaPoolDetail {
    pub Text: Vec<u8>,
    pub FiveStarRoles: Vec<GachaItem>,
    pub FiveStarWeapons: Vec<GachaItem>,
    pub FourStarRoles: Vec<GachaItem>,
    pub FourStarWeapons: Vec<GachaItem>,
    pub ThreeStarRoles: Vec<GachaItem>,
    pub FiveStarTitle: Vec<u8>,
    pub FileStarDetail: Vec<u8>,
    pub FourStarTitle: Vec<u8>,
    pub FourStarDetail: Vec<u8>,
    pub ThreeStarTitle: Vec<u8>,
    pub ThreeStarDetail: Vec<u8>,
}

impl PbMessage for GachaPoolDetail {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Text.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Text);
        }
        for item in &self.FiveStarRoles {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.FiveStarWeapons {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.FourStarRoles {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.FourStarWeapons {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.ThreeStarRoles {
            crate::wire::write_submessage(out, 6, item);
        }
        if !self.FiveStarTitle.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FiveStarTitle);
        }
        if !self.FileStarDetail.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FileStarDetail);
        }
        if !self.FourStarTitle.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FourStarTitle);
        }
        if !self.FourStarDetail.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FourStarDetail);
        }
        if !self.ThreeStarTitle.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ThreeStarTitle);
        }
        if !self.ThreeStarDetail.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ThreeStarDetail);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Text = r.read_len_delimited()?.to_vec(),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FiveStarRoles
                    .push(<GachaItem as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FiveStarWeapons
                    .push(<GachaItem as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FourStarRoles
                    .push(<GachaItem as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FourStarWeapons
                    .push(<GachaItem as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ThreeStarRoles
                    .push(<GachaItem as PbMessage>::decode(bytes)?);
            }
            7 => self.FiveStarTitle = r.read_len_delimited()?.to_vec(),
            8 => self.FileStarDetail = r.read_len_delimited()?.to_vec(),
            9 => self.FourStarTitle = r.read_len_delimited()?.to_vec(),
            10 => self.FourStarDetail = r.read_len_delimited()?.to_vec(),
            11 => self.ThreeStarTitle = r.read_len_delimited()?.to_vec(),
            12 => self.ThreeStarDetail = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaPoolDetailRequest {
    pub PoolId: i32,
}

impl GachaPoolDetailRequest {
    pub const MSG_ID: u16 = 21264;
}

impl PbMessage for GachaPoolDetailRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PoolId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PoolId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.PoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaPoolDetailResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GachaPoolDetail: Option<GachaPoolDetail>,
}

impl GachaPoolDetailResponse {
    pub const MSG_ID: u16 = 18488;
}

impl PbMessage for GachaPoolDetailResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.GachaPoolDetail {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.GachaPoolDetail
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackInfoRequest {}

impl GivebackInfoRequest {
    pub const MSG_ID: u16 = 15189;
}

impl PbMessage for GivebackInfoRequest {
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
pub struct GivebackInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GivebackInfoResponse {
    pub const MSG_ID: u16 = 19180;
}

impl PbMessage for GivebackInfoResponse {
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
pub struct GuideInfoRequest {}

impl GuideInfoRequest {
    pub const MSG_ID: u16 = 15732;
}

impl PbMessage for GuideInfoRequest {
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
pub struct GuideInfoResponse {
    pub GuideGroupFinishList: Vec<i32>,
}

impl GuideInfoResponse {
    pub const MSG_ID: u16 = 26611;
}

impl PbMessage for GuideInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GuideGroupFinishList {
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
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.GuideGroupFinishList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideTriggerRequest {
    pub GroupId: i32,
}

impl GuideTriggerRequest {
    pub const MSG_ID: u16 = 27818;
}

impl PbMessage for GuideTriggerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.GroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideTriggerResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl GuideTriggerResponse {
    pub const MSG_ID: u16 = 25835;
}

impl PbMessage for GuideTriggerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 5, crate::wire::WireType::LengthDelimited);
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideFinishRequest {
    pub GroupId: i32,
}

impl GuideFinishRequest {
    pub const MSG_ID: u16 = 22758;
}

impl PbMessage for GuideFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.GroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
}

impl GuideFinishResponse {
    pub const MSG_ID: u16 = 21373;
}

impl PbMessage for GuideFinishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 1, crate::wire::WireType::LengthDelimited);
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeartbeatRequest {
    pub AntiData: Vec<u8>,
}

impl HeartbeatRequest {
    pub const MSG_ID: u16 = 1650;
}

impl PbMessage for HeartbeatRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AntiData.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
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
            1 => self.AntiData = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeartbeatResponse {}

impl HeartbeatResponse {
    pub const MSG_ID: u16 = 1651;
}

impl PbMessage for HeartbeatResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum IllustratedEntrySubTypeOneof {
    PhotographSubType(PhotographSubType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedEntry {
    pub SubType: Option<IllustratedEntrySubTypeOneof>,
    pub Id: i32,
    pub CreateTime: u32,
    pub Num: i32,
    pub IsRead: bool,
}

impl PbMessage for IllustratedEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.SubType {
            match oneof {
                IllustratedEntrySubTypeOneof::PhotographSubType(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.CreateTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.CreateTime));
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.IsRead {
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
            5 => {
                self.SubType = Some(IllustratedEntrySubTypeOneof::PhotographSubType(
                    <PhotographSubType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            1 => self.Id = r.read_varint_i32()?,
            2 => self.CreateTime = r.read_varint_u32()?,
            3 => self.Num = r.read_varint_i32()?,
            4 => self.IsRead = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedClass {
    pub Type: Option<IllustratedType>,
    pub IllustratedEntryList: Vec<IllustratedEntry>,
}

impl PbMessage for IllustratedClass {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.IllustratedEntryList {
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
                self.Type = Some(
                    <IllustratedType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.IllustratedEntryList
                    .push(<IllustratedEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedInfoRequest {
    pub TypeList: Vec<IllustratedType>,
}

impl IllustratedInfoRequest {
    pub const MSG_ID: u16 = 15484;
}

impl PbMessage for IllustratedInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TypeList {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TypeList, |sr| {
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
pub struct IllustratedInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub IllustratedClassList: Vec<IllustratedClass>,
}

impl IllustratedInfoResponse {
    pub const MSG_ID: u16 = 28603;
}

impl PbMessage for IllustratedInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 15, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.IllustratedClassList {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            2 => {
                let bytes = r.read_len_delimited()?;
                self.IllustratedClassList
                    .push(<IllustratedClass as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleIllustratedInfoRequest {}

impl RoleIllustratedInfoRequest {
    pub const MSG_ID: u16 = 23991;
}

impl PbMessage for RoleIllustratedInfoRequest {
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
