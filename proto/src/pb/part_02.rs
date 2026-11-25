use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimelineTrackComponentPb {
    pub Index: i32,
    pub ControlDatas: Vec<TimelineTrackControlDataPb>,
}

impl PbMessage for TimelineTrackComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        for item in &self.ControlDatas {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ControlDatas
                    .push(<TimelineTrackControlDataPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardPb {
    pub OccupiedGridList: Vec<OccupiedBoardGridInfo>,
    pub DynamicGridConfigs: Vec<BoardGridDynamicConfig>,
    pub CanMove: bool,
}

impl PbMessage for BoardPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OccupiedGridList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.DynamicGridConfigs {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CanMove {
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
                self.OccupiedGridList
                    .push(<OccupiedBoardGridInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DynamicGridConfigs
                    .push(<BoardGridDynamicConfig as PbMessage>::decode(bytes)?);
            }
            3 => self.CanMove = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CrystalMonsterSlotInfo {
    pub EntityIds: Vec<i32>,
    pub MonsterType: i32,
}

impl PbMessage for CrystalMonsterSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MonsterType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterType);
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
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.MonsterType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CrystalMonsterInfoPb {
    pub SlotInfoList: Vec<CrystalMonsterSlotInfo>,
}

impl PbMessage for CrystalMonsterInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SlotInfoList {
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
                self.SlotInfoList
                    .push(<CrystalMonsterSlotInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MonsterGachaDataPb {
    pub MonsterCrystalInfoList: Vec<CrystalMonsterInfoPb>,
}

impl PbMessage for MonsterGachaDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MonsterCrystalInfoList {
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
                self.MonsterCrystalInfoList
                    .push(<CrystalMonsterInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FanComponentPb {
    pub NumOfTurns: i32,
}

impl PbMessage for FanComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NumOfTurns != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NumOfTurns);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.NumOfTurns = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillItemPb {
    pub CombatCommon: Option<CombatCommon>,
    pub SkillId: i64,
}

impl PbMessage for PassiveSkillItemPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.SkillId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveSkillComponentPb {
    pub PassiveSkillItemPbList: Vec<PassiveSkillItemPb>,
}

impl PbMessage for PassiveSkillComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PassiveSkillItemPbList {
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
                self.PassiveSkillItemPbList
                    .push(<PassiveSkillItemPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterSkillComponentPb {
    pub UseSkillInfo: Option<UseSkillInformation>,
    pub MontageIndex: i32,
    pub MontagePlayTime: i32,
    pub Section: Vec<u8>,
    pub SpeedRatio: f32,
    pub MessageId: i64,
    pub MontageContext: i64,
}

impl PbMessage for CharacterSkillComponentPb {
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
pub struct SkillComponentPb {
    pub SkillId: i32,
    pub ConstateId: i64,
}

impl PbMessage for SkillComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.SkillId = r.read_varint_i32()?,
            2 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PassiveGaSkillComponentPb {
    pub SkillInfoList: Vec<CharacterSkillComponentPb>,
    pub SkillComponentPb: Vec<SkillComponentPb>,
}

impl PbMessage for PassiveGaSkillComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkillInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.SkillComponentPb {
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
                self.SkillInfoList
                    .push(<CharacterSkillComponentPb as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SkillComponentPb
                    .push(<SkillComponentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UseSkillInformation {
    pub CombatCommon: Option<CombatCommon>,
    pub Id: i64,
    pub SkillId: i64,
    pub MovementInformation: Option<MovementInformation>,
    pub Location: Option<Vector>,
    pub TargetId: i64,
    pub TimeStamp: f32,
    pub IsSpecialSkill: bool,
    pub Duration: i32,
    pub SkillInterruptLevel: i32,
    pub FightState: i32,
}

impl PbMessage for UseSkillInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
        }
        if let Some(v) = &self.MovementInformation {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.TargetId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetId);
        }
        if self.TimeStamp != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TimeStamp).to_bits().to_le_bytes());
        }
        if self.IsSpecialSkill {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Duration);
        }
        if self.SkillInterruptLevel != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillInterruptLevel);
        }
        if self.FightState != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Id = r.read_varint_i64()?,
            3 => self.SkillId = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MovementInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.TargetId = r.read_varint_i64()?,
            7 => self.TimeStamp = r.read_f32()?,
            8 => self.IsSpecialSkill = r.read_bool()?,
            9 => self.Duration = r.read_varint_i32()?,
            10 => self.SkillInterruptLevel = r.read_varint_i32()?,
            11 => self.FightState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for StateComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffProducerComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for BuffProducerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffConsumerComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for BuffConsumerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemEventListenerComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for SceneItemEventListenerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BulletComponentPb {
    pub ConstateId: i64,
}

impl PbMessage for BulletComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConstateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConstateId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAddNotify {
    pub EntityPbs: Vec<EntityPb>,
    pub RemoveTagIds: bool,
}

impl EntityAddNotify {
    pub const MSG_ID: u16 = 20302;
}

impl PbMessage for EntityAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityPbs {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RemoveTagIds {
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
                self.EntityPbs.push(<EntityPb as PbMessage>::decode(bytes)?);
            }
            9 => self.RemoveTagIds = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRemoveNotify {
    pub RemoveInfos: Vec<EntityRemoveInfo>,
    pub IsRemove: bool,
}

impl EntityRemoveNotify {
    pub const MSG_ID: u16 = 17927;
}

impl PbMessage for EntityRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RemoveInfos {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.IsRemove {
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RemoveInfos
                    .push(<EntityRemoveInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.IsRemove = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarDefinePbValueOneof {
    Boolean(bool),
    Int(i64),
    String(Vec<u8>),
    Float(f32),
    Entity(i32),
    Quest(i32),
    QuestState(QuestState),
    Transform(Transform),
    Prefab(i64),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VarDefinePb {
    pub Value: Option<VarDefinePbValueOneof>,
    pub VarType: i32,
}

impl PbMessage for VarDefinePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Value {
            match oneof {
                VarDefinePbValueOneof::Boolean(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
                VarDefinePbValueOneof::Int(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                VarDefinePbValueOneof::String(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
                VarDefinePbValueOneof::Float(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
                VarDefinePbValueOneof::Entity(v) => {
                    crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                VarDefinePbValueOneof::Quest(v) => {
                    crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                VarDefinePbValueOneof::QuestState(v) => {
                    crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
                VarDefinePbValueOneof::Transform(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                VarDefinePbValueOneof::Prefab(v) => {
                    crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if self.VarType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VarType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.Value = Some(VarDefinePbValueOneof::Boolean(r.read_bool()?)),
            3 => self.Value = Some(VarDefinePbValueOneof::Int(r.read_varint_i64()?)),
            4 => {
                self.Value = Some(VarDefinePbValueOneof::String(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            5 => self.Value = Some(VarDefinePbValueOneof::Float(r.read_f32()?)),
            6 => self.Value = Some(VarDefinePbValueOneof::Entity(r.read_varint_i32()?)),
            7 => self.Value = Some(VarDefinePbValueOneof::Quest(r.read_varint_i32()?)),
            8 => {
                self.Value = Some(VarDefinePbValueOneof::QuestState(
                    <QuestState as crate::wire::PbEnum>::from_i32_or_default(r.read_varint_i32()?),
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <Transform as PbMessage>::decode(bytes)?;
                self.Value = Some(VarDefinePbValueOneof::Transform(val));
            }
            10 => self.Value = Some(VarDefinePbValueOneof::Prefab(r.read_varint_i64()?)),
            1 => self.VarType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcPb {
    pub SplineEntityId: i32,
    pub SpawnEntityId: i32,
}

impl PbMessage for NpcPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
        }
        if self.SpawnEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpawnEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SplineEntityId = r.read_varint_i32()?,
            3 => self.SpawnEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LogicStateInitRequest {
    pub CombatCommon: Option<CombatCommon>,
    pub EntityId: i64,
    pub InitData: Option<LogicStateComponentPb>,
    pub ClientEntityId: i64,
}

impl LogicStateInitRequest {
    pub const MSG_ID: u16 = 18260;
}

impl PbMessage for LogicStateInitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.InitData {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.ClientEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientEntityId);
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
            2 => self.EntityId = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.InitData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.ClientEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LogicStateInitPush {
    pub CombatCommon: Option<CombatCommon>,
    pub EntityId: i64,
    pub InitData: Option<LogicStateComponentPb>,
    pub ClientEntityId: i64,
}

impl LogicStateInitPush {
    pub const MSG_ID: u16 = 16642;
}

impl PbMessage for LogicStateInitPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 15, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.InitData {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.ClientEntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientEntityId);
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
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.EntityId = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InitData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.ClientEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LogicStateInitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LogicStateInitResponse {
    pub const MSG_ID: u16 = 29984;
}

impl PbMessage for LogicStateInitResponse {
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
pub struct LogicStateInitNotify {
    pub CombatCommon: Option<CombatCommon>,
    pub EntityId: i64,
    pub InitData: Option<LogicStateComponentPb>,
}

impl LogicStateInitNotify {
    pub const MSG_ID: u16 = 16553;
}

impl PbMessage for LogicStateInitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 15, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.InitData {
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
            15 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.EntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.InitData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchLogicStateRequest {
    pub States: Option<LogicStateComponentPb>,
    pub ClientEntityId: i64,
}

impl SwitchLogicStateRequest {
    pub const MSG_ID: u16 = 17062;
}

impl PbMessage for SwitchLogicStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.States {
            crate::wire::write_submessage(out, 9, v);
        }
        if self.ClientEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientEntityId);
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
                self.States
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.ClientEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchLogicStatePush {
    pub States: Option<LogicStateComponentPb>,
    pub ClientEntityId: i64,
}

impl SwitchLogicStatePush {
    pub const MSG_ID: u16 = 18461;
}

impl PbMessage for SwitchLogicStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.States {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.ClientEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ClientEntityId);
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
                self.States
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.ClientEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SwitchLogicStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SwitchLogicStateResponse {
    pub const MSG_ID: u16 = 22141;
}

impl PbMessage for SwitchLogicStateResponse {
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
pub struct SwitchLogicStateNotify {
    pub States: Option<LogicStateComponentPb>,
}

impl SwitchLogicStateNotify {
    pub const MSG_ID: u16 = 25764;
}

impl PbMessage for SwitchLogicStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.States {
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
                self.States
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityActiveRequest {
    pub EntityId: i64,
}

impl EntityActiveRequest {
    pub const MSG_ID: u16 = 26157;
}

impl PbMessage for EntityActiveRequest {
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
pub struct EntityActiveResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ComponentPbs: Vec<EntityComponentPb>,
    pub IsVisible: bool,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub AiControlPlayerId: i32,
}

impl EntityActiveResponse {
    pub const MSG_ID: u16 = 19734;
}

impl PbMessage for EntityActiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ComponentPbs {
            crate::wire::write_submessage(out, 13, item);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.AiControlPlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AiControlPlayerId);
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ComponentPbs
                    .push(<EntityComponentPb as PbMessage>::decode(bytes)?);
            }
            12 => self.IsVisible = r.read_bool()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.AiControlPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationGameplayTagRequest {
    pub AddTagIds: i32,
    pub RemoveTagIds: bool,
}

impl AnimationGameplayTagRequest {
    pub const MSG_ID: u16 = 20782;
}

impl PbMessage for AnimationGameplayTagRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AddTagIds != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddTagIds);
        }
        if self.RemoveTagIds {
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
            15 => self.AddTagIds = r.read_varint_i32()?,
            6 => self.RemoveTagIds = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationGameplayTagPush {
    pub AddTagIds: i32,
    pub RemoveTagIds: bool,
}

impl AnimationGameplayTagPush {
    pub const MSG_ID: u16 = 15510;
}

impl PbMessage for AnimationGameplayTagPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AddTagIds != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddTagIds);
        }
        if self.RemoveTagIds {
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
            8 => self.AddTagIds = r.read_varint_i32()?,
            9 => self.RemoveTagIds = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationGameplayTagResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimationGameplayTagResponse {
    pub const MSG_ID: u16 = 18524;
}

impl PbMessage for AnimationGameplayTagResponse {
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
pub struct AnimationGameplayTagNotify {
    pub AddTagIds: i32,
    pub RemoveTagIds: bool,
}

impl AnimationGameplayTagNotify {
    pub const MSG_ID: u16 = 17808;
}

impl PbMessage for AnimationGameplayTagNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AddTagIds != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddTagIds);
        }
        if self.RemoveTagIds {
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
            14 => self.AddTagIds = r.read_varint_i32()?,
            12 => self.RemoveTagIds = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimalDieRequest {
    pub EntityId: i64,
    pub Pos: Option<Vector>,
}

impl AnimalDieRequest {
    pub const MSG_ID: u16 = 20952;
}

impl PbMessage for AnimalDieRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Pos {
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
            13 => self.EntityId = r.read_varint_i64()?,
            12 => {
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
pub struct AnimalDieResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimalDieResponse {
    pub const MSG_ID: u16 = 15258;
}

impl PbMessage for AnimalDieResponse {
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
pub struct AnimalDestroyRequest {
    pub EntityId: i64,
}

impl AnimalDestroyRequest {
    pub const MSG_ID: u16 = 26360;
}

impl PbMessage for AnimalDestroyRequest {
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
pub struct AnimalDestroyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimalDestroyResponse {
    pub const MSG_ID: u16 = 26151;
}

impl PbMessage for AnimalDestroyResponse {
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
pub struct AnimalDropRequest {
    pub EntityId: i64,
}

impl AnimalDropRequest {
    pub const MSG_ID: u16 = 27068;
}

impl PbMessage for AnimalDropRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimalDropResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl AnimalDropResponse {
    pub const MSG_ID: u16 = 29883;
}

impl PbMessage for AnimalDropResponse {
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
pub struct EntityStateReadyNotify {
    pub EntityId: i64,
    pub TagId: i32,
    pub Ready: bool,
}

impl EntityStateReadyNotify {
    pub const MSG_ID: u16 = 27022;
}

impl PbMessage for EntityStateReadyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        if self.Ready {
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
            15 => self.EntityId = r.read_varint_i64()?,
            11 => self.TagId = r.read_varint_i32()?,
            12 => self.Ready = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityInteractRequest {
    pub EntityId: i64,
    pub OptionIndex: i32,
    pub VisionEntityId: i64,
}

impl EntityInteractRequest {
    pub const MSG_ID: u16 = 23858;
}

impl PbMessage for EntityInteractRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.OptionIndex != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OptionIndex);
        }
        if self.VisionEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VisionEntityId);
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
            6 => self.OptionIndex = r.read_varint_i32()?,
            14 => self.VisionEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityInteractResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Interacting: bool,
}

impl EntityInteractResponse {
    pub const MSG_ID: u16 = 29979;
}

impl PbMessage for EntityInteractResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Interacting {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.Interacting = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityDynamicInteractRequest {
    pub EntityId: i64,
    pub OptionGuid: Vec<u8>,
}

impl EntityDynamicInteractRequest {
    pub const MSG_ID: u16 = 15821;
}

impl PbMessage for EntityDynamicInteractRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.OptionGuid.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
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
            12 => self.EntityId = r.read_varint_i64()?,
            8 => self.OptionGuid = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityDynamicInteractResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Interacting: bool,
}

impl EntityDynamicInteractResponse {
    pub const MSG_ID: u16 = 29039;
}

impl PbMessage for EntityDynamicInteractResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Interacting {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.Interacting = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoneVisibleChangeRequest {
    pub BoneVisibleData: Option<BoneVisibleData>,
}

impl BoneVisibleChangeRequest {
    pub const MSG_ID: u16 = 18467;
}

impl PbMessage for BoneVisibleChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BoneVisibleData {
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
                self.BoneVisibleData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoneVisibleChangePush {
    pub BoneVisibleData: Option<BoneVisibleData>,
}

impl BoneVisibleChangePush {
    pub const MSG_ID: u16 = 20838;
}

impl PbMessage for BoneVisibleChangePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BoneVisibleData {
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
                self.BoneVisibleData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoneVisibleChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BoneVisibleChangeResponse {
    pub const MSG_ID: u16 = 21695;
}

impl PbMessage for BoneVisibleChangeResponse {
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
pub struct BoneVisibleChangeNotify {
    pub BoneVisibleData: Option<BoneVisibleData>,
}

impl BoneVisibleChangeNotify {
    pub const MSG_ID: u16 = 29713;
}

impl PbMessage for BoneVisibleChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BoneVisibleData {
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
                self.BoneVisibleData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipComponentPb {
    pub WeaponId: i32,
    pub WeaponBreachLevel: i32,
}

impl PbMessage for EquipComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        if self.WeaponBreachLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponBreachLevel);
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
            2 => self.WeaponBreachLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WeaponSkinComponentPb {
    pub WeaponSkinId: i32,
}

impl PbMessage for WeaponSkinComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponSkinId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.WeaponSkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrnamentComponentPb {
    pub OrnamentIds: Vec<i32>,
}

impl PbMessage for OrnamentComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OrnamentIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.OrnamentIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterAttachComponentPb {
    pub PbCombinePartInfoList: Vec<CharacterAttachInfo>,
    pub PbCombineTargetServerId: i64,
}

impl PbMessage for CharacterAttachComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PbCombinePartInfoList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.PbCombineTargetServerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PbCombineTargetServerId);
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
                self.PbCombinePartInfoList
                    .push(<CharacterAttachInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.PbCombineTargetServerId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityEquipChangeNotify {
    pub EntityId: i64,
    pub EquipComponent: Option<EquipComponentPb>,
}

impl EntityEquipChangeNotify {
    pub const MSG_ID: u16 = 27672;
}

impl PbMessage for EntityEquipChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.EquipComponent {
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
            11 => self.EntityId = r.read_varint_i64()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.EquipComponent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityEquipSkinChangeNotify {
    pub EntityId: i64,
    pub WeaponSkinComponentPb: Option<WeaponSkinComponentPb>,
}

impl EntityEquipSkinChangeNotify {
    pub const MSG_ID: u16 = 17965;
}

impl PbMessage for EntityEquipSkinChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.WeaponSkinComponentPb {
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
            5 => self.EntityId = r.read_varint_i64()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponSkinComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityDressOrnamentChangeNotify {
    pub EntityId: i64,
    pub OrnamentComponentPb: Option<OrnamentComponentPb>,
}

impl EntityDressOrnamentChangeNotify {
    pub const MSG_ID: u16 = 24410;
}

impl PbMessage for EntityDressOrnamentChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.OrnamentComponentPb {
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
            4 => self.EntityId = r.read_varint_i64()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.OrnamentComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityStaticHookMoveRequestTargetOneof {
    TargetEntityId(i64),
    TargetPos(Vector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityStaticHookMoveRequest {
    pub Target: Option<EntityStaticHookMoveRequestTargetOneof>,
    pub EntityId: i64,
    pub HookMoveType: Option<StaticHookMoveType>,
}

impl EntityStaticHookMoveRequest {
    pub const MSG_ID: u16 = 23825;
}

impl PbMessage for EntityStaticHookMoveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Target {
            match oneof {
                EntityStaticHookMoveRequestTargetOneof::TargetEntityId(v) => {
                    crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                EntityStaticHookMoveRequestTargetOneof::TargetPos(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.HookMoveType {
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
            10 => {
                self.Target = Some(EntityStaticHookMoveRequestTargetOneof::TargetEntityId(
                    r.read_varint_i64()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.Target = Some(EntityStaticHookMoveRequestTargetOneof::TargetPos(val));
            }
            2 => self.EntityId = r.read_varint_i64()?,
            12 => {
                self.HookMoveType = Some(
                    <StaticHookMoveType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityStaticHookMovePushTargetOneof {
    TargetEntityId(i64),
    TargetPos(Vector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityStaticHookMovePush {
    pub Target: Option<EntityStaticHookMovePushTargetOneof>,
    pub EntityId: i64,
    pub HookMoveType: Option<StaticHookMoveType>,
}

impl EntityStaticHookMovePush {
    pub const MSG_ID: u16 = 20537;
}

impl PbMessage for EntityStaticHookMovePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Target {
            match oneof {
                EntityStaticHookMovePushTargetOneof::TargetEntityId(v) => {
                    crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                EntityStaticHookMovePushTargetOneof::TargetPos(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.HookMoveType {
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
            15 => {
                self.Target = Some(EntityStaticHookMovePushTargetOneof::TargetEntityId(
                    r.read_varint_i64()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.Target = Some(EntityStaticHookMovePushTargetOneof::TargetPos(val));
            }
            12 => self.EntityId = r.read_varint_i64()?,
            6 => {
                self.HookMoveType = Some(
                    <StaticHookMoveType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EntityStaticHookMoveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityStaticHookMoveResponse {
    pub const MSG_ID: u16 = 15040;
}

impl PbMessage for EntityStaticHookMoveResponse {
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
pub enum EntityStaticHookMoveNotifyTargetOneof {
    TargetEntityId(i64),
    TargetPos(Vector),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityStaticHookMoveNotify {
    pub Target: Option<EntityStaticHookMoveNotifyTargetOneof>,
    pub EntityId: i64,
    pub HookMoveType: Option<StaticHookMoveType>,
}

impl EntityStaticHookMoveNotify {
    pub const MSG_ID: u16 = 26154;
}

impl PbMessage for EntityStaticHookMoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Target {
            match oneof {
                EntityStaticHookMoveNotifyTargetOneof::TargetEntityId(v) => {
                    crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                EntityStaticHookMoveNotifyTargetOneof::TargetPos(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.HookMoveType {
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
            14 => {
                self.Target = Some(EntityStaticHookMoveNotifyTargetOneof::TargetEntityId(
                    r.read_varint_i64()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.Target = Some(EntityStaticHookMoveNotifyTargetOneof::TargetPos(val));
            }
            9 => self.EntityId = r.read_varint_i64()?,
            13 => {
                self.HookMoveType = Some(
                    <StaticHookMoveType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct SilenceNpcNotify {
    pub vTs: BTreeMap<i32, bool>,
}

impl SilenceNpcNotify {
    pub const MSG_ID: u16 = 17104;
}

impl PbMessage for SilenceNpcNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.vTs.is_empty() {
            crate::wire::write_map(out, 2, &self.vTs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_map_entry(r, &mut self.vTs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolStopRequest {
    pub EntityId: i64,
}

impl EntityPatrolStopRequest {
    pub const MSG_ID: u16 = 24266;
}

impl PbMessage for EntityPatrolStopRequest {
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
pub struct EntityPatrolStopResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityPatrolStopResponse {
    pub const MSG_ID: u16 = 19611;
}

impl PbMessage for EntityPatrolStopResponse {
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
pub struct PatrolComponentPb {
    pub Dir: bool,
}

impl PbMessage for PatrolComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Dir {
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
            1 => self.Dir = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowerPollutionComponentPb {
    pub UnPollutionSpline: Vec<i32>,
}

impl PbMessage for FlowerPollutionComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnPollutionSpline {
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
                &mut self.UnPollutionSpline,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabMachineComponentPb {
    pub CanCapturedItems: Vec<i32>,
    pub HighScore: i32,
    pub AccumulatedScore: i32,
}

impl PbMessage for DollGrabMachineComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CanCapturedItems {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.HighScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HighScore);
        }
        if self.AccumulatedScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccumulatedScore);
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
                &mut self.CanCapturedItems,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => self.HighScore = r.read_varint_i32()?,
            3 => self.AccumulatedScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabShowcaseComponentPb {
    pub DollItems: Vec<i32>,
}

impl PbMessage for DollGrabShowcaseComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DollItems {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DollItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollSmallMapInfoRequest {}

impl DollSmallMapInfoRequest {
    pub const MSG_ID: u16 = 20965;
}

impl PbMessage for DollSmallMapInfoRequest {
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
pub struct DollSmallMapInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SceneUnlimitedScoreInfos: Vec<SceneUnlimitedScoreInfo>,
    pub SceneDollDeliveryInfos: Vec<SceneDollDeliveryInfo>,
}

impl DollSmallMapInfoResponse {
    pub const MSG_ID: u16 = 19076;
}

impl PbMessage for DollSmallMapInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SceneUnlimitedScoreInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.SceneDollDeliveryInfos {
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
            15 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.SceneUnlimitedScoreInfos
                    .push(<SceneUnlimitedScoreInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SceneDollDeliveryInfos
                    .push(<SceneDollDeliveryInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneUnlimitedScoreInfo {
    pub instId: i32,
    pub UnlimitedScoreInfos: Vec<UnlimitedScoreInfo>,
}

impl PbMessage for SceneUnlimitedScoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.instId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.instId);
        }
        for item in &self.UnlimitedScoreInfos {
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
            1 => self.instId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.UnlimitedScoreInfos
                    .push(<UnlimitedScoreInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnlimitedScoreInfo {
    pub entityId: i32,
    pub HighestScore: i32,
    pub AccumulatedScore: i32,
}

impl PbMessage for UnlimitedScoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.entityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.entityId);
        }
        if self.HighestScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HighestScore);
        }
        if self.AccumulatedScore != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccumulatedScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.entityId = r.read_varint_i32()?,
            2 => self.HighestScore = r.read_varint_i32()?,
            3 => self.AccumulatedScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneDollDeliveryInfo {
    pub instId: i32,
    pub DollDeliveryInfos: Vec<DollDeliveryInfo>,
}

impl PbMessage for SceneDollDeliveryInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.instId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.instId);
        }
        for item in &self.DollDeliveryInfos {
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
            1 => self.instId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DollDeliveryInfos
                    .push(<DollDeliveryInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollDeliveryInfo {
    pub ShowcaseEntityId: i32,
    pub DeliveredDolls: Vec<i32>,
}

impl PbMessage for DollDeliveryInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowcaseEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowcaseEntityId);
        }
        for item in &self.DeliveredDolls {
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
            1 => self.ShowcaseEntityId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DeliveredDolls, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPositionRequest {
    pub ConfigId: i32,
    pub DungeonInstanceId: i32,
}

impl EntityPositionRequest {
    pub const MSG_ID: u16 = 21153;
}

impl PbMessage for EntityPositionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.DungeonInstanceId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DungeonInstanceId);
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
            3 => self.DungeonInstanceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPositionResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Pos: Option<Vector>,
}

impl EntityPositionResponse {
    pub const MSG_ID: u16 = 16641;
}

impl PbMessage for EntityPositionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Pos {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
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
pub struct ShieldInfoPb {
    pub Handle: i32,
    pub ConfigId: i32,
    pub ShieldValue: i32,
    pub Priority: i32,
    pub BuffHandle: i32,
    pub IsValid: bool,
}

impl PbMessage for ShieldInfoPb {
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
        if self.Priority != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Priority);
        }
        if self.BuffHandle != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffHandle);
        }
        if self.IsValid {
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
            1 => self.Handle = r.read_varint_i32()?,
            2 => self.ConfigId = r.read_varint_i32()?,
            3 => self.ShieldValue = r.read_varint_i32()?,
            4 => self.Priority = r.read_varint_i32()?,
            5 => self.BuffHandle = r.read_varint_i32()?,
            6 => self.IsValid = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShieldComponentPb {
    pub ShieldInfoPbList: Vec<ShieldInfoPb>,
    pub ShieldValueTotal: i32,
}

impl PbMessage for ShieldComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShieldInfoPbList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.ShieldValueTotal != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShieldValueTotal);
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
                self.ShieldInfoPbList
                    .push(<ShieldInfoPb as PbMessage>::decode(bytes)?);
            }
            2 => self.ShieldValueTotal = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NPCPerformGroupComponentPb {
    pub Type: Vec<u8>,
    pub State: Vec<u8>,
}

impl PbMessage for NPCPerformGroupComponentPb {
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
pub struct PlayerSceneComponentPb {
    pub EntityIds: Vec<i64>,
}

impl PbMessage for PlayerSceneComponentPb {
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
pub struct AttributesIdsComponentPb {
    pub PbSceneItemAttributeIds: Vec<i32>,
}

impl PbMessage for AttributesIdsComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PbSceneItemAttributeIds {
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
                &mut self.PbSceneItemAttributeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleManipulateRequestExitOneof {
    ExitType(ExitVehicleType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleManipulateRequest {
    pub Exit: Option<VehicleManipulateRequestExitOneof>,
    pub EntityId: i64,
    pub HostPlayerId: i32,
    pub IsEntering: bool,
    pub Seat: i32,
    pub ClientPredicted: bool,
    pub ReasonMsg: Vec<u8>,
}

impl VehicleManipulateRequest {
    pub const MSG_ID: u16 = 21983;
}

impl PbMessage for VehicleManipulateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Exit {
            match oneof {
                VehicleManipulateRequestExitOneof::ExitType(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.IsEntering {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
        }
        if self.ClientPredicted {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            4 => {
                self.Exit = Some(VehicleManipulateRequestExitOneof::ExitType(
                    <ExitVehicleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            1 => self.EntityId = r.read_varint_i64()?,
            9 => self.HostPlayerId = r.read_varint_i32()?,
            8 => self.IsEntering = r.read_bool()?,
            11 => self.Seat = r.read_varint_i32()?,
            5 => self.ClientPredicted = r.read_bool()?,
            14 => self.ReasonMsg = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehicleManipulateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleManipulateResponse {
    pub const MSG_ID: u16 = 23682;
}

impl PbMessage for VehicleManipulateResponse {
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
pub struct VehicleFinishRequest {
    pub EntityId: i64,
}

impl VehicleFinishRequest {
    pub const MSG_ID: u16 = 19634;
}

impl PbMessage for VehicleFinishRequest {
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
pub struct VehicleFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl VehicleFinishResponse {
    pub const MSG_ID: u16 = 17721;
}

impl PbMessage for VehicleFinishResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum EntityEnterVehicleRequestExitOneof {
    ExitType(ExitVehicleType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityEnterVehicleRequest {
    pub Exit: Option<EntityEnterVehicleRequestExitOneof>,
    pub EntityId: i64,
    pub VehicleCreatureId: i64,
    pub HostPlayerId: i32,
    pub IsEntering: bool,
    pub Seat: i32,
    pub ClientPredicted: bool,
    pub ReasonMsg: Vec<u8>,
}

impl EntityEnterVehicleRequest {
    pub const MSG_ID: u16 = 27542;
}

impl PbMessage for EntityEnterVehicleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Exit {
            match oneof {
                EntityEnterVehicleRequestExitOneof::ExitType(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.VehicleCreatureId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleCreatureId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.IsEntering {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
        }
        if self.ClientPredicted {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.ReasonMsg.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
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
            4 => {
                self.Exit = Some(EntityEnterVehicleRequestExitOneof::ExitType(
                    <ExitVehicleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            10 => self.EntityId = r.read_varint_i64()?,
            13 => self.VehicleCreatureId = r.read_varint_i64()?,
            14 => self.HostPlayerId = r.read_varint_i32()?,
            2 => self.IsEntering = r.read_bool()?,
            12 => self.Seat = r.read_varint_i32()?,
            3 => self.ClientPredicted = r.read_bool()?,
            11 => self.ReasonMsg = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityEnterVehicleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityEnterVehicleResponse {
    pub const MSG_ID: u16 = 25217;
}

impl PbMessage for EntityEnterVehicleResponse {
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
pub struct VehicleUpdateEntityNotify {
    pub EntityId: i64,
    pub VehicleCreatureId: i64,
    pub Seat: i32,
    pub IsEntering: bool,
    pub ExitType: Option<ExitVehicleType>,
    pub ClientPredicted: bool,
}

impl VehicleUpdateEntityNotify {
    pub const MSG_ID: u16 = 16415;
}

impl PbMessage for VehicleUpdateEntityNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.VehicleCreatureId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleCreatureId);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
        }
        if self.IsEntering {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ExitType {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ClientPredicted {
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
            12 => self.EntityId = r.read_varint_i64()?,
            4 => self.VehicleCreatureId = r.read_varint_i64()?,
            10 => self.Seat = r.read_varint_i32()?,
            5 => self.IsEntering = r.read_bool()?,
            7 => {
                self.ExitType = Some(
                    <ExitVehicleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => self.ClientPredicted = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePlayerData {
    pub EntityId: i64,
    pub Seat: i32,
}

impl PbMessage for VehiclePlayerData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
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
            2 => self.Seat = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeVehicleRideSharingRequest {
    pub RoleId: i32,
    pub Seat: i32,
}

impl ChangeVehicleRideSharingRequest {
    pub const MSG_ID: u16 = 22200;
}

impl PbMessage for ChangeVehicleRideSharingRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
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
            7 => self.Seat = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeVehicleRideSharingResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ChangeVehicleRideSharingResponse {
    pub const MSG_ID: u16 = 21531;
}

impl PbMessage for ChangeVehicleRideSharingResponse {
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
pub struct RemoveRideSharingPassengerRequest {
    pub RoleId: i32,
}

impl RemoveRideSharingPassengerRequest {
    pub const MSG_ID: u16 = 26143;
}

impl PbMessage for RemoveRideSharingPassengerRequest {
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
pub struct RemoveRideSharingPassengerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RemoveRideSharingPassengerResponse {
    pub const MSG_ID: u16 = 20287;
}

impl PbMessage for RemoveRideSharingPassengerResponse {
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
pub struct UpdateVehicleRideSharingNotify {
    pub PlayerId: i32,
    pub RoleId: i32,
    pub Seat: i32,
    pub EntityId: i64,
}

impl UpdateVehicleRideSharingNotify {
    pub const MSG_ID: u16 = 20875;
}

impl PbMessage for UpdateVehicleRideSharingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
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
            6 => self.PlayerId = r.read_varint_i32()?,
            10 => self.RoleId = r.read_varint_i32()?,
            14 => self.Seat = r.read_varint_i32()?,
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendMovieModeRideSharingRequest {
    pub IsInMovieRideSharingMode: bool,
    pub ShareRideMode: Option<RideMode>,
}

impl SendMovieModeRideSharingRequest {
    pub const MSG_ID: u16 = 25279;
}

impl PbMessage for SendMovieModeRideSharingRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsInMovieRideSharingMode {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ShareRideMode {
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
            4 => self.IsInMovieRideSharingMode = r.read_bool()?,
            9 => {
                self.ShareRideMode = Some(<RideMode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendMovieModeRideSharingResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SendMovieModeRideSharingResponse {
    pub const MSG_ID: u16 = 25137;
}

impl PbMessage for SendMovieModeRideSharingResponse {
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
pub struct VehicleShareNotify {
    pub PlayerId: i32,
    pub ShareRideMode: Option<RideMode>,
    pub IsInMovieRideSharingMode: bool,
    pub Reason: Option<VehicleShareReason>,
}

impl VehicleShareNotify {
    pub const MSG_ID: u16 = 17830;
}

impl PbMessage for VehicleShareNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.ShareRideMode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsInMovieRideSharingMode {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            11 => self.PlayerId = r.read_varint_i32()?,
            7 => {
                self.ShareRideMode = Some(<RideMode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.IsInMovieRideSharingMode = r.read_bool()?,
            14 => {
                self.Reason = Some(
                    <VehicleShareReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PatrolInfoComponentPb {
    pub SceneAiEnabled: bool,
    pub PatrolInfo: Option<PatrolInfoPb>,
}

impl PbMessage for PatrolInfoComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneAiEnabled {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.PatrolInfo {
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
            1 => self.SceneAiEnabled = r.read_bool()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PatrolInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatrolInfoPbDataOneof {
    SmartObjectComponent(SmartObjectComponent),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PatrolInfoPb {
    pub Data: Option<PatrolInfoPbDataOneof>,
}

impl PbMessage for PatrolInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                PatrolInfoPbDataOneof::SmartObjectComponent(v) => {
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
                let val = <SmartObjectComponent as PbMessage>::decode(bytes)?;
                self.Data = Some(PatrolInfoPbDataOneof::SmartObjectComponent(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SmartObjectComponent {
    pub LastPassIndex: i32,
}

impl PbMessage for SmartObjectComponent {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LastPassIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastPassIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LastPassIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimalPerformComponentPb {
    pub AnimalInitialPartIds: Vec<i32>,
}

impl PbMessage for AnimalPerformComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AnimalInitialPartIds {
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
                &mut self.AnimalInitialPartIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NpcDriveVehicleComponentPb {
    pub VehicleCreatureId: i64,
    pub Seat: i32,
}

impl PbMessage for NpcDriveVehicleComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VehicleCreatureId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VehicleCreatureId);
        }
        if self.Seat != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Seat);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.VehicleCreatureId = r.read_varint_i64()?,
            2 => self.Seat = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GrapplingHookPointComponentPb {
    pub HookLockPointDisabled: bool,
}

impl PbMessage for GrapplingHookPointComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HookLockPointDisabled {
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
            1 => self.HookLockPointDisabled = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveToPointComponentPb {
    pub PbMoveToPointConfig: Option<PbMoveToPointConfig>,
}

impl PbMessage for MoveToPointComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PbMoveToPointConfig {
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
                self.PbMoveToPointConfig
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbMoveToPointConfig {
    pub TargetPos: Option<Vector>,
    pub MoveType: i32,
    pub IsExact: bool,
}

impl PbMessage for PbMoveToPointConfig {
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

#[derive(Debug, Clone, PartialEq)]
pub enum EntityMoveSplineComponentPbRuntimeDataOneof {
    SceneItemSplineRuntimeData(SceneItemSplineRuntimeData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityMoveSplineComponentPb {
    pub RuntimeData: Option<EntityMoveSplineComponentPbRuntimeDataOneof>,
    pub SplineEntityId: i32,
    pub MoveSplineConfig: Option<MoveSplineConfig>,
}

impl PbMessage for EntityMoveSplineComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.RuntimeData {
            match oneof {
                EntityMoveSplineComponentPbRuntimeDataOneof::SceneItemSplineRuntimeData(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
            }
        }
        if self.SplineEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityId);
        }
        if let Some(v) = &self.MoveSplineConfig {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <SceneItemSplineRuntimeData as PbMessage>::decode(bytes)?;
                self.RuntimeData = Some(
                    EntityMoveSplineComponentPbRuntimeDataOneof::SceneItemSplineRuntimeData(val),
                );
            }
            1 => self.SplineEntityId = r.read_varint_i32()?,
            2 => {
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

#[derive(Debug, Clone, PartialEq)]
pub enum MoveSplineConfigStartPointOneof {
    StartPointIndex(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveSplineConfigEndPointOneof {
    EndPointIndex(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveSplineConfigLookDirOneof {
    IsLookDir(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveSplineConfigCycleOneof {
    CycleCount(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveSplineConfigCircleOneof {
    IsCircle(bool),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MoveSplineConfig {
    pub StartPoint: Option<MoveSplineConfigStartPointOneof>,
    pub EndPoint: Option<MoveSplineConfigEndPointOneof>,
    pub LookDir: Option<MoveSplineConfigLookDirOneof>,
    pub Cycle: Option<MoveSplineConfigCycleOneof>,
    pub Circle: Option<MoveSplineConfigCircleOneof>,
}

impl PbMessage for MoveSplineConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.StartPoint {
            match oneof {
                MoveSplineConfigStartPointOneof::StartPointIndex(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.EndPoint {
            match oneof {
                MoveSplineConfigEndPointOneof::EndPointIndex(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.LookDir {
            match oneof {
                MoveSplineConfigLookDirOneof::IsLookDir(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
            }
        }
        if let Some(oneof) = &self.Cycle {
            match oneof {
                MoveSplineConfigCycleOneof::CycleCount(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.Circle {
            match oneof {
                MoveSplineConfigCircleOneof::IsCircle(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
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
                self.StartPoint = Some(MoveSplineConfigStartPointOneof::StartPointIndex(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.EndPoint = Some(MoveSplineConfigEndPointOneof::EndPointIndex(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.LookDir = Some(MoveSplineConfigLookDirOneof::IsLookDir(r.read_bool()?)),
            4 => self.Cycle = Some(MoveSplineConfigCycleOneof::CycleCount(r.read_varint_i32()?)),
            5 => self.Circle = Some(MoveSplineConfigCircleOneof::IsCircle(r.read_bool()?)),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItemSplineRuntimeDataDistanceOneof {
    DistanceAlongPath(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItemSplineRuntimeDataRotOneof {
    CurRot(Rotator),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemSplineRuntimeData {
    pub Distance: Option<SceneItemSplineRuntimeDataDistanceOneof>,
    pub Rot: Option<SceneItemSplineRuntimeDataRotOneof>,
    pub CurPos: Option<Vector>,
}

impl PbMessage for SceneItemSplineRuntimeData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Distance {
            match oneof {
                SceneItemSplineRuntimeDataDistanceOneof::DistanceAlongPath(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.Rot {
            match oneof {
                SceneItemSplineRuntimeDataRotOneof::CurRot(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if let Some(v) = &self.CurPos {
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
                self.Distance = Some(SceneItemSplineRuntimeDataDistanceOneof::DistanceAlongPath(
                    r.read_f32()?,
                ))
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <Rotator as PbMessage>::decode(bytes)?;
                self.Rot = Some(SceneItemSplineRuntimeDataRotOneof::CurRot(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CurPos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TemplateEntitySpawnerComponentPb {
    pub SpawnerType: Option<TemplateSpawnerType>,
    pub CreateEntityInfos: Vec<SpawnerEntityInfo>,
}

impl PbMessage for TemplateEntitySpawnerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SpawnerType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.CreateEntityInfos {
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
                self.SpawnerType = Some(
                    <TemplateSpawnerType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
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
pub struct GroupTypesWrapper {
    pub GroupTypes: Vec<i32>,
}

impl PbMessage for GroupTypesWrapper {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GroupTypes {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GroupTypes, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpawnerEntityInfoGroupOneof {
    GroupTypes(GroupTypesWrapper),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpawnerEntityInfoSpawnerSubTypeOneof {
    MatrixInfo(MatrixInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpawnerEntityInfo {
    pub Group: Option<SpawnerEntityInfoGroupOneof>,
    pub SpawnerSubType: Option<SpawnerEntityInfoSpawnerSubTypeOneof>,
    pub IncId: i64,
}

impl PbMessage for SpawnerEntityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Group {
            match oneof {
                SpawnerEntityInfoGroupOneof::GroupTypes(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
            }
        }
        if let Some(oneof) = &self.SpawnerSubType {
            match oneof {
                SpawnerEntityInfoSpawnerSubTypeOneof::MatrixInfo(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.IncId);
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
                let val = <GroupTypesWrapper as PbMessage>::decode(bytes)?;
                self.Group = Some(SpawnerEntityInfoGroupOneof::GroupTypes(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <MatrixInfo as PbMessage>::decode(bytes)?;
                self.SpawnerSubType = Some(SpawnerEntityInfoSpawnerSubTypeOneof::MatrixInfo(val));
            }
            1 => self.IncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatrixInfo {
    pub X: i32,
    pub Y: i32,
}

impl PbMessage for MatrixInfo {
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
pub struct GridObjectComponentPb {
    pub InitGridPlacementPbInfo: Option<GridPlacementPbInfo>,
}

impl PbMessage for GridObjectComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InitGridPlacementPbInfo {
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
                self.InitGridPlacementPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GridPlacementPbInfoGridPbOneof {
    Direction(GridPbDirection),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridPlacementPbInfo {
    pub GridPb: Option<GridPlacementPbInfoGridPbOneof>,
    pub ActorGuide: Vec<u8>,
    pub X: i32,
    pub Y: i32,
}

impl PbMessage for GridPlacementPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.GridPb {
            match oneof {
                GridPlacementPbInfoGridPbOneof::Direction(v) => {
                    crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
                }
            }
        }
        if !self.ActorGuide.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ActorGuide);
        }
        if self.X != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.X);
        }
        if self.Y != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            4 => {
                self.GridPb = Some(GridPlacementPbInfoGridPbOneof::Direction(
                    <GridPbDirection as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                ))
            }
            1 => self.ActorGuide = r.read_len_delimited()?.to_vec(),
            2 => self.X = r.read_varint_i32()?,
            3 => self.Y = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleCombatComponentPbSplineConfigOneof {
    SplineConfigId(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleCombatComponentPbSplineMoveOneof {
    SplineMoveType(SimpleCombatSplineMovePbType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatComponentPb {
    pub SplineConfig: Option<SimpleCombatComponentPbSplineConfigOneof>,
    pub SplineMove: Option<SimpleCombatComponentPbSplineMoveOneof>,
    pub SubTypeId: i32,
    pub BuffLayers: BTreeMap<i32, i32>,
    pub SimpleCombatEntityAttributePbInfo: BTreeMap<i32, i32>,
    pub LockedAttributeMap: BTreeMap<i32, i32>,
}

impl PbMessage for SimpleCombatComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.SplineConfig {
            match oneof {
                SimpleCombatComponentPbSplineConfigOneof::SplineConfigId(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.SplineMove {
            match oneof {
                SimpleCombatComponentPbSplineMoveOneof::SplineMoveType(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
            }
        }
        if self.SubTypeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubTypeId);
        }
        if !self.BuffLayers.is_empty() {
            crate::wire::write_map(out, 4, &self.BuffLayers);
        }
        if !self.SimpleCombatEntityAttributePbInfo.is_empty() {
            crate::wire::write_map(out, 5, &self.SimpleCombatEntityAttributePbInfo);
        }
        if !self.LockedAttributeMap.is_empty() {
            crate::wire::write_map(out, 6, &self.LockedAttributeMap);
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
                self.SplineConfig = Some(SimpleCombatComponentPbSplineConfigOneof::SplineConfigId(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <SimpleCombatSplineMovePbType as PbMessage>::decode(bytes)?;
                self.SplineMove = Some(SimpleCombatComponentPbSplineMoveOneof::SplineMoveType(val));
            }
            1 => self.SubTypeId = r.read_varint_i32()?,
            4 => crate::wire::decode_map_entry(r, &mut self.BuffLayers)?,
            5 => crate::wire::decode_map_entry(r, &mut self.SimpleCombatEntityAttributePbInfo)?,
            6 => crate::wire::decode_map_entry(r, &mut self.LockedAttributeMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCombatSplineMovePbType {
    pub ConfigId: i32,
}

impl PbMessage for SimpleCombatSplineMovePbType {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrapDefenseComponentPbDataOneof {
    BuildingPbData(TrapDefenseBuildingPbData),
    AuxiliaryPbData(TrapDefenseAuxiliaryPbData),
    MonsterPbData(TrapDefenseMonsterPbData),
    GoldenCointPbData(TrapDefenseGoldenCoinPbData),
    SpecialCellPbdata(TrapDefenseSpecialCellPbData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseComponentPb {
    pub Data: Option<TrapDefenseComponentPbDataOneof>,
}

impl PbMessage for TrapDefenseComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                TrapDefenseComponentPbDataOneof::BuildingPbData(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                TrapDefenseComponentPbDataOneof::AuxiliaryPbData(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                TrapDefenseComponentPbDataOneof::MonsterPbData(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                TrapDefenseComponentPbDataOneof::GoldenCointPbData(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                TrapDefenseComponentPbDataOneof::SpecialCellPbdata(v) => {
                    crate::wire::write_submessage(out, 5, v);
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
                let val = <TrapDefenseBuildingPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(TrapDefenseComponentPbDataOneof::BuildingPbData(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseAuxiliaryPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(TrapDefenseComponentPbDataOneof::AuxiliaryPbData(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseMonsterPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(TrapDefenseComponentPbDataOneof::MonsterPbData(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseGoldenCoinPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(TrapDefenseComponentPbDataOneof::GoldenCointPbData(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <TrapDefenseSpecialCellPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(TrapDefenseComponentPbDataOneof::SpecialCellPbdata(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseBuildingPbData {
    pub ConfigId: i32,
    pub battleLevel: i32,
    pub ConstructCost: i32,
    pub DeconstructReturn: i32,
}

impl PbMessage for TrapDefenseBuildingPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.battleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.battleLevel);
        }
        if self.ConstructCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConstructCost);
        }
        if self.DeconstructReturn != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.battleLevel = r.read_varint_i32()?,
            3 => self.ConstructCost = r.read_varint_i32()?,
            4 => self.DeconstructReturn = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseAuxiliaryPbData {
    pub ConfigId: Vec<i32>,
}

impl PbMessage for TrapDefenseAuxiliaryPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConfigId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ConfigId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseMonsterPbData {
    pub ConfigId: i32,
}

impl PbMessage for TrapDefenseMonsterPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseGoldenCoinPbData {
    pub ConfigId: i32,
}

impl PbMessage for TrapDefenseGoldenCoinPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrapDefenseSpecialCellPbData {
    pub ConfigId: i32,
}

impl PbMessage for TrapDefenseSpecialCellPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HoldHandComponentPb {
    pub TargetEntityId: i64,
    pub HandType: i32,
    pub IsFollow: bool,
    pub ActionType: i32,
}

impl PbMessage for HoldHandComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.HandType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandType);
        }
        if self.IsFollow {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActionType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetEntityId = r.read_varint_i64()?,
            2 => self.HandType = r.read_varint_i32()?,
            3 => self.IsFollow = r.read_bool()?,
            4 => self.ActionType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActivityComponentPbDataOneof {
    SurvivorsMonsterPbData(SurvivorsMonsterPbData),
    SurvivorsWeaponPbData(SurvivorsWeaponPbData),
    SurvivorsPlayerCharacterPbData(SurvivorsPlayerCharacterPbData),
    SurvivorsGoldenCoinPbData(SurvivorsGoldenCoinPbData),
    PinballKSCRolePbData(PinballKSCRolePbData),
    KurotatoCharacterEntityPbData(KurotatoCharacterEntityPbData),
    KurotatoDropEntityPbData(KurotatoDropEntityPbData),
    KurotatoWeaponEntityPbData(KurotatoWeaponEntityPbData),
    KurotatoMonsterEntityPbData(KurotatoMonsterEntityPbData),
    KurotatoStructureEntityPbData(KurotatoStructureEntityPbData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivityComponentPb {
    pub Data: Option<ActivityComponentPbDataOneof>,
    pub ConfigId: i32,
}

impl PbMessage for ActivityComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Data {
            match oneof {
                ActivityComponentPbDataOneof::SurvivorsMonsterPbData(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                ActivityComponentPbDataOneof::SurvivorsWeaponPbData(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                ActivityComponentPbDataOneof::SurvivorsPlayerCharacterPbData(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                ActivityComponentPbDataOneof::SurvivorsGoldenCoinPbData(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                ActivityComponentPbDataOneof::PinballKSCRolePbData(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                ActivityComponentPbDataOneof::KurotatoCharacterEntityPbData(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                ActivityComponentPbDataOneof::KurotatoDropEntityPbData(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                ActivityComponentPbDataOneof::KurotatoWeaponEntityPbData(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                ActivityComponentPbDataOneof::KurotatoMonsterEntityPbData(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                ActivityComponentPbDataOneof::KurotatoStructureEntityPbData(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
            }
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsMonsterPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::SurvivorsMonsterPbData(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsWeaponPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::SurvivorsWeaponPbData(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsPlayerCharacterPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::SurvivorsPlayerCharacterPbData(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <SurvivorsGoldenCoinPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::SurvivorsGoldenCoinPbData(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <PinballKSCRolePbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::PinballKSCRolePbData(val));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoCharacterEntityPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::KurotatoCharacterEntityPbData(
                    val,
                ));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoDropEntityPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::KurotatoDropEntityPbData(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoWeaponEntityPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::KurotatoWeaponEntityPbData(
                    val,
                ));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoMonsterEntityPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::KurotatoMonsterEntityPbData(
                    val,
                ));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <KurotatoStructureEntityPbData as PbMessage>::decode(bytes)?;
                self.Data = Some(ActivityComponentPbDataOneof::KurotatoStructureEntityPbData(
                    val,
                ));
            }
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsMonsterPbData {
    pub SpawnPointEntityId: i32,
}

impl PbMessage for SurvivorsMonsterPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SpawnPointEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpawnPointEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SpawnPointEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SurvivorsWeaponPbData {}

impl PbMessage for SurvivorsWeaponPbData {
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
pub struct SurvivorsPlayerCharacterPbData {}

impl PbMessage for SurvivorsPlayerCharacterPbData {
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
pub struct SurvivorsGoldenCoinPbData {}

impl PbMessage for SurvivorsGoldenCoinPbData {
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
pub enum KurotatoMonsterEntityPbDataJl0Oneof {
    SpawnConfigId(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KurotatoMonsterEntityPbDataTv0Oneof {
    SpawnConfigGroupIndex(i32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoMonsterEntityPbData {
    pub jl0: Option<KurotatoMonsterEntityPbDataJl0Oneof>,
    pub Tv0: Option<KurotatoMonsterEntityPbDataTv0Oneof>,
}

impl PbMessage for KurotatoMonsterEntityPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.jl0 {
            match oneof {
                KurotatoMonsterEntityPbDataJl0Oneof::SpawnConfigId(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.Tv0 {
            match oneof {
                KurotatoMonsterEntityPbDataTv0Oneof::SpawnConfigGroupIndex(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
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
                self.jl0 = Some(KurotatoMonsterEntityPbDataJl0Oneof::SpawnConfigId(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.Tv0 = Some(KurotatoMonsterEntityPbDataTv0Oneof::SpawnConfigGroupIndex(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoWeaponEntityPbData {
    pub IncId: i32,
}

impl PbMessage for KurotatoWeaponEntityPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoCharacterEntityPbData {}

impl PbMessage for KurotatoCharacterEntityPbData {
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
pub struct KurotatoDropEntityPbData {}

impl PbMessage for KurotatoDropEntityPbData {
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
pub struct KurotatoStructureEntityPbData {}

impl PbMessage for KurotatoStructureEntityPbData {
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
pub enum GpuEntityComponentPbDataOneof {
    GpuMonsterEntityPbData(GpuMonsterEntityPbData),
    GpuRolePbEntityData(GpuRoleEntityPbData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GpuEntityComponentPb {
    pub data: Option<GpuEntityComponentPbDataOneof>,
    pub ConfigId: i32,
}

impl PbMessage for GpuEntityComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.data {
            match oneof {
                GpuEntityComponentPbDataOneof::GpuMonsterEntityPbData(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                GpuEntityComponentPbDataOneof::GpuRolePbEntityData(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
            }
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <GpuMonsterEntityPbData as PbMessage>::decode(bytes)?;
                self.data = Some(GpuEntityComponentPbDataOneof::GpuMonsterEntityPbData(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <GpuRoleEntityPbData as PbMessage>::decode(bytes)?;
                self.data = Some(GpuEntityComponentPbDataOneof::GpuRolePbEntityData(val));
            }
            1 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GpuMonsterEntityPbData {}

impl PbMessage for GpuMonsterEntityPbData {
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
pub struct GpuRoleEntityPbData {
    pub IncId: i64,
}

impl PbMessage for GpuRoleEntityPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.IncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashSkinComponentPb {
    pub CalabashSkinId: i32,
}

impl PbMessage for CalabashSkinComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CalabashSkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CalabashSkinId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CalabashSkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityCalabashSkinChangeNotify {
    pub EntityId: i64,
    pub CalabashSkinCoponent: Option<CalabashSkinComponentPb>,
}

impl EntityCalabashSkinChangeNotify {
    pub const MSG_ID: u16 = 25793;
}

impl PbMessage for EntityCalabashSkinChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.CalabashSkinCoponent {
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
            6 => self.EntityId = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.CalabashSkinCoponent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryDropItemComponentPb {
    pub Item: Option<HonamiStoryItemInfo>,
}

impl PbMessage for HonamiStoryDropItemComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Item {
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
                self.Item
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryEnhanceLevelComponentPb {
    pub Level: i32,
}

impl PbMessage for HonamiStoryEnhanceLevelComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbGridPosition {
    pub X: i32,
    pub Y: i32,
}

impl PbMessage for RbGridPosition {
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

#[derive(Debug, Clone, PartialEq)]
pub enum RbBlockComponentPbTypeOneof {
    DefaultBlockType(RbDefaultBlockPbType),
    VisionBlockType(RbVisionBlockPbType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockComponentPb {
    pub Type: Option<RbBlockComponentPbTypeOneof>,
    pub CenterPosition: Option<Vector>,
    pub SizeX: i32,
    pub SizeY: i32,
    pub SizeZ: i32,
    pub Forward: Option<Vector>,
    pub Right: Option<Vector>,
    pub State: Option<RbBlockPbState>,
    pub GamePlayIncId: i32,
    pub OccupiedCellPositions: Vec<RbGridPosition>,
}

impl PbMessage for RbBlockComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Type {
            match oneof {
                RbBlockComponentPbTypeOneof::DefaultBlockType(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                RbBlockComponentPbTypeOneof::VisionBlockType(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
            }
        }
        if let Some(v) = &self.CenterPosition {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.SizeX != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SizeX);
        }
        if self.SizeY != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SizeY);
        }
        if self.SizeZ != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SizeZ);
        }
        if let Some(v) = &self.Forward {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Right {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.State {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.GamePlayIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayIncId);
        }
        for item in &self.OccupiedCellPositions {
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
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbDefaultBlockPbType as PbMessage>::decode(bytes)?;
                self.Type = Some(RbBlockComponentPbTypeOneof::DefaultBlockType(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbVisionBlockPbType as PbMessage>::decode(bytes)?;
                self.Type = Some(RbBlockComponentPbTypeOneof::VisionBlockType(val));
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CenterPosition
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.SizeX = r.read_varint_i32()?,
            3 => self.SizeY = r.read_varint_i32()?,
            4 => self.SizeZ = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Forward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Right
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.State
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.GamePlayIncId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.OccupiedCellPositions
                    .push(<RbGridPosition as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbDefaultBlockPbType {
    pub IsMainControl: bool,
}

impl PbMessage for RbDefaultBlockPbType {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsMainControl {
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
            1 => self.IsMainControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbVisionBlockPbType {}

impl PbMessage for RbVisionBlockPbType {
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
pub struct RbFloorComponentPb {
    pub GamePlayIncId: i32,
    pub Type: i32,
    pub OccupiedCellPositions: Vec<RbGridPosition>,
}

impl PbMessage for RbFloorComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamePlayIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayIncId);
        }
        if self.Type != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        for item in &self.OccupiedCellPositions {
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
            1 => self.GamePlayIncId = r.read_varint_i32()?,
            2 => self.Type = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.OccupiedCellPositions
                    .push(<RbGridPosition as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RbItemComponentPbTypeOneof {
    BreakableObstacleType(RbBreakableObstaclePbType),
    RbLaserEmitterType(RbLaserEmitterPbType),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbItemComponentPb {
    pub Type: Option<RbItemComponentPbTypeOneof>,
    pub GamePlayIncId: i32,
    pub OccupiedCellPositions: Vec<RbGridPosition>,
}

impl PbMessage for RbItemComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Type {
            match oneof {
                RbItemComponentPbTypeOneof::BreakableObstacleType(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                RbItemComponentPbTypeOneof::RbLaserEmitterType(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
            }
        }
        if self.GamePlayIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayIncId);
        }
        for item in &self.OccupiedCellPositions {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbBreakableObstaclePbType as PbMessage>::decode(bytes)?;
                self.Type = Some(RbItemComponentPbTypeOneof::BreakableObstacleType(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbLaserEmitterPbType as PbMessage>::decode(bytes)?;
                self.Type = Some(RbItemComponentPbTypeOneof::RbLaserEmitterType(val));
            }
            1 => self.GamePlayIncId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.OccupiedCellPositions
                    .push(<RbGridPosition as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBreakableObstaclePbType {
    pub LinkPoints: Vec<Vector>,
}

impl PbMessage for RbBreakableObstaclePbType {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LinkPoints {
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
                self.LinkPoints.push(<Vector as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbLaserEmitterPbType {
    pub LaserPoints: Vec<Vector>,
}

impl PbMessage for RbLaserEmitterPbType {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LaserPoints {
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
                self.LaserPoints.push(<Vector as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritTakeUpPb {
    pub TrapEntityConfigId: i32,
    pub Index: i32,
}

impl PbMessage for SunSpiritTakeUpPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrapEntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrapEntityConfigId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.TrapEntityConfigId = r.read_varint_i32()?,
            2 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritPb {
    pub InstId: i32,
    pub EntityConfigId: i32,
    pub TakeUpData: Option<SunSpiritTakeUpPb>,
}

impl PbMessage for SunSpiritPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if let Some(v) = &self.TakeUpData {
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.EntityConfigId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TakeUpData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SunSpiritGearComponentPb {
    pub TakeUpInfo: Vec<SunSpiritPb>,
}

impl PbMessage for SunSpiritGearComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TakeUpInfo {
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
                self.TakeUpInfo
                    .push(<SunSpiritPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VehiclePb {
    pub Source: Option<VehicleSource>,
}

impl PbMessage for VehiclePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Source {
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
                self.Source = Some(<VehicleSource as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadNavMoveData {
    pub DestRoadId: i32,
    pub DestIndex: i32,
    pub GenRoadId: i32,
    pub GenRoadIndex: i32,
}

impl PbMessage for RoadNavMoveData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DestRoadId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestRoadId);
        }
        if self.DestIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestIndex);
        }
        if self.GenRoadId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GenRoadId);
        }
        if self.GenRoadIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GenRoadIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DestRoadId = r.read_varint_i32()?,
            2 => self.DestIndex = r.read_varint_i32()?,
            3 => self.GenRoadId = r.read_varint_i32()?,
            4 => self.GenRoadIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RoadNetworkComponentPbMoveDataOneof {
    NavMoveData(RoadNavMoveData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoadNetworkComponentPb {
    pub MoveData: Option<RoadNetworkComponentPbMoveDataOneof>,
    pub DestRoadId: i32,
    pub DestIndex: i32,
    pub GenRoadId: i32,
    pub GenRoadIndex: i32,
}

impl PbMessage for RoadNetworkComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.MoveData {
            match oneof {
                RoadNetworkComponentPbMoveDataOneof::NavMoveData(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
            }
        }
        if self.DestRoadId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestRoadId);
        }
        if self.DestIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DestIndex);
        }
        if self.GenRoadId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GenRoadId);
        }
        if self.GenRoadIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GenRoadIndex);
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
                let val = <RoadNavMoveData as PbMessage>::decode(bytes)?;
                self.MoveData = Some(RoadNetworkComponentPbMoveDataOneof::NavMoveData(val));
            }
            1 => self.DestRoadId = r.read_varint_i32()?,
            2 => self.DestIndex = r.read_varint_i32()?,
            3 => self.GenRoadId = r.read_varint_i32()?,
            4 => self.GenRoadIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExhibitionComponentPb {
    pub ItemId: i32,
}

impl PbMessage for ExhibitionComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FurnitureComponentPb {
    pub SlotId: i32,
    pub FurnitureId: i32,
}

impl PbMessage for FurnitureComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.FurnitureId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FurnitureId);
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
            2 => self.FurnitureId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballKSCRolePbData {}

impl PbMessage for PinballKSCRolePbData {
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
pub struct ElementalHolderComponentPb {
    pub Enable: bool,
}

impl PbMessage for ElementalHolderComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Enable {
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
            1 => self.Enable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityFoundationComponentPbEgMOneof {
    FoundationCoreInfo(FoundationCoreInfo),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFoundationComponentPb {
    pub egM: Option<EntityFoundationComponentPbEgMOneof>,
}

impl PbMessage for EntityFoundationComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.egM {
            match oneof {
                EntityFoundationComponentPbEgMOneof::FoundationCoreInfo(v) => {
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
                let val = <FoundationCoreInfo as PbMessage>::decode(bytes)?;
                self.egM = Some(EntityFoundationComponentPbEgMOneof::FoundationCoreInfo(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoundationCoreInfo {
    pub SourceEntityId: i32,
    pub ElementalBoxId: i32,
    pub State: Vec<u8>,
    pub ElementalType: Vec<u8>,
    pub Priority: i32,
    pub EntityId: i32,
}

impl PbMessage for FoundationCoreInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceEntityId);
        }
        if self.ElementalBoxId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementalBoxId);
        }
        if !self.State.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.State);
        }
        if !self.ElementalType.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ElementalType);
        }
        if self.Priority != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Priority);
        }
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
            1 => self.SourceEntityId = r.read_varint_i32()?,
            2 => self.ElementalBoxId = r.read_varint_i32()?,
            3 => self.State = r.read_len_delimited()?.to_vec(),
            4 => self.ElementalType = r.read_len_delimited()?.to_vec(),
            5 => self.Priority = r.read_varint_i32()?,
            6 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientEntityInfoPbDOLLARFyOneof {
    IncId(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientEntityInfoPbWFyOneof {
    ServerRealExist(bool),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClientEntityInfoPb {
    pub DOLLARFy: Option<ClientEntityInfoPbDOLLARFyOneof>,
    pub WFy: Option<ClientEntityInfoPbWFyOneof>,
    pub EntityConfigId: i32,
    pub IsClientEntityConfig: bool,
}

impl PbMessage for ClientEntityInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.DOLLARFy {
            match oneof {
                ClientEntityInfoPbDOLLARFyOneof::IncId(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
            }
        }
        if let Some(oneof) = &self.WFy {
            match oneof {
                ClientEntityInfoPbWFyOneof::ServerRealExist(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
            }
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.IsClientEntityConfig {
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
            2 => self.DOLLARFy = Some(ClientEntityInfoPbDOLLARFyOneof::IncId(r.read_varint_i64()?)),
            3 => self.WFy = Some(ClientEntityInfoPbWFyOneof::ServerRealExist(r.read_bool()?)),
            1 => self.EntityConfigId = r.read_varint_i32()?,
            4 => self.IsClientEntityConfig = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplySceneClientEntityIncIdRequest {
    pub EntityConfigId: Vec<i32>,
}

impl ApplySceneClientEntityIncIdRequest {
    pub const MSG_ID: u16 = 24599;
}

impl PbMessage for ApplySceneClientEntityIncIdRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityConfigId {
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
            8 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityConfigId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplySceneClientEntityIncIdResponse {
    pub ErrorCode: i32,
    pub InstId: i32,
    pub ClientEntityInfo: Vec<ClientEntityInfoPb>,
}

impl ApplySceneClientEntityIncIdResponse {
    pub const MSG_ID: u16 = 20448;
}

impl PbMessage for ApplySceneClientEntityIncIdResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ErrorCode != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ErrorCode);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.ClientEntityInfo {
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
            6 => self.ErrorCode = r.read_varint_i32()?,
            15 => self.InstId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ClientEntityInfo
                    .push(<ClientEntityInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SquadPropComponentPb {
    pub Attributes: Vec<GameplayAttributeData>,
}

impl PbMessage for SquadPropComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct ConnectorComponentPb {
    pub HasInitiatedConnect: bool,
}

impl PbMessage for ConnectorComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasInitiatedConnect {
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
            1 => self.HasInitiatedConnect = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionSkillInformation {
    pub SkillId: i32,
    pub Level: i32,
    pub Quality: i32,
    pub VisionEntityId: i64,
    pub Index: i32,
}

impl PbMessage for VisionSkillInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Quality != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Quality);
        }
        if self.VisionEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.VisionEntityId);
        }
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
            1 => self.SkillId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.Quality = r.read_varint_i32()?,
            4 => self.VisionEntityId = r.read_varint_i64()?,
            5 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct VisionSkillChangeNotify {
    pub EntityId: i64,
    pub VisionSkillInfos: Vec<VisionSkillInformation>,
    pub PhantomSkillInfo: Option<VisionSkillInformation>,
}

impl VisionSkillChangeNotify {
    pub const MSG_ID: u16 = 25196;
}

impl PbMessage for VisionSkillChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.VisionSkillInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.PhantomSkillInfo {
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
            11 => self.EntityId = r.read_varint_i64()?,
            1 => {
                let bytes = r.read_len_delimited()?;
                self.VisionSkillInfos
                    .push(<VisionSkillInformation as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomSkillInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartComponentPb {
    pub PartLifeInfos: Vec<PartInformation>,
}

impl PbMessage for PartComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PartLifeInfos {
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
                self.PartLifeInfos
                    .push(<PartInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartComponentInitNotify {
    pub EntityId: i64,
    pub PartComponent: Option<PartComponentPb>,
}

impl PartComponentInitNotify {
    pub const MSG_ID: u16 = 25520;
}

impl PbMessage for PartComponentInitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.PartComponent {
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
            6 => self.EntityId = r.read_varint_i64()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PartComponent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartInformation {
    pub PartIndex: i32,
    pub LifeValue: f32,
    pub LifeMax: f32,
    pub Activated: bool,
    pub PartTag: i32,
}

impl PbMessage for PartInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
        }
        if self.LifeValue != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.LifeValue).to_bits().to_le_bytes());
        }
        if self.LifeMax != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.LifeMax).to_bits().to_le_bytes());
        }
        if self.Activated {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PartTag != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartTag);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PartIndex = r.read_varint_i32()?,
            2 => self.LifeValue = r.read_f32()?,
            3 => self.LifeMax = r.read_f32()?,
            4 => self.Activated = r.read_bool()?,
            5 => self.PartTag = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartUpdateInfo {
    pub PartIndex: i32,
    pub Activated: bool,
    pub Reset: bool,
}

impl PbMessage for PartUpdateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
        }
        if self.Activated {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Reset {
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
            1 => self.PartIndex = r.read_varint_i32()?,
            2 => self.Activated = r.read_bool()?,
            3 => self.Reset = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartUpdateRequest {
    pub EntityId: i64,
    pub PartUpdateInfos: Vec<PartUpdateInfo>,
}

impl PartUpdateRequest {
    pub const MSG_ID: u16 = 22158;
}

impl PbMessage for PartUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.PartUpdateInfos {
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
                self.PartUpdateInfos
                    .push(<PartUpdateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartUpdatePush {
    pub EntityId: i64,
    pub PartUpdateInfos: Vec<PartUpdateInfo>,
}

impl PartUpdatePush {
    pub const MSG_ID: u16 = 25777;
}

impl PbMessage for PartUpdatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.PartUpdateInfos {
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
            8 => self.EntityId = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PartUpdateInfos
                    .push(<PartUpdateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PartUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PartUpdateResponse {
    pub const MSG_ID: u16 = 23891;
}

impl PbMessage for PartUpdateResponse {
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
pub struct PartUpdateNotify {
    pub EntityId: i64,
    pub PartInfos: Vec<PartInformation>,
}

impl PartUpdateNotify {
    pub const MSG_ID: u16 = 24482;
}

impl PbMessage for PartUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.PartInfos {
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
            13 => self.EntityId = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PartInfos
                    .push(<PartInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyGameplayEffectRequestTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyGameplayEffectRequest {
    pub Time: Option<ApplyGameplayEffectRequestTimeOneof>,
    pub Handle: i32,
    pub Id: i64,
    pub Level: i32,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub ServerId: i32,
    pub StackCount: i32,
    pub IsActive: bool,
}

impl ApplyGameplayEffectRequest {
    pub const MSG_ID: u16 = 26992;
}

impl PbMessage for ApplyGameplayEffectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                ApplyGameplayEffectRequestTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Handle != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.IsActive {
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
            12 => self.Time = Some(ApplyGameplayEffectRequestTimeOneof::Duration(r.read_f32()?)),
            5 => self.Handle = r.read_varint_i32()?,
            13 => self.Id = r.read_varint_i64()?,
            6 => self.Level = r.read_varint_i32()?,
            10 => self.InstigatorId = r.read_varint_i64()?,
            1 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.ServerId = r.read_varint_i32()?,
            15 => self.StackCount = r.read_varint_i32()?,
            2 => self.IsActive = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyGameplayEffectPushTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyGameplayEffectPush {
    pub Time: Option<ApplyGameplayEffectPushTimeOneof>,
    pub Handle: i32,
    pub Id: i64,
    pub Level: i32,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub ServerId: i32,
    pub StackCount: i32,
    pub IsActive: bool,
    pub Reason: Vec<u8>,
    pub ConfBuffId: i64,
}

impl ApplyGameplayEffectPush {
    pub const MSG_ID: u16 = 17445;
}

impl PbMessage for ApplyGameplayEffectPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                ApplyGameplayEffectPushTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Handle != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.IsActive {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Reason.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Reason);
        }
        if self.ConfBuffId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ConfBuffId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.Time = Some(ApplyGameplayEffectPushTimeOneof::Duration(r.read_f32()?)),
            8 => self.Handle = r.read_varint_i32()?,
            15 => self.Id = r.read_varint_i64()?,
            7 => self.Level = r.read_varint_i32()?,
            10 => self.InstigatorId = r.read_varint_i64()?,
            6 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.ServerId = r.read_varint_i32()?,
            12 => self.StackCount = r.read_varint_i32()?,
            4 => self.IsActive = r.read_bool()?,
            1 => self.Reason = r.read_len_delimited()?.to_vec(),
            14 => self.ConfBuffId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyGameplayEffectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ApplyGameplayEffectResponse {
    pub const MSG_ID: u16 = 23904;
}

impl PbMessage for ApplyGameplayEffectResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyGameplayEffectNotifyCRoundActionOneof {
    Duration(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyGameplayEffectNotifyTimeOneof {
    LeftDuration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyGameplayEffectNotify {
    pub CRoundAction: Option<ApplyGameplayEffectNotifyCRoundActionOneof>,
    pub Time: Option<ApplyGameplayEffectNotifyTimeOneof>,
    pub Handle: i32,
    pub Id: i64,
    pub Level: i32,
    pub EntityId: i64,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub IsActive: bool,
    pub ServerId: i32,
    pub StackCount: i32,
    pub ConfBuffId: i64,
}

impl ApplyGameplayEffectNotify {
    pub const MSG_ID: u16 = 26040;
}

impl PbMessage for ApplyGameplayEffectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.CRoundAction {
            match oneof {
                ApplyGameplayEffectNotifyCRoundActionOneof::Duration(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.Time {
            match oneof {
                ApplyGameplayEffectNotifyTimeOneof::LeftDuration(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Handle != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsActive {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.ConfBuffId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ConfBuffId);
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
                self.CRoundAction = Some(ApplyGameplayEffectNotifyCRoundActionOneof::Duration(
                    r.read_f32()?,
                ))
            }
            3 => {
                self.Time = Some(ApplyGameplayEffectNotifyTimeOneof::LeftDuration(
                    r.read_f32()?,
                ))
            }
            6 => self.Handle = r.read_varint_i32()?,
            13 => self.Id = r.read_varint_i64()?,
            11 => self.Level = r.read_varint_i32()?,
            14 => self.EntityId = r.read_varint_i64()?,
            9 => self.InstigatorId = r.read_varint_i64()?,
            4 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.IsActive = r.read_bool()?,
            2 => self.ServerId = r.read_varint_i32()?,
            10 => self.StackCount = r.read_varint_i32()?,
            12 => self.ConfBuffId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveGameplayEffectRequest {
    pub Handle: i32,
    pub EntityId: i64,
    pub IsPrematureRemoval: bool,
}

impl RemoveGameplayEffectRequest {
    pub const MSG_ID: u16 = 15998;
}

impl PbMessage for RemoveGameplayEffectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsPrematureRemoval {
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
            1 => self.Handle = r.read_varint_i32()?,
            6 => self.EntityId = r.read_varint_i64()?,
            14 => self.IsPrematureRemoval = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveGameplayEffectPush {
    pub Handle: i32,
    pub EntityId: i64,
    pub IsPrematureRemoval: bool,
    pub Reason: Vec<u8>,
    pub InstigatorId: i64,
}

impl RemoveGameplayEffectPush {
    pub const MSG_ID: u16 = 29542;
}

impl PbMessage for RemoveGameplayEffectPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsPrematureRemoval {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.Reason.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Reason);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Handle = r.read_varint_i32()?,
            14 => self.EntityId = r.read_varint_i64()?,
            1 => self.IsPrematureRemoval = r.read_bool()?,
            5 => self.Reason = r.read_len_delimited()?.to_vec(),
            10 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveGameplayEffectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Handle: i32,
}

impl RemoveGameplayEffectResponse {
    pub const MSG_ID: u16 = 29948;
}

impl PbMessage for RemoveGameplayEffectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Handle != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
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
            3 => self.Handle = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveGameplayEffectNotify {
    pub Handle: i32,
    pub EntityId: i64,
    pub InstigatorId: i64,
}

impl RemoveGameplayEffectNotify {
    pub const MSG_ID: u16 = 18172;
}

impl PbMessage for RemoveGameplayEffectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
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
            10 => self.EntityId = r.read_varint_i64()?,
            7 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderApplyBuffRequestTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderApplyBuffRequest {
    pub Time: Option<OrderApplyBuffRequestTimeOneof>,
    pub Id: i64,
    pub Level: i32,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub ServerId: i32,
    pub StackCount: i32,
    pub IsIterable: bool,
    pub TransferContextId: Option<TransferContextId>,
    pub Reason: Vec<u8>,
}

impl OrderApplyBuffRequest {
    pub const MSG_ID: u16 = 21297;
}

impl PbMessage for OrderApplyBuffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                OrderApplyBuffRequestTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 15, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.IsIterable {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.TransferContextId {
            crate::wire::write_submessage(out, 4, v);
        }
        if !self.Reason.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Reason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Time = Some(OrderApplyBuffRequestTimeOneof::Duration(r.read_f32()?)),
            7 => self.Id = r.read_varint_i64()?,
            3 => self.Level = r.read_varint_i32()?,
            1 => self.InstigatorId = r.read_varint_i64()?,
            13 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.ServerId = r.read_varint_i32()?,
            6 => self.StackCount = r.read_varint_i32()?,
            2 => self.IsIterable = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TransferContextId
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.Reason = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderApplyBuffResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl OrderApplyBuffResponse {
    pub const MSG_ID: u16 = 15979;
}

impl PbMessage for OrderApplyBuffResponse {
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
pub struct OrderRemoveBuffRequest {
    pub Id: i64,
    pub StackCount: i32,
    pub Reason: Vec<u8>,
    pub InstigatorId: i64,
}

impl OrderRemoveBuffRequest {
    pub const MSG_ID: u16 = 26903;
}

impl PbMessage for OrderRemoveBuffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if !self.Reason.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Reason);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
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
            8 => self.StackCount = r.read_varint_i32()?,
            6 => self.Reason = r.read_len_delimited()?.to_vec(),
            13 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderRemoveBuffResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl OrderRemoveBuffResponse {
    pub const MSG_ID: u16 = 17733;
}

impl PbMessage for OrderRemoveBuffResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum OrderApplyBuffNotifyTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderApplyBuffNotify {
    pub Time: Option<OrderApplyBuffNotifyTimeOneof>,
    pub Id: i64,
    pub Level: i32,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub ServerId: i32,
    pub StackCount: i32,
    pub IsIterable: bool,
}

impl OrderApplyBuffNotify {
    pub const MSG_ID: u16 = 19792;
}

impl PbMessage for OrderApplyBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                OrderApplyBuffNotifyTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.IsIterable {
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
            12 => self.Time = Some(OrderApplyBuffNotifyTimeOneof::Duration(r.read_f32()?)),
            3 => self.Id = r.read_varint_i64()?,
            6 => self.Level = r.read_varint_i32()?,
            14 => self.InstigatorId = r.read_varint_i64()?,
            4 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.ServerId = r.read_varint_i32()?,
            13 => self.StackCount = r.read_varint_i32()?,
            9 => self.IsIterable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderRemoveBuffNotify {
    pub Id: i64,
    pub StackCount: i32,
}

impl OrderRemoveBuffNotify {
    pub const MSG_ID: u16 = 20935;
}

impl PbMessage for OrderRemoveBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            2 => self.Id = r.read_varint_i64()?,
            15 => self.StackCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApplyBuffS2cRequestNotifyTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyBuffS2cRequestNotify {
    pub Time: Option<ApplyBuffS2cRequestNotifyTimeOneof>,
    pub Id: i64,
    pub Level: i32,
    pub InstigatorId: i64,
    pub ApplyType: Option<ApplyGEType>,
    pub ServerId: i32,
    pub StackCount: i32,
    pub IsIterable: bool,
    pub Reason: i32,
}

impl ApplyBuffS2cRequestNotify {
    pub const MSG_ID: u16 = 17614;
}

impl PbMessage for ApplyBuffS2cRequestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                ApplyBuffS2cRequestNotifyTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 9, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.ApplyType {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.IsIterable {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            9 => self.Time = Some(ApplyBuffS2cRequestNotifyTimeOneof::Duration(r.read_f32()?)),
            5 => self.Id = r.read_varint_i64()?,
            7 => self.Level = r.read_varint_i32()?,
            3 => self.InstigatorId = r.read_varint_i64()?,
            15 => {
                self.ApplyType = Some(<ApplyGEType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.ServerId = r.read_varint_i32()?,
            13 => self.StackCount = r.read_varint_i32()?,
            6 => self.IsIterable = r.read_bool()?,
            4 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApplyBuffS2cResponsePush {
    pub ErrorCode: Option<ErrorCode>,
    pub Handle: i32,
    pub IsActive: bool,
}

impl ApplyBuffS2cResponsePush {
    pub const MSG_ID: u16 = 17435;
}

impl PbMessage for ApplyBuffS2cResponsePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Handle != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.IsActive {
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
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.Handle = r.read_varint_i32()?,
            8 => self.IsActive = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveBuffS2cRequestNotify {
    pub Handle: i32,
    pub StackCount: i32,
    pub Reason: i32,
    pub InstigatorId: i64,
}

impl RemoveBuffS2cRequestNotify {
    pub const MSG_ID: u16 = 28392;
}

impl PbMessage for RemoveBuffS2cRequestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.Handle = r.read_varint_i32()?,
            1 => self.StackCount = r.read_varint_i32()?,
            11 => self.Reason = r.read_varint_i32()?,
            15 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveBuffS2cResponsePush {
    pub ErrorCode: Option<ErrorCode>,
}

impl RemoveBuffS2cResponsePush {
    pub const MSG_ID: u16 = 16300;
}

impl PbMessage for RemoveBuffS2cResponsePush {
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
pub struct RemoveBuffByIdS2cRequestNotify {
    pub BuffId: i64,
    pub StackCount: i32,
    pub Reason: i32,
    pub InstigatorId: i64,
}

impl RemoveBuffByIdS2cRequestNotify {
    pub const MSG_ID: u16 = 25284;
}

impl PbMessage for RemoveBuffByIdS2cRequestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.BuffId = r.read_varint_i64()?,
            14 => self.StackCount = r.read_varint_i32()?,
            13 => self.Reason = r.read_varint_i32()?,
            4 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveBuffByIdS2cResponsePush {
    pub ErrorCode: Option<ErrorCode>,
}

impl RemoveBuffByIdS2cResponsePush {
    pub const MSG_ID: u16 = 15939;
}

impl PbMessage for RemoveBuffByIdS2cResponsePush {
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
pub struct BroadcastAddBuffFailedNotify {
    pub BuffId: i64,
    pub StackCount: i32,
    pub InstigatorId: i64,
    pub TransferContextId: Option<TransferContextId>,
}

impl BroadcastAddBuffFailedNotify {
    pub const MSG_ID: u16 = 19969;
}

impl PbMessage for BroadcastAddBuffFailedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if let Some(v) = &self.TransferContextId {
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
            1 => self.BuffId = r.read_varint_i64()?,
            14 => self.StackCount = r.read_varint_i32()?,
            3 => self.InstigatorId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TransferContextId
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActivateBuffRequest {
    pub Handle: i32,
    pub On: bool,
}

impl ActivateBuffRequest {
    pub const MSG_ID: u16 = 27416;
}

impl PbMessage for ActivateBuffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.On {
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
            5 => self.Handle = r.read_varint_i32()?,
            4 => self.On = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ActiveBuffPush {
    pub Handle: i32,
    pub On: bool,
}

impl ActiveBuffPush {
    pub const MSG_ID: u16 = 16028;
}

impl PbMessage for ActiveBuffPush {
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
pub struct ActivateBuffResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ActivateBuffResponse {
    pub const MSG_ID: u16 = 19166;
}

impl PbMessage for ActivateBuffResponse {
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
pub struct ActivateBuffNotify {
    pub Handle: i32,
    pub On: bool,
}

impl ActivateBuffNotify {
    pub const MSG_ID: u16 = 29719;
}

impl PbMessage for ActivateBuffNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Handle != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Handle);
        }
        if self.On {
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
            7 => self.Handle = r.read_varint_i32()?,
            2 => self.On = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderRemoveBuffByTagsRequest {
    pub TagIds: Vec<i32>,
    pub InstigatorId: i64,
}

impl OrderRemoveBuffByTagsRequest {
    pub const MSG_ID: u16 = 20210;
}

impl PbMessage for OrderRemoveBuffByTagsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TagIds {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TagIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OrderRemoveBuffByTagsResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl OrderRemoveBuffByTagsResponse {
    pub const MSG_ID: u16 = 20335;
}

impl PbMessage for OrderRemoveBuffByTagsResponse {
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
pub struct OrderRemoveBuffByTagsNotify {
    pub TagIds: Vec<i32>,
    pub InstigatorId: i64,
}

impl OrderRemoveBuffByTagsNotify {
    pub const MSG_ID: u16 = 23871;
}

impl PbMessage for OrderRemoveBuffByTagsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TagIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TagIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttributeEventEffectData {
    pub TriggeredActiveHandles: Vec<i32>,
}

impl PbMessage for AttributeEventEffectData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TriggeredActiveHandles {
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
                &mut self.TriggeredActiveHandles,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FightBuffEffectContextDRoundActionOneof {
    LeftCooldown(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FightBuffEffectContextEffectOneof {
    AttributeEventEffectData(AttributeEventEffectData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightBuffEffectContext {
    pub dRoundAction: Option<FightBuffEffectContextDRoundActionOneof>,
    pub Effect: Option<FightBuffEffectContextEffectOneof>,
}

impl PbMessage for FightBuffEffectContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.dRoundAction {
            match oneof {
                FightBuffEffectContextDRoundActionOneof::LeftCooldown(v) => {
                    crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.Effect {
            match oneof {
                FightBuffEffectContextEffectOneof::AttributeEventEffectData(v) => {
                    crate::wire::write_submessage(out, 6, v);
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
                self.dRoundAction = Some(FightBuffEffectContextDRoundActionOneof::LeftCooldown(
                    r.read_f32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <AttributeEventEffectData as PbMessage>::decode(bytes)?;
                self.Effect = Some(FightBuffEffectContextEffectOneof::AttributeEventEffectData(
                    val,
                ));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffStackCountRequest {
    pub HandleId: i32,
    pub NewStackCount: i32,
    pub IsPrematureRemoval: bool,
    pub InstigatorId: i64,
}

impl BuffStackCountRequest {
    pub const MSG_ID: u16 = 24150;
}

impl PbMessage for BuffStackCountRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
        }
        if self.NewStackCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewStackCount);
        }
        if self.IsPrematureRemoval {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.HandleId = r.read_varint_i32()?,
            9 => self.NewStackCount = r.read_varint_i32()?,
            3 => self.IsPrematureRemoval = r.read_bool()?,
            15 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffStackCountPush {
    pub HandleId: i32,
    pub NewStackCount: i32,
    pub IsPrematureRemoval: bool,
    pub InstigatorId: i64,
    pub NotRefreshDuration: bool,
    pub NotRefreshPeriod: bool,
    pub Duration: f32,
    pub Reason: Vec<u8>,
}

impl BuffStackCountPush {
    pub const MSG_ID: u16 = 17747;
}

impl PbMessage for BuffStackCountPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
        }
        if self.NewStackCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewStackCount);
        }
        if self.IsPrematureRemoval {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if self.NotRefreshDuration {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NotRefreshPeriod {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Duration != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Duration).to_bits().to_le_bytes());
        }
        if !self.Reason.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Reason);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.HandleId = r.read_varint_i32()?,
            8 => self.NewStackCount = r.read_varint_i32()?,
            2 => self.IsPrematureRemoval = r.read_bool()?,
            10 => self.InstigatorId = r.read_varint_i64()?,
            3 => self.NotRefreshDuration = r.read_bool()?,
            1 => self.NotRefreshPeriod = r.read_bool()?,
            5 => self.Duration = r.read_f32()?,
            7 => self.Reason = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffStackCountResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BuffStackCountResponse {
    pub const MSG_ID: u16 = 16438;
}

impl PbMessage for BuffStackCountResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum BuffStackCountNotifyTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuffStackCountNotifyGFsOneof {
    LeftDuration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffStackCountNotify {
    pub Time: Option<BuffStackCountNotifyTimeOneof>,
    pub gFs: Option<BuffStackCountNotifyGFsOneof>,
    pub HandleId: i32,
    pub NewStackCount: i32,
    pub InstigatorId: i64,
    pub NotRefreshDuration: bool,
    pub NotRefreshPeriod: bool,
}

impl BuffStackCountNotify {
    pub const MSG_ID: u16 = 19317;
}

impl PbMessage for BuffStackCountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                BuffStackCountNotifyTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.gFs {
            match oneof {
                BuffStackCountNotifyGFsOneof::LeftDuration(v) => {
                    crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
        }
        if self.NewStackCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewStackCount);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
        if self.NotRefreshDuration {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NotRefreshPeriod {
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
            13 => self.Time = Some(BuffStackCountNotifyTimeOneof::Duration(r.read_f32()?)),
            2 => self.gFs = Some(BuffStackCountNotifyGFsOneof::LeftDuration(r.read_f32()?)),
            5 => self.HandleId = r.read_varint_i32()?,
            6 => self.NewStackCount = r.read_varint_i32()?,
            10 => self.InstigatorId = r.read_varint_i64()?,
            7 => self.NotRefreshDuration = r.read_bool()?,
            15 => self.NotRefreshPeriod = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectCd {
    pub BuffId: i64,
    pub ListCdRemaining: Vec<i32>,
}

impl PbMessage for BuffEffectCd {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffId);
        }
        for item in &self.ListCdRemaining {
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
            1 => self.BuffId = r.read_varint_i64()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ListCdRemaining,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectRequest {
    pub HandleId: i32,
    pub Index: i32,
}

impl BuffEffectRequest {
    pub const MSG_ID: u16 = 22402;
}

impl PbMessage for BuffEffectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
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
            3 => self.HandleId = r.read_varint_i32()?,
            12 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectPush {
    pub HandleId: i32,
    pub Index: i32,
}

impl BuffEffectPush {
    pub const MSG_ID: u16 = 26677;
}

impl PbMessage for BuffEffectPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
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
            4 => self.HandleId = r.read_varint_i32()?,
            14 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl BuffEffectResponse {
    pub const MSG_ID: u16 = 20053;
}

impl PbMessage for BuffEffectResponse {
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

#[derive(Debug, Clone, PartialEq)]
pub enum BuffDurationNotifyTimeOneof {
    Duration(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuffDurationNotifyGFsOneof {
    LeftDuration(f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffDurationNotify {
    pub Time: Option<BuffDurationNotifyTimeOneof>,
    pub gFs: Option<BuffDurationNotifyGFsOneof>,
    pub HandleId: i32,
}

impl BuffDurationNotify {
    pub const MSG_ID: u16 = 24676;
}

impl PbMessage for BuffDurationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Time {
            match oneof {
                BuffDurationNotifyTimeOneof::Duration(v) => {
                    crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if let Some(oneof) = &self.gFs {
            match oneof {
                BuffDurationNotifyGFsOneof::LeftDuration(v) => {
                    crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
            }
        }
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            13 => self.Time = Some(BuffDurationNotifyTimeOneof::Duration(r.read_f32()?)),
            6 => self.gFs = Some(BuffDurationNotifyGFsOneof::LeftDuration(r.read_f32()?)),
            8 => self.HandleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RefreshBuffDurationPush {
    pub BuffIds: Vec<i64>,
}

impl RefreshBuffDurationPush {
    pub const MSG_ID: u16 = 24982;
}

impl PbMessage for RefreshBuffDurationPush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BuffIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BuffIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffEffectExecutePush {
    pub HandleId: i32,
    pub Index: i32,
}

impl BuffEffectExecutePush {
    pub const MSG_ID: u16 = 18599;
}

impl PbMessage for BuffEffectExecutePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandleId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            8 => self.HandleId = r.read_varint_i32()?,
            11 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoveBuffByServerIdS2cRequestNotify {
    pub ServerId: i32,
    pub StackCount: i32,
    pub Reason: i32,
    pub InstigatorId: i64,
}

impl RemoveBuffByServerIdS2cRequestNotify {
    pub const MSG_ID: u16 = 15505;
}

impl PbMessage for RemoveBuffByServerIdS2cRequestNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ServerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ServerId);
        }
        if self.StackCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StackCount);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        if self.InstigatorId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InstigatorId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.ServerId = r.read_varint_i32()?,
            3 => self.StackCount = r.read_varint_i32()?,
            2 => self.Reason = r.read_varint_i32()?,
            7 => self.InstigatorId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateRequest {
    pub FsmId: i32,
    pub FromState: i32,
    pub ToState: i32,
}

impl ChangeStateRequest {
    pub const MSG_ID: u16 = 20524;
}

impl PbMessage for ChangeStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.FromState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromState);
        }
        if self.ToState != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.FsmId = r.read_varint_i32()?,
            2 => self.FromState = r.read_varint_i32()?,
            14 => self.ToState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateResponse {
    pub FsmId: i32,
    pub Error: Option<DErrorResult>,
    pub CurrentState: i32,
}

impl ChangeStateResponse {
    pub const MSG_ID: u16 = 28519;
}

impl PbMessage for ChangeStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if let Some(v) = &self.Error {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.CurrentState != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.FsmId = r.read_varint_i32()?,
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Error
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.CurrentState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateNotify {
    pub FsmId: i32,
    pub FromState: i32,
    pub ToState: i32,
}

impl ChangeStateNotify {
    pub const MSG_ID: u16 = 19168;
}

impl PbMessage for ChangeStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.FromState != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromState);
        }
        if self.ToState != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.FsmId = r.read_varint_i32()?,
            7 => self.FromState = r.read_varint_i32()?,
            4 => self.ToState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateConfirmRequest {
    pub FsmId: i32,
    pub State: i32,
}

impl ChangeStateConfirmRequest {
    pub const MSG_ID: u16 = 17371;
}

impl PbMessage for ChangeStateConfirmRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.FsmId = r.read_varint_i32()?,
            5 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateConfirmPush {
    pub FsmId: i32,
    pub State: i32,
}

impl ChangeStateConfirmPush {
    pub const MSG_ID: u16 = 24241;
}

impl PbMessage for ChangeStateConfirmPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.FsmId = r.read_varint_i32()?,
            5 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChangeStateConfirmResponse {
    pub FsmId: i32,
    pub State: i32,
    pub Error: Option<DErrorResult>,
}

impl ChangeStateConfirmResponse {
    pub const MSG_ID: u16 = 22660;
}

impl PbMessage for ChangeStateConfirmResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if let Some(v) = &self.Error {
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
            2 => self.FsmId = r.read_varint_i32()?,
            13 => self.State = r.read_varint_i32()?,
            12 => {
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
pub struct ChangeStateConfirmNotify {
    pub FsmId: i32,
    pub State: i32,
}

impl ChangeStateConfirmNotify {
    pub const MSG_ID: u16 = 27547;
}

impl PbMessage for ChangeStateConfirmNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.FsmId = r.read_varint_i32()?,
            11 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DFsm {
    pub FsmId: i32,
    pub CurrentState: i32,
    pub Flag: Option<EFsmStateFlag>,
    pub StateElapseTime: i32,
}

impl PbMessage for DFsm {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.CurrentState != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentState);
        }
        if let Some(v) = &self.Flag {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.StateElapseTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateElapseTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FsmId = r.read_varint_i32()?,
            2 => self.CurrentState = r.read_varint_i32()?,
            3 => {
                self.Flag = Some(<EFsmStateFlag as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.StateElapseTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFsmComponentPb {
    pub Fsms: Vec<DFsm>,
    pub HashCode: i32,
    pub CommonHashCode: i32,
    pub BlackBoard: Vec<DFsmBlackBoard>,
    pub FsmCustomBlackboardDatas: Option<FsmCustomBlackboardDatas>,
}

impl PbMessage for EntityFsmComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Fsms {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.HashCode != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HashCode);
        }
        if self.CommonHashCode != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CommonHashCode);
        }
        for item in &self.BlackBoard {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.FsmCustomBlackboardDatas {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Fsms.push(<DFsm as PbMessage>::decode(bytes)?);
            }
            2 => self.HashCode = r.read_varint_i32()?,
            3 => self.CommonHashCode = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.BlackBoard
                    .push(<DFsmBlackBoard as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FsmCustomBlackboardDatas
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmConditionPassRequest {
    pub FsmId: i32,
    pub FromState: i32,
    pub ToState: i32,
    pub ConditionIndex: i32,
    pub Value: bool,
}

impl FsmConditionPassRequest {
    pub const MSG_ID: u16 = 21831;
}

impl PbMessage for FsmConditionPassRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.FromState != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromState);
        }
        if self.ToState != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToState);
        }
        if self.ConditionIndex != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionIndex);
        }
        if self.Value {
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
            3 => self.FsmId = r.read_varint_i32()?,
            10 => self.FromState = r.read_varint_i32()?,
            15 => self.ToState = r.read_varint_i32()?,
            6 => self.ConditionIndex = r.read_varint_i32()?,
            7 => self.Value = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmConditionPassResponse {
    pub FsmId: i32,
    pub Error: Option<DErrorResult>,
}

impl FsmConditionPassResponse {
    pub const MSG_ID: u16 = 18028;
}

impl PbMessage for FsmConditionPassResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if let Some(v) = &self.Error {
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
            13 => self.FsmId = r.read_varint_i32()?,
            14 => {
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
pub struct FsmConditionPassPush {
    pub FsmId: i32,
    pub FromState: i32,
    pub ToState: i32,
    pub ConditionIndex: i32,
    pub Value: bool,
}

impl FsmConditionPassPush {
    pub const MSG_ID: u16 = 24334;
}

impl PbMessage for FsmConditionPassPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.FromState != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromState);
        }
        if self.ToState != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToState);
        }
        if self.ConditionIndex != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionIndex);
        }
        if self.Value {
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
            13 => self.FsmId = r.read_varint_i32()?,
            12 => self.FromState = r.read_varint_i32()?,
            14 => self.ToState = r.read_varint_i32()?,
            15 => self.ConditionIndex = r.read_varint_i32()?,
            2 => self.Value = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmResetNotify {
    pub EntityFsmComponentPb: Option<EntityFsmComponentPb>,
}

impl FsmResetNotify {
    pub const MSG_ID: u16 = 28366;
}

impl PbMessage for FsmResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EntityFsmComponentPb {
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
                self.EntityFsmComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmBlackboardNotify {
    pub FsmBlackBoards: Vec<DFsmBlackBoard>,
}

impl FsmBlackboardNotify {
    pub const MSG_ID: u16 = 21151;
}

impl PbMessage for FsmBlackboardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FsmBlackBoards {
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
                self.FsmBlackBoards
                    .push(<DFsmBlackBoard as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmCustomBlackboardDatas {
    pub BlackboardIntValues: Vec<DFsmBlackboardCustom>,
}

impl PbMessage for FsmCustomBlackboardDatas {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BlackboardIntValues {
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
                self.BlackboardIntValues
                    .push(<DFsmBlackboardCustom as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmCustomBlackboardNotify {
    pub FsmCustomBlackboardDatas: Option<FsmCustomBlackboardDatas>,
}

impl FsmCustomBlackboardNotify {
    pub const MSG_ID: u16 = 18543;
}

impl PbMessage for FsmCustomBlackboardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FsmCustomBlackboardDatas {
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
                self.FsmCustomBlackboardDatas
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DFsmBlackBoard {
    pub Key: i32,
    pub Value: i32,
}

impl PbMessage for DFsmBlackBoard {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Key != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Key);
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Key = r.read_varint_i32()?,
            2 => self.Value = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DFsmBlackboardCustom {
    pub Key: Vec<u8>,
    pub Value: i32,
}

impl PbMessage for DFsmBlackboardCustom {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Key.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Key);
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Key = r.read_len_delimited()?.to_vec(),
            2 => self.Value = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmStateBehaviorRequest {
    pub FsmId: i32,
    pub State: i32,
    pub Index: i32,
    pub Type: Option<FsmStateBehaviorType>,
}

impl FsmStateBehaviorRequest {
    pub const MSG_ID: u16 = 23005;
}

impl PbMessage for FsmStateBehaviorRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.Type {
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
            14 => self.FsmId = r.read_varint_i32()?,
            7 => self.State = r.read_varint_i32()?,
            1 => self.Index = r.read_varint_i32()?,
            15 => {
                self.Type = Some(
                    <FsmStateBehaviorType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FsmStateBehaviorResponse {
    pub FsmId: i32,
    pub State: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl FsmStateBehaviorResponse {
    pub const MSG_ID: u16 = 21048;
}

impl PbMessage for FsmStateBehaviorResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FsmId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FsmId);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
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
            14 => self.FsmId = r.read_varint_i32()?,
            15 => self.State = r.read_varint_i32()?,
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
pub struct FsmPlayMontageRequest {
    pub MontageName: Vec<u8>,
    pub MontagePathHash: i32,
    pub SpeedRatio: f32,
    pub StartSection: Vec<u8>,
    pub StartTimeSeconds: f32,
}

impl FsmPlayMontageRequest {
    pub const MSG_ID: u16 = 25815;
}

impl PbMessage for FsmPlayMontageRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.MontageName.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MontageName);
        }
        if self.MontagePathHash != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontagePathHash);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
        if !self.StartSection.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StartSection);
        }
        if self.StartTimeSeconds != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
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
            8 => self.MontageName = r.read_len_delimited()?.to_vec(),
            14 => self.MontagePathHash = r.read_varint_i32()?,
            3 => self.SpeedRatio = r.read_f32()?,
            12 => self.StartSection = r.read_len_delimited()?.to_vec(),
            1 => self.StartTimeSeconds = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmPlayMontagePush {
    pub MontageName: Vec<u8>,
    pub MontagePathHash: i32,
    pub SpeedRatio: f32,
    pub StartSection: Vec<u8>,
    pub StartTimeSeconds: f32,
}

impl FsmPlayMontagePush {
    pub const MSG_ID: u16 = 28981;
}

impl PbMessage for FsmPlayMontagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.MontageName.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MontageName);
        }
        if self.MontagePathHash != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontagePathHash);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
        if !self.StartSection.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StartSection);
        }
        if self.StartTimeSeconds != 0.0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Fixed32);
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
            12 => self.MontageName = r.read_len_delimited()?.to_vec(),
            2 => self.MontagePathHash = r.read_varint_i32()?,
            13 => self.SpeedRatio = r.read_f32()?,
            9 => self.StartSection = r.read_len_delimited()?.to_vec(),
            15 => self.StartTimeSeconds = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FsmPlayMontageResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FsmPlayMontageResponse {
    pub const MSG_ID: u16 = 25141;
}

impl PbMessage for FsmPlayMontageResponse {
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
pub struct FsmMontageDurationNotify {
    pub MontageHashCode: i32,
    pub DurationTime: i32,
}

impl FsmMontageDurationNotify {
    pub const MSG_ID: u16 = 24356;
}

impl PbMessage for FsmMontageDurationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MontageHashCode != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageHashCode);
        }
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
            5 => self.MontageHashCode = r.read_varint_i32()?,
            1 => self.DurationTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardGridDynamicConfig {
    pub RowIndex: i32,
    pub ColumnIndex: i32,
    pub Flags: i64,
}

impl PbMessage for BoardGridDynamicConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RowIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, self.RowIndex as i32);
        }
        if self.ColumnIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_sint32(out, self.ColumnIndex as i32);
        }
        if self.Flags != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_sint64(out, self.Flags as i64);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RowIndex = r.read_sint32()?,
            2 => self.ColumnIndex = r.read_sint32()?,
            3 => self.Flags = r.read_sint64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlacementItemPb {
    pub LocatedBoardEntityConfigId: i32,
}

impl PbMessage for PlacementItemPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LocatedBoardEntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LocatedBoardEntityConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LocatedBoardEntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoardGridPositionInfo {
    pub Row: i32,
    pub Column: i32,
    pub RotAngle: i32,
}

impl PbMessage for BoardGridPositionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Row != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Row);
        }
        if self.Column != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Column);
        }
        if self.RotAngle != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RotAngle);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Row = r.read_varint_i32()?,
            2 => self.Column = r.read_varint_i32()?,
            3 => self.RotAngle = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct OccupiedBoardGridInfo {
    pub Pos: Option<BoardGridPositionInfo>,
    pub OccupyingEntityConfigId: i32,
    pub EntityConfigType: i32,
}

impl PbMessage for OccupiedBoardGridInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.OccupyingEntityConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OccupyingEntityConfigId);
        }
        if self.EntityConfigType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigType);
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
            2 => self.OccupyingEntityConfigId = r.read_varint_i32()?,
            3 => self.EntityConfigType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JigsawBaseComponentPb {
    pub MoveCount: i32,
    pub EntityId: i32,
    pub Winner: i32,
}

impl PbMessage for JigsawBaseComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoveCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveCount);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityId);
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
            2 => self.EntityId = r.read_varint_i32()?,
            3 => self.Winner = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonerComponentPb {
    pub SummonerId: i64,
    pub SummonCfgId: i32,
    pub SummonSkillId: i32,
    pub PlayerId: i32,
    pub Type: Option<ESummonType>,
    pub AvatarCfgId: i32,
}

impl PbMessage for SummonerComponentPb {
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
                self.Type = Some(<ESummonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.AvatarCfgId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SummonsComponentPb {
    pub Version: i32,
}

impl PbMessage for SummonsComponentPb {
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
pub struct FollowerList {
    pub Type: Option<FollowerType>,
    pub EntityId: i64,
}

impl PbMessage for FollowerList {
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
                self.Type = Some(<FollowerType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FollowerComponentPb {
    pub FollowerList: Vec<FollowerList>,
}

impl PbMessage for FollowerComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FollowerList {
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
                self.FollowerList
                    .push(<FollowerList as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FollowShooterComponentPb {
    pub PlayerEntityId: i64,
    pub SummonConfigId: i32,
}

impl PbMessage for FollowShooterComponentPb {
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
pub struct CharacterAttachInfo {
    pub EntityId: i64,
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub PartIndex: i32,
}

impl PbMessage for CharacterAttachInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.PartIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PartIndex);
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.PartIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItemBlackboardParamValueOneof {
    IntValue(i32),
    IntValues(IntArrayBlackboard),
    LongValue(i64),
    LongValues(LongArrayBlackboard),
    BooleanValue(bool),
    StringValue(Vec<u8>),
    FloatValue(f32),
    FloatValues(FloatArrayBlackboard),
    VectorValue(Vector),
    RotatorValue(Rotator),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemBlackboardParam {
    pub Value: Option<SceneItemBlackboardParamValueOneof>,
    pub Key: Option<SceneItemBBKey>,
}

impl PbMessage for SceneItemBlackboardParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Value {
            match oneof {
                SceneItemBlackboardParamValueOneof::IntValue(v) => {
                    crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i32(out, *v);
                }
                SceneItemBlackboardParamValueOneof::IntValues(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                SceneItemBlackboardParamValueOneof::LongValue(v) => {
                    crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
                    crate::wire::write_varint_i64(out, *v);
                }
                SceneItemBlackboardParamValueOneof::LongValues(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                SceneItemBlackboardParamValueOneof::BooleanValue(v) => {
                    crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
                    crate::wire::write_varint_u64(out, u64::from(*v));
                }
                SceneItemBlackboardParamValueOneof::StringValue(v) => {
                    crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
                    crate::wire::write_bytes(out, v);
                }
                SceneItemBlackboardParamValueOneof::FloatValue(v) => {
                    crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed32);
                    out.extend_from_slice(&(*v).to_bits().to_le_bytes());
                }
                SceneItemBlackboardParamValueOneof::FloatValues(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                SceneItemBlackboardParamValueOneof::VectorValue(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                SceneItemBlackboardParamValueOneof::RotatorValue(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
            }
        }
        if let Some(v) = &self.Key {
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
            3 => {
                self.Value = Some(SceneItemBlackboardParamValueOneof::IntValue(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <IntArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(SceneItemBlackboardParamValueOneof::IntValues(val));
            }
            5 => {
                self.Value = Some(SceneItemBlackboardParamValueOneof::LongValue(
                    r.read_varint_i64()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <LongArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(SceneItemBlackboardParamValueOneof::LongValues(val));
            }
            7 => {
                self.Value = Some(SceneItemBlackboardParamValueOneof::BooleanValue(
                    r.read_bool()?,
                ))
            }
            8 => {
                self.Value = Some(SceneItemBlackboardParamValueOneof::StringValue(
                    r.read_len_delimited()?.to_vec(),
                ))
            }
            10 => {
                self.Value = Some(SceneItemBlackboardParamValueOneof::FloatValue(
                    r.read_f32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <FloatArrayBlackboard as PbMessage>::decode(bytes)?;
                self.Value = Some(SceneItemBlackboardParamValueOneof::FloatValues(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <Vector as PbMessage>::decode(bytes)?;
                self.Value = Some(SceneItemBlackboardParamValueOneof::VectorValue(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <Rotator as PbMessage>::decode(bytes)?;
                self.Value = Some(SceneItemBlackboardParamValueOneof::RotatorValue(val));
            }
            1 => {
                self.Key = Some(
                    <SceneItemBBKey as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct HackingComponentPb {
    pub EntityIds: Vec<i64>,
}

impl PbMessage for HackingComponentPb {
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
pub struct HackTargetComponentPb {
    pub HackTargetEntityId: i64,
}

impl PbMessage for HackTargetComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HackTargetEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HackTargetEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HackTargetEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GravityFlipComponent {
    pub Direction: Option<DirectionType>,
}

impl PbMessage for GravityFlipComponent {
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
                self.Direction = Some(<DirectionType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRewardItemPb {
    pub HasCount: i32,
    pub NextResetTime: i64,
}

impl PbMessage for EntityRewardItemPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HasCount);
        }
        if self.NextResetTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NextResetTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HasCount = r.read_varint_i32()?,
            2 => self.NextResetTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RbBlockPbStateStateOneof {
    MovingState(RbBlockMovingPbState),
    IdleState(RbBlockIdlePbState),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockPbState {
    pub State: Option<RbBlockPbStateStateOneof>,
}

impl PbMessage for RbBlockPbState {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.State {
            match oneof {
                RbBlockPbStateStateOneof::MovingState(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                RbBlockPbStateStateOneof::IdleState(v) => {
                    crate::wire::write_submessage(out, 2, v);
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
                let val = <RbBlockMovingPbState as PbMessage>::decode(bytes)?;
                self.State = Some(RbBlockPbStateStateOneof::MovingState(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbBlockIdlePbState as PbMessage>::decode(bytes)?;
                self.State = Some(RbBlockPbStateStateOneof::IdleState(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockMovingPbState {
    pub Action: Option<RbBlockMovementPbAction>,
}

impl PbMessage for RbBlockMovingPbState {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Action {
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
                self.Action
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockIdlePbState {
    pub Position: Option<Vector>,
    pub Rotation: Option<Vector>,
}

impl PbMessage for RbBlockIdlePbState {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Position {
            crate::wire::write_submessage(out, 6, v);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Position
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

#[derive(Debug, Clone, PartialEq)]
pub enum RbBlockMovementPbActionTypeOneof {
    Roll(RbRollMovement),
    Jump(RbJumpMovement),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbBlockMovementPbAction {
    pub Type: Option<RbBlockMovementPbActionTypeOneof>,
}

impl PbMessage for RbBlockMovementPbAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Type {
            match oneof {
                RbBlockMovementPbActionTypeOneof::Roll(v) => {
                    crate::wire::write_submessage(out, 1, v);
                }
                RbBlockMovementPbActionTypeOneof::Jump(v) => {
                    crate::wire::write_submessage(out, 2, v);
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
                let val = <RbRollMovement as PbMessage>::decode(bytes)?;
                self.Type = Some(RbBlockMovementPbActionTypeOneof::Roll(val));
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <RbJumpMovement as PbMessage>::decode(bytes)?;
                self.Type = Some(RbBlockMovementPbActionTypeOneof::Jump(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RbRollMovement {
    pub Direction: Option<RbGridDirection>,
}

impl PbMessage for RbRollMovement {
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
pub struct RbJumpMovement {
    pub Direction: Option<RbGridDirection>,
}

impl PbMessage for RbJumpMovement {
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
pub struct MotorDiyInfoRequest {}

impl MotorDiyInfoRequest {
    pub const MSG_ID: u16 = 16618;
}

impl PbMessage for MotorDiyInfoRequest {
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
pub struct MotorDiyInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorDiy: Option<MotorDiyPb>,
}

impl MotorDiyInfoResponse {
    pub const MSG_ID: u16 = 18366;
}

impl PbMessage for MotorDiyInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MotorDiy {
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
                self.MotorDiy
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorUseSkinRequest {
    pub SkinId: i32,
}

impl MotorUseSkinRequest {
    pub const MSG_ID: u16 = 19760;
}

impl PbMessage for MotorUseSkinRequest {
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
pub struct MotorUseSkinResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorUseSkinResponse {
    pub const MSG_ID: u16 = 22488;
}

impl PbMessage for MotorUseSkinResponse {
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
pub struct MotorUseSceneRequest {
    pub SceneId: i32,
}

impl MotorUseSceneRequest {
    pub const MSG_ID: u16 = 15459;
}

impl PbMessage for MotorUseSceneRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.SceneId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorUseSceneResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SceneInUse: i32,
}

impl MotorUseSceneResponse {
    pub const MSG_ID: u16 = 26625;
}

impl PbMessage for MotorUseSceneResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SceneInUse != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => self.SceneInUse = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorChangeOutlookRequest {
    pub StickerEquipped: Vec<i32>,
    pub DecorationsEquipped: Vec<i32>,
    pub FrameEquipped: i32,
}

impl MotorChangeOutlookRequest {
    pub const MSG_ID: u16 = 27172;
}

impl PbMessage for MotorChangeOutlookRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StickerEquipped {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DecorationsEquipped {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FrameEquipped != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.StickerEquipped,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DecorationsEquipped,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            1 => self.FrameEquipped = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorChangeOutlookResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MotorChangeOutlookResponse {
    pub const MSG_ID: u16 = 26904;
}

impl PbMessage for MotorChangeOutlookResponse {
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
pub struct MotorOutlookCreatePresetRequest {
    pub Preset: Option<MotorDiyEquippedPb>,
    pub name: Vec<u8>,
}

impl MotorOutlookCreatePresetRequest {
    pub const MSG_ID: u16 = 19553;
}

impl PbMessage for MotorOutlookCreatePresetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Preset {
            crate::wire::write_submessage(out, 4, v);
        }
        if !self.name.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.name);
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
                self.Preset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookCreatePresetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorOutlookPreset: Option<MotorOutlookPlayerPresetPb>,
}

impl MotorOutlookCreatePresetResponse {
    pub const MSG_ID: u16 = 22153;
}

impl PbMessage for MotorOutlookCreatePresetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MotorOutlookPreset {
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
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlookPreset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookDeletePresetRequest {
    pub id: i32,
}

impl MotorOutlookDeletePresetRequest {
    pub const MSG_ID: u16 = 27629;
}

impl PbMessage for MotorOutlookDeletePresetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.id);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookDeletePresetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorOutlookPreset: Option<MotorOutlookPlayerPresetPb>,
}

impl MotorOutlookDeletePresetResponse {
    pub const MSG_ID: u16 = 16772;
}

impl PbMessage for MotorOutlookDeletePresetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MotorOutlookPreset {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlookPreset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookEditPresetRequest {
    pub PresetPlan: Option<MotorOutlookPresetPlanPb>,
}

impl MotorOutlookEditPresetRequest {
    pub const MSG_ID: u16 = 26832;
}

impl PbMessage for MotorOutlookEditPresetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PresetPlan {
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
                self.PresetPlan
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookEditPresetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MotorOutlookPreset: Option<MotorOutlookPlayerPresetPb>,
}

impl MotorOutlookEditPresetResponse {
    pub const MSG_ID: u16 = 23910;
}

impl PbMessage for MotorOutlookEditPresetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MotorOutlookPreset {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MotorOutlookPreset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookEquippedChangeNotify {
    pub MotorDiyEquipped: Option<MotorDiyEquippedPb>,
    pub LatestMotorSkinSuit: Vec<MotorDiyEquippedPb>,
}

impl MotorOutlookEquippedChangeNotify {
    pub const MSG_ID: u16 = 15683;
}

impl PbMessage for MotorOutlookEquippedChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorDiyEquipped {
            crate::wire::write_submessage(out, 8, v);
        }
        for item in &self.LatestMotorSkinSuit {
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MotorDiyEquipped
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.LatestMotorSkinSuit
                    .push(<MotorDiyEquippedPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityMotorOutlookChangeNotify {
    pub EntityId: i64,
    pub MotorDiyEquipped: Option<MotorDiyEquippedPb>,
}

impl EntityMotorOutlookChangeNotify {
    pub const MSG_ID: u16 = 29444;
}

impl PbMessage for EntityMotorOutlookChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.MotorDiyEquipped {
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
            9 => self.EntityId = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.MotorDiyEquipped
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookRegionInfoNotify {
    pub MotorOutlookRegion: Option<MotorOutlookRegionPb>,
}

impl MotorOutlookRegionInfoNotify {
    pub const MSG_ID: u16 = 17258;
}

impl PbMessage for MotorOutlookRegionInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorOutlookRegion {
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
                self.MotorOutlookRegion
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDiyEquippedPb {
    pub SkinEquipped: i32,
    pub StickerEquipped: Vec<i32>,
    pub DecorationsEquipped: Vec<i32>,
    pub FrameEquipped: i32,
}

impl PbMessage for MotorDiyEquippedPb {
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
pub struct MotorDiyOnwedPb {
    pub SkinOwned: Vec<i32>,
    pub StickerOnwed: Vec<i32>,
    pub DecorationsOwned: Vec<i32>,
    pub FrameOwned: Vec<i32>,
}

impl PbMessage for MotorDiyOnwedPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinOwned {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.StickerOnwed {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinOwned, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StickerOnwed, |sr| {
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
pub struct MotorOutlookRegionPb {
    pub MotorSticker: Vec<MotorOutlookIdTimePairPb>,
    pub MotorDecoration: Vec<MotorOutlookIdTimePairPb>,
    pub MotorFrame: Vec<MotorOutlookIdTimePairPb>,
    pub MotorStickerId: Vec<i32>,
    pub MotorDecorationId: Vec<i32>,
    pub MotorFrameId: Vec<i32>,
}

impl PbMessage for MotorOutlookRegionPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MotorSticker {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.MotorDecoration {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.MotorFrame {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.MotorStickerId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MotorDecorationId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MotorFrameId {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.MotorSticker
                    .push(<MotorOutlookIdTimePairPb as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.MotorDecoration
                    .push(<MotorOutlookIdTimePairPb as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MotorFrame
                    .push(<MotorOutlookIdTimePairPb as PbMessage>::decode(bytes)?);
            }
            1 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MotorStickerId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MotorDecorationId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MotorFrameId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookIdTimePairPb {
    pub Id: i32,
    pub OpenTime: i64,
}

impl PbMessage for MotorOutlookIdTimePairPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
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
            2 => self.OpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookPlayerPresetPb {
    pub Plan: Vec<MotorOutlookPresetPlanPb>,
}

impl PbMessage for MotorOutlookPlayerPresetPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Plan {
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
                self.Plan
                    .push(<MotorOutlookPresetPlanPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorOutlookPresetPlanPb {
    pub Preset: Option<MotorDiyEquippedPb>,
    pub Mame: Vec<u8>,
    pub Id: i32,
}

impl PbMessage for MotorOutlookPresetPlanPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Preset {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.Mame.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Mame);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
                self.Preset
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Mame = r.read_len_delimited()?.to_vec(),
            3 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MotorDiyPb {
    pub MotorDiyOnwer: Option<MotorDiyOnwedPb>,
    pub MotorDiyEquipped: Option<MotorDiyEquippedPb>,
    pub MotorOutlookPreset: Option<MotorOutlookPlayerPresetPb>,
    pub LatestMotorSkinSuit: Vec<MotorDiyEquippedPb>,
    pub SceneInUse: i32,
}

impl PbMessage for MotorDiyPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MotorDiyOnwer {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.MotorDiyEquipped {
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
                self.MotorDiyOnwer
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MotorDiyEquipped
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
                    .push(<MotorDiyEquippedPb as PbMessage>::decode(bytes)?);
            }
            5 => self.SceneInUse = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthRoleSelect {
    pub Year: i32,
    pub Role: i32,
}

impl PbMessage for BirthRoleSelect {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Year != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Year);
        }
        if self.Role != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Role);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Year = r.read_varint_i32()?,
            2 => self.Role = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BirthdayInfoUpdateNotify {
    pub BirthDayReset: bool,
    pub RecentRewardTime: i32,
    pub Roles: Vec<BirthRoleSelect>,
}

impl BirthdayInfoUpdateNotify {
    pub const MSG_ID: u16 = 20525;
}

impl PbMessage for BirthdayInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BirthDayReset {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RecentRewardTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecentRewardTime);
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
            2 => self.BirthDayReset = r.read_bool()?,
            1 => self.RecentRewardTime = r.read_varint_i32()?,
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
pub struct BuffItem {
    pub ItemId: i32,
    pub CdTime: i64,
}

impl PbMessage for BuffItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.CdTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CdTime);
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
            3 => self.CdTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipBuffItem {
    pub ItemId: i32,
    pub Equiped: bool,
}

impl PbMessage for EquipBuffItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Equiped {
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
            2 => self.Equiped = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BuffItemNotify {
    pub ItemBuffList: Vec<BuffItem>,
    pub EquipItemList: Vec<EquipBuffItem>,
}

impl BuffItemNotify {
    pub const MSG_ID: u16 = 19426;
}

impl PbMessage for BuffItemNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemBuffList {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.EquipItemList {
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ItemBuffList
                    .push(<BuffItem as PbMessage>::decode(bytes)?);
            }
            8 => {
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
pub struct ButtonEnableResult {
    pub Type: Option<ButtonType>,
    pub Enabled: bool,
}

impl PbMessage for ButtonEnableResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Enabled {
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
            2 => {
                self.Type = Some(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.Enabled = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtnStateRequest {
    pub Type: Option<ButtonType>,
    pub Types: Vec<ButtonType>,
}

impl BtnStateRequest {
    pub const MSG_ID: u16 = 18232;
}

impl PbMessage for BtnStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Types {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            9 => {
                self.Type = Some(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Types, |sr| {
                Ok(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    sr.read_varint_i32()?,
                ))
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtnStateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Type: Option<ButtonType>,
    pub Enabled: bool,
    pub Result: Vec<ButtonEnableResult>,
}

impl BtnStateResponse {
    pub const MSG_ID: u16 = 25924;
}

impl PbMessage for BtnStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Enabled {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Result {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.Type = Some(<ButtonType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.Enabled = r.read_bool()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Result
                    .push(<ButtonEnableResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashDevelopConditionState {
    pub ConditionId: i32,
    pub Rewarded: bool,
}

impl PbMessage for CalabashDevelopConditionState {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.Rewarded {
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
            1 => self.ConditionId = r.read_varint_i32()?,
            2 => self.Rewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashDevelopInfo {
    pub MonsterId: i32,
    pub UnlockConditions: Vec<CalabashDevelopConditionState>,
}

impl PbMessage for CalabashDevelopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MonsterId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MonsterId);
        }
        for item in &self.UnlockConditions {
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
            1 => self.MonsterId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockConditions
                    .push(<CalabashDevelopConditionState as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashMsg {
    pub Level: i32,
    pub Exp: i32,
    pub UnlockedLevels: Vec<i32>,
    pub UnlockedDevelopRewards: Vec<CalabashDevelopInfo>,
    pub IdentifyGuaranteeCount: i32,
    pub LowCostGuaranteeCount: i32,
}

impl PbMessage for CalabashMsg {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        for item in &self.UnlockedLevels {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockedDevelopRewards {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.IdentifyGuaranteeCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IdentifyGuaranteeCount);
        }
        if self.LowCostGuaranteeCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LowCostGuaranteeCount);
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
            2 => self.Exp = r.read_varint_i32()?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockedLevels, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockedDevelopRewards
                    .push(<CalabashDevelopInfo as PbMessage>::decode(bytes)?);
            }
            5 => self.IdentifyGuaranteeCount = r.read_varint_i32()?,
            6 => self.LowCostGuaranteeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashCfg {
    pub LevelUpExp: i32,
    pub LevelUpCondition: i32,
    pub CatchGain: BTreeMap<i32, i32>,
}

impl PbMessage for CalabashCfg {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelUpExp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelUpExp);
        }
        if self.LevelUpCondition != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelUpCondition);
        }
        if !self.CatchGain.is_empty() {
            crate::wire::write_map(out, 4, &self.CatchGain);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LevelUpExp = r.read_varint_i32()?,
            3 => self.LevelUpCondition = r.read_varint_i32()?,
            4 => crate::wire::decode_map_entry(r, &mut self.CatchGain)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashMsgNotify {
    pub CalabashMsg: Option<CalabashMsg>,
    pub CalabashCfg: Option<CalabashCfg>,
}

impl CalabashMsgNotify {
    pub const MSG_ID: u16 = 29927;
}

impl PbMessage for CalabashMsgNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CalabashMsg {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.CalabashCfg {
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.CalabashMsg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.CalabashCfg
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashLevelsRewardNotify {
    pub RewardedLevels: Vec<i32>,
}

impl CalabashLevelsRewardNotify {
    pub const MSG_ID: u16 = 19440;
}

impl PbMessage for CalabashLevelsRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardedLevels {
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
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedLevels, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashLevelRewardRequest {
    pub Level: i32,
}

impl CalabashLevelRewardRequest {
    pub const MSG_ID: u16 = 28983;
}

impl PbMessage for CalabashLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashLevelRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CalabashLevelRewardResponse {
    pub const MSG_ID: u16 = 23187;
}

impl PbMessage for CalabashLevelRewardResponse {
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
pub struct CalabashSkinDataRequest {}

impl CalabashSkinDataRequest {
    pub const MSG_ID: u16 = 20817;
}

impl PbMessage for CalabashSkinDataRequest {
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
pub struct CalabashSkinDataResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EquipedSkinId: i32,
    pub SkinIdList: Vec<i32>,
}

impl CalabashSkinDataResponse {
    pub const MSG_ID: u16 = 18646;
}

impl PbMessage for CalabashSkinDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EquipedSkinId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipedSkinId);
        }
        for item in &self.SkinIdList {
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
            10 => self.EquipedSkinId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashSkinTakeOnRequest {
    pub SkinId: i32,
}

impl CalabashSkinTakeOnRequest {
    pub const MSG_ID: u16 = 27489;
}

impl PbMessage for CalabashSkinTakeOnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            10 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CalabashSkinTakeOnResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkinId: i32,
}

impl CalabashSkinTakeOnResponse {
    pub const MSG_ID: u16 = 28468;
}

impl PbMessage for CalabashSkinTakeOnResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraParamInfo {
    pub CameraMode: i32,
    pub PitchValue: f32,
    pub YawValue: f32,
    pub RollValue: f32,
    pub PosX: f32,
    pub PosY: f32,
    pub PosZ: f32,
    pub Fov: i32,
    pub FocusDistance: i32,
    pub BlurRadius: i32,
    pub CameraTiltAngle: i32,
    pub Weather: i32,
    pub Atmosphere: i32,
    pub WindDirection: i32,
}

impl PbMessage for CameraParamInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CameraMode != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CameraMode);
        }
        if self.PitchValue != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PitchValue).to_bits().to_le_bytes());
        }
        if self.YawValue != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.YawValue).to_bits().to_le_bytes());
        }
        if self.RollValue != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.RollValue).to_bits().to_le_bytes());
        }
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.PosZ != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosZ).to_bits().to_le_bytes());
        }
        if self.Fov != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Fov);
        }
        if self.FocusDistance != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FocusDistance);
        }
        if self.BlurRadius != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlurRadius);
        }
        if self.CameraTiltAngle != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CameraTiltAngle);
        }
        if self.Weather != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Weather);
        }
        if self.Atmosphere != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Atmosphere);
        }
        if self.WindDirection != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WindDirection);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CameraMode = r.read_varint_i32()?,
            2 => self.PitchValue = r.read_f32()?,
            3 => self.YawValue = r.read_f32()?,
            4 => self.RollValue = r.read_f32()?,
            5 => self.PosX = r.read_f32()?,
            6 => self.PosY = r.read_f32()?,
            7 => self.PosZ = r.read_f32()?,
            8 => self.Fov = r.read_varint_i32()?,
            9 => self.FocusDistance = r.read_varint_i32()?,
            10 => self.BlurRadius = r.read_varint_i32()?,
            11 => self.CameraTiltAngle = r.read_varint_i32()?,
            12 => self.Weather = r.read_varint_i32()?,
            13 => self.Atmosphere = r.read_varint_i32()?,
            14 => self.WindDirection = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraSettingInfo {
    pub CameraParam: Option<CameraParamInfo>,
}

impl PbMessage for CameraSettingInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CameraParam {
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
                self.CameraParam
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraSettingQueryRequest {}

impl CameraSettingQueryRequest {
    pub const MSG_ID: u16 = 26920;
}

impl PbMessage for CameraSettingQueryRequest {
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
pub struct CameraSettingQueryResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CameraSetting: Option<CameraSettingInfo>,
}

impl CameraSettingQueryResponse {
    pub const MSG_ID: u16 = 25637;
}

impl PbMessage for CameraSettingQueryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CameraSetting {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.CameraSetting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraSettingModifyRequest {
    pub CameraSetting: Option<CameraSettingInfo>,
}

impl CameraSettingModifyRequest {
    pub const MSG_ID: u16 = 28953;
}

impl PbMessage for CameraSettingModifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CameraSetting {
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
                self.CameraSetting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraSettingModifyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl CameraSettingModifyResponse {
    pub const MSG_ID: u16 = 26725;
}

impl PbMessage for CameraSettingModifyResponse {
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
pub struct SelfPortraitStateRequest {
    pub State: Option<SelfPortraitOpState>,
}

impl SelfPortraitStateRequest {
    pub const MSG_ID: u16 = 17018;
}

impl PbMessage for SelfPortraitStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            13 => {
                self.State = Some(
                    <SelfPortraitOpState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct SelfPortraitStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl SelfPortraitStateResponse {
    pub const MSG_ID: u16 = 28546;
}

impl PbMessage for SelfPortraitStateResponse {
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
pub struct PrivateChatRequest {
    pub TargetUid: i32,
    pub ChatContentType: Option<ChatContentType>,
    pub Content: Vec<u8>,
    pub XboxBlockedPlayerIds: Vec<i32>,
}

impl PrivateChatRequest {
    pub const MSG_ID: u16 = 18832;
}

impl PbMessage for PrivateChatRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUid != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUid);
        }
        if let Some(v) = &self.ChatContentType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
        for item in &self.XboxBlockedPlayerIds {
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
            9 => self.TargetUid = r.read_varint_i32()?,
            3 => {
                self.ChatContentType = Some(
                    <ChatContentType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.Content = r.read_len_delimited()?.to_vec(),
            8 => crate::wire::decode_repeated_scalar(
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
pub struct PrivateChatResponse {
    pub TargetUid: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub MsgId: Vec<u8>,
    pub FilterMsg: Vec<u8>,
    pub BanEndTime: i64,
}

impl PrivateChatResponse {
    pub const MSG_ID: u16 = 21677;
}

impl PbMessage for PrivateChatResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUid != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUid);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.MsgId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MsgId);
        }
        if !self.FilterMsg.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FilterMsg);
        }
        if self.BanEndTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            5 => self.TargetUid = r.read_varint_i32()?,
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.MsgId = r.read_len_delimited()?.to_vec(),
            9 => self.FilterMsg = r.read_len_delimited()?.to_vec(),
            1 => self.BanEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChatContentProto {
    pub SenderUid: i32,
    pub ChatContentType: Option<ChatContentType>,
    pub Content: Vec<u8>,
    pub OfflineMsg: bool,
    pub UtcTime: i64,
    pub MsgId: Vec<u8>,
    pub PsAccountId: Vec<u8>,
    pub XboxAccountId: Vec<u8>,
}

impl PbMessage for ChatContentProto {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SenderUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SenderUid);
        }
        if let Some(v) = &self.ChatContentType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Content);
        }
        if self.OfflineMsg {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UtcTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UtcTime);
        }
        if !self.MsgId.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MsgId);
        }
        if !self.PsAccountId.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsAccountId);
        }
        if !self.XboxAccountId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxAccountId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SenderUid = r.read_varint_i32()?,
            2 => {
                self.ChatContentType = Some(
                    <ChatContentType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Content = r.read_len_delimited()?.to_vec(),
            4 => self.OfflineMsg = r.read_bool()?,
            5 => self.UtcTime = r.read_varint_i64()?,
            6 => self.MsgId = r.read_len_delimited()?.to_vec(),
            7 => self.PsAccountId = r.read_len_delimited()?.to_vec(),
            8 => self.XboxAccountId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateMessageNotify {
    pub ChatContent: Option<ChatContentProto>,
}

impl PrivateMessageNotify {
    pub const MSG_ID: u16 = 21956;
}

impl PbMessage for PrivateMessageNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ChatContent {
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
                self.ChatContent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatHistoryRequest {
    pub TargetUid: i32,
    pub StartIndex: i32,
}

impl PrivateChatHistoryRequest {
    pub const MSG_ID: u16 = 20338;
}

impl PbMessage for PrivateChatHistoryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUid != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUid);
        }
        if self.StartIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            10 => self.TargetUid = r.read_varint_i32()?,
            9 => self.StartIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatHistoryResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Data: Option<PrivateChatHistoryContentProto>,
}

impl PrivateChatHistoryResponse {
    pub const MSG_ID: u16 = 24958;
}

impl PbMessage for PrivateChatHistoryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Data {
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
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
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
pub struct PrivateChatHistoryContentProto {
    pub TargetUid: i32,
    pub Chats: Vec<ChatContentProto>,
    pub HistoryIsEnd: bool,
    pub TotalNums: i32,
}

impl PbMessage for PrivateChatHistoryContentProto {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUid);
        }
        for item in &self.Chats {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.HistoryIsEnd {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TotalNums != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalNums);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetUid = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Chats
                    .push(<ChatContentProto as PbMessage>::decode(bytes)?);
            }
            3 => self.HistoryIsEnd = r.read_bool()?,
            4 => self.TotalNums = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatHistoryNotify {
    pub AllChats: Vec<PrivateChatHistoryContentProto>,
}

impl PrivateChatHistoryNotify {
    pub const MSG_ID: u16 = 16299;
}

impl PbMessage for PrivateChatHistoryNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AllChats {
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
                self.AllChats
                    .push(<PrivateChatHistoryContentProto as PbMessage>::decode(
                        bytes,
                    )?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatDataRequest {}

impl PrivateChatDataRequest {
    pub const MSG_ID: u16 = 17748;
}

impl PbMessage for PrivateChatDataRequest {
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
pub struct PrivateChatDataResponse {
    pub LoadSucc: bool,
}

impl PrivateChatDataResponse {
    pub const MSG_ID: u16 = 27414;
}

impl PbMessage for PrivateChatDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoadSucc {
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
            13 => self.LoadSucc = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatOperateRequest {
    pub OperateType: Option<PrivateChatOperateType>,
    pub TargetPlayerId: i32,
}

impl PrivateChatOperateRequest {
    pub const MSG_ID: u16 = 16521;
}

impl PbMessage for PrivateChatOperateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OperateType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TargetPlayerId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetPlayerId);
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
                self.OperateType = Some(
                    <PrivateChatOperateType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.TargetPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatOperateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PrivateChatOperateResponse {
    pub const MSG_ID: u16 = 26788;
}

impl PbMessage for PrivateChatOperateResponse {
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
pub enum CombatPushDataMessageOneof {
    ApplyBuffS2cResponsePush(ApplyBuffS2cResponsePush),
    RemoveBuffS2cResponsePush(RemoveBuffS2cResponsePush),
    RemoveBuffByIdS2cResponsePush(RemoveBuffByIdS2cResponsePush),
    CreateBulletResponsePush(CreateBulletResponsePush),
    DestroyBulletResponsePush(DestroyBulletResponsePush),
    ApplyGameplayEffectPush(ApplyGameplayEffectPush),
    RemoveGameplayEffectPush(RemoveGameplayEffectPush),
    HitEndPush(HitEndPush),
    EndSkillPush(EndSkillPush),
    PartUpdatePush(PartUpdatePush),
    MaterialPush(MaterialPush),
    GameplayCuePush(GameplayCuePush),
    EntityIsVisiblePush(EntityIsVisiblePush),
    SwitchCharacterStatePush(SwitchCharacterStatePush),
    LogicStateInitPush(LogicStateInitPush),
    SwitchLogicStatePush(SwitchLogicStatePush),
    AnimationStateChangedPush(AnimationStateChangedPush),
    AnimationStateInitPush(AnimationStateInitPush),
    ModifyBulletParamsPush(ModifyBulletParamsPush),
    DrownPush(DrownPush),
    ActiveBuffPush(ActiveBuffPush),
    AiInformationPush(AiInformationPush),
    ToughCalcExtraRatioChangePush(ToughCalcExtraRatioChangePush),
    BattleStateChangePush(BattleStateChangePush),
    AnimationGameplayTagPush(AnimationGameplayTagPush),
    BoneVisibleChangePush(BoneVisibleChangePush),
    AiBlackboardsPush(AiBlackboardsPush),
    AiBlackboardCdPush(AiBlackboardCdPush),
    AiHatePush(AiHatePush),
    MonsterBoomPush(MonsterBoomPush),
    CaughtPush(CaughtPush),
    EntityStaticHookMovePush(EntityStaticHookMovePush),
    ChangeStateConfirmPush(ChangeStateConfirmPush),
    BuffStackCountPush(BuffStackCountPush),
    ANStartPush(ANStartPush),
    UseSkillFailPush(UseSkillFailPush),
    EnterViewDirectionPush(EnterViewDirectionPush),
    ExitViewDirectionPush(ExitViewDirectionPush),
    PassiveSkillAddPush(PassiveSkillAddPush),
    InterruptSkillInDelayPush(InterruptSkillInDelayPush),
    TriggerExitSkillPush(TriggerExitSkillPush),
    ActorVisiblePush(ActorVisiblePush),
    BuffEffectPush(BuffEffectPush),
    RTimeStopPush(RTimeStopPush),
    DrownEndTeleportPush(DrownEndTeleportPush),
    MonsterDrownPush(MonsterDrownPush),
    PassiveSkillRemovePush(PassiveSkillRemovePush),
    RTimeStopInstPush(RTimeStopInstPush),
    FsmPlayMontagePush(FsmPlayMontagePush),
    TsAnimNotifyStateAbsoluteTimeStopPush(TsAnimNotifyStateAbsoluteTimeStopPush),
    RoleTagChangePush(RoleTagChangePush),
    ExecuteQtePush(ExecuteQtePush),
    ClientCurrentRoleReportPush(ClientCurrentRoleReportPush),
    MontagePlayPush(MontagePlayPush),
    CounterAttackPush(CounterAttackPush),
    NewLinkBurstPush(NewLinkBurstPush),
    RefreshBuffDurationPush(RefreshBuffDurationPush),
    RoleGoDownPush(RoleGoDownPush),
    FsmConditionPassPush(FsmConditionPassPush),
    BuffEffectExecutePush(BuffEffectExecutePush),
    VisionTriggerPush(VisionTriggerPush),
    MotorIsEnablePush(MotorIsEnablePush),
    MotorSummonAndRidePush(MotorSummonAndRidePush),
    BulletPatternPush(BulletPatternPush),
    QuickHackRamVerifyPush(QuickHackRamVerifyPush),
    QuickHackOpenPush(QuickHackOpenPush),
    DodgeInfoPush(DodgeInfoPush),
    RoleTrialSceneRollbackPush(RoleTrialSceneRollbackPush),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatPushData {
    pub Message: Option<CombatPushDataMessageOneof>,
    pub CombatCommon: Option<CombatCommon>,
}

impl PbMessage for CombatPushData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatPushDataMessageOneof::ApplyBuffS2cResponsePush(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                CombatPushDataMessageOneof::RemoveBuffS2cResponsePush(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                CombatPushDataMessageOneof::RemoveBuffByIdS2cResponsePush(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                CombatPushDataMessageOneof::CreateBulletResponsePush(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                CombatPushDataMessageOneof::DestroyBulletResponsePush(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                CombatPushDataMessageOneof::ApplyGameplayEffectPush(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                CombatPushDataMessageOneof::RemoveGameplayEffectPush(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                CombatPushDataMessageOneof::HitEndPush(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                CombatPushDataMessageOneof::EndSkillPush(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                CombatPushDataMessageOneof::PartUpdatePush(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                CombatPushDataMessageOneof::MaterialPush(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                CombatPushDataMessageOneof::GameplayCuePush(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                CombatPushDataMessageOneof::EntityIsVisiblePush(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                CombatPushDataMessageOneof::SwitchCharacterStatePush(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                CombatPushDataMessageOneof::LogicStateInitPush(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                CombatPushDataMessageOneof::SwitchLogicStatePush(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                CombatPushDataMessageOneof::AnimationStateChangedPush(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                CombatPushDataMessageOneof::AnimationStateInitPush(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                CombatPushDataMessageOneof::ModifyBulletParamsPush(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                CombatPushDataMessageOneof::DrownPush(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                CombatPushDataMessageOneof::ActiveBuffPush(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                CombatPushDataMessageOneof::AiInformationPush(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                CombatPushDataMessageOneof::ToughCalcExtraRatioChangePush(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                CombatPushDataMessageOneof::BattleStateChangePush(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                CombatPushDataMessageOneof::AnimationGameplayTagPush(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                CombatPushDataMessageOneof::BoneVisibleChangePush(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                CombatPushDataMessageOneof::AiBlackboardsPush(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                CombatPushDataMessageOneof::AiBlackboardCdPush(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                CombatPushDataMessageOneof::AiHatePush(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                CombatPushDataMessageOneof::MonsterBoomPush(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                CombatPushDataMessageOneof::CaughtPush(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                CombatPushDataMessageOneof::EntityStaticHookMovePush(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                CombatPushDataMessageOneof::ChangeStateConfirmPush(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                CombatPushDataMessageOneof::BuffStackCountPush(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                CombatPushDataMessageOneof::ANStartPush(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                CombatPushDataMessageOneof::UseSkillFailPush(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                CombatPushDataMessageOneof::EnterViewDirectionPush(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                CombatPushDataMessageOneof::ExitViewDirectionPush(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                CombatPushDataMessageOneof::PassiveSkillAddPush(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                CombatPushDataMessageOneof::InterruptSkillInDelayPush(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                CombatPushDataMessageOneof::TriggerExitSkillPush(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                CombatPushDataMessageOneof::ActorVisiblePush(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                CombatPushDataMessageOneof::BuffEffectPush(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                CombatPushDataMessageOneof::RTimeStopPush(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                CombatPushDataMessageOneof::DrownEndTeleportPush(v) => {
                    crate::wire::write_submessage(out, 48, v);
                }
                CombatPushDataMessageOneof::MonsterDrownPush(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                CombatPushDataMessageOneof::PassiveSkillRemovePush(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                CombatPushDataMessageOneof::RTimeStopInstPush(v) => {
                    crate::wire::write_submessage(out, 51, v);
                }
                CombatPushDataMessageOneof::FsmPlayMontagePush(v) => {
                    crate::wire::write_submessage(out, 52, v);
                }
                CombatPushDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopPush(v) => {
                    crate::wire::write_submessage(out, 53, v);
                }
                CombatPushDataMessageOneof::RoleTagChangePush(v) => {
                    crate::wire::write_submessage(out, 54, v);
                }
                CombatPushDataMessageOneof::ExecuteQtePush(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                CombatPushDataMessageOneof::ClientCurrentRoleReportPush(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                CombatPushDataMessageOneof::MontagePlayPush(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                CombatPushDataMessageOneof::CounterAttackPush(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                CombatPushDataMessageOneof::NewLinkBurstPush(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                CombatPushDataMessageOneof::RefreshBuffDurationPush(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                CombatPushDataMessageOneof::RoleGoDownPush(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                CombatPushDataMessageOneof::FsmConditionPassPush(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                CombatPushDataMessageOneof::BuffEffectExecutePush(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                CombatPushDataMessageOneof::VisionTriggerPush(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                CombatPushDataMessageOneof::MotorIsEnablePush(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                CombatPushDataMessageOneof::MotorSummonAndRidePush(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                CombatPushDataMessageOneof::BulletPatternPush(v) => {
                    crate::wire::write_submessage(out, 67, v);
                }
                CombatPushDataMessageOneof::QuickHackRamVerifyPush(v) => {
                    crate::wire::write_submessage(out, 68, v);
                }
                CombatPushDataMessageOneof::QuickHackOpenPush(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                CombatPushDataMessageOneof::DodgeInfoPush(v) => {
                    crate::wire::write_submessage(out, 70, v);
                }
                CombatPushDataMessageOneof::RoleTrialSceneRollbackPush(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
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
                let val = <ApplyBuffS2cResponsePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ApplyBuffS2cResponsePush(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveBuffS2cResponsePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RemoveBuffS2cResponsePush(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveBuffByIdS2cResponsePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RemoveBuffByIdS2cResponsePush(
                    val,
                ));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <CreateBulletResponsePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::CreateBulletResponsePush(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <DestroyBulletResponsePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::DestroyBulletResponsePush(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <ApplyGameplayEffectPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ApplyGameplayEffectPush(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveGameplayEffectPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RemoveGameplayEffectPush(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitEndPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::HitEndPush(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <EndSkillPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::EndSkillPush(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartUpdatePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::PartUpdatePush(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <MaterialPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MaterialPush(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <GameplayCuePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::GameplayCuePush(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityIsVisiblePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::EntityIsVisiblePush(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchCharacterStatePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::SwitchCharacterStatePush(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <LogicStateInitPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::LogicStateInitPush(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchLogicStatePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::SwitchLogicStatePush(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateChangedPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AnimationStateChangedPush(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateInitPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AnimationStateInitPush(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <ModifyBulletParamsPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ModifyBulletParamsPush(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::DrownPush(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActiveBuffPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ActiveBuffPush(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiInformationPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AiInformationPush(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <ToughCalcExtraRatioChangePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ToughCalcExtraRatioChangePush(
                    val,
                ));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <BattleStateChangePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BattleStateChangePush(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationGameplayTagPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AnimationGameplayTagPush(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <BoneVisibleChangePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BoneVisibleChangePush(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardsPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AiBlackboardsPush(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardCdPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AiBlackboardCdPush(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiHatePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::AiHatePush(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterBoomPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MonsterBoomPush(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <CaughtPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::CaughtPush(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityStaticHookMovePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::EntityStaticHookMovePush(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateConfirmPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ChangeStateConfirmPush(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffStackCountPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BuffStackCountPush(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <ANStartPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ANStartPush(val));
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillFailPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::UseSkillFailPush(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <EnterViewDirectionPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::EnterViewDirectionPush(val));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExitViewDirectionPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ExitViewDirectionPush(val));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillAddPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::PassiveSkillAddPush(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <InterruptSkillInDelayPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::InterruptSkillInDelayPush(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <TriggerExitSkillPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::TriggerExitSkillPush(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActorVisiblePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ActorVisiblePush(val));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffEffectPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BuffEffectPush(val));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RTimeStopPush(val));
            }
            48 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownEndTeleportPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::DrownEndTeleportPush(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterDrownPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MonsterDrownPush(val));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillRemovePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::PassiveSkillRemovePush(val));
            }
            51 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopInstPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RTimeStopInstPush(val));
            }
            52 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmPlayMontagePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::FsmPlayMontagePush(val));
            }
            53 => {
                let bytes = r.read_len_delimited()?;
                let val = <TsAnimNotifyStateAbsoluteTimeStopPush as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatPushDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopPush(val));
            }
            54 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTagChangePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RoleTagChangePush(val));
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExecuteQtePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ExecuteQtePush(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientCurrentRoleReportPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::ClientCurrentRoleReportPush(val));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <MontagePlayPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MontagePlayPush(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <CounterAttackPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::CounterAttackPush(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewLinkBurstPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::NewLinkBurstPush(val));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <RefreshBuffDurationPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RefreshBuffDurationPush(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleGoDownPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RoleGoDownPush(val));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmConditionPassPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::FsmConditionPassPush(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffEffectExecutePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BuffEffectExecutePush(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <VisionTriggerPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::VisionTriggerPush(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorIsEnablePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MotorIsEnablePush(val));
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorSummonAndRidePush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::MotorSummonAndRidePush(val));
            }
            67 => {
                let bytes = r.read_len_delimited()?;
                let val = <BulletPatternPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::BulletPatternPush(val));
            }
            68 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuickHackRamVerifyPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::QuickHackRamVerifyPush(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <QuickHackOpenPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::QuickHackOpenPush(val));
            }
            70 => {
                let bytes = r.read_len_delimited()?;
                let val = <DodgeInfoPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::DodgeInfoPush(val));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTrialSceneRollbackPush as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatPushDataMessageOneof::RoleTrialSceneRollbackPush(val));
            }
            1 => {
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

#[derive(Debug, Clone, PartialEq)]
pub enum CombatNotifyDataMessageOneof {
    CreateBulletNotify(CreateBulletNotify),
    DestroyBulletNotify(DestroyBulletNotify),
    DamageExecuteNotify(DamageExecuteNotify),
    ApplyGameplayEffectNotify(ApplyGameplayEffectNotify),
    RemoveGameplayEffectNotify(RemoveGameplayEffectNotify),
    HitNotify(HitNotify),
    SkillNotify(SkillNotify),
    UseSkillNotify(UseSkillNotify),
    EndSkillNotify(EndSkillNotify),
    EntityLoadCompleteNotify(EntityLoadCompleteNotify),
    PartUpdateNotify(PartUpdateNotify),
    PartComponentInitNotify(PartComponentInitNotify),
    MaterialNotify(MaterialNotify),
    GameplayCueNotify(GameplayCueNotify),
    EntityIsVisibleNotify(EntityIsVisibleNotify),
    SwitchCharacterStateNotify(SwitchCharacterStateNotify),
    PlayerRebackSceneNotify(PlayerRebackSceneNotify),
    LogicStateInitNotify(LogicStateInitNotify),
    SwitchLogicStateNotify(SwitchLogicStateNotify),
    AttributeChangedNotify(AttributeChangedNotify),
    AnimationStateChangedNotify(AnimationStateChangedNotify),
    AnimationStateInitNotify(AnimationStateInitNotify),
    ModifyBulletParamsNotify(ModifyBulletParamsNotify),
    DrownNotify(DrownNotify),
    OrderApplyBuffNotify(OrderApplyBuffNotify),
    OrderRemoveBuffNotify(OrderRemoveBuffNotify),
    ActivateBuffNotify(ActivateBuffNotify),
    OrderRemoveBuffByTagsNotify(OrderRemoveBuffByTagsNotify),
    AiInformationNotify(AiInformationNotify),
    BattleStateChangeNotify(BattleStateChangeNotify),
    AnimationGameplayTagNotify(AnimationGameplayTagNotify),
    BoneVisibleChangeNotify(BoneVisibleChangeNotify),
    AiBlackboardCdNotify(AiBlackboardCdNotify),
    CaughtNotify(CaughtNotify),
    EntityStaticHookMoveNotify(EntityStaticHookMoveNotify),
    ChangeStateNotify(ChangeStateNotify),
    ChangeStateConfirmNotify(ChangeStateConfirmNotify),
    BuffStackCountNotify(BuffStackCountNotify),
    MontagePlayNotify(MontagePlayNotify),
    ANStartNotify(ANStartNotify),
    FsmResetNotify(FsmResetNotify),
    DamageRecordNotify(DamageRecordNotify),
    AiHateNotify(AiHateNotify),
    FsmBlackboardNotify(FsmBlackboardNotify),
    CharacterBattleStateChangeNotify(CharacterBattleStateChangeNotify),
    ApplyBuffS2cRequestNotify(ApplyBuffS2cRequestNotify),
    RemoveBuffS2cRequestNotify(RemoveBuffS2cRequestNotify),
    ActorVisibleNotify(ActorVisibleNotify),
    RecoverPropChangedNotify(RecoverPropChangedNotify),
    RemoveBuffByIdS2cRequestNotify(RemoveBuffByIdS2cRequestNotify),
    ShieldUpdateNotify(ShieldUpdateNotify),
    PlayerBattleStateChangeNotify(PlayerBattleStateChangeNotify),
    FsmCustomBlackboardNotify(FsmCustomBlackboardNotify),
    PassiveSkillAddNotify(PassiveSkillAddNotify),
    PassiveSkillRemoveNotify(PassiveSkillRemoveNotify),
    ExecuteQteNotify(ExecuteQteNotify),
    ModifyEntityCampNotify(ModifyEntityCampNotify),
    AddCombineEntitiesRelationNotify(AddCombineEntitiesRelationNotify),
    RemoveCombineRelationNotify(RemoveCombineRelationNotify),
    TestDamageRecordNotify(TestDamageRecordNotify),
    BuffDurationNotify(BuffDurationNotify),
    EntityLivingStatusNotify(EntityLivingStatusNotify),
    NewLinkStateNotify(NewLinkStateNotify),
    BroadcastAddBuffFailedNotify(BroadcastAddBuffFailedNotify),
    PackAnimChangedNotify(PackAnimChangedNotify),
    VisionTriggerNotify(VisionTriggerNotify),
    RemoveBuffByServerIdS2cRequestNotify(RemoveBuffByServerIdS2cRequestNotify),
    TransformBuffStackNotify(TransformBuffStackNotify),
    MotorSummonAndRideNotify(MotorSummonAndRideNotify),
    BulletPatternNotify(BulletPatternNotify),
    FsmMontageDurationNotify(FsmMontageDurationNotify),
    CombatValNotify(CombatValNotify),
    CombatValDecayBeginNotify(CombatValDecayBeginNotify),
    SquadPropChangedNotify(SquadPropChangedNotify),
    DamageShowNotify(DamageShowNotify),
    CombatDataMaxNotify(CombatDataMaxNotify),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatNotifyData {
    pub Message: Option<CombatNotifyDataMessageOneof>,
    pub CombatCommon: Option<CombatCommon>,
}

impl PbMessage for CombatNotifyData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatNotifyDataMessageOneof::CreateBulletNotify(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                CombatNotifyDataMessageOneof::DestroyBulletNotify(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                CombatNotifyDataMessageOneof::DamageExecuteNotify(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                CombatNotifyDataMessageOneof::HitNotify(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                CombatNotifyDataMessageOneof::SkillNotify(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                CombatNotifyDataMessageOneof::UseSkillNotify(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                CombatNotifyDataMessageOneof::EndSkillNotify(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                CombatNotifyDataMessageOneof::EntityLoadCompleteNotify(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                CombatNotifyDataMessageOneof::PartUpdateNotify(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                CombatNotifyDataMessageOneof::PartComponentInitNotify(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                CombatNotifyDataMessageOneof::MaterialNotify(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                CombatNotifyDataMessageOneof::GameplayCueNotify(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                CombatNotifyDataMessageOneof::EntityIsVisibleNotify(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                CombatNotifyDataMessageOneof::SwitchCharacterStateNotify(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                CombatNotifyDataMessageOneof::PlayerRebackSceneNotify(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                CombatNotifyDataMessageOneof::LogicStateInitNotify(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                CombatNotifyDataMessageOneof::SwitchLogicStateNotify(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                CombatNotifyDataMessageOneof::AttributeChangedNotify(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                CombatNotifyDataMessageOneof::AnimationStateChangedNotify(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                CombatNotifyDataMessageOneof::AnimationStateInitNotify(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                CombatNotifyDataMessageOneof::ModifyBulletParamsNotify(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                CombatNotifyDataMessageOneof::DrownNotify(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                CombatNotifyDataMessageOneof::OrderApplyBuffNotify(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                CombatNotifyDataMessageOneof::OrderRemoveBuffNotify(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                CombatNotifyDataMessageOneof::ActivateBuffNotify(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                CombatNotifyDataMessageOneof::OrderRemoveBuffByTagsNotify(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                CombatNotifyDataMessageOneof::AiInformationNotify(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                CombatNotifyDataMessageOneof::BattleStateChangeNotify(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                CombatNotifyDataMessageOneof::AnimationGameplayTagNotify(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                CombatNotifyDataMessageOneof::BoneVisibleChangeNotify(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                CombatNotifyDataMessageOneof::AiBlackboardCdNotify(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                CombatNotifyDataMessageOneof::CaughtNotify(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                CombatNotifyDataMessageOneof::EntityStaticHookMoveNotify(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                CombatNotifyDataMessageOneof::ChangeStateNotify(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                CombatNotifyDataMessageOneof::ChangeStateConfirmNotify(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                CombatNotifyDataMessageOneof::BuffStackCountNotify(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                CombatNotifyDataMessageOneof::MontagePlayNotify(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                CombatNotifyDataMessageOneof::ANStartNotify(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                CombatNotifyDataMessageOneof::FsmResetNotify(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                CombatNotifyDataMessageOneof::DamageRecordNotify(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                CombatNotifyDataMessageOneof::AiHateNotify(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                CombatNotifyDataMessageOneof::FsmBlackboardNotify(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                CombatNotifyDataMessageOneof::CharacterBattleStateChangeNotify(v) => {
                    crate::wire::write_submessage(out, 48, v);
                }
                CombatNotifyDataMessageOneof::ApplyBuffS2cRequestNotify(v) => {
                    crate::wire::write_submessage(out, 53, v);
                }
                CombatNotifyDataMessageOneof::RemoveBuffS2cRequestNotify(v) => {
                    crate::wire::write_submessage(out, 54, v);
                }
                CombatNotifyDataMessageOneof::ActorVisibleNotify(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                CombatNotifyDataMessageOneof::RecoverPropChangedNotify(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                CombatNotifyDataMessageOneof::RemoveBuffByIdS2cRequestNotify(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                CombatNotifyDataMessageOneof::ShieldUpdateNotify(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                CombatNotifyDataMessageOneof::PlayerBattleStateChangeNotify(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                CombatNotifyDataMessageOneof::FsmCustomBlackboardNotify(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                CombatNotifyDataMessageOneof::PassiveSkillAddNotify(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                CombatNotifyDataMessageOneof::PassiveSkillRemoveNotify(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                CombatNotifyDataMessageOneof::ExecuteQteNotify(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                CombatNotifyDataMessageOneof::ModifyEntityCampNotify(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                CombatNotifyDataMessageOneof::AddCombineEntitiesRelationNotify(v) => {
                    crate::wire::write_submessage(out, 70, v);
                }
                CombatNotifyDataMessageOneof::RemoveCombineRelationNotify(v) => {
                    crate::wire::write_submessage(out, 71, v);
                }
                CombatNotifyDataMessageOneof::TestDamageRecordNotify(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
                CombatNotifyDataMessageOneof::BuffDurationNotify(v) => {
                    crate::wire::write_submessage(out, 73, v);
                }
                CombatNotifyDataMessageOneof::EntityLivingStatusNotify(v) => {
                    crate::wire::write_submessage(out, 74, v);
                }
                CombatNotifyDataMessageOneof::NewLinkStateNotify(v) => {
                    crate::wire::write_submessage(out, 75, v);
                }
                CombatNotifyDataMessageOneof::BroadcastAddBuffFailedNotify(v) => {
                    crate::wire::write_submessage(out, 76, v);
                }
                CombatNotifyDataMessageOneof::PackAnimChangedNotify(v) => {
                    crate::wire::write_submessage(out, 77, v);
                }
                CombatNotifyDataMessageOneof::VisionTriggerNotify(v) => {
                    crate::wire::write_submessage(out, 78, v);
                }
                CombatNotifyDataMessageOneof::RemoveBuffByServerIdS2cRequestNotify(v) => {
                    crate::wire::write_submessage(out, 79, v);
                }
                CombatNotifyDataMessageOneof::TransformBuffStackNotify(v) => {
                    crate::wire::write_submessage(out, 80, v);
                }
                CombatNotifyDataMessageOneof::MotorSummonAndRideNotify(v) => {
                    crate::wire::write_submessage(out, 81, v);
                }
                CombatNotifyDataMessageOneof::BulletPatternNotify(v) => {
                    crate::wire::write_submessage(out, 82, v);
                }
                CombatNotifyDataMessageOneof::FsmMontageDurationNotify(v) => {
                    crate::wire::write_submessage(out, 83, v);
                }
                CombatNotifyDataMessageOneof::CombatValNotify(v) => {
                    crate::wire::write_submessage(out, 85, v);
                }
                CombatNotifyDataMessageOneof::CombatValDecayBeginNotify(v) => {
                    crate::wire::write_submessage(out, 86, v);
                }
                CombatNotifyDataMessageOneof::SquadPropChangedNotify(v) => {
                    crate::wire::write_submessage(out, 87, v);
                }
                CombatNotifyDataMessageOneof::DamageShowNotify(v) => {
                    crate::wire::write_submessage(out, 88, v);
                }
                CombatNotifyDataMessageOneof::CombatDataMaxNotify(v) => {
                    crate::wire::write_submessage(out, 100, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <CreateBulletNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::CreateBulletNotify(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <DestroyBulletNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::DestroyBulletNotify(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <DamageExecuteNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::DamageExecuteNotify(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <ApplyGameplayEffectNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ApplyGameplayEffectNotify(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveGameplayEffectNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::RemoveGameplayEffectNotify(
                    val,
                ));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::HitNotify(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <SkillNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::SkillNotify(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::UseSkillNotify(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <EndSkillNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::EndSkillNotify(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityLoadCompleteNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::EntityLoadCompleteNotify(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartUpdateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PartUpdateNotify(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartComponentInitNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PartComponentInitNotify(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <MaterialNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::MaterialNotify(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <GameplayCueNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::GameplayCueNotify(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityIsVisibleNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::EntityIsVisibleNotify(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchCharacterStateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::SwitchCharacterStateNotify(
                    val,
                ));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlayerRebackSceneNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PlayerRebackSceneNotify(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <LogicStateInitNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::LogicStateInitNotify(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchLogicStateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::SwitchLogicStateNotify(val));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <AttributeChangedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AttributeChangedNotify(val));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateChangedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AnimationStateChangedNotify(
                    val,
                ));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateInitNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AnimationStateInitNotify(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <ModifyBulletParamsNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ModifyBulletParamsNotify(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::DrownNotify(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderApplyBuffNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::OrderApplyBuffNotify(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::OrderRemoveBuffNotify(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivateBuffNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ActivateBuffNotify(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffByTagsNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::OrderRemoveBuffByTagsNotify(
                    val,
                ));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiInformationNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AiInformationNotify(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <BattleStateChangeNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BattleStateChangeNotify(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationGameplayTagNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AnimationGameplayTagNotify(
                    val,
                ));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <BoneVisibleChangeNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BoneVisibleChangeNotify(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardCdNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AiBlackboardCdNotify(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <CaughtNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::CaughtNotify(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityStaticHookMoveNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::EntityStaticHookMoveNotify(
                    val,
                ));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ChangeStateNotify(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateConfirmNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ChangeStateConfirmNotify(val));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffStackCountNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BuffStackCountNotify(val));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <MontagePlayNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::MontagePlayNotify(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <ANStartNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ANStartNotify(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmResetNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::FsmResetNotify(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <DamageRecordNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::DamageRecordNotify(val));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiHateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::AiHateNotify(val));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmBlackboardNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::FsmBlackboardNotify(val));
            }
            48 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterBattleStateChangeNotify as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatNotifyDataMessageOneof::CharacterBattleStateChangeNotify(val));
            }
            53 => {
                let bytes = r.read_len_delimited()?;
                let val = <ApplyBuffS2cRequestNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ApplyBuffS2cRequestNotify(val));
            }
            54 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveBuffS2cRequestNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::RemoveBuffS2cRequestNotify(
                    val,
                ));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActorVisibleNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ActorVisibleNotify(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <RecoverPropChangedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::RecoverPropChangedNotify(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveBuffByIdS2cRequestNotify as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatNotifyDataMessageOneof::RemoveBuffByIdS2cRequestNotify(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <ShieldUpdateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ShieldUpdateNotify(val));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <PlayerBattleStateChangeNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PlayerBattleStateChangeNotify(
                    val,
                ));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmCustomBlackboardNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::FsmCustomBlackboardNotify(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillAddNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PassiveSkillAddNotify(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillRemoveNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PassiveSkillRemoveNotify(val));
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExecuteQteNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ExecuteQteNotify(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <ModifyEntityCampNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::ModifyEntityCampNotify(val));
            }
            70 => {
                let bytes = r.read_len_delimited()?;
                let val = <AddCombineEntitiesRelationNotify as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatNotifyDataMessageOneof::AddCombineEntitiesRelationNotify(val));
            }
            71 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveCombineRelationNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::RemoveCombineRelationNotify(
                    val,
                ));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <TestDamageRecordNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::TestDamageRecordNotify(val));
            }
            73 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffDurationNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BuffDurationNotify(val));
            }
            74 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityLivingStatusNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::EntityLivingStatusNotify(val));
            }
            75 => {
                let bytes = r.read_len_delimited()?;
                let val = <NewLinkStateNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::NewLinkStateNotify(val));
            }
            76 => {
                let bytes = r.read_len_delimited()?;
                let val = <BroadcastAddBuffFailedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BroadcastAddBuffFailedNotify(
                    val,
                ));
            }
            77 => {
                let bytes = r.read_len_delimited()?;
                let val = <PackAnimChangedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::PackAnimChangedNotify(val));
            }
            78 => {
                let bytes = r.read_len_delimited()?;
                let val = <VisionTriggerNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::VisionTriggerNotify(val));
            }
            79 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveBuffByServerIdS2cRequestNotify as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatNotifyDataMessageOneof::RemoveBuffByServerIdS2cRequestNotify(val));
            }
            80 => {
                let bytes = r.read_len_delimited()?;
                let val = <TransformBuffStackNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::TransformBuffStackNotify(val));
            }
            81 => {
                let bytes = r.read_len_delimited()?;
                let val = <MotorSummonAndRideNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::MotorSummonAndRideNotify(val));
            }
            82 => {
                let bytes = r.read_len_delimited()?;
                let val = <BulletPatternNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::BulletPatternNotify(val));
            }
            83 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmMontageDurationNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::FsmMontageDurationNotify(val));
            }
            85 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatValNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::CombatValNotify(val));
            }
            86 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatValDecayBeginNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::CombatValDecayBeginNotify(val));
            }
            87 => {
                let bytes = r.read_len_delimited()?;
                let val = <SquadPropChangedNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::SquadPropChangedNotify(val));
            }
            88 => {
                let bytes = r.read_len_delimited()?;
                let val = <DamageShowNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::DamageShowNotify(val));
            }
            100 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatDataMaxNotify as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatNotifyDataMessageOneof::CombatDataMaxNotify(val));
            }
            1 => {
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
pub struct CombatDataMaxNotify {}

impl CombatDataMaxNotify {
    pub const MSG_ID: u16 = 27399;
}

impl PbMessage for CombatDataMaxNotify {
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
pub enum CombatRequestDataMessageOneof {
    CreateBulletRequest(CreateBulletRequest),
    DestroyBulletRequest(DestroyBulletRequest),
    DamageExecuteRequest(DamageExecuteRequest),
    ApplyGameplayEffectRequest(ApplyGameplayEffectRequest),
    RemoveGameplayEffectRequest(RemoveGameplayEffectRequest),
    HitRequest(HitRequest),
    HitEndRequest(HitEndRequest),
    SkillRequest(SkillRequest),
    UseSkillRequest(UseSkillRequest),
    EndSkillRequest(EndSkillRequest),
    PartUpdateRequest(PartUpdateRequest),
    MaterialRequest(MaterialRequest),
    GameplayCueRequest(GameplayCueRequest),
    EntityIsVisibleRequest(EntityIsVisibleRequest),
    SwitchCharacterStateRequest(SwitchCharacterStateRequest),
    LogicStateInitRequest(LogicStateInitRequest),
    SwitchLogicStateRequest(SwitchLogicStateRequest),
    AnimationStateChangedRequest(AnimationStateChangedRequest),
    AnimationStateInitRequest(AnimationStateInitRequest),
    ModifyBulletParamsRequest(ModifyBulletParamsRequest),
    DrownRequest(DrownRequest),
    OrderApplyBuffRequest(OrderApplyBuffRequest),
    OrderRemoveBuffRequest(OrderRemoveBuffRequest),
    ActivateBuffRequest(ActivateBuffRequest),
    OrderRemoveBuffByTagsRequest(OrderRemoveBuffByTagsRequest),
    AiInformationRequest(AiInformationRequest),
    ToughCalcExtraRatioChangeRequest(ToughCalcExtraRatioChangeRequest),
    BattleStateChangeRequest(BattleStateChangeRequest),
    AnimationGameplayTagRequest(AnimationGameplayTagRequest),
    BoneVisibleChangeRequest(BoneVisibleChangeRequest),
    AiBlackboardsRequest(AiBlackboardsRequest),
    AiBlackboardCdRequest(AiBlackboardCdRequest),
    AiHateRequest(AiHateRequest),
    MonsterBoomRequest(MonsterBoomRequest),
    CaughtRequest(CaughtRequest),
    EntityStaticHookMoveRequest(EntityStaticHookMoveRequest),
    ChangeStateRequest(ChangeStateRequest),
    ChangeStateConfirmRequest(ChangeStateConfirmRequest),
    FsmConditionPassRequest(FsmConditionPassRequest),
    BuffStackCountRequest(BuffStackCountRequest),
    ANStartRequest(ANStartRequest),
    UseSkillFailRequest(UseSkillFailRequest),
    EnterViewDirectionRequest(EnterViewDirectionRequest),
    ExitViewDirectionRequest(ExitViewDirectionRequest),
    PassiveSkillAddRequest(PassiveSkillAddRequest),
    InterruptSkillInDelayRequest(InterruptSkillInDelayRequest),
    TriggerExitSkillRequest(TriggerExitSkillRequest),
    ActorVisibleRequest(ActorVisibleRequest),
    BuffEffectRequest(BuffEffectRequest),
    FragileChangeRequest(FragileChangeRequest),
    RTimeStopRequest(RTimeStopRequest),
    DrownEndTeleportRequest(DrownEndTeleportRequest),
    MonsterDrownRequest(MonsterDrownRequest),
    PassiveSkillRemoveRequest(PassiveSkillRemoveRequest),
    RTimeStopInstRequest(RTimeStopInstRequest),
    FsmStateBehaviorRequest(FsmStateBehaviorRequest),
    FsmPlayMontageRequest(FsmPlayMontageRequest),
    TsAnimNotifyStateAbsoluteTimeStopRequest(TsAnimNotifyStateAbsoluteTimeStopRequest),
    SwitchRoleRequest(SwitchRoleRequest),
    RoleTagChangeRequest(RoleTagChangeRequest),
    ExecuteQteRequest(ExecuteQteRequest),
    CharacterAttachRequest(CharacterAttachRequest),
    CharacterDetachRequest(CharacterDetachRequest),
    ClientCurrentRoleReportRequest(ClientCurrentRoleReportRequest),
    GaSwitchCommonEnemyProCampRequest(GaSwitchCommonEnemyProCampRequest),
    ChangeSummonEntityStateRequest(ChangeSummonEntityStateRequest),
    CombatMaxCaseMessageRequest(CombatMaxCaseMessageRequest),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatRequestData {
    pub Message: Option<CombatRequestDataMessageOneof>,
    pub CombatCommon: Option<CombatCommon>,
    pub RequestId: i32,
}

impl PbMessage for CombatRequestData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatRequestDataMessageOneof::CreateBulletRequest(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                CombatRequestDataMessageOneof::DestroyBulletRequest(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                CombatRequestDataMessageOneof::DamageExecuteRequest(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                CombatRequestDataMessageOneof::ApplyGameplayEffectRequest(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                CombatRequestDataMessageOneof::RemoveGameplayEffectRequest(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                CombatRequestDataMessageOneof::HitRequest(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                CombatRequestDataMessageOneof::HitEndRequest(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                CombatRequestDataMessageOneof::SkillRequest(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                CombatRequestDataMessageOneof::UseSkillRequest(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                CombatRequestDataMessageOneof::EndSkillRequest(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                CombatRequestDataMessageOneof::PartUpdateRequest(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                CombatRequestDataMessageOneof::MaterialRequest(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                CombatRequestDataMessageOneof::GameplayCueRequest(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                CombatRequestDataMessageOneof::EntityIsVisibleRequest(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                CombatRequestDataMessageOneof::SwitchCharacterStateRequest(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                CombatRequestDataMessageOneof::LogicStateInitRequest(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                CombatRequestDataMessageOneof::SwitchLogicStateRequest(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                CombatRequestDataMessageOneof::AnimationStateChangedRequest(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                CombatRequestDataMessageOneof::AnimationStateInitRequest(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                CombatRequestDataMessageOneof::ModifyBulletParamsRequest(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                CombatRequestDataMessageOneof::DrownRequest(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                CombatRequestDataMessageOneof::OrderApplyBuffRequest(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                CombatRequestDataMessageOneof::OrderRemoveBuffRequest(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                CombatRequestDataMessageOneof::ActivateBuffRequest(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                CombatRequestDataMessageOneof::OrderRemoveBuffByTagsRequest(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                CombatRequestDataMessageOneof::AiInformationRequest(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                CombatRequestDataMessageOneof::ToughCalcExtraRatioChangeRequest(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                CombatRequestDataMessageOneof::BattleStateChangeRequest(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                CombatRequestDataMessageOneof::AnimationGameplayTagRequest(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                CombatRequestDataMessageOneof::BoneVisibleChangeRequest(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                CombatRequestDataMessageOneof::AiBlackboardsRequest(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                CombatRequestDataMessageOneof::AiBlackboardCdRequest(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                CombatRequestDataMessageOneof::AiHateRequest(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                CombatRequestDataMessageOneof::MonsterBoomRequest(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                CombatRequestDataMessageOneof::CaughtRequest(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                CombatRequestDataMessageOneof::EntityStaticHookMoveRequest(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                CombatRequestDataMessageOneof::ChangeStateRequest(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                CombatRequestDataMessageOneof::ChangeStateConfirmRequest(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                CombatRequestDataMessageOneof::FsmConditionPassRequest(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                CombatRequestDataMessageOneof::BuffStackCountRequest(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                CombatRequestDataMessageOneof::ANStartRequest(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                CombatRequestDataMessageOneof::UseSkillFailRequest(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                CombatRequestDataMessageOneof::EnterViewDirectionRequest(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                CombatRequestDataMessageOneof::ExitViewDirectionRequest(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                CombatRequestDataMessageOneof::PassiveSkillAddRequest(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                CombatRequestDataMessageOneof::InterruptSkillInDelayRequest(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                CombatRequestDataMessageOneof::TriggerExitSkillRequest(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                CombatRequestDataMessageOneof::ActorVisibleRequest(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                CombatRequestDataMessageOneof::BuffEffectRequest(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                CombatRequestDataMessageOneof::FragileChangeRequest(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                CombatRequestDataMessageOneof::RTimeStopRequest(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                CombatRequestDataMessageOneof::DrownEndTeleportRequest(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                CombatRequestDataMessageOneof::MonsterDrownRequest(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                CombatRequestDataMessageOneof::PassiveSkillRemoveRequest(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                CombatRequestDataMessageOneof::RTimeStopInstRequest(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                CombatRequestDataMessageOneof::FsmStateBehaviorRequest(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                CombatRequestDataMessageOneof::FsmPlayMontageRequest(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                CombatRequestDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopRequest(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                CombatRequestDataMessageOneof::SwitchRoleRequest(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                CombatRequestDataMessageOneof::RoleTagChangeRequest(v) => {
                    crate::wire::write_submessage(out, 67, v);
                }
                CombatRequestDataMessageOneof::ExecuteQteRequest(v) => {
                    crate::wire::write_submessage(out, 68, v);
                }
                CombatRequestDataMessageOneof::CharacterAttachRequest(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                CombatRequestDataMessageOneof::CharacterDetachRequest(v) => {
                    crate::wire::write_submessage(out, 70, v);
                }
                CombatRequestDataMessageOneof::ClientCurrentRoleReportRequest(v) => {
                    crate::wire::write_submessage(out, 71, v);
                }
                CombatRequestDataMessageOneof::GaSwitchCommonEnemyProCampRequest(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
                CombatRequestDataMessageOneof::ChangeSummonEntityStateRequest(v) => {
                    crate::wire::write_submessage(out, 73, v);
                }
                CombatRequestDataMessageOneof::CombatMaxCaseMessageRequest(v) => {
                    crate::wire::write_submessage(out, 99, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.RequestId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RequestId);
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
                let val = <CreateBulletRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::CreateBulletRequest(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <DestroyBulletRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::DestroyBulletRequest(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <DamageExecuteRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::DamageExecuteRequest(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <ApplyGameplayEffectRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ApplyGameplayEffectRequest(
                    val,
                ));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveGameplayEffectRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::RemoveGameplayEffectRequest(
                    val,
                ));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::HitRequest(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitEndRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::HitEndRequest(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <SkillRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::SkillRequest(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::UseSkillRequest(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <EndSkillRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::EndSkillRequest(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartUpdateRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::PartUpdateRequest(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <MaterialRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::MaterialRequest(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <GameplayCueRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::GameplayCueRequest(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityIsVisibleRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::EntityIsVisibleRequest(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchCharacterStateRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::SwitchCharacterStateRequest(
                    val,
                ));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <LogicStateInitRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::LogicStateInitRequest(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchLogicStateRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::SwitchLogicStateRequest(val));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateChangedRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AnimationStateChangedRequest(
                    val,
                ));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateInitRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AnimationStateInitRequest(
                    val,
                ));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <ModifyBulletParamsRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ModifyBulletParamsRequest(
                    val,
                ));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::DrownRequest(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderApplyBuffRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::OrderApplyBuffRequest(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::OrderRemoveBuffRequest(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivateBuffRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ActivateBuffRequest(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffByTagsRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::OrderRemoveBuffByTagsRequest(
                    val,
                ));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiInformationRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AiInformationRequest(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <ToughCalcExtraRatioChangeRequest as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatRequestDataMessageOneof::ToughCalcExtraRatioChangeRequest(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <BattleStateChangeRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::BattleStateChangeRequest(val));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationGameplayTagRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AnimationGameplayTagRequest(
                    val,
                ));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <BoneVisibleChangeRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::BoneVisibleChangeRequest(val));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardsRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AiBlackboardsRequest(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardCdRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AiBlackboardCdRequest(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiHateRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::AiHateRequest(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterBoomRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::MonsterBoomRequest(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <CaughtRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::CaughtRequest(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityStaticHookMoveRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::EntityStaticHookMoveRequest(
                    val,
                ));
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ChangeStateRequest(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateConfirmRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ChangeStateConfirmRequest(
                    val,
                ));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmConditionPassRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::FsmConditionPassRequest(val));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffStackCountRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::BuffStackCountRequest(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <ANStartRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ANStartRequest(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillFailRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::UseSkillFailRequest(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <EnterViewDirectionRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::EnterViewDirectionRequest(
                    val,
                ));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExitViewDirectionRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ExitViewDirectionRequest(val));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillAddRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::PassiveSkillAddRequest(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <InterruptSkillInDelayRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::InterruptSkillInDelayRequest(
                    val,
                ));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <TriggerExitSkillRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::TriggerExitSkillRequest(val));
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActorVisibleRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ActorVisibleRequest(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffEffectRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::BuffEffectRequest(val));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <FragileChangeRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::FragileChangeRequest(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::RTimeStopRequest(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownEndTeleportRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::DrownEndTeleportRequest(val));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterDrownRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::MonsterDrownRequest(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillRemoveRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::PassiveSkillRemoveRequest(
                    val,
                ));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopInstRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::RTimeStopInstRequest(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmStateBehaviorRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::FsmStateBehaviorRequest(val));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmPlayMontageRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::FsmPlayMontageRequest(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <TsAnimNotifyStateAbsoluteTimeStopRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(
                    CombatRequestDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopRequest(val),
                );
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchRoleRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::SwitchRoleRequest(val));
            }
            67 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTagChangeRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::RoleTagChangeRequest(val));
            }
            68 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExecuteQteRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::ExecuteQteRequest(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterAttachRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::CharacterAttachRequest(val));
            }
            70 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterDetachRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::CharacterDetachRequest(val));
            }
            71 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientCurrentRoleReportRequest as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatRequestDataMessageOneof::ClientCurrentRoleReportRequest(val));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <GaSwitchCommonEnemyProCampRequest as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatRequestDataMessageOneof::GaSwitchCommonEnemyProCampRequest(val));
            }
            73 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeSummonEntityStateRequest as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatRequestDataMessageOneof::ChangeSummonEntityStateRequest(val));
            }
            99 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatMaxCaseMessageRequest as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatRequestDataMessageOneof::CombatMaxCaseMessageRequest(
                    val,
                ));
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.RequestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombatResponseDataMessageOneof {
    CreateBulletResponse(CreateBulletResponse),
    DestroyBulletResponse(DestroyBulletResponse),
    DamageExecuteResponse(DamageExecuteResponse),
    ApplyGameplayEffectResponse(ApplyGameplayEffectResponse),
    RemoveGameplayEffectResponse(RemoveGameplayEffectResponse),
    HitResponse(HitResponse),
    HitEndResponse(HitEndResponse),
    SkillResponse(SkillResponse),
    UseSkillResponse(UseSkillResponse),
    EndSkillResponse(EndSkillResponse),
    PartUpdateResponse(PartUpdateResponse),
    MaterialResponse(MaterialResponse),
    GameplayCueResponse(GameplayCueResponse),
    EntityIsVisibleResponse(EntityIsVisibleResponse),
    SwitchCharacterStateResponse(SwitchCharacterStateResponse),
    LogicStateInitResponse(LogicStateInitResponse),
    SwitchLogicStateResponse(SwitchLogicStateResponse),
    AnimationStateChangedResponse(AnimationStateChangedResponse),
    AnimationStateInitResponse(AnimationStateInitResponse),
    ModifyBulletParamsResponse(ModifyBulletParamsResponse),
    DrownResponse(DrownResponse),
    OrderApplyBuffResponse(OrderApplyBuffResponse),
    OrderRemoveBuffResponse(OrderRemoveBuffResponse),
    ActivateBuffResponse(ActivateBuffResponse),
    OrderRemoveBuffByTagsResponse(OrderRemoveBuffByTagsResponse),
    AiInformationResponse(AiInformationResponse),
    ToughCalcExtraRatioChangeResponse(ToughCalcExtraRatioChangeResponse),
    BattleStateChangeResponse(BattleStateChangeResponse),
    AnimationGameplayTagResponse(AnimationGameplayTagResponse),
    BoneVisibleChangeResponse(BoneVisibleChangeResponse),
    AiBlackboardsResponse(AiBlackboardsResponse),
    AiBlackboardCdResponse(AiBlackboardCdResponse),
    AiHateResponse(AiHateResponse),
    MonsterBoomResponse(MonsterBoomResponse),
    CaughtResponse(CaughtResponse),
    EntityStaticHookMoveResponse(EntityStaticHookMoveResponse),
    ChangeStateResponse(ChangeStateResponse),
    ChangeStateConfirmResponse(ChangeStateConfirmResponse),
    FsmConditionPassResponse(FsmConditionPassResponse),
    BuffStackCountResponse(BuffStackCountResponse),
    ANStartResponse(ANStartResponse),
    UseSkillFailResponse(UseSkillFailResponse),
    EnterViewDirectionResponse(EnterViewDirectionResponse),
    ExitViewDirectionResponse(ExitViewDirectionResponse),
    PassiveSkillAddResponse(PassiveSkillAddResponse),
    InterruptSkillInDelayResponse(InterruptSkillInDelayResponse),
    TriggerExitSkillResponse(TriggerExitSkillResponse),
    ActorVisibleResponse(ActorVisibleResponse),
    BuffEffectResponse(BuffEffectResponse),
    FragileChangeResponse(FragileChangeResponse),
    RTimeStopResponse(RTimeStopResponse),
    DrownEndTeleportResponse(DrownEndTeleportResponse),
    MonsterDrownResponse(MonsterDrownResponse),
    PassiveSkillRemoveResponse(PassiveSkillRemoveResponse),
    RTimeStopInstResponse(RTimeStopInstResponse),
    FsmStateBehaviorResponse(FsmStateBehaviorResponse),
    FsmPlayMontageResponse(FsmPlayMontageResponse),
    TsAnimNotifyStateAbsoluteTimeStopResponse(TsAnimNotifyStateAbsoluteTimeStopResponse),
    SwitchRoleResponse(SwitchRoleResponse),
    RoleTagChangeResponse(RoleTagChangeResponse),
    ExecuteQteResponse(ExecuteQteResponse),
    CharacterAttachResponse(CharacterAttachResponse),
    CharacterDetachResponse(CharacterDetachResponse),
    ClientCurrentRoleReportResponse(ClientCurrentRoleReportResponse),
    GaSwitchCommonEnemyProCampResponse(GaSwitchCommonEnemyProCampResponse),
    ChangeSummonEntityStateResponse(ChangeSummonEntityStateResponse),
    CombatDataMaxResponse(CombatDataMaxResponse),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatResponseData {
    pub Message: Option<CombatResponseDataMessageOneof>,
    pub CombatCommon: Option<CombatCommon>,
    pub RequestId: i32,
}

impl PbMessage for CombatResponseData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatResponseDataMessageOneof::CreateBulletResponse(v) => {
                    crate::wire::write_submessage(out, 3, v);
                }
                CombatResponseDataMessageOneof::DestroyBulletResponse(v) => {
                    crate::wire::write_submessage(out, 4, v);
                }
                CombatResponseDataMessageOneof::DamageExecuteResponse(v) => {
                    crate::wire::write_submessage(out, 5, v);
                }
                CombatResponseDataMessageOneof::ApplyGameplayEffectResponse(v) => {
                    crate::wire::write_submessage(out, 6, v);
                }
                CombatResponseDataMessageOneof::RemoveGameplayEffectResponse(v) => {
                    crate::wire::write_submessage(out, 7, v);
                }
                CombatResponseDataMessageOneof::HitResponse(v) => {
                    crate::wire::write_submessage(out, 8, v);
                }
                CombatResponseDataMessageOneof::HitEndResponse(v) => {
                    crate::wire::write_submessage(out, 9, v);
                }
                CombatResponseDataMessageOneof::SkillResponse(v) => {
                    crate::wire::write_submessage(out, 10, v);
                }
                CombatResponseDataMessageOneof::UseSkillResponse(v) => {
                    crate::wire::write_submessage(out, 11, v);
                }
                CombatResponseDataMessageOneof::EndSkillResponse(v) => {
                    crate::wire::write_submessage(out, 12, v);
                }
                CombatResponseDataMessageOneof::PartUpdateResponse(v) => {
                    crate::wire::write_submessage(out, 13, v);
                }
                CombatResponseDataMessageOneof::MaterialResponse(v) => {
                    crate::wire::write_submessage(out, 14, v);
                }
                CombatResponseDataMessageOneof::GameplayCueResponse(v) => {
                    crate::wire::write_submessage(out, 15, v);
                }
                CombatResponseDataMessageOneof::EntityIsVisibleResponse(v) => {
                    crate::wire::write_submessage(out, 16, v);
                }
                CombatResponseDataMessageOneof::SwitchCharacterStateResponse(v) => {
                    crate::wire::write_submessage(out, 17, v);
                }
                CombatResponseDataMessageOneof::LogicStateInitResponse(v) => {
                    crate::wire::write_submessage(out, 18, v);
                }
                CombatResponseDataMessageOneof::SwitchLogicStateResponse(v) => {
                    crate::wire::write_submessage(out, 19, v);
                }
                CombatResponseDataMessageOneof::AnimationStateChangedResponse(v) => {
                    crate::wire::write_submessage(out, 20, v);
                }
                CombatResponseDataMessageOneof::AnimationStateInitResponse(v) => {
                    crate::wire::write_submessage(out, 21, v);
                }
                CombatResponseDataMessageOneof::ModifyBulletParamsResponse(v) => {
                    crate::wire::write_submessage(out, 22, v);
                }
                CombatResponseDataMessageOneof::DrownResponse(v) => {
                    crate::wire::write_submessage(out, 23, v);
                }
                CombatResponseDataMessageOneof::OrderApplyBuffResponse(v) => {
                    crate::wire::write_submessage(out, 24, v);
                }
                CombatResponseDataMessageOneof::OrderRemoveBuffResponse(v) => {
                    crate::wire::write_submessage(out, 25, v);
                }
                CombatResponseDataMessageOneof::ActivateBuffResponse(v) => {
                    crate::wire::write_submessage(out, 26, v);
                }
                CombatResponseDataMessageOneof::OrderRemoveBuffByTagsResponse(v) => {
                    crate::wire::write_submessage(out, 27, v);
                }
                CombatResponseDataMessageOneof::AiInformationResponse(v) => {
                    crate::wire::write_submessage(out, 28, v);
                }
                CombatResponseDataMessageOneof::ToughCalcExtraRatioChangeResponse(v) => {
                    crate::wire::write_submessage(out, 29, v);
                }
                CombatResponseDataMessageOneof::BattleStateChangeResponse(v) => {
                    crate::wire::write_submessage(out, 30, v);
                }
                CombatResponseDataMessageOneof::AnimationGameplayTagResponse(v) => {
                    crate::wire::write_submessage(out, 31, v);
                }
                CombatResponseDataMessageOneof::BoneVisibleChangeResponse(v) => {
                    crate::wire::write_submessage(out, 32, v);
                }
                CombatResponseDataMessageOneof::AiBlackboardsResponse(v) => {
                    crate::wire::write_submessage(out, 33, v);
                }
                CombatResponseDataMessageOneof::AiBlackboardCdResponse(v) => {
                    crate::wire::write_submessage(out, 34, v);
                }
                CombatResponseDataMessageOneof::AiHateResponse(v) => {
                    crate::wire::write_submessage(out, 35, v);
                }
                CombatResponseDataMessageOneof::MonsterBoomResponse(v) => {
                    crate::wire::write_submessage(out, 36, v);
                }
                CombatResponseDataMessageOneof::CaughtResponse(v) => {
                    crate::wire::write_submessage(out, 37, v);
                }
                CombatResponseDataMessageOneof::EntityStaticHookMoveResponse(v) => {
                    crate::wire::write_submessage(out, 38, v);
                }
                CombatResponseDataMessageOneof::ChangeStateResponse(v) => {
                    crate::wire::write_submessage(out, 39, v);
                }
                CombatResponseDataMessageOneof::ChangeStateConfirmResponse(v) => {
                    crate::wire::write_submessage(out, 40, v);
                }
                CombatResponseDataMessageOneof::FsmConditionPassResponse(v) => {
                    crate::wire::write_submessage(out, 41, v);
                }
                CombatResponseDataMessageOneof::BuffStackCountResponse(v) => {
                    crate::wire::write_submessage(out, 42, v);
                }
                CombatResponseDataMessageOneof::ANStartResponse(v) => {
                    crate::wire::write_submessage(out, 43, v);
                }
                CombatResponseDataMessageOneof::UseSkillFailResponse(v) => {
                    crate::wire::write_submessage(out, 44, v);
                }
                CombatResponseDataMessageOneof::EnterViewDirectionResponse(v) => {
                    crate::wire::write_submessage(out, 45, v);
                }
                CombatResponseDataMessageOneof::ExitViewDirectionResponse(v) => {
                    crate::wire::write_submessage(out, 46, v);
                }
                CombatResponseDataMessageOneof::PassiveSkillAddResponse(v) => {
                    crate::wire::write_submessage(out, 47, v);
                }
                CombatResponseDataMessageOneof::InterruptSkillInDelayResponse(v) => {
                    crate::wire::write_submessage(out, 49, v);
                }
                CombatResponseDataMessageOneof::TriggerExitSkillResponse(v) => {
                    crate::wire::write_submessage(out, 50, v);
                }
                CombatResponseDataMessageOneof::ActorVisibleResponse(v) => {
                    crate::wire::write_submessage(out, 55, v);
                }
                CombatResponseDataMessageOneof::BuffEffectResponse(v) => {
                    crate::wire::write_submessage(out, 56, v);
                }
                CombatResponseDataMessageOneof::FragileChangeResponse(v) => {
                    crate::wire::write_submessage(out, 57, v);
                }
                CombatResponseDataMessageOneof::RTimeStopResponse(v) => {
                    crate::wire::write_submessage(out, 58, v);
                }
                CombatResponseDataMessageOneof::DrownEndTeleportResponse(v) => {
                    crate::wire::write_submessage(out, 59, v);
                }
                CombatResponseDataMessageOneof::MonsterDrownResponse(v) => {
                    crate::wire::write_submessage(out, 60, v);
                }
                CombatResponseDataMessageOneof::PassiveSkillRemoveResponse(v) => {
                    crate::wire::write_submessage(out, 61, v);
                }
                CombatResponseDataMessageOneof::RTimeStopInstResponse(v) => {
                    crate::wire::write_submessage(out, 62, v);
                }
                CombatResponseDataMessageOneof::FsmStateBehaviorResponse(v) => {
                    crate::wire::write_submessage(out, 63, v);
                }
                CombatResponseDataMessageOneof::FsmPlayMontageResponse(v) => {
                    crate::wire::write_submessage(out, 64, v);
                }
                CombatResponseDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopResponse(v) => {
                    crate::wire::write_submessage(out, 65, v);
                }
                CombatResponseDataMessageOneof::SwitchRoleResponse(v) => {
                    crate::wire::write_submessage(out, 66, v);
                }
                CombatResponseDataMessageOneof::RoleTagChangeResponse(v) => {
                    crate::wire::write_submessage(out, 67, v);
                }
                CombatResponseDataMessageOneof::ExecuteQteResponse(v) => {
                    crate::wire::write_submessage(out, 68, v);
                }
                CombatResponseDataMessageOneof::CharacterAttachResponse(v) => {
                    crate::wire::write_submessage(out, 69, v);
                }
                CombatResponseDataMessageOneof::CharacterDetachResponse(v) => {
                    crate::wire::write_submessage(out, 70, v);
                }
                CombatResponseDataMessageOneof::ClientCurrentRoleReportResponse(v) => {
                    crate::wire::write_submessage(out, 71, v);
                }
                CombatResponseDataMessageOneof::GaSwitchCommonEnemyProCampResponse(v) => {
                    crate::wire::write_submessage(out, 72, v);
                }
                CombatResponseDataMessageOneof::ChangeSummonEntityStateResponse(v) => {
                    crate::wire::write_submessage(out, 73, v);
                }
                CombatResponseDataMessageOneof::CombatDataMaxResponse(v) => {
                    crate::wire::write_submessage(out, 100, v);
                }
            }
        }
        if let Some(v) = &self.CombatCommon {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.RequestId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RequestId);
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
                let val = <CreateBulletResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::CreateBulletResponse(val));
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                let val = <DestroyBulletResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::DestroyBulletResponse(val));
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                let val = <DamageExecuteResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::DamageExecuteResponse(val));
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                let val = <ApplyGameplayEffectResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ApplyGameplayEffectResponse(
                    val,
                ));
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                let val = <RemoveGameplayEffectResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::RemoveGameplayEffectResponse(val));
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::HitResponse(val));
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                let val = <HitEndResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::HitEndResponse(val));
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                let val = <SkillResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::SkillResponse(val));
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::UseSkillResponse(val));
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                let val = <EndSkillResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::EndSkillResponse(val));
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                let val = <PartUpdateResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::PartUpdateResponse(val));
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                let val = <MaterialResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::MaterialResponse(val));
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                let val = <GameplayCueResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::GameplayCueResponse(val));
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityIsVisibleResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::EntityIsVisibleResponse(val));
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchCharacterStateResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::SwitchCharacterStateResponse(val));
            }
            18 => {
                let bytes = r.read_len_delimited()?;
                let val = <LogicStateInitResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::LogicStateInitResponse(val));
            }
            19 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchLogicStateResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::SwitchLogicStateResponse(
                    val,
                ));
            }
            20 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateChangedResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::AnimationStateChangedResponse(val));
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationStateInitResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::AnimationStateInitResponse(
                    val,
                ));
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                let val = <ModifyBulletParamsResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ModifyBulletParamsResponse(
                    val,
                ));
            }
            23 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::DrownResponse(val));
            }
            24 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderApplyBuffResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::OrderApplyBuffResponse(val));
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::OrderRemoveBuffResponse(val));
            }
            26 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActivateBuffResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ActivateBuffResponse(val));
            }
            27 => {
                let bytes = r.read_len_delimited()?;
                let val = <OrderRemoveBuffByTagsResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::OrderRemoveBuffByTagsResponse(val));
            }
            28 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiInformationResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::AiInformationResponse(val));
            }
            29 => {
                let bytes = r.read_len_delimited()?;
                let val = <ToughCalcExtraRatioChangeResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::ToughCalcExtraRatioChangeResponse(val));
            }
            30 => {
                let bytes = r.read_len_delimited()?;
                let val = <BattleStateChangeResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::BattleStateChangeResponse(
                    val,
                ));
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                let val = <AnimationGameplayTagResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::AnimationGameplayTagResponse(val));
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                let val = <BoneVisibleChangeResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::BoneVisibleChangeResponse(
                    val,
                ));
            }
            33 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardsResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::AiBlackboardsResponse(val));
            }
            34 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiBlackboardCdResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::AiBlackboardCdResponse(val));
            }
            35 => {
                let bytes = r.read_len_delimited()?;
                let val = <AiHateResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::AiHateResponse(val));
            }
            36 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterBoomResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::MonsterBoomResponse(val));
            }
            37 => {
                let bytes = r.read_len_delimited()?;
                let val = <CaughtResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::CaughtResponse(val));
            }
            38 => {
                let bytes = r.read_len_delimited()?;
                let val = <EntityStaticHookMoveResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::EntityStaticHookMoveResponse(val));
            }
            39 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ChangeStateResponse(val));
            }
            40 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeStateConfirmResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ChangeStateConfirmResponse(
                    val,
                ));
            }
            41 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmConditionPassResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::FsmConditionPassResponse(
                    val,
                ));
            }
            42 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffStackCountResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::BuffStackCountResponse(val));
            }
            43 => {
                let bytes = r.read_len_delimited()?;
                let val = <ANStartResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ANStartResponse(val));
            }
            44 => {
                let bytes = r.read_len_delimited()?;
                let val = <UseSkillFailResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::UseSkillFailResponse(val));
            }
            45 => {
                let bytes = r.read_len_delimited()?;
                let val = <EnterViewDirectionResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::EnterViewDirectionResponse(
                    val,
                ));
            }
            46 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExitViewDirectionResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ExitViewDirectionResponse(
                    val,
                ));
            }
            47 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillAddResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::PassiveSkillAddResponse(val));
            }
            49 => {
                let bytes = r.read_len_delimited()?;
                let val = <InterruptSkillInDelayResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::InterruptSkillInDelayResponse(val));
            }
            50 => {
                let bytes = r.read_len_delimited()?;
                let val = <TriggerExitSkillResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::TriggerExitSkillResponse(
                    val,
                ));
            }
            55 => {
                let bytes = r.read_len_delimited()?;
                let val = <ActorVisibleResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ActorVisibleResponse(val));
            }
            56 => {
                let bytes = r.read_len_delimited()?;
                let val = <BuffEffectResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::BuffEffectResponse(val));
            }
            57 => {
                let bytes = r.read_len_delimited()?;
                let val = <FragileChangeResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::FragileChangeResponse(val));
            }
            58 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::RTimeStopResponse(val));
            }
            59 => {
                let bytes = r.read_len_delimited()?;
                let val = <DrownEndTeleportResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::DrownEndTeleportResponse(
                    val,
                ));
            }
            60 => {
                let bytes = r.read_len_delimited()?;
                let val = <MonsterDrownResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::MonsterDrownResponse(val));
            }
            61 => {
                let bytes = r.read_len_delimited()?;
                let val = <PassiveSkillRemoveResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::PassiveSkillRemoveResponse(
                    val,
                ));
            }
            62 => {
                let bytes = r.read_len_delimited()?;
                let val = <RTimeStopInstResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::RTimeStopInstResponse(val));
            }
            63 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmStateBehaviorResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::FsmStateBehaviorResponse(
                    val,
                ));
            }
            64 => {
                let bytes = r.read_len_delimited()?;
                let val = <FsmPlayMontageResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::FsmPlayMontageResponse(val));
            }
            65 => {
                let bytes = r.read_len_delimited()?;
                let val = <TsAnimNotifyStateAbsoluteTimeStopResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(
                    CombatResponseDataMessageOneof::TsAnimNotifyStateAbsoluteTimeStopResponse(val),
                );
            }
            66 => {
                let bytes = r.read_len_delimited()?;
                let val = <SwitchRoleResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::SwitchRoleResponse(val));
            }
            67 => {
                let bytes = r.read_len_delimited()?;
                let val = <RoleTagChangeResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::RoleTagChangeResponse(val));
            }
            68 => {
                let bytes = r.read_len_delimited()?;
                let val = <ExecuteQteResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::ExecuteQteResponse(val));
            }
            69 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterAttachResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::CharacterAttachResponse(val));
            }
            70 => {
                let bytes = r.read_len_delimited()?;
                let val = <CharacterDetachResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::CharacterDetachResponse(val));
            }
            71 => {
                let bytes = r.read_len_delimited()?;
                let val = <ClientCurrentRoleReportResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::ClientCurrentRoleReportResponse(val));
            }
            72 => {
                let bytes = r.read_len_delimited()?;
                let val = <GaSwitchCommonEnemyProCampResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::GaSwitchCommonEnemyProCampResponse(val));
            }
            73 => {
                let bytes = r.read_len_delimited()?;
                let val = <ChangeSummonEntityStateResponse as PbMessage>::decode(bytes)?;
                self.Message =
                    Some(CombatResponseDataMessageOneof::ChangeSummonEntityStateResponse(val));
            }
            100 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatDataMaxResponse as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatResponseDataMessageOneof::CombatDataMaxResponse(val));
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.CombatCommon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.RequestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatDataMaxResponse {}

impl PbMessage for CombatDataMaxResponse {
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
pub enum CombatSendDataMessageOneof {
    Push(CombatPushData),
    Request(CombatRequestData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatSendData {
    pub Message: Option<CombatSendDataMessageOneof>,
}

impl PbMessage for CombatSendData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatSendDataMessageOneof::Push(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                CombatSendDataMessageOneof::Request(v) => {
                    crate::wire::write_submessage(out, 3, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatPushData as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatSendDataMessageOneof::Push(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatRequestData as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatSendDataMessageOneof::Request(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombatReceiveDataMessageOneof {
    CombatNotifyData(CombatNotifyData),
    CombatResponseData(CombatResponseData),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatReceiveData {
    pub Message: Option<CombatReceiveDataMessageOneof>,
}

impl PbMessage for CombatReceiveData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(oneof) = &self.Message {
            match oneof {
                CombatReceiveDataMessageOneof::CombatNotifyData(v) => {
                    crate::wire::write_submessage(out, 2, v);
                }
                CombatReceiveDataMessageOneof::CombatResponseData(v) => {
                    crate::wire::write_submessage(out, 3, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatNotifyData as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatReceiveDataMessageOneof::CombatNotifyData(val));
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                let val = <CombatResponseData as PbMessage>::decode(bytes)?;
                self.Message = Some(CombatReceiveDataMessageOneof::CombatResponseData(val));
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatSendPackRequest {
    pub Data: Vec<CombatSendData>,
    pub HostPlayerId: i32,
}

impl CombatSendPackRequest {
    pub const MSG_ID: u16 = 26497;
}

impl PbMessage for CombatSendPackRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<CombatSendData as PbMessage>::decode(bytes)?);
            }
            10 => self.HostPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatSendPackResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ReceivePackNotify: Option<CombatReceivePackNotify>,
}

impl CombatSendPackResponse {
    pub const MSG_ID: u16 = 21536;
}

impl PbMessage for CombatSendPackResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ReceivePackNotify {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ReceivePackNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatReceivePackNotify {
    pub Data: Vec<CombatReceiveData>,
}

impl CombatReceivePackNotify {
    pub const MSG_ID: u16 = 29973;
}

impl PbMessage for CombatReceivePackNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
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
                self.Data
                    .push(<CombatReceiveData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatMaxCaseMessageRequest {}

impl CombatMaxCaseMessageRequest {
    pub const MSG_ID: u16 = 28528;
}

impl PbMessage for CombatMaxCaseMessageRequest {
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
pub struct CombatMaxCaseMessageResponse {}

impl CombatMaxCaseMessageResponse {
    pub const MSG_ID: u16 = 15035;
}

impl PbMessage for CombatMaxCaseMessageResponse {
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
pub struct CombatMessagePrefixInfoRequest {
    pub Id: Vec<u8>,
}

impl CombatMessagePrefixInfoRequest {
    pub const MSG_ID: u16 = 28795;
}

impl PbMessage for CombatMessagePrefixInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
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
            8 => self.Id = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatMessagePrefixInfoResponse {
    pub PrefixInfo: Vec<u8>,
}

impl CombatMessagePrefixInfoResponse {
    pub const MSG_ID: u16 = 16765;
}

impl PbMessage for CombatMessagePrefixInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PrefixInfo.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PrefixInfo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.PrefixInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatMessagePostInfoRequest {
    pub PostId: Vec<u8>,
}

impl CombatMessagePostInfoRequest {
    pub const MSG_ID: u16 = 17769;
}

impl PbMessage for CombatMessagePostInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PostId.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PostId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.PostId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombatMessagePostInfoResponse {
    pub PostInfo: Vec<u8>,
}

impl CombatMessagePostInfoResponse {
    pub const MSG_ID: u16 = 29434;
}

impl PbMessage for CombatMessagePostInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PostInfo.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PostInfo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.PostInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityLoadCompleteNotify {
    pub PlayerId: i32,
    pub EntityIds: Vec<i64>,
    pub EntityIdsUnload: Vec<i64>,
}

impl EntityLoadCompleteNotify {
    pub const MSG_ID: u16 = 21435;
}

impl PbMessage for EntityLoadCompleteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        for item in &self.EntityIdsUnload {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EntityIdsUnload,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MaterialRequest {
    pub MaterialInfo: Option<MaterialInfo>,
    pub CombatCommon: Option<CombatCommon>,
}

impl MaterialRequest {
    pub const MSG_ID: u16 = 17530;
}

impl PbMessage for MaterialRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MaterialInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.CombatCommon {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MaterialInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
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
pub struct MaterialPush {
    pub MaterialInfo: Option<MaterialInfo>,
    pub CombatCommon: Option<CombatCommon>,
}

impl MaterialPush {
    pub const MSG_ID: u16 = 27552;
}

impl PbMessage for MaterialPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MaterialInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.CombatCommon {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MaterialInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
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
pub struct MaterialInfo {
    pub EntityId: i64,
    pub AssetName: Vec<u8>,
    pub IsGroup: bool,
}

impl PbMessage for MaterialInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.AssetName.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AssetName);
        }
        if self.IsGroup {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.AssetName = r.read_len_delimited()?.to_vec(),
            3 => self.IsGroup = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MaterialResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MaterialResponse {
    pub const MSG_ID: u16 = 26538;
}

impl PbMessage for MaterialResponse {
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
pub struct MaterialNotify {
    pub MaterialInfo: Option<MaterialInfo>,
    pub CombatCommon: Option<CombatCommon>,
}

impl MaterialNotify {
    pub const MSG_ID: u16 = 26008;
}

impl PbMessage for MaterialNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MaterialInfo {
            crate::wire::write_submessage(out, 2, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MaterialInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
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
pub struct DynamicEntityInformation {
    pub Id: i64,
    pub EntityType: Option<EEntityType>,
    pub ConfigId: i32,
    pub PlayerId: i32,
    pub OwnerId: i64,
    pub MovementInformation: Option<MovementInformation>,
    pub GameAttributes: Vec<GameplayAttributeData>,
    pub InitAttribute: bool,
    pub IsVisible: bool,
    pub AnimationStates: Vec<i32>,
    pub InitGameplayTag: bool,
    pub GameplayTags: Vec<GameplayTagData>,
    pub Level: i32,
    pub BlackboardParams: Vec<BlackboardParam>,
    pub Tags: Vec<Vec<u8>>,
    pub PrivateTags: Vec<PrivateTag>,
    pub DeathStatus: bool,
    pub HardnessModeId: i32,
    pub PartLifeInfos: Vec<PartInformation>,
    pub VisionSkillInfos: Vec<VisionSkillInformation>,
    pub FightBuffInfos: Vec<FightBuffInformation>,
    pub CreatureGroup: i32,
    pub ListenInformation: Option<ListenInformation>,
    pub SysBuffInfos: Vec<SysBuffInformation>,
    pub LivingStatus: Option<LivingStatus>,
    pub EntityCommonTags: Vec<i32>,
    pub WeaponConfId: i32,
    pub DurabilityValue: i32,
    pub InitLocation: Option<Vector>,
    pub SummonInfo: Option<SummonInfo>,
    pub ComponentPbs: Vec<EntityComponentPb>,
}

impl PbMessage for DynamicEntityInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.EntityType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerId);
        }
        if let Some(v) = &self.MovementInformation {
            crate::wire::write_submessage(out, 6, v);
        }
        for item in &self.GameAttributes {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.InitAttribute {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsVisible {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.AnimationStates {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InitGameplayTag {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.GameplayTags {
            crate::wire::write_submessage(out, 12, item);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        for item in &self.BlackboardParams {
            crate::wire::write_submessage(out, 14, item);
        }
        for item in &self.Tags {
            crate::wire::write_tag(out, 15, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.PrivateTags {
            crate::wire::write_submessage(out, 16, item);
        }
        if self.DeathStatus {
            crate::wire::write_tag(out, 17, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HardnessModeId != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HardnessModeId);
        }
        for item in &self.PartLifeInfos {
            crate::wire::write_submessage(out, 20, item);
        }
        for item in &self.VisionSkillInfos {
            crate::wire::write_submessage(out, 21, item);
        }
        for item in &self.FightBuffInfos {
            crate::wire::write_submessage(out, 22, item);
        }
        if self.CreatureGroup != 0 {
            crate::wire::write_tag(out, 23, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CreatureGroup);
        }
        if let Some(v) = &self.ListenInformation {
            crate::wire::write_submessage(out, 24, v);
        }
        for item in &self.SysBuffInfos {
            crate::wire::write_submessage(out, 25, item);
        }
        if let Some(v) = &self.LivingStatus {
            crate::wire::write_tag(out, 26, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.EntityCommonTags {
            crate::wire::write_tag(out, 27, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.WeaponConfId != 0 {
            crate::wire::write_tag(out, 28, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponConfId);
        }
        if self.DurabilityValue != 0 {
            crate::wire::write_tag(out, 29, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DurabilityValue);
        }
        if let Some(v) = &self.InitLocation {
            crate::wire::write_submessage(out, 30, v);
        }
        if let Some(v) = &self.SummonInfo {
            crate::wire::write_submessage(out, 31, v);
        }
        for item in &self.ComponentPbs {
            crate::wire::write_submessage(out, 32, item);
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
            2 => {
                self.EntityType = Some(<EEntityType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            3 => self.ConfigId = r.read_varint_i32()?,
            4 => self.PlayerId = r.read_varint_i32()?,
            5 => self.OwnerId = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MovementInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.GameAttributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            8 => self.InitAttribute = r.read_bool()?,
            9 => self.IsVisible = r.read_bool()?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AnimationStates,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => self.InitGameplayTag = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.GameplayTags
                    .push(<GameplayTagData as PbMessage>::decode(bytes)?);
            }
            13 => self.Level = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.BlackboardParams
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            15 => self.Tags.push(r.read_len_delimited()?.to_vec()),
            16 => {
                let bytes = r.read_len_delimited()?;
                self.PrivateTags
                    .push(<PrivateTag as PbMessage>::decode(bytes)?);
            }
            17 => self.DeathStatus = r.read_bool()?,
            19 => self.HardnessModeId = r.read_varint_i32()?,
            20 => {
                let bytes = r.read_len_delimited()?;
                self.PartLifeInfos
                    .push(<PartInformation as PbMessage>::decode(bytes)?);
            }
            21 => {
                let bytes = r.read_len_delimited()?;
                self.VisionSkillInfos
                    .push(<VisionSkillInformation as PbMessage>::decode(bytes)?);
            }
            22 => {
                let bytes = r.read_len_delimited()?;
                self.FightBuffInfos
                    .push(<FightBuffInformation as PbMessage>::decode(bytes)?);
            }
            23 => self.CreatureGroup = r.read_varint_i32()?,
            24 => {
                let bytes = r.read_len_delimited()?;
                self.ListenInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            25 => {
                let bytes = r.read_len_delimited()?;
                self.SysBuffInfos
                    .push(<SysBuffInformation as PbMessage>::decode(bytes)?);
            }
            26 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            27 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EntityCommonTags,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            28 => self.WeaponConfId = r.read_varint_i32()?,
            29 => self.DurabilityValue = r.read_varint_i32()?,
            30 => {
                let bytes = r.read_len_delimited()?;
                self.InitLocation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            31 => {
                let bytes = r.read_len_delimited()?;
                self.SummonInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            32 => {
                let bytes = r.read_len_delimited()?;
                self.ComponentPbs
                    .push(<EntityComponentPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityLivingStatusNotify {
    pub Id: i64,
    pub LivingStatus: Option<LivingStatus>,
    pub DropVisionItem: Vec<DropVisionItemResult>,
}

impl EntityLivingStatusNotify {
    pub const MSG_ID: u16 = 24917;
}

impl PbMessage for EntityLivingStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.LivingStatus {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DropVisionItem {
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
            6 => self.Id = r.read_varint_i64()?,
            7 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.DropVisionItem
                    .push(<DropVisionItemResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropVisionItemResult {
    pub PlayerId: i32,
    pub Drop: bool,
}

impl PbMessage for DropVisionItemResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Drop {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.Drop = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListenInformation {
    pub Id: Vec<i32>,
    pub Range: f32,
}

impl PbMessage for ListenInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Id {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Range != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Range).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Id, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.Range = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScenePlayerInformation {
    pub PlayerId: i32,
    pub PlayerName: Vec<u8>,
    pub PlayerIcon: i32,
    pub Level: i32,
    pub GuildName: Vec<u8>,
    pub GuildIntro: Vec<u8>,
    pub Location: Option<Vector>,
    pub IsOffline: bool,
    pub PlayerPrefix: i32,
    pub PlayerGEIncHandle: i32,
    pub FightRoleInfos: Vec<FightRoleInfos>,
    pub Rotation: Option<Rotator>,
    pub GroupType: i32,
    pub CurRole: i32,
    pub VehiclePlayerData: Option<VehiclePlayerData>,
    pub Gravity: Option<Vector>,
    pub AreaId: i32,
}

impl PbMessage for ScenePlayerInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
        if self.PlayerIcon != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerIcon);
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
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.IsOffline {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerPrefix != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerPrefix);
        }
        if self.PlayerGEIncHandle != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerGEIncHandle);
        }
        for item in &self.FightRoleInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.GroupType != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupType);
        }
        if self.CurRole != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRole);
        }
        if let Some(v) = &self.VehiclePlayerData {
            crate::wire::write_submessage(out, 16, v);
        }
        if let Some(v) = &self.Gravity {
            crate::wire::write_submessage(out, 17, v);
        }
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
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
            3 => self.PlayerIcon = r.read_varint_i32()?,
            4 => self.Level = r.read_varint_i32()?,
            5 => self.GuildName = r.read_len_delimited()?.to_vec(),
            6 => self.GuildIntro = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.IsOffline = r.read_bool()?,
            9 => self.PlayerPrefix = r.read_varint_i32()?,
            10 => self.PlayerGEIncHandle = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfos
                    .push(<FightRoleInfos as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.GroupType = r.read_varint_i32()?,
            15 => self.CurRole = r.read_varint_i32()?,
            16 => {
                let bytes = r.read_len_delimited()?;
                self.VehiclePlayerData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                self.Gravity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            18 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GroupFormation {
    pub PlayerId: i32,
    pub FightRoleInfos: Vec<FightRoleInfos>,
    pub CurrentGroupType: i32,
}

impl PbMessage for GroupFormation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.FightRoleInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.CurrentGroupType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentGroupType);
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
                self.FightRoleInfos
                    .push(<FightRoleInfos as PbMessage>::decode(bytes)?);
            }
            3 => self.CurrentGroupType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightRoleInfo {
    pub RoleId: i32,
    pub EntityId: i64,
    pub OnStageWithoutControl: bool,
}

impl PbMessage for FightRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.OnStageWithoutControl {
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
            3 => self.OnStageWithoutControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightRoleInfos {
    pub GroupType: i32,
    pub FightRoleInfos: Vec<FightRoleInfo>,
    pub CurRole: i32,
    pub LivingStatus: Option<LivingStatus>,
    pub IsFixedLocation: bool,
}

impl PbMessage for FightRoleInfos {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupType);
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
        if self.IsFixedLocation {
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
            1 => self.GroupType = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FightRoleInfos
                    .push(<FightRoleInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.CurRole = r.read_varint_i32()?,
            4 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            5 => self.IsFixedLocation = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UpdateGroupFormationNotify {
    pub GroupFormation: Vec<GroupFormation>,
}

impl UpdateGroupFormationNotify {
    pub const MSG_ID: u16 = 22332;
}

impl PbMessage for UpdateGroupFormationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GroupFormation {
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
                self.GroupFormation
                    .push(<GroupFormation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneInformation {
    pub SceneId: Vec<u8>,
    pub InstanceId: i32,
    pub OwnerId: i32,
    pub PlayerInfos: Vec<ScenePlayerInformation>,
    pub DynamicEntityList: Vec<DynamicEntityInformation>,
    pub BlackboardParams: Vec<BlackboardParam>,
    pub EndTime: i64,
    pub AoiData: Option<PlayerSceneAoiData>,
    pub OwnerFinishMingSuGens: Vec<i64>,
    pub Mode: Option<SceneMode>,
    pub TimeInfo: Option<SceneTimeInfo>,
    pub HostFogIds: Vec<i32>,
    pub LoadedSubLevels: Vec<Vec<u8>>,
    pub AreaStates: Vec<SceneAreaState>,
    pub ResetPointEntityId: i32,
    pub DataLayers: Vec<i32>,
    pub AreaMpc: BTreeMap<i32, i32>,
    pub CurContextId: i64,
    pub AudioState: Vec<AudioState>,
    pub SceneBulletOwnerId: i64,
    pub SceneTraceId: i64,
    pub HideSubLevels: Vec<Vec<u8>>,
    pub LastHighLevelArea: i32,
    pub EnableRoads: Vec<i32>,
}

impl PbMessage for SceneInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SceneId);
        }
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerId);
        }
        for item in &self.PlayerInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.DynamicEntityList {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.BlackboardParams {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if let Some(v) = &self.AoiData {
            crate::wire::write_submessage(out, 11, v);
        }
        for item in &self.OwnerFinishMingSuGens {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.Mode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.TimeInfo {
            crate::wire::write_submessage(out, 14, v);
        }
        for item in &self.HostFogIds {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LoadedSubLevels {
            crate::wire::write_tag(out, 16, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.AreaStates {
            crate::wire::write_submessage(out, 17, item);
        }
        if self.ResetPointEntityId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResetPointEntityId);
        }
        for item in &self.DataLayers {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.AreaMpc.is_empty() {
            crate::wire::write_map(out, 20, &self.AreaMpc);
        }
        if self.CurContextId != 0 {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurContextId);
        }
        for item in &self.AudioState {
            crate::wire::write_submessage(out, 23, item);
        }
        if self.SceneBulletOwnerId != 0 {
            crate::wire::write_tag(out, 24, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SceneBulletOwnerId);
        }
        if self.SceneTraceId != 0 {
            crate::wire::write_tag(out, 25, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SceneTraceId);
        }
        for item in &self.HideSubLevels {
            crate::wire::write_tag(out, 26, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.LastHighLevelArea != 0 {
            crate::wire::write_tag(out, 27, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastHighLevelArea);
        }
        for item in &self.EnableRoads {
            crate::wire::write_tag(out, 28, crate::wire::WireType::Varint);
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
            1 => self.SceneId = r.read_len_delimited()?.to_vec(),
            2 => self.InstanceId = r.read_varint_i32()?,
            3 => self.OwnerId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfos
                    .push(<ScenePlayerInformation as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DynamicEntityList
                    .push(<DynamicEntityInformation as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.BlackboardParams
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            8 => self.EndTime = r.read_varint_i64()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.AoiData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.OwnerFinishMingSuGens,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            13 => {
                self.Mode = Some(<SceneMode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.TimeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HostFogIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            16 => self.LoadedSubLevels.push(r.read_len_delimited()?.to_vec()),
            17 => {
                let bytes = r.read_len_delimited()?;
                self.AreaStates
                    .push(<SceneAreaState as PbMessage>::decode(bytes)?);
            }
            18 => self.ResetPointEntityId = r.read_varint_i32()?,
            19 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DataLayers, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            20 => crate::wire::decode_map_entry(r, &mut self.AreaMpc)?,
            21 => self.CurContextId = r.read_varint_i64()?,
            23 => {
                let bytes = r.read_len_delimited()?;
                self.AudioState
                    .push(<AudioState as PbMessage>::decode(bytes)?);
            }
            24 => self.SceneBulletOwnerId = r.read_varint_i64()?,
            25 => self.SceneTraceId = r.read_varint_i64()?,
            26 => self.HideSubLevels.push(r.read_len_delimited()?.to_vec()),
            27 => self.LastHighLevelArea = r.read_varint_i32()?,
            28 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EnableRoads, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneAreaState {
    pub AreaId: i32,
    pub State: bool,
}

impl PbMessage for SceneAreaState {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        if self.State {
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
            1 => self.AreaId = r.read_varint_i32()?,
            2 => self.State = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HostTeleportUnlockNotify {
    pub HostPlayerId: i32,
    pub HostTeleportId: i32,
}

impl HostTeleportUnlockNotify {
    pub const MSG_ID: u16 = 25070;
}

impl PbMessage for HostTeleportUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.HostTeleportId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostTeleportId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.HostPlayerId = r.read_varint_i32()?,
            11 => self.HostTeleportId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneTimeInfo {
    pub Hour: i32,
    pub Minute: i32,
    pub OwnerTimeClockTimeSpan: i64,
}

impl PbMessage for SceneTimeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Hour != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Hour);
        }
        if self.Minute != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Minute);
        }
        if self.OwnerTimeClockTimeSpan != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OwnerTimeClockTimeSpan);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Hour = r.read_varint_i32()?,
            2 => self.Minute = r.read_varint_i32()?,
            3 => self.OwnerTimeClockTimeSpan = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JoinSceneNotify {
    pub SceneInfo: Option<SceneInformation>,
    pub MaxEntityId: i64,
    pub TransitionOption: Option<TransitionOptionPb>,
}

impl JoinSceneNotify {
    pub const MSG_ID: u16 = 20597;
}

impl PbMessage for JoinSceneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SceneInfo {
            crate::wire::write_submessage(out, 11, v);
        }
        if self.MaxEntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxEntityId);
        }
        if let Some(v) = &self.TransitionOption {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SceneInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.MaxEntityId = r.read_varint_i64()?,
            3 => {
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
pub struct SceneTraceRequest {
    pub SceneTraceId: i64,
}

impl SceneTraceRequest {
    pub const MSG_ID: u16 = 22643;
}

impl PbMessage for SceneTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneTraceId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SceneTraceId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.SceneTraceId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneTraceResponse {}

impl SceneTraceResponse {
    pub const MSG_ID: u16 = 17440;
}

impl PbMessage for SceneTraceResponse {
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
pub struct AfterJoinSceneNotify {}

impl AfterJoinSceneNotify {
    pub const MSG_ID: u16 = 28489;
}

impl PbMessage for AfterJoinSceneNotify {
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
pub struct LeaveSceneNotify {
    pub PlayerId: i32,
    pub SceneId: Vec<u8>,
    pub TransitionOption: Option<TransitionOptionPb>,
}

impl LeaveSceneNotify {
    pub const MSG_ID: u16 = 21944;
}

impl PbMessage for LeaveSceneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.SceneId.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SceneId);
        }
        if let Some(v) = &self.TransitionOption {
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
            7 => self.PlayerId = r.read_varint_i32()?,
            4 => self.SceneId = r.read_len_delimited()?.to_vec(),
            12 => {
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
pub struct MovingEntityData {
    pub EntityId: i64,
    pub Originator: i64,
    pub MoveInfos: Vec<MoveReplaySample>,
    pub ForcePush: bool,
}

impl PbMessage for MovingEntityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Originator != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Originator);
        }
        for item in &self.MoveInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.ForcePush {
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.Originator = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MoveInfos
                    .push(<MoveReplaySample as PbMessage>::decode(bytes)?);
            }
            4 => self.ForcePush = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovePackagePush {
    pub MovingEntities: Vec<MovingEntityData>,
    pub SceneOwnerId: i32,
}

impl MovePackagePush {
    pub const MSG_ID: u16 = 18855;
}

impl PbMessage for MovePackagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MovingEntities {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.SceneOwnerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.MovingEntities
                    .push(<MovingEntityData as PbMessage>::decode(bytes)?);
            }
            13 => self.SceneOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
