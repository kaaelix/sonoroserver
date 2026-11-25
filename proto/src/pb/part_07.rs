use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagSlotInfo {
    pub X: i32,
    pub Y: i32,
    pub Rotation: i32,
}

impl PbMessage for DangoBrawlBagSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.X);
        }
        if self.Y != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Y);
        }
        if self.Rotation != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rotation);
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
            3 => self.Rotation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBagUpdateNotify {
    pub Shop: Option<DangoBrawlShopInfo>,
    pub Bag: Option<DangoBrawlBagInfo>,
}

impl DangoBrawlBagUpdateNotify {
    pub const MSG_ID: u16 = 24695;
}

impl PbMessage for DangoBrawlBagUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.Bag {
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Bag
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBaseItemInfo {
    pub Uid: i32,
    pub ConfigId: i32,
}

impl PbMessage for DangoBrawlBaseItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Uid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Uid);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.Uid = r.read_varint_i32()?,
            2 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlBenchItemInfo {
    pub Inst: Option<DangoBrawlBaseItemInfo>,
}

impl PbMessage for DangoBrawlBenchItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Inst {
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
                self.Inst
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlChallengeUnlockNotify {
    pub ChallengeInfos: Vec<DangoBrawlChallengeInfo>,
}

impl DangoBrawlChallengeUnlockNotify {
    pub const MSG_ID: u16 = 22943;
}

impl PbMessage for DangoBrawlChallengeUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeInfos {
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
                self.ChallengeInfos
                    .push(<DangoBrawlChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlCoinUpdateNotify {
    pub Coin: i32,
}

impl DangoBrawlCoinUpdateNotify {
    pub const MSG_ID: u16 = 29648;
}

impl PbMessage for DangoBrawlCoinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Coin != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Coin);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.Coin = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlDamageInfo {
    pub Damages: Vec<DangoBrawlDamageSegment>,
    pub IsCrit: bool,
}

impl PbMessage for DangoBrawlDamageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Damages {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.IsCrit {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Damages
                    .push(<DangoBrawlDamageSegment as PbMessage>::decode(bytes)?);
            }
            2 => self.IsCrit = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlDamageSegment {
    pub DamageType: Option<DangoBrawlDamageType>,
    pub Damage: i32,
    pub ItemUId: i32,
    pub ShowType: i32,
    pub BuffConfigId: i32,
}

impl PbMessage for DangoBrawlDamageSegment {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.DamageType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Damage);
        }
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
        if self.ShowType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowType);
        }
        if self.BuffConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.DamageType = Some(
                    <DangoBrawlDamageType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Damage = r.read_varint_i32()?,
            3 => self.ItemUId = r.read_varint_i32()?,
            4 => self.ShowType = r.read_varint_i32()?,
            5 => self.BuffConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEnterCtx {
    pub ChallengeId: i32,
    pub RoleId: i32,
    pub IsContinue: bool,
}

impl PbMessage for DangoBrawlEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.IsContinue {
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            2 => self.RoleId = r.read_varint_i32()?,
            3 => self.IsContinue = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEventInfo {
    pub Slots: Vec<DangoBrawlEventSlotInfo>,
    pub PerSlotRefreshLimit: i32,
}

impl PbMessage for DangoBrawlEventInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Slots {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.PerSlotRefreshLimit != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PerSlotRefreshLimit);
        }
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
                self.Slots
                    .push(<DangoBrawlEventSlotInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.PerSlotRefreshLimit = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEventRefreshRequest {
    pub SlotIndex: i32,
}

impl DangoBrawlEventRefreshRequest {
    pub const MSG_ID: u16 = 28024;
}

impl PbMessage for DangoBrawlEventRefreshRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.SlotIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEventRefreshResponse {
    pub Slot: Option<DangoBrawlEventSlotInfo>,
    pub SlotIndex: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlEventRefreshResponse {
    pub const MSG_ID: u16 = 27098;
}

impl PbMessage for DangoBrawlEventRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Slot {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Slot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.SlotIndex = r.read_varint_i32()?,
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
pub struct DangoBrawlEventSelectRequest {
    pub SlotIndex: i32,
}

impl DangoBrawlEventSelectRequest {
    pub const MSG_ID: u16 = 19472;
}

impl PbMessage for DangoBrawlEventSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.SlotIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEventSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ActiveEventIds: Vec<i32>,
}

impl DangoBrawlEventSelectResponse {
    pub const MSG_ID: u16 = 29706;
}

impl PbMessage for DangoBrawlEventSelectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ActiveEventIds {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActiveEventIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlEventSlotInfo {
    pub EventConfigId: i32,
    pub RefreshedCount: i32,
}

impl PbMessage for DangoBrawlEventSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventConfigId);
        }
        if self.RefreshedCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshedCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EventConfigId = r.read_varint_i32()?,
            2 => self.RefreshedCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionAttrChangeInfo {
    pub TargetFighterUId: i32,
    pub FinalAttrs: BTreeMap<i32, i32>,
}

impl PbMessage for DangoBrawlFightActionAttrChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if !self.FinalAttrs.is_empty() {
            crate::wire::write_map(out, 2, &self.FinalAttrs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.FinalAttrs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionBuffChangeInfo {
    pub BuffUId: i32,
    pub SourceFighterUId: i32,
    pub TargetFighterUId: i32,
    pub BuffConfigId: i32,
    pub Stack: i32,
    pub RemainingMs: i32,
    pub SourceItemUId: i32,
    pub NextReadyMs: i32,
}

impl PbMessage for DangoBrawlFightActionBuffChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffUId);
        }
        if self.SourceFighterUId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceFighterUId);
        }
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if self.BuffConfigId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffConfigId);
        }
        if self.Stack != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Stack);
        }
        if self.RemainingMs != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainingMs);
        }
        if self.SourceItemUId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceItemUId);
        }
        if self.NextReadyMs != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextReadyMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BuffUId = r.read_varint_i32()?,
            2 => self.SourceFighterUId = r.read_varint_i32()?,
            3 => self.TargetFighterUId = r.read_varint_i32()?,
            4 => self.BuffConfigId = r.read_varint_i32()?,
            5 => self.Stack = r.read_varint_i32()?,
            6 => self.RemainingMs = r.read_varint_i32()?,
            7 => self.SourceItemUId = r.read_varint_i32()?,
            8 => self.NextReadyMs = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionBuffRemoveInfo {
    pub BuffUId: i32,
    pub TargetFighterUId: i32,
    pub SourceItemUId: i32,
}

impl PbMessage for DangoBrawlFightActionBuffRemoveInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffUId);
        }
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if self.SourceItemUId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceItemUId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BuffUId = r.read_varint_i32()?,
            2 => self.TargetFighterUId = r.read_varint_i32()?,
            3 => self.SourceItemUId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionBuffTickInfo {
    pub BuffUId: i32,
    pub DamageInfo: Option<DangoBrawlDamageInfo>,
    pub NextReadyMs: i32,
}

impl PbMessage for DangoBrawlFightActionBuffTickInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffUId);
        }
        if let Some(v) = &self.DamageInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.NextReadyMs != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextReadyMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.BuffUId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DamageInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.NextReadyMs = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionDeathInfo {
    pub TargetFighterUId: i32,
}

impl PbMessage for DangoBrawlFightActionDeathInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionEffectShowInfo {
    pub OwnerFighterUId: i32,
    pub EffectId: i32,
    pub ItemUId: i32,
    pub IsTriggerLimit: bool,
}

impl PbMessage for DangoBrawlFightActionEffectShowInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OwnerFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerFighterUId);
        }
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
        if self.IsTriggerLimit {
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
            1 => self.OwnerFighterUId = r.read_varint_i32()?,
            2 => self.EffectId = r.read_varint_i32()?,
            3 => self.ItemUId = r.read_varint_i32()?,
            4 => self.IsTriggerLimit = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionFatigueDamageInfo {
    pub TargetFighterUId: i32,
    pub Damage: i32,
}

impl PbMessage for DangoBrawlFightActionFatigueDamageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Damage);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => self.Damage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionHealInfo {
    pub TargetFighterUId: i32,
    pub HealValues: Vec<i32>,
    pub ItemConfigId: i32,
    pub ItemUId: i32,
}

impl PbMessage for DangoBrawlFightActionHealInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        for item in &self.HealValues {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ItemConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemConfigId);
        }
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HealValues, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.ItemConfigId = r.read_varint_i32()?,
            4 => self.ItemUId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionInfo {
    pub TimeMs: i32,
    pub ActionGroupUId: i32,
    pub SkillCast: Option<DangoBrawlFightActionSkillCastInfo>,
    pub FatigueDamage: Option<DangoBrawlFightActionFatigueDamageInfo>,
    pub BuffChange: Option<DangoBrawlFightActionBuffChangeInfo>,
    pub BuffRemove: Option<DangoBrawlFightActionBuffRemoveInfo>,
    pub AttrChange: Option<DangoBrawlFightActionAttrChangeInfo>,
    pub Death: Option<DangoBrawlFightActionDeathInfo>,
    pub BuffTick: Option<DangoBrawlFightActionBuffTickInfo>,
    pub Heal: Option<DangoBrawlFightActionHealInfo>,
    pub ReflectDamage: Option<DangoBrawlFightActionReflectDamageInfo>,
    pub WeaponStop: Option<DangoBrawlFightActionWeaponStopInfo>,
    pub ShieldAdd: Option<DangoBrawlFightActionShieldAddInfo>,
    pub Lifesteal: Option<DangoBrawlFightActionLifestealInfo>,
    pub EffectShow: Option<DangoBrawlFightActionEffectShowInfo>,
    pub InitWeaponCd: Option<DangoBrawlFightActionInitWeaponCdInfo>,
}

impl PbMessage for DangoBrawlFightActionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TimeMs != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TimeMs);
        }
        if self.ActionGroupUId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionGroupUId);
        }
        if let Some(v) = &self.SkillCast {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.FatigueDamage {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.BuffChange {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.BuffRemove {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.AttrChange {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.Death {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.BuffTick {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.Heal {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.ReflectDamage {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.WeaponStop {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.ShieldAdd {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.Lifesteal {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.EffectShow {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.InitWeaponCd {
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
            1 => self.TimeMs = r.read_varint_i32()?,
            2 => self.ActionGroupUId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SkillCast
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.FatigueDamage
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.BuffChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.BuffRemove
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AttrChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Death
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.BuffTick
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Heal
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ReflectDamage
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponStop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ShieldAdd
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Lifesteal
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.EffectShow
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                self.InitWeaponCd
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionInitWeaponCdInfo {
    pub WeaponNextReadyMs: BTreeMap<i32, i32>,
}

impl PbMessage for DangoBrawlFightActionInitWeaponCdInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.WeaponNextReadyMs.is_empty() {
            crate::wire::write_map(out, 1, &self.WeaponNextReadyMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.WeaponNextReadyMs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionLifestealInfo {
    pub TargetFighterUId: i32,
    pub HealValues: Vec<i32>,
}

impl PbMessage for DangoBrawlFightActionLifestealInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        for item in &self.HealValues {
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
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.HealValues, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionNotify {
    pub IsEnd: bool,
    pub Actions: Vec<DangoBrawlFightActionInfo>,
    pub PlayerWin: bool,
}

impl DangoBrawlFightActionNotify {
    pub const MSG_ID: u16 = 21289;
}

impl PbMessage for DangoBrawlFightActionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnd {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Actions {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.PlayerWin {
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
            2 => self.IsEnd = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Actions
                    .push(<DangoBrawlFightActionInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.PlayerWin = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionReflectDamageInfo {
    pub TargetFighterUId: i32,
    pub Damages: Vec<i32>,
}

impl PbMessage for DangoBrawlFightActionReflectDamageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        for item in &self.Damages {
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
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Damages, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionShieldAddInfo {
    pub TargetFighterUId: i32,
    pub ShieldValue: i32,
    pub ItemUId: i32,
}

impl PbMessage for DangoBrawlFightActionShieldAddInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if self.ShieldValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShieldValue);
        }
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => self.ShieldValue = r.read_varint_i32()?,
            3 => self.ItemUId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionSkillCastInfo {
    pub SourceFighterUId: i32,
    pub SkillConfigId: i32,
    pub WeaponItemId: i32,
    pub TargetFighterUId: i32,
    pub DamageInfo: Option<DangoBrawlDamageInfo>,
    pub WeaponItemUId: i32,
    pub NextReadyMs: i32,
}

impl PbMessage for DangoBrawlFightActionSkillCastInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceFighterUId);
        }
        if self.SkillConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillConfigId);
        }
        if self.WeaponItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponItemId);
        }
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if let Some(v) = &self.DamageInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.WeaponItemUId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponItemUId);
        }
        if self.NextReadyMs != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextReadyMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SourceFighterUId = r.read_varint_i32()?,
            2 => self.SkillConfigId = r.read_varint_i32()?,
            3 => self.WeaponItemId = r.read_varint_i32()?,
            4 => self.TargetFighterUId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.DamageInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.WeaponItemUId = r.read_varint_i32()?,
            7 => self.NextReadyMs = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightActionWeaponStopInfo {
    pub TargetFighterUId: i32,
    pub WeaponSourceItemUid: i32,
    pub StopMs: i32,
}

impl PbMessage for DangoBrawlFightActionWeaponStopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetFighterUId);
        }
        if self.WeaponSourceItemUid != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponSourceItemUid);
        }
        if self.StopMs != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StopMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetFighterUId = r.read_varint_i32()?,
            2 => self.WeaponSourceItemUid = r.read_varint_i32()?,
            3 => self.StopMs = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightOverRequest {
    pub ContinueAddition: bool,
}

impl DangoBrawlFightOverRequest {
    pub const MSG_ID: u16 = 28918;
}

impl PbMessage for DangoBrawlFightOverRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ContinueAddition {
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
            5 => self.ContinueAddition = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFightOverResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlFightOverResponse {
    pub const MSG_ID: u16 = 16631;
}

impl PbMessage for DangoBrawlFightOverResponse {
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
pub struct DangoBrawlFighterAttrChangeNotify {
    pub FighterUId: i32,
    pub FinalAttrs: BTreeMap<i32, i32>,
}

impl DangoBrawlFighterAttrChangeNotify {
    pub const MSG_ID: u16 = 29224;
}

impl PbMessage for DangoBrawlFighterAttrChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FighterUId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FighterUId);
        }
        if !self.FinalAttrs.is_empty() {
            crate::wire::write_map(out, 14, &self.FinalAttrs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.FighterUId = r.read_varint_i32()?,
            14 => crate::wire::decode_map_entry(r, &mut self.FinalAttrs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlFighterInfo {
    pub FighterUId: i32,
    pub Camp: Option<DangoBrawlFightCamp>,
    pub RoleConfigId: i32,
    pub InitFinalAttrs: BTreeMap<i32, i32>,
}

impl PbMessage for DangoBrawlFighterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FighterUId);
        }
        if let Some(v) = &self.Camp {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleConfigId);
        }
        if !self.InitFinalAttrs.is_empty() {
            crate::wire::write_map(out, 4, &self.InitFinalAttrs);
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
            2 => {
                self.Camp = Some(
                    <DangoBrawlFightCamp as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.RoleConfigId = r.read_varint_i32()?,
            4 => crate::wire::decode_map_entry(r, &mut self.InitFinalAttrs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlInstBaseNotify {
    pub Shop: Option<DangoBrawlShopInfo>,
    pub Coin: i32,
    pub Bag: Option<DangoBrawlBagInfo>,
    pub ActiveEventIds: Vec<i32>,
    pub Phase: Option<DangoBrawlPhase>,
    pub OpponentBagItems: Vec<DangoBrawlNormalBagItemInfo>,
    pub ChallengeId: i32,
    pub OpponentId: i32,
    pub Fighters: Vec<DangoBrawlFighterInfo>,
    pub RoleId: i32,
    pub RoundNum: i32,
    pub Event: Option<DangoBrawlEventInfo>,
    pub LoseCount: i32,
    pub WinCount: i32,
}

impl DangoBrawlInstBaseNotify {
    pub const MSG_ID: u16 = 16916;
}

impl PbMessage for DangoBrawlInstBaseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Coin != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Coin);
        }
        if let Some(v) = &self.Bag {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.ActiveEventIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Phase {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.OpponentBagItems {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.OpponentId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OpponentId);
        }
        for item in &self.Fighters {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
        }
        if let Some(v) = &self.Event {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.LoseCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoseCount);
        }
        if self.WinCount != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WinCount);
        }
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
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Coin = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Bag
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActiveEventIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => {
                self.Phase = Some(
                    <DangoBrawlPhase as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.OpponentBagItems
                    .push(<DangoBrawlNormalBagItemInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.ChallengeId = r.read_varint_i32()?,
            8 => self.OpponentId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Fighters
                    .push(<DangoBrawlFighterInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.RoleId = r.read_varint_i32()?,
            12 => self.RoundNum = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Event
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.LoseCount = r.read_varint_i32()?,
            15 => self.WinCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlInstEventSelectNotify {}

impl DangoBrawlInstEventSelectNotify {
    pub const MSG_ID: u16 = 16117;
}

impl PbMessage for DangoBrawlInstEventSelectNotify {
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
pub struct DangoBrawlInstPrepareNotify {}

impl DangoBrawlInstPrepareNotify {
    pub const MSG_ID: u16 = 27221;
}

impl PbMessage for DangoBrawlInstPrepareNotify {
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
pub struct DangoBrawlItemEffectShowNotify {
    pub ItemUId: i32,
    pub EffectId: i32,
}

impl DangoBrawlItemEffectShowNotify {
    pub const MSG_ID: u16 = 15034;
}

impl PbMessage for DangoBrawlItemEffectShowNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemUId = r.read_varint_i32()?,
            15 => self.EffectId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlItemGiveCoinInfo {
    pub ItemUId: i32,
    pub Delta: i32,
}

impl PbMessage for DangoBrawlItemGiveCoinInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUId);
        }
        if self.Delta != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Delta);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ItemUId = r.read_varint_i32()?,
            2 => self.Delta = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlItemGiveCoinNotify {
    pub Infos: Vec<DangoBrawlItemGiveCoinInfo>,
    pub Coin: i32,
}

impl DangoBrawlItemGiveCoinNotify {
    pub const MSG_ID: u16 = 26647;
}

impl PbMessage for DangoBrawlItemGiveCoinNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.Coin != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Coin);
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
                self.Infos
                    .push(<DangoBrawlItemGiveCoinInfo as PbMessage>::decode(bytes)?);
            }
            14 => self.Coin = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlItemUnlockNotify {
    pub ItemIds: Vec<i32>,
}

impl DangoBrawlItemUnlockNotify {
    pub const MSG_ID: u16 = 22794;
}

impl PbMessage for DangoBrawlItemUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemIds {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlLoseCountUpdateNotify {
    pub LoseCount: i32,
}

impl DangoBrawlLoseCountUpdateNotify {
    pub const MSG_ID: u16 = 26926;
}

impl PbMessage for DangoBrawlLoseCountUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LoseCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoseCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.LoseCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlNormalBagItemInfo {
    pub Inst: Option<DangoBrawlBaseItemInfo>,
    pub X: i32,
    pub Y: i32,
    pub Rotation: i32,
}

impl PbMessage for DangoBrawlNormalBagItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Inst {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.X != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.X);
        }
        if self.Y != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Y);
        }
        if self.Rotation != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rotation);
        }
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
                self.Inst
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.X = r.read_varint_i32()?,
            3 => self.Y = r.read_varint_i32()?,
            4 => self.Rotation = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlRoleUnlockNotify {
    pub RoleIds: Vec<i32>,
}

impl DangoBrawlRoleUnlockNotify {
    pub const MSG_ID: u16 = 15179;
}

impl PbMessage for DangoBrawlRoleUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleIds {
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
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlRoundSwitchNotify {
    pub Event: Option<DangoBrawlEventInfo>,
    pub WinCount: i32,
    pub Fighters: Vec<DangoBrawlFighterInfo>,
    pub Coin: i32,
    pub OpponentBagItems: Vec<DangoBrawlNormalBagItemInfo>,
    pub RoundNum: i32,
    pub LoseCount: i32,
    pub ActiveEventIds: Vec<i32>,
    pub Shop: Option<DangoBrawlShopInfo>,
    pub OpponentId: i32,
}

impl DangoBrawlRoundSwitchNotify {
    pub const MSG_ID: u16 = 28886;
}

impl PbMessage for DangoBrawlRoundSwitchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Event {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.WinCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WinCount);
        }
        for item in &self.Fighters {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.Coin != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Coin);
        }
        for item in &self.OpponentBagItems {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
        }
        if self.LoseCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoseCount);
        }
        for item in &self.ActiveEventIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.OpponentId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OpponentId);
        }
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
                self.Event
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.WinCount = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Fighters
                    .push(<DangoBrawlFighterInfo as PbMessage>::decode(bytes)?);
            }
            6 => self.Coin = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.OpponentBagItems
                    .push(<DangoBrawlNormalBagItemInfo as PbMessage>::decode(bytes)?);
            }
            8 => self.RoundNum = r.read_varint_i32()?,
            10 => self.LoseCount = r.read_varint_i32()?,
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActiveEventIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.OpponentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlSaveMatchBriefNotify {
    pub SaveMatchBrief: Option<DangoBrawlSaveMatchBrief>,
}

impl DangoBrawlSaveMatchBriefNotify {
    pub const MSG_ID: u16 = 15653;
}

impl PbMessage for DangoBrawlSaveMatchBriefNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SaveMatchBrief {
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
                self.SaveMatchBrief
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlScoreRewardRequest {
    pub ActivityId: i32,
    pub ScoreIds: Vec<i32>,
}

impl DangoBrawlScoreRewardRequest {
    pub const MSG_ID: u16 = 15066;
}

impl PbMessage for DangoBrawlScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ScoreIds {
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
            2 => self.ActivityId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlScoreRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl DangoBrawlScoreRewardResponse {
    pub const MSG_ID: u16 = 24133;
}

impl PbMessage for DangoBrawlScoreRewardResponse {
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
pub struct DangoBrawlSettleNotify {
    pub RewardedItems: BTreeMap<i32, i32>,
    pub IsWin: bool,
    pub LoseCount: i32,
    pub WinCount: i32,
    pub FirstPass: bool,
}

impl DangoBrawlSettleNotify {
    pub const MSG_ID: u16 = 18644;
}

impl PbMessage for DangoBrawlSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RewardedItems.is_empty() {
            crate::wire::write_map(out, 1, &self.RewardedItems);
        }
        if self.IsWin {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LoseCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LoseCount);
        }
        if self.WinCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WinCount);
        }
        if self.FirstPass {
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
            1 => crate::wire::decode_map_entry(r, &mut self.RewardedItems)?,
            5 => self.IsWin = r.read_bool()?,
            6 => self.LoseCount = r.read_varint_i32()?,
            8 => self.WinCount = r.read_varint_i32()?,
            9 => self.FirstPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlSettleRequest {}

impl DangoBrawlSettleRequest {
    pub const MSG_ID: u16 = 28335;
}

impl PbMessage for DangoBrawlSettleRequest {
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
pub struct DangoBrawlSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlSettleResponse {
    pub const MSG_ID: u16 = 24979;
}

impl PbMessage for DangoBrawlSettleResponse {
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
pub struct DangoBrawlShopBuyRequest {
    pub ToBenchUids: Vec<i32>,
    pub Slot: Option<DangoBrawlBagSlotInfo>,
    pub ToBench: bool,
    pub SlotUid: i32,
}

impl DangoBrawlShopBuyRequest {
    pub const MSG_ID: u16 = 24895;
}

impl PbMessage for DangoBrawlShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ToBenchUids {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Slot {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.ToBench {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SlotUid != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ToBenchUids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Slot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.ToBench = r.read_bool()?,
            11 => self.SlotUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopBuyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub UseServerBagUpdate: bool,
}

impl DangoBrawlShopBuyResponse {
    pub const MSG_ID: u16 = 25948;
}

impl PbMessage for DangoBrawlShopBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UseServerBagUpdate {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.UseServerBagUpdate = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopComposeRequest {
    pub Slot: Option<DangoBrawlBagSlotInfo>,
    pub ToBench: bool,
    pub DangoBrawlItemComposeId: i32,
    pub ItemUids: Vec<i32>,
}

impl DangoBrawlShopComposeRequest {
    pub const MSG_ID: u16 = 20473;
}

impl PbMessage for DangoBrawlShopComposeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Slot {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.ToBench {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DangoBrawlItemComposeId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoBrawlItemComposeId);
        }
        for item in &self.ItemUids {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Slot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => self.ToBench = r.read_bool()?,
            6 => self.DangoBrawlItemComposeId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemUids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopComposeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub NewItemUid: i32,
}

impl DangoBrawlShopComposeResponse {
    pub const MSG_ID: u16 = 15368;
}

impl PbMessage for DangoBrawlShopComposeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NewItemUid != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewItemUid);
        }
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
            4 => self.NewItemUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopInfo {
    pub Slots: Vec<DangoBrawlShopSlotInfo>,
    pub NextRefreshCost: i32,
    pub ManualRefreshCount: i32,
}

impl PbMessage for DangoBrawlShopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Slots {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.NextRefreshCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextRefreshCost);
        }
        if self.ManualRefreshCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ManualRefreshCount);
        }
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
                self.Slots
                    .push(<DangoBrawlShopSlotInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.NextRefreshCost = r.read_varint_i32()?,
            3 => self.ManualRefreshCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopLockRequest {
    pub SlotUid: i32,
    pub Locked: bool,
}

impl DangoBrawlShopLockRequest {
    pub const MSG_ID: u16 = 24357;
}

impl PbMessage for DangoBrawlShopLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotUid != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotUid);
        }
        if self.Locked {
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
            6 => self.SlotUid = r.read_varint_i32()?,
            12 => self.Locked = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopLockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlShopLockResponse {
    pub const MSG_ID: u16 = 19957;
}

impl PbMessage for DangoBrawlShopLockResponse {
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
pub struct DangoBrawlShopRefreshRequest {}

impl DangoBrawlShopRefreshRequest {
    pub const MSG_ID: u16 = 23068;
}

impl PbMessage for DangoBrawlShopRefreshRequest {
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
pub struct DangoBrawlShopRefreshResponse {
    pub Shop: Option<DangoBrawlShopInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlShopRefreshResponse {
    pub const MSG_ID: u16 = 15810;
}

impl PbMessage for DangoBrawlShopRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 5, v);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
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
pub struct DangoBrawlShopSellRequest {
    pub FromBench: bool,
    pub ToBenchUids: Vec<i32>,
    pub ItemUid: i32,
}

impl DangoBrawlShopSellRequest {
    pub const MSG_ID: u16 = 29894;
}

impl PbMessage for DangoBrawlShopSellRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FromBench {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ToBenchUids {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ItemUid != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.FromBench = r.read_bool()?,
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ToBenchUids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.ItemUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlShopSellResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlShopSellResponse {
    pub const MSG_ID: u16 = 24125;
}

impl PbMessage for DangoBrawlShopSellResponse {
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
pub struct DangoBrawlShopSlotInfo {
    pub Item: Option<DangoBrawlBaseItemInfo>,
    pub Locked: bool,
    pub Price: i32,
}

impl PbMessage for DangoBrawlShopSlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Item {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.Locked {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Price != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Item
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.Locked = r.read_bool()?,
            3 => self.Price = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlStartFightNotify {
    pub Fighters: Vec<DangoBrawlFighterInfo>,
}

impl DangoBrawlStartFightNotify {
    pub const MSG_ID: u16 = 21162;
}

impl PbMessage for DangoBrawlStartFightNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Fighters {
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
                self.Fighters
                    .push(<DangoBrawlFighterInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlStartFightRequest {}

impl DangoBrawlStartFightRequest {
    pub const MSG_ID: u16 = 26094;
}

impl PbMessage for DangoBrawlStartFightRequest {
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
pub struct DangoBrawlStartFightResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoBrawlStartFightResponse {
    pub const MSG_ID: u16 = 28659;
}

impl PbMessage for DangoBrawlStartFightResponse {
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
pub struct DangoBrawlTaskRewardRequest {
    pub ActivityId: i32,
    pub TaskIds: Vec<i32>,
}

impl DangoBrawlTaskRewardRequest {
    pub const MSG_ID: u16 = 19998;
}

impl PbMessage for DangoBrawlTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            9 => self.ActivityId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoBrawlTaskRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl DangoBrawlTaskRewardResponse {
    pub const MSG_ID: u16 = 20255;
}

impl PbMessage for DangoBrawlTaskRewardResponse {
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
pub struct DangoBrawlTaskUpdateNotify {
    pub Tasks: Vec<ConditionTask>,
}

impl DangoBrawlTaskUpdateNotify {
    pub const MSG_ID: u16 = 21615;
}

impl PbMessage for DangoBrawlTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Tasks {
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
                self.Tasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoIdToDiceNum {
    pub DangoId: i32,
    pub Num: i32,
    pub DiceId: i32,
}

impl PbMessage for DangoIdToDiceNum {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangoId);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.DiceId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiceId);
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
            2 => self.Num = r.read_varint_i32()?,
            3 => self.DiceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyActivityPbData {
    pub BoardId: i32,
    pub GridId: i32,
    pub RewardedGridId: i32,
    pub RewardedBoardIds: Vec<i32>,
    pub TaskDatas: Vec<DangoMonopolyTaskData>,
    pub TaskRefreshTime: BTreeMap<i32, i64>,
    pub BoardUnlockTime: i64,
    pub BoardRecords: BTreeMap<i32, DangoMonopolyBoardRecordPb>,
}

impl PbMessage for DangoMonopolyActivityPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoardId);
        }
        if self.GridId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridId);
        }
        if self.RewardedGridId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedGridId);
        }
        for item in &self.RewardedBoardIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TaskDatas {
            crate::wire::write_submessage(out, 5, item);
        }
        if !self.TaskRefreshTime.is_empty() {
            crate::wire::write_map(out, 6, &self.TaskRefreshTime);
        }
        if self.BoardUnlockTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardUnlockTime);
        }
        if !self.BoardRecords.is_empty() {
            crate::wire::write_map(out, 8, &self.BoardRecords);
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
            2 => self.GridId = r.read_varint_i32()?,
            3 => self.RewardedGridId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedBoardIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDatas
                    .push(<DangoMonopolyTaskData as PbMessage>::decode(bytes)?);
            }
            6 => crate::wire::decode_map_entry(r, &mut self.TaskRefreshTime)?,
            7 => self.BoardUnlockTime = r.read_varint_i64()?,
            8 => crate::wire::decode_map_entry(r, &mut self.BoardRecords)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyBoardRecordPb {
    pub PropertyIds: Vec<i32>,
    pub DiceTimes: i32,
    pub PropertyTriggerTimes: BTreeMap<i32, i32>,
}

impl PbMessage for DangoMonopolyBoardRecordPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PropertyIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.DiceTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiceTimes);
        }
        if !self.PropertyTriggerTimes.is_empty() {
            crate::wire::write_map(out, 3, &self.PropertyTriggerTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PropertyIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.DiceTimes = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.PropertyTriggerTimes)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyBoardRewardRequest {
    pub BoardIds: Vec<i32>,
    pub ActivityId: i32,
}

impl DangoMonopolyBoardRewardRequest {
    pub const MSG_ID: u16 = 29315;
}

impl PbMessage for DangoMonopolyBoardRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BoardIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BoardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyBoardRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoMonopolyBoardRewardResponse {
    pub const MSG_ID: u16 = 26889;
}

impl PbMessage for DangoMonopolyBoardRewardResponse {
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
pub struct DangoMonopolyCtx {
    pub ActivityId: i32,
}

impl PbMessage for DangoMonopolyCtx {
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
pub struct DangoMonopolyDiceRequest {
    pub ActivityId: i32,
}

impl DangoMonopolyDiceRequest {
    pub const MSG_ID: u16 = 19414;
}

impl PbMessage for DangoMonopolyDiceRequest {
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
pub struct DangoMonopolyDiceResponse {
    pub DiceNum: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub PropertyData: Option<DangoMonopolyPropertyData>,
    pub MoveGridNum: i32,
    pub PropertyIds: Vec<i32>,
}

impl DangoMonopolyDiceResponse {
    pub const MSG_ID: u16 = 24491;
}

impl PbMessage for DangoMonopolyDiceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DiceNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiceNum);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PropertyData {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.MoveGridNum != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MoveGridNum);
        }
        for item in &self.PropertyIds {
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
            1 => self.DiceNum = r.read_varint_i32()?,
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PropertyData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.MoveGridNum = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PropertyIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyGridRewardRequest {
    pub ActivityId: i32,
}

impl DangoMonopolyGridRewardRequest {
    pub const MSG_ID: u16 = 22355;
}

impl PbMessage for DangoMonopolyGridRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyGridRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub HasReward: bool,
}

impl DangoMonopolyGridRewardResponse {
    pub const MSG_ID: u16 = 18776;
}

impl PbMessage for DangoMonopolyGridRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HasReward {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.HasReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyNextBoardRequest {
    pub ActivityId: i32,
}

impl DangoMonopolyNextBoardRequest {
    pub const MSG_ID: u16 = 19220;
}

impl PbMessage for DangoMonopolyNextBoardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyNextBoardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub BoardUnlockTime: i64,
    pub PropertyIds: Vec<i32>,
    pub BoardId: i32,
}

impl DangoMonopolyNextBoardResponse {
    pub const MSG_ID: u16 = 18128;
}

impl PbMessage for DangoMonopolyNextBoardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BoardUnlockTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BoardUnlockTime);
        }
        for item in &self.PropertyIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.BoardId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.BoardUnlockTime = r.read_varint_i64()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PropertyIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.BoardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyPropertyData {
    pub PropertyId: i32,
    pub PropertyData: Vec<i32>,
}

impl PbMessage for DangoMonopolyPropertyData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PropertyId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PropertyId);
        }
        for item in &self.PropertyData {
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
            1 => self.PropertyId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PropertyData, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyRewardNotify {
    pub Reason: i32,
    pub RewardDatas: Vec<DangoMonopolyRewardPb>,
}

impl DangoMonopolyRewardNotify {
    pub const MSG_ID: u16 = 18485;
}

impl PbMessage for DangoMonopolyRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Reason != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Reason);
        }
        for item in &self.RewardDatas {
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
            1 => self.Reason = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RewardDatas
                    .push(<DangoMonopolyRewardPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyRewardPb {
    pub ItemId: i32,
    pub ItemNum: i32,
    pub Multiple: i32,
}

impl PbMessage for DangoMonopolyRewardPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemNum);
        }
        if self.Multiple != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Multiple);
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
            3 => self.Multiple = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolySceneGridInfoNotify {
    pub MainRoleEntityId: i64,
    pub GridInfos: BTreeMap<i32, i64>,
    pub ActivityId: i32,
}

impl DangoMonopolySceneGridInfoNotify {
    pub const MSG_ID: u16 = 23339;
}

impl PbMessage for DangoMonopolySceneGridInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MainRoleEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MainRoleEntityId);
        }
        if !self.GridInfos.is_empty() {
            crate::wire::write_map(out, 7, &self.GridInfos);
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
            4 => self.MainRoleEntityId = r.read_varint_i64()?,
            7 => crate::wire::decode_map_entry(r, &mut self.GridInfos)?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyTaskAddNotify {
    pub TaskDatas: Vec<DangoMonopolyTaskData>,
    pub ActivityId: i32,
}

impl DangoMonopolyTaskAddNotify {
    pub const MSG_ID: u16 = 18048;
}

impl PbMessage for DangoMonopolyTaskAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskDatas {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDatas
                    .push(<DangoMonopolyTaskData as PbMessage>::decode(bytes)?);
            }
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyTaskData {
    pub TaskId: i32,
    pub State: Option<DangoMonopolyTaskState>,
    pub Progress: i32,
    pub Target: i32,
}

impl PbMessage for DangoMonopolyTaskData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            1 => self.TaskId = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <DangoMonopolyTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Progress = r.read_varint_i32()?,
            4 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyTaskRemoveNotify {
    pub ActivityId: i32,
    pub TaskIds: Vec<i32>,
}

impl DangoMonopolyTaskRemoveNotify {
    pub const MSG_ID: u16 = 22199;
}

impl PbMessage for DangoMonopolyTaskRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TaskIds {
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
            6 => self.ActivityId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyTaskRewardRequest {
    pub TaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl DangoMonopolyTaskRewardRequest {
    pub const MSG_ID: u16 = 28152;
}

impl PbMessage for DangoMonopolyTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DangoMonopolyTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DangoMonopolyTaskRewardResponse {
    pub const MSG_ID: u16 = 20830;
}

impl PbMessage for DangoMonopolyTaskRewardResponse {
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
pub struct DangoMonopolyTaskUpdateNotify {
    pub TaskDatas: Vec<DangoMonopolyTaskData>,
    pub ActivityId: i32,
}

impl DangoMonopolyTaskUpdateNotify {
    pub const MSG_ID: u16 = 22180;
}

impl PbMessage for DangoMonopolyTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskDatas {
            crate::wire::write_submessage(out, 4, item);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TaskDatas
                    .push(<DangoMonopolyTaskData as PbMessage>::decode(bytes)?);
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DarkCoastInfoRequest {
    pub DragonPoolId: i32,
}

impl DarkCoastInfoRequest {
    pub const MSG_ID: u16 = 20643;
}

impl PbMessage for DarkCoastInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => self.DragonPoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DarkCoastInfoResponse {
    pub LevelGains: i32,
    pub DragonPoolConf: Option<DragonPoolConf>,
    pub GuardIds: Vec<i32>,
    pub TreasureBoxDone: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl DarkCoastInfoResponse {
    pub const MSG_ID: u16 = 20081;
}

impl PbMessage for DarkCoastInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelGains != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGains);
        }
        if let Some(v) = &self.DragonPoolConf {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.GuardIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TreasureBoxDone {
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
            2 => self.LevelGains = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DragonPoolConf
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GuardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TreasureBoxDone,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
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
pub struct DarkTideEnterCtx {
    pub InstanceId: i32,
    pub RoleId: i32,
}

impl PbMessage for DarkTideEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstanceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstanceId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.InstanceId = r.read_varint_i32()?,
            2 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeadeyeShootHitRequest {
    pub InstId: i32,
    pub EntityIds: Vec<i64>,
}

impl DeadeyeShootHitRequest {
    pub const MSG_ID: u16 = 15008;
}

impl PbMessage for DeadeyeShootHitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.EntityIds {
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
            3 => self.InstId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeadeyeShootHitResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl DeadeyeShootHitResponse {
    pub const MSG_ID: u16 = 21965;
}

impl PbMessage for DeadeyeShootHitResponse {
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
pub struct DecalAddNotify {
    pub Info: Vec<DecalInfo>,
}

impl DecalAddNotify {
    pub const MSG_ID: u16 = 17780;
}

impl PbMessage for DecalAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Info {
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
                self.Info.push(<DecalInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecalInfo {
    pub DecalId: i64,
    pub SceneId: i32,
    pub Position: Option<Vector>,
    pub Rotation: Option<Vector>,
    pub EffectId: i32,
}

impl PbMessage for DecalInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DecalId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DecalId);
        }
        if self.SceneId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneId);
        }
        if let Some(v) = &self.Position {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DecalId = r.read_varint_i64()?,
            2 => self.SceneId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Position
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.EffectId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecalRemoveNotify {
    pub Info: Vec<DecalInfo>,
}

impl DecalRemoveNotify {
    pub const MSG_ID: u16 = 23152;
}

impl PbMessage for DecalRemoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Info {
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
                self.Info.push(<DecalInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecomposeItemInfo {
    pub Count: i32,
    pub ItemId: i32,
    pub IncrId: i32,
}

impl PbMessage for DecomposeItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            2 => self.Count = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            4 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeconstructTrapRequest {
    pub Id: i64,
}

impl DeconstructTrapRequest {
    pub const MSG_ID: u16 = 20360;
}

impl PbMessage for DeconstructTrapRequest {
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
pub struct DeconstructTrapResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DeconstructTrapResponse {
    pub const MSG_ID: u16 = 20270;
}

impl PbMessage for DeconstructTrapResponse {
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
pub struct DeletePhantomEquipGroupRequest {
    pub RoleId: i32,
    pub Index: i32,
}

impl DeletePhantomEquipGroupRequest {
    pub const MSG_ID: u16 = 28863;
}

impl PbMessage for DeletePhantomEquipGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            6 => self.RoleId = r.read_varint_i32()?,
            7 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeletePhantomEquipGroupResponse {
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl DeletePhantomEquipGroupResponse {
    pub const MSG_ID: u16 = 16721;
}

impl PbMessage for DeletePhantomEquipGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomEquipGroupInfos {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
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
pub struct DeleteStoryCharacterRequest {}

impl DeleteStoryCharacterRequest {
    pub const MSG_ID: u16 = 22583;
}

impl PbMessage for DeleteStoryCharacterRequest {
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
pub struct DeleteStoryCharacterResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FightRoleId: i32,
}

impl DeleteStoryCharacterResponse {
    pub const MSG_ID: u16 = 16614;
}

impl PbMessage for DeleteStoryCharacterResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FightRoleId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightRoleId);
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
            13 => self.FightRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DependenceComponentPb {
    pub DependenceEntityIds: Vec<i64>,
}

impl PbMessage for DependenceComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DependenceEntityIds {
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
                &mut self.DependenceEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DestroyBulletPush {
    pub Handle: Option<ActiveBulletHandle>,
    pub CombatCommon: Option<CombatCommon>,
}

impl DestroyBulletPush {
    pub const MSG_ID: u16 = 28662;
}

impl PbMessage for DestroyBulletPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Handle {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Handle
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct DestroyRecallQuestsNotify {
    pub RecallQuestIds: Vec<i32>,
}

impl DestroyRecallQuestsNotify {
    pub const MSG_ID: u16 = 16440;
}

impl PbMessage for DestroyRecallQuestsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RecallQuestIds {
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
pub struct DetectionEntityPosInfo {
    pub Pos: Option<Vector>,
    pub ConfigId: i32,
}

impl PbMessage for DetectionEntityPosInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionInvisibleNotify {
    pub TargetChange: Option<DetectionTarget>,
}

impl DetectionInvisibleNotify {
    pub const MSG_ID: u16 = 15166;
}

impl PbMessage for DetectionInvisibleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TargetChange {
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
                self.TargetChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionPreOpenNotify {
    pub PreOpenSilenceAreaDetections: Vec<PreOpenDetections>,
    pub PreOpenDungeonDetections: Vec<PreOpenDetections>,
}

impl DetectionPreOpenNotify {
    pub const MSG_ID: u16 = 23923;
}

impl PbMessage for DetectionPreOpenNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PreOpenSilenceAreaDetections {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.PreOpenDungeonDetections {
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PreOpenSilenceAreaDetections
                    .push(<PreOpenDetections as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PreOpenDungeonDetections
                    .push(<PreOpenDetections as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionRequest {
    pub DetectionConfId: i32,
    pub DetectionId: Vec<i32>,
    pub DetectionType: Option<DetectionType>,
}

impl DetectionRequest {
    pub const MSG_ID: u16 = 15268;
}

impl PbMessage for DetectionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DetectionConfId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionConfId);
        }
        for item in &self.DetectionId {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.DetectionType {
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
            10 => self.DetectionConfId = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DetectionId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => {
                self.DetectionType =
                    Some(<DetectionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DetectionResponse {
    pub DetectionTarget: Vec<DetectionTarget>,
    pub DetectionConfId: i32,
    pub Code: Option<ErrorCode>,
}

impl DetectionResponse {
    pub const MSG_ID: u16 = 17575;
}

impl PbMessage for DetectionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DetectionTarget {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.DetectionConfId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DetectionConfId);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DetectionTarget
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            9 => self.DetectionConfId = r.read_varint_i32()?,
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
pub struct DetectionTargetChangeNotify {
    pub Reason: Option<DetectionTargetChangeReason>,
    pub MapId: i32,
    pub EntityId: i32,
}

impl DetectionTargetChangeNotify {
    pub const MSG_ID: u16 = 15512;
}

impl PbMessage for DetectionTargetChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MapId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            8 => {
                self.Reason = Some(
                    <DetectionTargetChangeReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => self.MapId = r.read_varint_i32()?,
            11 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DieSwitchRoleNotify {
    pub CurRoleId: i32,
}

impl DieSwitchRoleNotify {
    pub const MSG_ID: u16 = 23281;
}

impl PbMessage for DieSwitchRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurRoleId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.CurRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainInfoNotify {
    pub Activity: Option<ActivityData>,
}

impl DirectTrainInfoNotify {
    pub const MSG_ID: u16 = 28851;
}

impl PbMessage for DirectTrainInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Activity {
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
                self.Activity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainInfoRequest {}

impl DirectTrainInfoRequest {
    pub const MSG_ID: u16 = 29272;
}

impl PbMessage for DirectTrainInfoRequest {
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
pub struct DirectTrainInfoResponse {
    pub Activitys: Vec<ActivityData>,
    pub MainActivity: Option<ActivityData>,
}

impl DirectTrainInfoResponse {
    pub const MSG_ID: u16 = 19087;
}

impl PbMessage for DirectTrainInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Activitys {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.MainActivity {
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
                self.Activitys
                    .push(<ActivityData as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.MainActivity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainInfoUpdateNotify {
    pub Activitys: Vec<ActivityData>,
}

impl DirectTrainInfoUpdateNotify {
    pub const MSG_ID: u16 = 16954;
}

impl PbMessage for DirectTrainInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Activitys {
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
                self.Activitys
                    .push(<ActivityData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DirectTrainPreOpenCloseNotify {
    pub ActivityId: i32,
}

impl DirectTrainPreOpenCloseNotify {
    pub const MSG_ID: u16 = 24510;
}

impl PbMessage for DirectTrainPreOpenCloseNotify {
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
pub struct DiscardWeaponPush {
    pub EntityId: i64,
    pub MovementInformation: Option<MovementInformation>,
}

impl DiscardWeaponPush {
    pub const MSG_ID: u16 = 20653;
}

impl PbMessage for DiscardWeaponPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.MovementInformation {
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
            2 => self.EntityId = r.read_varint_i64()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.MovementInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DiscardWeaponRequest {
    pub MovementInformation: Option<MovementInformation>,
    pub EntityId: i64,
}

impl DiscardWeaponRequest {
    pub const MSG_ID: u16 = 18527;
}

impl PbMessage for DiscardWeaponRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MovementInformation {
            crate::wire::write_submessage(out, 6, v);
        }
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.MovementInformation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DiscardWeaponResponse {
    pub DiscardSucc: bool,
}

impl DiscardWeaponResponse {
    pub const MSG_ID: u16 = 20992;
}

impl PbMessage for DiscardWeaponResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DiscardSucc {
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
            10 => self.DiscardSucc = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DiscountInfo {
    pub BuffPrice: BTreeMap<i32, i32>,
    pub Discounted: i32,
}

impl PbMessage for DiscountInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BuffPrice.is_empty() {
            crate::wire::write_map(out, 1, &self.BuffPrice);
        }
        if self.Discounted != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Discounted);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.BuffPrice)?,
            2 => self.Discounted = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DissolveCombineRelationRequest {
    pub EntityA: i64,
    pub EntityB: i64,
}

impl DissolveCombineRelationRequest {
    pub const MSG_ID: u16 = 20585;
}

impl PbMessage for DissolveCombineRelationRequest {
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
pub struct DissolveCombineRelationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DissolveCombineRelationResponse {
    pub const MSG_ID: u16 = 20084;
}

impl PbMessage for DissolveCombineRelationResponse {
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
pub struct DollDeliveryNotify {
    pub DropIds: Vec<i32>,
    pub ItemIds: Vec<i32>,
}

impl DollDeliveryNotify {
    pub const MSG_ID: u16 = 15165;
}

impl PbMessage for DollDeliveryNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DropIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.ItemIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DropIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollDeliveryRequest {
    pub EntityCfgId: i32,
}

impl DollDeliveryRequest {
    pub const MSG_ID: u16 = 17654;
}

impl PbMessage for DollDeliveryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityCfgId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityCfgId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.EntityCfgId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollDeliveryResponse {
    pub ItemIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl DollDeliveryResponse {
    pub const MSG_ID: u16 = 16482;
}

impl PbMessage for DollDeliveryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct DollGarbInsertCoinRequest {
    pub EntityIncId: i64,
}

impl DollGarbInsertCoinRequest {
    pub const MSG_ID: u16 = 28609;
}

impl PbMessage for DollGarbInsertCoinRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGarbInsertCoinResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DollGarbInsertCoinResponse {
    pub const MSG_ID: u16 = 28733;
}

impl PbMessage for DollGarbInsertCoinResponse {
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
pub struct DollGarbStartRequest {
    pub EntityIncId: i64,
}

impl DollGarbStartRequest {
    pub const MSG_ID: u16 = 25749;
}

impl PbMessage for DollGarbStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGarbStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DollGarbStartResponse {
    pub const MSG_ID: u16 = 22696;
}

impl PbMessage for DollGarbStartResponse {
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
pub struct DollGrabItemInfo {
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for DollGrabItemInfo {
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
pub struct DollGrabPauseEndRequest {
    pub EntityIncId: i64,
}

impl DollGrabPauseEndRequest {
    pub const MSG_ID: u16 = 15196;
}

impl PbMessage for DollGrabPauseEndRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabPauseEndResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DollGrabPauseEndResponse {
    pub const MSG_ID: u16 = 17158;
}

impl PbMessage for DollGrabPauseEndResponse {
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
pub struct DollGrabPauseStartRequest {
    pub EntityIncId: i64,
}

impl DollGrabPauseStartRequest {
    pub const MSG_ID: u16 = 16209;
}

impl PbMessage for DollGrabPauseStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabPauseStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl DollGrabPauseStartResponse {
    pub const MSG_ID: u16 = 17732;
}

impl PbMessage for DollGrabPauseStartResponse {
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
pub struct DollGrabRequest {
    pub ItemUniId: i32,
    pub EntityIncId: i64,
}

impl DollGrabRequest {
    pub const MSG_ID: u16 = 17827;
}

impl PbMessage for DollGrabRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemUniId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUniId);
        }
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            2 => self.ItemUniId = r.read_varint_i32()?,
            8 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GrabItemInfos: Vec<DollGrabItemInfo>,
}

impl DollGrabResponse {
    pub const MSG_ID: u16 = 17987;
}

impl PbMessage for DollGrabResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GrabItemInfos {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.GrabItemInfos
                    .push(<DollGrabItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabStopRequest {
    pub EntityIncId: i64,
}

impl DollGrabStopRequest {
    pub const MSG_ID: u16 = 18186;
}

impl PbMessage for DollGrabStopRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabStopResponse {
    pub HighScore: i32,
    pub AccumulatedScore: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub SessionScore: i32,
}

impl DollGrabStopResponse {
    pub const MSG_ID: u16 = 22720;
}

impl PbMessage for DollGrabStopResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HighScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HighScore);
        }
        if self.AccumulatedScore != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccumulatedScore);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.SessionScore != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SessionScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.HighScore = r.read_varint_i32()?,
            5 => self.AccumulatedScore = r.read_varint_i32()?,
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.SessionScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabTimeBallRequest {
    pub ItemUniId: i32,
    pub EntityIncId: i64,
}

impl DollGrabTimeBallRequest {
    pub const MSG_ID: u16 = 15812;
}

impl PbMessage for DollGrabTimeBallRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemUniId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUniId);
        }
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            3 => self.ItemUniId = r.read_varint_i32()?,
            5 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DollGrabTimeBallResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GrabItemInfos: Vec<DollGrabItemInfo>,
}

impl DollGrabTimeBallResponse {
    pub const MSG_ID: u16 = 18865;
}

impl PbMessage for DollGrabTimeBallResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GrabItemInfos {
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
                self.GrabItemInfos
                    .push(<DollGrabItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonPoolConf {
    pub DragonPoolId: i32,
    pub DropIds: Vec<i32>,
    pub DropItems: Vec<ItemDict>,
}

impl PbMessage for DragonPoolConf {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
        }
        for item in &self.DropIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DropItems {
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
            1 => self.DragonPoolId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DropIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.DropItems.push(<ItemDict as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonPoolConfRequest {
    pub DragonPoolId: i32,
}

impl DragonPoolConfRequest {
    pub const MSG_ID: u16 = 18045;
}

impl PbMessage for DragonPoolConfRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.DragonPoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DragonPoolConfResponse {
    pub DragonPoolConf: Option<DragonPoolConf>,
    pub ErrorCode: Option<ErrorCode>,
}

impl DragonPoolConfResponse {
    pub const MSG_ID: u16 = 16385;
}

impl PbMessage for DragonPoolConfResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.DragonPoolConf {
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
                self.DragonPoolConf
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
pub struct DropBagItemInfo {
    pub ShowPlanId: i32,
    pub ItemId: i32,
    pub ItemCount: i32,
}

impl PbMessage for DropBagItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowPlanId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowPlanId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.ShowPlanId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.ItemCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchInfoRequest {}

impl DropCatchInfoRequest {
    pub const MSG_ID: u16 = 21673;
}

impl PbMessage for DropCatchInfoRequest {
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
pub struct DropCatchInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Infos: Vec<DropCatchLevelInfo>,
}

impl DropCatchInfoResponse {
    pub const MSG_ID: u16 = 26513;
}

impl PbMessage for DropCatchInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Infos {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<DropCatchLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchLevelFinishRequest {
    pub DropCatchId: i32,
    pub Score: i32,
}

impl DropCatchLevelFinishRequest {
    pub const MSG_ID: u16 = 23994;
}

impl PbMessage for DropCatchLevelFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropCatchId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropCatchId);
        }
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
            9 => self.DropCatchId = r.read_varint_i32()?,
            13 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchLevelFinishResponse {
    pub Error: Option<ErrorCode>,
}

impl DropCatchLevelFinishResponse {
    pub const MSG_ID: u16 = 24353;
}

impl PbMessage for DropCatchLevelFinishResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
pub struct DropCatchLevelRewardRequest {
    pub DropCatchId: i32,
    pub RewardIdxs: Vec<i32>,
}

impl DropCatchLevelRewardRequest {
    pub const MSG_ID: u16 = 26230;
}

impl PbMessage for DropCatchLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropCatchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropCatchId);
        }
        for item in &self.RewardIdxs {
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
            4 => self.DropCatchId = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIdxs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropCatchLevelRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl DropCatchLevelRewardResponse {
    pub const MSG_ID: u16 = 27201;
}

impl PbMessage for DropCatchLevelRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
pub struct DropCatchLevelRewardUpdateNotify {
    pub DropCatchLevelInfos: Vec<DropCatchLevelInfo>,
}

impl DropCatchLevelRewardUpdateNotify {
    pub const MSG_ID: u16 = 22284;
}

impl PbMessage for DropCatchLevelRewardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DropCatchLevelInfos {
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
                self.DropCatchLevelInfos
                    .push(<DropCatchLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DropInBagNotify {
    pub DropItemInfos: Vec<DropBagItemInfo>,
    pub DropId: i32,
}

impl DropInBagNotify {
    pub const MSG_ID: u16 = 28463;
}

impl PbMessage for DropInBagNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DropItemInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.DropId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DropId);
        }
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
                self.DropItemInfos
                    .push(<DropBagItemInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.DropId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DynamicAttachComponentPb {
    pub AttachTarget: i32,
    pub AttachActor: Vec<u8>,
    pub RelatePos: Option<Vector>,
    pub RelateRot: Option<Rotator>,
    pub AttachPath: Vec<u8>,
}

impl PbMessage for DynamicAttachComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttachTarget != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttachTarget);
        }
        if !self.AttachActor.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AttachActor);
        }
        if let Some(v) = &self.RelatePos {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.RelateRot {
            crate::wire::write_submessage(out, 4, v);
        }
        if !self.AttachPath.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AttachPath);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AttachTarget = r.read_varint_i32()?,
            2 => self.AttachActor = r.read_len_delimited()?.to_vec(),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RelatePos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RelateRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.AttachPath = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerFunctionPreUnlockRequest {
    pub FuncId: i32,
}

impl EdgeRunnerFunctionPreUnlockRequest {
    pub const MSG_ID: u16 = 21322;
}

impl PbMessage for EdgeRunnerFunctionPreUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FuncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.FuncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerFunctionPreUnlockResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl EdgeRunnerFunctionPreUnlockResponse {
    pub const MSG_ID: u16 = 20110;
}

impl PbMessage for EdgeRunnerFunctionPreUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct EdgeRunnerLordGymCtx {
    pub LordGymId: i32,
}

impl PbMessage for EdgeRunnerLordGymCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LordGymId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LordGymId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LordGymId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerLordGymLevelPlayResultNotify {
    pub ItemDatas: Vec<ItemData>,
    pub LordGymPassRecord: Option<EdgeRunnerLordGymPassRecord>,
    pub IsPkgFull: bool,
    pub PassTime: i32,
    pub IsSuccess: bool,
    pub IsNewRecord: bool,
    pub FailLoadGymId: i32,
}

impl EdgeRunnerLordGymLevelPlayResultNotify {
    pub const MSG_ID: u16 = 24470;
}

impl PbMessage for EdgeRunnerLordGymLevelPlayResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemDatas {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.LordGymPassRecord {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.IsPkgFull {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PassTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassTime);
        }
        if self.IsSuccess {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsNewRecord {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FailLoadGymId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailLoadGymId);
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
                self.ItemDatas.push(<ItemData as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.LordGymPassRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.IsPkgFull = r.read_bool()?,
            8 => self.PassTime = r.read_varint_i32()?,
            12 => self.IsSuccess = r.read_bool()?,
            13 => self.IsNewRecord = r.read_bool()?,
            14 => self.FailLoadGymId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerPreUnlockAvailableNotify {
    pub PreUnlockIds: Vec<i32>,
}

impl EdgeRunnerPreUnlockAvailableNotify {
    pub const MSG_ID: u16 = 29069;
}

impl PbMessage for EdgeRunnerPreUnlockAvailableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PreUnlockIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PreUnlockIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerScoreRewardRequest {
    pub ScoreId: i32,
}

impl EdgeRunnerScoreRewardRequest {
    pub const MSG_ID: u16 = 28427;
}

impl PbMessage for EdgeRunnerScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ScoreId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ScoreId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ScoreId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerScoreRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl EdgeRunnerScoreRewardResponse {
    pub const MSG_ID: u16 = 20097;
}

impl PbMessage for EdgeRunnerScoreRewardResponse {
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
pub struct EdgeRunnerTaskRewardRequest {
    pub TaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl EdgeRunnerTaskRewardRequest {
    pub const MSG_ID: u16 = 28256;
}

impl PbMessage for EdgeRunnerTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EdgeRunnerTaskRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl EdgeRunnerTaskRewardResponse {
    pub const MSG_ID: u16 = 17834;
}

impl PbMessage for EdgeRunnerTaskRewardResponse {
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
pub struct EdgeRunnerUpdateNotify {
    pub FunctionId: i32,
    pub Task: Option<ConditionTask>,
}

impl EdgeRunnerUpdateNotify {
    pub const MSG_ID: u16 = 23668;
}

impl PbMessage for EdgeRunnerUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FunctionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FunctionId);
        }
        if let Some(v) = &self.Task {
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
            1 => self.FunctionId = r.read_varint_i32()?,
            11 => {
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
pub struct EffectiveWeekCardInfoNotify {
    pub WeekCardInfos: Vec<WeekCardInfo>,
    pub Infos: Option<WeekCardInfo>,
}

impl EffectiveWeekCardInfoNotify {
    pub const MSG_ID: u16 = 22090;
}

impl PbMessage for EffectiveWeekCardInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeekCardInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.Infos {
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
                self.WeekCardInfos
                    .push(<WeekCardInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElementUnit {
    pub ElementId: i32,
    pub Count: i32,
}

impl PbMessage for ElementUnit {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ElementId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ElementId);
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
            1 => self.ElementId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElementalAbilityUpdateNotify {
    pub EntityIncId: i64,
    pub ElementalType: Vec<u8>,
}

impl ElementalAbilityUpdateNotify {
    pub const MSG_ID: u16 = 28668;
}

impl PbMessage for ElementalAbilityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityIncId);
        }
        if !self.ElementalType.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ElementalType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.EntityIncId = r.read_varint_i64()?,
            5 => self.ElementalType = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElevatorMoveNotify {
    pub EntityId: i64,
    pub Location: i32,
    pub MoveType: Option<LiftMoveTypePb>,
}

impl ElevatorMoveNotify {
    pub const MSG_ID: u16 = 19828;
}

impl PbMessage for ElevatorMoveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Location != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Location);
        }
        if let Some(v) = &self.MoveType {
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
            1 => self.EntityId = r.read_varint_i64()?,
            4 => self.Location = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.MoveType
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElevatorStateChangeRequest {
    pub State: Option<ElevatorState>,
    pub EntityId: i64,
    pub TagState: i32,
}

impl ElevatorStateChangeRequest {
    pub const MSG_ID: u16 = 22336;
}

impl PbMessage for ElevatorStateChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.TagState != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagState);
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
                self.State = Some(<ElevatorState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.EntityId = r.read_varint_i64()?,
            11 => self.TagState = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ElevatorStateChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ElevatorStateChangeResponse {
    pub const MSG_ID: u16 = 19002;
}

impl PbMessage for ElevatorStateChangeResponse {
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
pub struct EnableFBParam {
    pub ParamIntList: Vec<i32>,
}

impl PbMessage for EnableFBParam {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ParamIntList {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ParamIntList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnableFunctionInfo {
    pub Type: Option<EnableFunctionType>,
    pub IsEnable: bool,
    pub FBParamList: Option<EnableFBParam>,
}

impl PbMessage for EnableFunctionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsEnable {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.FBParamList {
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
                self.Type = Some(
                    <EnableFunctionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.IsEnable = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FBParamList
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnableNearbyTrackingNotify {
    pub EntityIds: Vec<i64>,
    pub IsEnable: bool,
}

impl EnableNearbyTrackingNotify {
    pub const MSG_ID: u16 = 26906;
}

impl PbMessage for EnableNearbyTrackingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.IsEnable {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            8 => self.IsEnable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnableSystemFuncInfo {
    pub FuncId: i32,
    pub IsOpen: bool,
}

impl PbMessage for EnableSystemFuncInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FuncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FuncId);
        }
        if self.IsOpen {
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
            1 => self.FuncId = r.read_varint_i32()?,
            2 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EncircleChallengeCompleteRequest {
    pub GameSeconds: i32,
    pub Result: i32,
    pub ReducedDifficulty: bool,
    pub Step: i32,
    pub ChallengeId: i32,
}

impl EncircleChallengeCompleteRequest {
    pub const MSG_ID: u16 = 23593;
}

impl PbMessage for EncircleChallengeCompleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GameSeconds != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GameSeconds);
        }
        if self.Result != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Result);
        }
        if self.ReducedDifficulty {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Step != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Step);
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
            2 => self.GameSeconds = r.read_varint_i32()?,
            4 => self.Result = r.read_varint_i32()?,
            6 => self.ReducedDifficulty = r.read_bool()?,
            7 => self.Step = r.read_varint_i32()?,
            11 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EncircleChallengeCompleteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EncircleChallengeCompleteResponse {
    pub const MSG_ID: u16 = 23637;
}

impl PbMessage for EncircleChallengeCompleteResponse {
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
pub struct EncircleChallengeStartRequest {
    pub ChallengeId: i32,
}

impl EncircleChallengeStartRequest {
    pub const MSG_ID: u16 = 28751;
}

impl PbMessage for EncircleChallengeStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EncircleChallengeStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EncircleChallengeStartResponse {
    pub const MSG_ID: u16 = 25305;
}

impl PbMessage for EncircleChallengeStartResponse {
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
pub struct EncircleChallengeUpdateNotify {
    pub Challenges: Vec<EncircleChallengePb>,
}

impl EncircleChallengeUpdateNotify {
    pub const MSG_ID: u16 = 21412;
}

impl PbMessage for EncircleChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Challenges {
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
                self.Challenges
                    .push(<EncircleChallengePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EndLessHistoryRequest {}

impl EndLessHistoryRequest {
    pub const MSG_ID: u16 = 20333;
}

impl PbMessage for EndLessHistoryRequest {
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
pub struct EndLessHistoryResponse {
    pub NowSeasonRecord: Option<EndlessBattleRecord>,
    pub ErrorCode: Option<ErrorCode>,
    pub HistoryRecord: Option<EndlessBattleRecord>,
}

impl EndLessHistoryResponse {
    pub const MSG_ID: u16 = 29560;
}

impl PbMessage for EndLessHistoryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NowSeasonRecord {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.HistoryRecord {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NowSeasonRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.HistoryRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EndlessBattleRecord {
    pub Id: i32,
    pub FirstRecord: Option<HalfEndlessBattleRecord>,
    pub SecondRecord: Option<HalfEndlessBattleRecord>,
    pub SaveTime: i64,
}

impl PbMessage for EndlessBattleRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.FirstRecord {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.SecondRecord {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.SaveTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SaveTime);
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
                self.FirstRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SecondRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.SaveTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnrichmentAreaRequest {
    pub ItemId: i32,
}

impl EnrichmentAreaRequest {
    pub const MSG_ID: u16 = 27870;
}

impl PbMessage for EnrichmentAreaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnrichmentAreaResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnrichmentAreaResponse {
    pub const MSG_ID: u16 = 27649;
}

impl PbMessage for EnrichmentAreaResponse {
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
pub struct EnterAoiNotify {
    pub EntityInfos: Vec<DynamicEntityInformation>,
}

impl EnterAoiNotify {
    pub const MSG_ID: u16 = 20155;
}

impl PbMessage for EnterAoiNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityInfos {
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
                self.EntityInfos
                    .push(<DynamicEntityInformation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterAreaNotify {
    pub PlayerId: i32,
    pub AreaId: i32,
}

impl EnterAreaNotify {
    pub const MSG_ID: u16 = 25064;
}

impl PbMessage for EnterAreaNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            8 => self.PlayerId = r.read_varint_i32()?,
            12 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterLevelPlayNotify {
    pub CanGetReward: bool,
    pub State: i32,
    pub Id: i32,
}

impl EnterLevelPlayNotify {
    pub const MSG_ID: u16 = 28613;
}

impl PbMessage for EnterLevelPlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanGetReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
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
            2 => self.CanGetReward = r.read_bool()?,
            4 => self.State = r.read_varint_i32()?,
            6 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnterMatchInstRequest {}

impl EnterMatchInstRequest {
    pub const MSG_ID: u16 = 23840;
}

impl PbMessage for EnterMatchInstRequest {
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
pub struct EnterMatchInstResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EnterMatchInstResponse {
    pub const MSG_ID: u16 = 15610;
}

impl PbMessage for EnterMatchInstResponse {
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
pub struct EnterMatchTeamNotify {
    pub PlayerInfo: Option<MatchPlayerInfo>,
}

impl EnterMatchTeamNotify {
    pub const MSG_ID: u16 = 23456;
}

impl PbMessage for EnterMatchTeamNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct EntityAccessCasketRequest {
    pub AccessEntityIncIds: Vec<i64>,
    pub EntityIncId: i64,
    pub IsEnter: bool,
    pub CasketEntityIncId: i64,
}

impl EntityAccessCasketRequest {
    pub const MSG_ID: u16 = 17160;
}

impl PbMessage for EntityAccessCasketRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AccessEntityIncIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityIncId);
        }
        if self.IsEnter {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CasketEntityIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CasketEntityIncId);
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
                &mut self.AccessEntityIncIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            5 => self.EntityIncId = r.read_varint_i64()?,
            11 => self.IsEnter = r.read_bool()?,
            14 => self.CasketEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAccessCasketResponse {
    pub CasketEntityIncId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityAccessCasketResponse {
    pub const MSG_ID: u16 = 21699;
}

impl PbMessage for EntityAccessCasketResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CasketEntityIncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CasketEntityIncId);
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
            7 => self.CasketEntityIncId = r.read_varint_i64()?,
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
pub struct EntityAccessRangeNotify {
    pub RangeEntiyId: i64,
    pub AccessRangeType: Option<AccessRangeType>,
    pub AccessEntityIds: Vec<i64>,
}

impl EntityAccessRangeNotify {
    pub const MSG_ID: u16 = 23916;
}

impl PbMessage for EntityAccessRangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RangeEntiyId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RangeEntiyId);
        }
        if let Some(v) = &self.AccessRangeType {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.AccessEntityIds {
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
            1 => self.RangeEntiyId = r.read_varint_i64()?,
            11 => {
                self.AccessRangeType = Some(
                    <AccessRangeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AccessEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAdsorbRequest {
    pub EntityId: i32,
}

impl EntityAdsorbRequest {
    pub const MSG_ID: u16 = 24232;
}

impl PbMessage for EntityAdsorbRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => self.EntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityAdsorbResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityAdsorbResponse {
    pub const MSG_ID: u16 = 27477;
}

impl PbMessage for EntityAdsorbResponse {
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
pub struct EntityAttachChangeNotify {
    pub AttachPath: Vec<u8>,
    pub AttachEntity: Option<AttachTargetEntityInfo>,
    pub RelateRot: Option<Rotator>,
    pub TargetType: Option<AttachTargetType>,
    pub RelatePos: Option<Vector>,
    pub EntityId: i64,
}

impl EntityAttachChangeNotify {
    pub const MSG_ID: u16 = 26817;
}

impl PbMessage for EntityAttachChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AttachPath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AttachPath);
        }
        if let Some(v) = &self.AttachEntity {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.RelateRot {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.TargetType {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RelatePos {
            crate::wire::write_submessage(out, 11, v);
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
            1 => self.AttachPath = r.read_len_delimited()?.to_vec(),
            3 => {
                let bytes = r.read_len_delimited()?;
                self.AttachEntity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RelateRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                self.TargetType = Some(
                    <AttachTargetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.RelatePos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBattleInfoRequest {
    pub EntityId: i64,
}

impl EntityBattleInfoRequest {
    pub const MSG_ID: u16 = 23936;
}

impl PbMessage for EntityBattleInfoRequest {
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
pub struct EntityBattleInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub HateList: Vec<Ai_AiHateEntity>,
    pub PlayerTagInfos: Vec<EntityBattleTagInfo>,
    pub Attributes: Vec<GameplayAttributeData>,
    pub EntityPassiveSkillInfo: Vec<u8>,
    pub FightBuffComponentPb: Option<FightBuffComponentPb>,
    pub PartComponentPb: Option<PartComponentPb>,
    pub EntityId: i64,
    pub EntityBattleTagInfo: Vec<EntityBattleTagInfo>,
    pub FormationAttrs: Vec<FormationAttr>,
    pub ShieldComponentPb: Option<ShieldComponentPb>,
}

impl EntityBattleInfoResponse {
    pub const MSG_ID: u16 = 23069;
}

impl PbMessage for EntityBattleInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.HateList {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.PlayerTagInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.Attributes {
            crate::wire::write_submessage(out, 6, item);
        }
        if !self.EntityPassiveSkillInfo.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.EntityPassiveSkillInfo);
        }
        if let Some(v) = &self.FightBuffComponentPb {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.PartComponentPb {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.EntityBattleTagInfo {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.FormationAttrs {
            crate::wire::write_submessage(out, 14, item);
        }
        if let Some(v) = &self.ShieldComponentPb {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.HateList
                    .push(<Ai_AiHateEntity as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTagInfos
                    .push(<EntityBattleTagInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Attributes
                    .push(<GameplayAttributeData as PbMessage>::decode(bytes)?);
            }
            8 => self.EntityPassiveSkillInfo = r.read_len_delimited()?.to_vec(),
            9 => {
                let bytes = r.read_len_delimited()?;
                self.FightBuffComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PartComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.EntityId = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.EntityBattleTagInfo
                    .push(<EntityBattleTagInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.FormationAttrs
                    .push(<FormationAttr as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.ShieldComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBattleTagInfo {
    pub TagId: i32,
    pub Count: i32,
}

impl PbMessage for EntityBattleTagInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TagId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
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
            1 => self.TagId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBeamReceiveCtxPb {
    pub ReceiveType: Option<EntityBeamReceiveType>,
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for EntityBeamReceiveCtxPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ReceiveType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.EntityCtx {
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
                self.ReceiveType = Some(
                    <EntityBeamReceiveType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
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
pub struct EntityBeamReceiveRequest {
    pub EntityId: i64,
    pub ReceiveType: Option<EntityBeamReceiveType>,
}

impl EntityBeamReceiveRequest {
    pub const MSG_ID: u16 = 22992;
}

impl PbMessage for EntityBeamReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.ReceiveType {
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
            9 => self.EntityId = r.read_varint_i64()?,
            10 => {
                self.ReceiveType = Some(
                    <EntityBeamReceiveType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EntityBeamReceiveResponse {
    pub ReceiveType: Option<EntityBeamReceiveType>,
    pub ErrorCode: Option<ErrorCode>,
    pub EntityId: i64,
}

impl EntityBeamReceiveResponse {
    pub const MSG_ID: u16 = 27467;
}

impl PbMessage for EntityBeamReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ReceiveType {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            9 => {
                self.ReceiveType = Some(
                    <EntityBeamReceiveType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBlackboardNotify {
    pub Params: Vec<BlackboardParam>,
    pub EntityId: i64,
}

impl EntityBlackboardNotify {
    pub const MSG_ID: u16 = 17963;
}

impl PbMessage for EntityBlackboardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Params
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBlackboardRequest {
    pub Params: Vec<BlackboardParam>,
    pub EntityId: i64,
}

impl EntityBlackboardRequest {
    pub const MSG_ID: u16 = 19528;
}

impl PbMessage for EntityBlackboardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Params {
            crate::wire::write_submessage(out, 9, item);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Params
                    .push(<BlackboardParam as PbMessage>::decode(bytes)?);
            }
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBlackboardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityBlackboardResponse {
    pub const MSG_ID: u16 = 26328;
}

impl PbMessage for EntityBlackboardResponse {
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
pub struct EntityBuffProducerOperateRequest {
    pub OpEntityId: i64,
    pub OpType: Option<BuffOperateType>,
}

impl EntityBuffProducerOperateRequest {
    pub const MSG_ID: u16 = 25261;
}

impl PbMessage for EntityBuffProducerOperateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OpEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpEntityId);
        }
        if let Some(v) = &self.OpType {
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
            4 => self.OpEntityId = r.read_varint_i64()?,
            9 => {
                self.OpType = Some(
                    <BuffOperateType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EntityBuffProducerOperateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub OpEntityId: i64,
    pub OpType: Option<BuffOperateType>,
}

impl EntityBuffProducerOperateResponse {
    pub const MSG_ID: u16 = 22978;
}

impl PbMessage for EntityBuffProducerOperateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.OpEntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpEntityId);
        }
        if let Some(v) = &self.OpType {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.OpEntityId = r.read_varint_i64()?,
            11 => {
                self.OpType = Some(
                    <BuffOperateType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct EntityBuffProducerRequest {
    pub BuffConsumerId: i64,
}

impl EntityBuffProducerRequest {
    pub const MSG_ID: u16 = 18892;
}

impl PbMessage for EntityBuffProducerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffConsumerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BuffConsumerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.BuffConsumerId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityBuffProducerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityBuffProducerResponse {
    pub const MSG_ID: u16 = 18914;
}

impl PbMessage for EntityBuffProducerResponse {
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
pub struct EntityBulletComponentPb {
    pub ContextId: i64,
}

impl PbMessage for EntityBulletComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ContextId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ContextId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityCameraAlertRequest {
    pub EntityId: i64,
    pub IsAlert: bool,
}

impl EntityCameraAlertRequest {
    pub const MSG_ID: u16 = 15507;
}

impl PbMessage for EntityCameraAlertRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsAlert {
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
            13 => self.EntityId = r.read_varint_i64()?,
            14 => self.IsAlert = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityCameraAlertResponse {
    pub EntityId: i64,
    pub ErrorCode: Option<ErrorCode>,
    pub IsAlert: bool,
}

impl EntityCameraAlertResponse {
    pub const MSG_ID: u16 = 21236;
}

impl PbMessage for EntityCameraAlertResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsAlert {
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
            5 => self.EntityId = r.read_varint_i64()?,
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.IsAlert = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityDestroyResult {
    pub EntityId: i64,
    pub IsDestroyed: bool,
}

impl PbMessage for EntityDestroyResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsDestroyed {
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
            2 => self.IsDestroyed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityElementalAbilityEnableNotify {
    pub EntityIncId: i64,
    pub Enable: bool,
}

impl EntityElementalAbilityEnableNotify {
    pub const MSG_ID: u16 = 16885;
}

impl PbMessage for EntityElementalAbilityEnableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityIncId);
        }
        if self.Enable {
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
            4 => self.EntityIncId = r.read_varint_i64()?,
            6 => self.Enable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFlyEquipChangeNotify {
    pub SkinChanges: Vec<EntityFlySkin>,
}

impl EntityFlyEquipChangeNotify {
    pub const MSG_ID: u16 = 16196;
}

impl PbMessage for EntityFlyEquipChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinChanges {
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
                self.SkinChanges
                    .push(<EntityFlySkin as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFlySkin {
    pub EntityId: i64,
    pub SkinInfos: Vec<FlySkinInfo>,
}

impl PbMessage for EntityFlySkin {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.SkinInfos {
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
                self.SkinInfos
                    .push(<FlySkinInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityFoundationCoreChangeNotify {
    pub FoundationCoreInfo: Option<FoundationCoreInfo>,
    pub FoundationEntityId: i32,
    pub HasFoundationCore: bool,
}

impl EntityFoundationCoreChangeNotify {
    pub const MSG_ID: u16 = 24775;
}

impl PbMessage for EntityFoundationCoreChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FoundationCoreInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.FoundationEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FoundationEntityId);
        }
        if self.HasFoundationCore {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FoundationCoreInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.FoundationEntityId = r.read_varint_i32()?,
            14 => self.HasFoundationCore = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityGravityUpdateNotify {
    pub PreGravityDirection: Option<Vector>,
    pub EntityId: i64,
    pub CurGravityDirection: Option<Vector>,
}

impl EntityGravityUpdateNotify {
    pub const MSG_ID: u16 = 26798;
}

impl PbMessage for EntityGravityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PreGravityDirection {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.CurGravityDirection {
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
                self.PreGravityDirection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.EntityId = r.read_varint_i64()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.CurGravityDirection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityGridPbData {
    pub Id: i64,
}

impl PbMessage for EntityGridPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityGroupFailureActionCtxPb {
    pub EntityCtx: Option<EntityCtxPb>,
}

impl PbMessage for EntityGroupFailureActionCtxPb {
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
pub struct EntityHeadInfoNotify {
    pub TidName: Vec<u8>,
    pub SecondaryName: Vec<u8>,
    pub EntityId: i64,
    pub Icon: Vec<u8>,
}

impl EntityHeadInfoNotify {
    pub const MSG_ID: u16 = 19315;
}

impl PbMessage for EntityHeadInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TidName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TidName);
        }
        if !self.SecondaryName.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SecondaryName);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.Icon.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Icon);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.TidName = r.read_len_delimited()?.to_vec(),
            5 => self.SecondaryName = r.read_len_delimited()?.to_vec(),
            10 => self.EntityId = r.read_varint_i64()?,
            13 => self.Icon = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityHonamiStoryEnhanceLevelNotify {
    pub HonamiStoryEnhanceLevelComponentPb: Option<HonamiStoryEnhanceLevelComponentPb>,
    pub EntityId: i64,
}

impl EntityHonamiStoryEnhanceLevelNotify {
    pub const MSG_ID: u16 = 16677;
}

impl PbMessage for EntityHonamiStoryEnhanceLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryEnhanceLevelComponentPb {
            crate::wire::write_submessage(out, 4, v);
        }
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryEnhanceLevelComponentPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityInteractingNotify {
    pub IsInteracting: bool,
    pub EntityId: i64,
}

impl EntityInteractingNotify {
    pub const MSG_ID: u16 = 17894;
}

impl PbMessage for EntityInteractingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsInteracting {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            2 => self.IsInteracting = r.read_bool()?,
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityItemBundleInfo {
    pub EntityId: i64,
    pub ItemId: i32,
}

impl PbMessage for EntityItemBundleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            1 => self.EntityId = r.read_varint_i64()?,
            2 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityOnLandedNotify {
    pub Id: i64,
}

impl EntityOnLandedNotify {
    pub const MSG_ID: u16 = 29646;
}

impl PbMessage for EntityOnLandedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolAddTagRequest {
    pub Index: i32,
    pub EntityId: i64,
    pub TagId: i32,
    pub SplineEntityConfigId: i32,
}

impl EntityPatrolAddTagRequest {
    pub const MSG_ID: u16 = 22474;
}

impl PbMessage for EntityPatrolAddTagRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        if self.SplineEntityConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SplineEntityConfigId);
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
            7 => self.EntityId = r.read_varint_i64()?,
            9 => self.TagId = r.read_varint_i32()?,
            11 => self.SplineEntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolAddTagResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityPatrolAddTagResponse {
    pub const MSG_ID: u16 = 21942;
}

impl PbMessage for EntityPatrolAddTagResponse {
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
pub struct EntityPatrolChangeDirRequest {
    pub EntityId: i64,
    pub Dir: bool,
}

impl EntityPatrolChangeDirRequest {
    pub const MSG_ID: u16 = 16389;
}

impl PbMessage for EntityPatrolChangeDirRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.Dir {
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
            1 => self.EntityId = r.read_varint_i64()?,
            6 => self.Dir = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityPatrolChangeDirResponse {
    pub Code: Option<ErrorCode>,
}

impl EntityPatrolChangeDirResponse {
    pub const MSG_ID: u16 = 26449;
}

impl PbMessage for EntityPatrolChangeDirResponse {
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
pub struct EntityPreloadNotify {
    pub PreloadEntityIds: Vec<i64>,
}

impl EntityPreloadNotify {
    pub const MSG_ID: u16 = 28870;
}

impl PbMessage for EntityPreloadNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PreloadEntityIds {
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
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PreloadEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRandomInteractRequest {
    pub OptionIndex: i32,
    pub EntityId: i64,
}

impl EntityRandomInteractRequest {
    pub const MSG_ID: u16 = 18332;
}

impl PbMessage for EntityRandomInteractRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OptionIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OptionIndex);
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
            9 => self.OptionIndex = r.read_varint_i32()?,
            11 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRandomInteractResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Interacting: bool,
}

impl EntityRandomInteractResponse {
    pub const MSG_ID: u16 = 25922;
}

impl PbMessage for EntityRandomInteractResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Interacting {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.Interacting = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityRewardItemNotify {
    pub Id: i64,
    pub EntityRewardItemPb: Option<EntityRewardItemPb>,
}

impl EntityRewardItemNotify {
    pub const MSG_ID: u16 = 29972;
}

impl PbMessage for EntityRewardItemNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        if let Some(v) = &self.EntityRewardItemPb {
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
            3 => self.Id = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.EntityRewardItemPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntitySendEventRequest {
    pub EventKey: Vec<u8>,
    pub EntityId: i64,
}

impl EntitySendEventRequest {
    pub const MSG_ID: u16 = 22433;
}

impl PbMessage for EntitySendEventRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.EventKey.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.EventKey);
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
            5 => self.EventKey = r.read_len_delimited()?.to_vec(),
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntitySendEventResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntitySendEventResponse {
    pub const MSG_ID: u16 = 21355;
}

impl PbMessage for EntitySendEventResponse {
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
pub struct EntityStateChangeResult {
    pub EntityId: i64,
    pub StateTagId: i32,
}

impl PbMessage for EntityStateChangeResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.StateTagId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateTagId);
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
            2 => self.StateTagId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityTimeFlowPush {
    pub EntityId: i64,
    pub TimeDilation: f32,
}

impl EntityTimeFlowPush {
    pub const MSG_ID: u16 = 25621;
}

impl PbMessage for EntityTimeFlowPush {
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
pub struct EntityVarUpdateNotify {
    pub EntityId: i64,
    pub VarName: Vec<u8>,
    pub VarDefine: Option<VarDefinePb>,
}

impl EntityVarUpdateNotify {
    pub const MSG_ID: u16 = 28972;
}

impl PbMessage for EntityVarUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if !self.VarName.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.VarName);
        }
        if let Some(v) = &self.VarDefine {
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
            1 => self.EntityId = r.read_varint_i64()?,
            6 => self.VarName = r.read_len_delimited()?.to_vec(),
            7 => {
                let bytes = r.read_len_delimited()?;
                self.VarDefine
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityWalkingPatternFinishRequest {
    pub EntityId: i64,
    pub HostPlayerId: i32,
    pub Point: i32,
}

impl EntityWalkingPatternFinishRequest {
    pub const MSG_ID: u16 = 29045;
}

impl PbMessage for EntityWalkingPatternFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            2 => self.EntityId = r.read_varint_i64()?,
            8 => self.HostPlayerId = r.read_varint_i32()?,
            9 => self.Point = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntityWalkingPatternFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EntityWalkingPatternFinishResponse {
    pub const MSG_ID: u16 = 23002;
}

impl PbMessage for EntityWalkingPatternFinishResponse {
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
pub struct EntranceStateNotify {
    pub State: Option<EntranceState>,
    pub Id: i64,
}

impl EntranceStateNotify {
    pub const MSG_ID: u16 = 25612;
}

impl PbMessage for EntranceStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            12 => {
                self.State = Some(<EntranceState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntrustInfo {
    pub EntrustId: i32,
    pub IsVisiable: bool,
    pub HistoryLevel: i32,
}

impl PbMessage for EntrustInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustId);
        }
        if self.IsVisiable {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HistoryLevel != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.EntrustId = r.read_varint_i32()?,
            2 => self.IsVisiable = r.read_bool()?,
            3 => self.HistoryLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EntrustRoleInfo {
    pub Level: i32,
    pub RoleAttr: Option<RoleProperSettle>,
}

impl PbMessage for EntrustRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if let Some(v) = &self.RoleAttr {
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
            1 => self.Level = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleAttr
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipSkinTakeOnRequest {
    pub Data: Option<RoleSkinEquipData>,
}

impl EquipSkinTakeOnRequest {
    pub const MSG_ID: u16 = 16426;
}

impl PbMessage for EquipSkinTakeOnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct EquipSkinTakeOnResponse {
    pub DataList: Vec<RoleSkinEquipData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl EquipSkinTakeOnResponse {
    pub const MSG_ID: u16 = 26442;
}

impl PbMessage for EquipSkinTakeOnResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DataList {
            crate::wire::write_submessage(out, 10, item);
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.DataList
                    .push(<RoleSkinEquipData as PbMessage>::decode(bytes)?);
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
pub struct EquipSkinUnLoadRequest {
    pub RoleID: i32,
}

impl EquipSkinUnLoadRequest {
    pub const MSG_ID: u16 = 25017;
}

impl PbMessage for EquipSkinUnLoadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleID != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleID);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.RoleID = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EquipSkinUnLoadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl EquipSkinUnLoadResponse {
    pub const MSG_ID: u16 = 15525;
}

impl PbMessage for EquipSkinUnLoadResponse {
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
pub struct EventInfo {
    pub EventType: i32,
    pub Count: i32,
}

impl PbMessage for EventInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventType);
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
            1 => self.EventType = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventOption {
    pub ConfigId: i32,
    pub MustSelect: bool,
    pub CondInfo: Option<CondInfo>,
}

impl PbMessage for EventOption {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.MustSelect {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.CondInfo {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.MustSelect = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.CondInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventPlotInfo {
    pub PlotId: i32,
    pub StepId: i32,
    pub StepState: Option<StepState>,
    pub Options: Vec<EventOption>,
    pub ModifyImage: i32,
    pub ModifyBgm: i32,
}

impl PbMessage for EventPlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlotId);
        }
        if self.StepId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StepId);
        }
        if let Some(v) = &self.StepState {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Options {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.ModifyImage != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModifyImage);
        }
        if self.ModifyBgm != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ModifyBgm);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PlotId = r.read_varint_i32()?,
            2 => self.StepId = r.read_varint_i32()?,
            3 => {
                self.StepState = Some(<StepState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Options
                    .push(<EventOption as PbMessage>::decode(bytes)?);
            }
            5 => self.ModifyImage = r.read_varint_i32()?,
            6 => self.ModifyBgm = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EvolveView {
    pub SurvivorsGainDatas: Vec<SurvivorsGainData>,
}

impl PbMessage for EvolveView {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SurvivorsGainDatas {
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
                self.SurvivorsGainDatas
                    .push(<SurvivorsGainData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExamFinishSyncDataPb {
    pub ExamId: i32,
    pub Score: i32,
}

impl PbMessage for ExamFinishSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExamId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExamId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.ExamId = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExchangeRewardInfoNotify {
    pub Count: i32,
    pub ExchangeRewardId: i32,
}

impl ExchangeRewardInfoNotify {
    pub const MSG_ID: u16 = 22486;
}

impl PbMessage for ExchangeRewardInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Count != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.ExchangeRewardId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExchangeRewardId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Count = r.read_varint_i32()?,
            13 => self.ExchangeRewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExchangeRewardInfoRequest {}

impl ExchangeRewardInfoRequest {
    pub const MSG_ID: u16 = 25606;
}

impl PbMessage for ExchangeRewardInfoRequest {
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
pub struct ExchangeRewardInfoResponse {
    pub ExchangeRewardDict: BTreeMap<i32, i32>,
    pub SharedDict: BTreeMap<i32, i32>,
}

impl ExchangeRewardInfoResponse {
    pub const MSG_ID: u16 = 23945;
}

impl PbMessage for ExchangeRewardInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ExchangeRewardDict.is_empty() {
            crate::wire::write_map(out, 12, &self.ExchangeRewardDict);
        }
        if !self.SharedDict.is_empty() {
            crate::wire::write_map(out, 13, &self.SharedDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => crate::wire::decode_map_entry(r, &mut self.ExchangeRewardDict)?,
            13 => crate::wire::decode_map_entry(r, &mut self.SharedDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExchangeSharedInfoNotify {
    pub SharedUpdateDict: BTreeMap<i32, i32>,
}

impl ExchangeSharedInfoNotify {
    pub const MSG_ID: u16 = 23282;
}

impl PbMessage for ExchangeSharedInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.SharedUpdateDict.is_empty() {
            crate::wire::write_map(out, 9, &self.SharedUpdateDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => crate::wire::decode_map_entry(r, &mut self.SharedUpdateDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExhibitionItemSaveRequest {
    pub EntityItemBundleInfos: Vec<EntityItemBundleInfo>,
}

impl ExhibitionItemSaveRequest {
    pub const MSG_ID: u16 = 24520;
}

impl PbMessage for ExhibitionItemSaveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityItemBundleInfos {
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
                self.EntityItemBundleInfos
                    .push(<EntityItemBundleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExhibitionItemSaveResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ExhibitionItemSaveResponse {
    pub const MSG_ID: u16 = 18661;
}

impl PbMessage for ExhibitionItemSaveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrCode {
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
pub struct ExitGamePush {}

impl ExitGamePush {
    pub const MSG_ID: u16 = 114;
}

impl PbMessage for ExitGamePush {
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
pub struct ExpTips {
    pub RoleId: i32,
    pub Exp: i32,
}

impl PbMessage for ExpTips {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Exp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreActivityInfo {
    pub ActivityTasks: BTreeMap<i32, ActivityTaskState>,
}

impl PbMessage for ExploreActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActivityTasks.is_empty() {
            crate::wire::write_map(out, 1, &self.ActivityTasks);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.ActivityTasks)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreActivityTaskInfoNotify {
    pub ActivityTasks: BTreeMap<i32, ActivityTaskState>,
}

impl ExploreActivityTaskInfoNotify {
    pub const MSG_ID: u16 = 16491;
}

impl PbMessage for ExploreActivityTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ActivityTasks.is_empty() {
            crate::wire::write_map(out, 11, &self.ActivityTasks);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => crate::wire::decode_map_entry(r, &mut self.ActivityTasks)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreActivityTaskRewardRequest {
    pub TaskId: i32,
}

impl ExploreActivityTaskRewardRequest {
    pub const MSG_ID: u16 = 27918;
}

impl PbMessage for ExploreActivityTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreActivityTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ExploreActivityTaskRewardResponse {
    pub const MSG_ID: u16 = 19374;
}

impl PbMessage for ExploreActivityTaskRewardResponse {
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
pub struct ExploreEntityTraceRequest {
    pub ExploratoryDegree: i32,
    pub AreaId: i32,
}

impl ExploreEntityTraceRequest {
    pub const MSG_ID: u16 = 20228;
}

impl PbMessage for ExploreEntityTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExploratoryDegree != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploratoryDegree);
        }
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            1 => self.ExploratoryDegree = r.read_varint_i32()?,
            13 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreEntityTraceResponse {
    pub EntityIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ExploreEntityTraceResponse {
    pub const MSG_ID: u16 = 29477;
}

impl PbMessage for ExploreEntityTraceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
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
pub struct ExploreProgressRewardRequest {
    pub RewardIds: Vec<i32>,
}

impl ExploreProgressRewardRequest {
    pub const MSG_ID: u16 = 29451;
}

impl PbMessage for ExploreProgressRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIds {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreProgressRewardResponse {
    pub RewardIds: Vec<i32>,
}

impl ExploreProgressRewardResponse {
    pub const MSG_ID: u16 = 28094;
}

impl PbMessage for ExploreProgressRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreProgressRewardTake {
    pub AreaId: i32,
    pub ExploreProgress: Vec<i32>,
}

impl PbMessage for ExploreProgressRewardTake {
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
pub struct ExploreScoreRewardRequest {
    pub AreaId: i32,
    pub ExploreProgress: i32,
}

impl ExploreScoreRewardRequest {
    pub const MSG_ID: u16 = 23970;
}

impl PbMessage for ExploreScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaId);
        }
        if self.ExploreProgress != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExploreProgress);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.AreaId = r.read_varint_i32()?,
            15 => self.ExploreProgress = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreScoreRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ExploreScoreRewardResponse {
    pub const MSG_ID: u16 = 18852;
}

impl PbMessage for ExploreScoreRewardResponse {
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
pub struct ExploreSkillActionRequest {
    pub EntityId: i64,
}

impl ExploreSkillActionRequest {
    pub const MSG_ID: u16 = 17369;
}

impl PbMessage for ExploreSkillActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            13 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreSkillActionResponse {
    pub EntityId: i64,
    pub Code: Option<ErrorCode>,
}

impl ExploreSkillActionResponse {
    pub const MSG_ID: u16 = 19712;
}

impl PbMessage for ExploreSkillActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Code {
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
            6 => self.EntityId = r.read_varint_i64()?,
            7 => {
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
pub struct ExploreSkillCustomRequest {
    pub EntityId: i64,
}

impl ExploreSkillCustomRequest {
    pub const MSG_ID: u16 = 28056;
}

impl PbMessage for ExploreSkillCustomRequest {
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
pub struct ExploreSkillCustomResponse {
    pub EntityId: i64,
    pub Code: Option<ErrorCode>,
}

impl ExploreSkillCustomResponse {
    pub const MSG_ID: u16 = 25414;
}

impl PbMessage for ExploreSkillCustomResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            11 => self.EntityId = r.read_varint_i64()?,
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
pub struct ExploreSkillPullGiantRequest {
    pub EntityId: i64,
}

impl ExploreSkillPullGiantRequest {
    pub const MSG_ID: u16 = 25809;
}

impl PbMessage for ExploreSkillPullGiantRequest {
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
pub struct ExploreSkillPullGiantResponse {
    pub Code: Option<ErrorCode>,
    pub EntityId: i64,
}

impl ExploreSkillPullGiantResponse {
    pub const MSG_ID: u16 = 21630;
}

impl PbMessage for ExploreSkillPullGiantResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            3 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreToolAuthorizationNotify {
    pub AuthorizationItems: Vec<i32>,
}

impl ExploreToolAuthorizationNotify {
    pub const MSG_ID: u16 = 16622;
}

impl PbMessage for ExploreToolAuthorizationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AuthorizationItems {
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
                &mut self.AuthorizationItems,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExploreToolUpdateNotify {
    pub SkillId: i32,
}

impl ExploreToolUpdateNotify {
    pub const MSG_ID: u16 = 26462;
}

impl PbMessage for ExploreToolUpdateNotify {
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
pub struct FRActionChangeTag {
    pub UnitIncId: i32,
    pub TagId: i64,
}

impl PbMessage for FRActionChangeTag {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnitIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnitIncId);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TagId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.UnitIncId = r.read_varint_i32()?,
            2 => self.TagId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRActionSelfDefineValue {
    pub UnitIncId: i32,
    pub N: i64,
    pub M: i64,
    pub P: i64,
}

impl PbMessage for FRActionSelfDefineValue {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnitIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnitIncId);
        }
        if self.N != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.N);
        }
        if self.M != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.M);
        }
        if self.P != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.P);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.UnitIncId = r.read_varint_i32()?,
            2 => self.N = r.read_varint_i64()?,
            3 => self.M = r.read_varint_i64()?,
            4 => self.P = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRActionStageTributeChange {
    pub TributeCount: i64,
}

impl PbMessage for FRActionStageTributeChange {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TributeCount != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TributeCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TributeCount = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRActionStop {
    pub TextKey: Vec<u8>,
}

impl PbMessage for FRActionStop {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TextKey.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TextKey);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TextKey = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRActionToyLvUp {
    pub ToyId: i32,
    pub ToyLv: i64,
}

impl PbMessage for FRActionToyLvUp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ToyId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToyId);
        }
        if self.ToyLv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ToyLv);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ToyId = r.read_varint_i32()?,
            2 => self.ToyLv = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRChangePointAction {
    pub Phantom: Vec<FloroRanchPlayUnit>,
}

impl PbMessage for FRChangePointAction {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Phantom {
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
                self.Phantom
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRDebugActionInfo {
    pub ActionId: i32,
    pub TargetIncId: Vec<i32>,
    pub ExInfo: Vec<u8>,
}

impl PbMessage for FRDebugActionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionId);
        }
        for item in &self.TargetIncId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.ExInfo.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ExInfo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ActionId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetIncId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.ExInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FREvolveData {
    pub CurExp: i32,
    pub CurLevel: i32,
    pub ExpPerLevel: i32,
}

impl PbMessage for FREvolveData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurExp != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurExp);
        }
        if self.CurLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLevel);
        }
        if self.ExpPerLevel != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExpPerLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurExp = r.read_varint_i32()?,
            2 => self.CurLevel = r.read_varint_i32()?,
            3 => self.ExpPerLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FREvolveUpdateAction {
    pub TargetIncId: i32,
    pub NewData: Option<FREvolveData>,
}

impl PbMessage for FREvolveUpdateAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetIncId);
        }
        if let Some(v) = &self.NewData {
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
            1 => self.TargetIncId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NewData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRMixAction {
    pub TargetUnit: Option<FloroRanchPlayUnit>,
    pub MixMaterialIncId: Vec<i32>,
}

impl PbMessage for FRMixAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TargetUnit {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.MixMaterialIncId {
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
                self.TargetUnit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MixMaterialIncId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRSacrifice {
    pub ActionList: Vec<FloroRanchUnitActionMsg>,
}

impl PbMessage for FRSacrifice {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActionList {
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
                self.ActionList
                    .push(<FloroRanchUnitActionMsg as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRSkillData {
    pub CurCanUseNum: i32,
    pub CurDayCanUseNum: i32,
    pub SkillId: i32,
}

impl PbMessage for FRSkillData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCanUseNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCanUseNum);
        }
        if self.CurDayCanUseNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurDayCanUseNum);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.CurCanUseNum = r.read_varint_i32()?,
            2 => self.CurDayCanUseNum = r.read_varint_i32()?,
            3 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitActions {
    pub UnitActions: Vec<FloroRanchUnitActionMsg>,
}

impl PbMessage for FRUnitActions {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnitActions {
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
                self.UnitActions
                    .push(<FloroRanchUnitActionMsg as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitBuffOperateActionGroup {
    pub Actions: Vec<FRUnitOperateBuffAction>,
}

impl PbMessage for FRUnitBuffOperateActionGroup {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Actions {
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
                self.Actions
                    .push(<FRUnitOperateBuffAction as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitEatingAction {
    pub TargetUnitIncId: i32,
    pub ResourcesChange: Option<FRUnitResourcesChangeAction>,
    pub Actions: Vec<FloroRanchUnitActionMsg>,
}

impl PbMessage for FRUnitEatingAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUnitIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUnitIncId);
        }
        if let Some(v) = &self.ResourcesChange {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.Actions {
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
            1 => self.TargetUnitIncId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ResourcesChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Actions
                    .push(<FloroRanchUnitActionMsg as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitEatingActionGroup {
    pub Actions: Vec<FRUnitEatingAction>,
}

impl PbMessage for FRUnitEatingActionGroup {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Actions {
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
                self.Actions
                    .push(<FRUnitEatingAction as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitOperateAction {
    pub OpType: Option<FRUnitOperate>,
    pub UnitType: Option<FloroRanchActionUnitType>,
    pub Phantom: Option<FloroRanchPlayUnit>,
}

impl PbMessage for FRUnitOperateAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OpType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UnitType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Phantom {
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
                self.OpType = Some(<FRUnitOperate as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                self.UnitType = Some(
                    <FloroRanchActionUnitType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
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
pub struct FRUnitOperateActionGroup {
    pub Actions: Vec<FRUnitOperateAction>,
}

impl PbMessage for FRUnitOperateActionGroup {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Actions {
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
                self.Actions
                    .push(<FRUnitOperateAction as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitOperateBuffAction {
    pub Type: Option<FRBuffOperate>,
    pub TargetUnitIncId: i32,
    pub Buff: Option<FloroRanchPlayBuff>,
}

impl PbMessage for FRUnitOperateBuffAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TargetUnitIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUnitIncId);
        }
        if let Some(v) = &self.Buff {
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
                self.Type = Some(<FRBuffOperate as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.TargetUnitIncId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Buff
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FRUnitResourcesChangeAction {
    pub Type: Option<FRUnitResourcesChangeType>,
    pub ChangeCount: i64,
    pub TargetUnitIncId: i32,
}

impl PbMessage for FRUnitResourcesChangeAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ChangeCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ChangeCount);
        }
        if self.TargetUnitIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUnitIncId);
        }
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
                    <FRUnitResourcesChangeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.ChangeCount = r.read_varint_i64()?,
            3 => self.TargetUnitIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FailRollbackData {
    pub FailNodeId: i32,
    pub FailReason: Option<FailReason>,
}

impl PbMessage for FailRollbackData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FailNodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailNodeId);
        }
        if let Some(v) = &self.FailReason {
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
            1 => self.FailNodeId = r.read_varint_i32()?,
            2 => {
                self.FailReason = Some(<FailReason as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FallFinishNotify {}

impl FallFinishNotify {
    pub const MSG_ID: u16 = 22427;
}

impl PbMessage for FallFinishNotify {
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
pub struct FallbackViewOp {
    pub ShowRestCount: bool,
    pub RestCount: i32,
}

impl PbMessage for FallbackViewOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShowRestCount {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RestCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RestCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ShowRestCount = r.read_bool()?,
            2 => self.RestCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldCtx {
    pub SkillBranchId: Vec<i32>,
}

impl PbMessage for FarmGoldCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkillBranchId {
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
            1 => {
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
pub struct FarmGoldInfo {
    pub PointsRewardGet: Vec<i32>,
    pub LevelPlayTasks: Vec<FarmGoldLevelPlayInfo>,
}

impl PbMessage for FarmGoldInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PointsRewardGet {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.LevelPlayTasks {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.PointsRewardGet,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.LevelPlayTasks
                    .push(<FarmGoldLevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelDiffRequest {
    pub ActivityId: i32,
    pub Diff: i32,
    pub InstId: i32,
}

impl FarmGoldLevelDiffRequest {
    pub const MSG_ID: u16 = 19651;
}

impl PbMessage for FarmGoldLevelDiffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Diff != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Diff);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            12 => self.Diff = r.read_varint_i32()?,
            15 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelDiffResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldLevelDiffResponse {
    pub const MSG_ID: u16 = 26855;
}

impl PbMessage for FarmGoldLevelDiffResponse {
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
pub struct FarmGoldLevelNotify {
    pub LevelPlayTasks: Option<FarmGoldLevelPlayInfo>,
    pub ActivityId: i32,
}

impl FarmGoldLevelNotify {
    pub const MSG_ID: u16 = 24429;
}

impl PbMessage for FarmGoldLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LevelPlayTasks {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.LevelPlayTasks
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelPlayRepeatedRequest {
    pub InstIdList: Vec<i32>,
    pub ActivityId: i32,
}

impl FarmGoldLevelPlayRepeatedRequest {
    pub const MSG_ID: u16 = 21472;
}

impl PbMessage for FarmGoldLevelPlayRepeatedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstIdList {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InstIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelPlayRepeatedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldLevelPlayRepeatedResponse {
    pub const MSG_ID: u16 = 16947;
}

impl PbMessage for FarmGoldLevelPlayRepeatedResponse {
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
pub struct FarmGoldLevelPlayRequest {
    pub ActivityId: i32,
    pub InstId: i32,
}

impl FarmGoldLevelPlayRequest {
    pub const MSG_ID: u16 = 22663;
}

impl PbMessage for FarmGoldLevelPlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            3 => self.ActivityId = r.read_varint_i32()?,
            13 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldLevelPlayResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldLevelPlayResponse {
    pub const MSG_ID: u16 = 26415;
}

impl PbMessage for FarmGoldLevelPlayResponse {
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
pub struct FarmGoldPointRepeatedRequest {
    pub ActivityId: i32,
    pub Ids: Vec<i32>,
}

impl FarmGoldPointRepeatedRequest {
    pub const MSG_ID: u16 = 29763;
}

impl PbMessage for FarmGoldPointRepeatedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
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
            8 => self.ActivityId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldPointRepeatedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldPointRepeatedResponse {
    pub const MSG_ID: u16 = 18879;
}

impl PbMessage for FarmGoldPointRepeatedResponse {
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
pub struct FarmGoldPointRequest {
    pub ActivityId: i32,
    pub Id: i32,
}

impl FarmGoldPointRequest {
    pub const MSG_ID: u16 = 25630;
}

impl PbMessage for FarmGoldPointRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
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
            6 => self.ActivityId = r.read_varint_i32()?,
            10 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldPointResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldPointResponse {
    pub const MSG_ID: u16 = 27220;
}

impl PbMessage for FarmGoldPointResponse {
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
pub struct FarmGoldResultNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub HisPoint: i32,
    pub Succ: bool,
    pub CurPoint: i32,
}

impl FarmGoldResultNotify {
    pub const MSG_ID: u16 = 20566;
}

impl PbMessage for FarmGoldResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HisPoint != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HisPoint);
        }
        if self.Succ {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurPoint != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPoint);
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
            9 => self.HisPoint = r.read_varint_i32()?,
            12 => self.Succ = r.read_bool()?,
            14 => self.CurPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FarmGoldResultRequest {}

impl FarmGoldResultRequest {
    pub const MSG_ID: u16 = 20486;
}

impl PbMessage for FarmGoldResultRequest {
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
pub struct FarmGoldResultResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FarmGoldResultResponse {
    pub const MSG_ID: u16 = 24579;
}

impl PbMessage for FarmGoldResultResponse {
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
pub struct FastReturnDungeonRequest {
    pub ContinueInst: bool,
    pub NodeId: i32,
    pub RoleIds: Vec<i32>,
    pub TreeIncId: i64,
    pub TreeOwnerId: i32,
}

impl FastReturnDungeonRequest {
    pub const MSG_ID: u16 = 26812;
}

impl PbMessage for FastReturnDungeonRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ContinueInst {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TreeIncId);
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
            6 => self.ContinueInst = r.read_bool()?,
            10 => self.NodeId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => self.TreeIncId = r.read_varint_i64()?,
            15 => self.TreeOwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FastReturnDungeonResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FastReturnDungeonResponse {
    pub const MSG_ID: u16 = 24553;
}

impl PbMessage for FastReturnDungeonResponse {
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
pub struct FeiXuePreheatInfoRewardRequest {
    pub ActivityId: i32,
    pub FeiXuePreheatInfoIds: Vec<i32>,
}

impl FeiXuePreheatInfoRewardRequest {
    pub const MSG_ID: u16 = 17951;
}

impl PbMessage for FeiXuePreheatInfoRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.FeiXuePreheatInfoIds {
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
            4 => self.ActivityId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FeiXuePreheatInfoIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeiXuePreheatInfoRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FeiXuePreheatInfoRewardResponse {
    pub const MSG_ID: u16 = 26680;
}

impl PbMessage for FeiXuePreheatInfoRewardResponse {
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
pub struct FeiXuePreheatInfoUpdateNotify {
    pub FeiXuePreheatInfos: Vec<FeiXuePreheatInfo>,
}

impl FeiXuePreheatInfoUpdateNotify {
    pub const MSG_ID: u16 = 21876;
}

impl PbMessage for FeiXuePreheatInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FeiXuePreheatInfos {
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
                self.FeiXuePreheatInfos
                    .push(<FeiXuePreheatInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightDataCheckNotify {
    pub ChangeList: i32,
    pub Result: bool,
}

impl FightDataCheckNotify {
    pub const MSG_ID: u16 = 22139;
}

impl PbMessage for FightDataCheckNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChangeList != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeList);
        }
        if self.Result {
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
            5 => self.ChangeList = r.read_varint_i32()?,
            7 => self.Result = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FightRoleInformation {
    pub RoleId: i32,
    pub EntityId: i64,
    pub IsControl: bool,
}

impl PbMessage for FightRoleInformation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsControl {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.EntityId = r.read_varint_i64()?,
            5 => self.IsControl = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FireShopCoinNotify {
    pub MoneyCount: i64,
    pub MoneyHistorySpent: i64,
}

impl FireShopCoinNotify {
    pub const MSG_ID: u16 = 29285;
}

impl PbMessage for FireShopCoinNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MoneyCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MoneyCount);
        }
        if self.MoneyHistorySpent != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MoneyHistorySpent);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.MoneyCount = r.read_varint_i64()?,
            10 => self.MoneyHistorySpent = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FirecrackerEntityCreatedNotify {
    pub EntityId: i64,
}

impl FirecrackerEntityCreatedNotify {
    pub const MSG_ID: u16 = 27748;
}

impl PbMessage for FirecrackerEntityCreatedNotify {
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
pub struct FishingActivityInfo {
    pub LimitActivityTasks: Vec<ActivityTask>,
    pub MilestoneRewarded: BTreeMap<i32, SignState>,
    pub LimitedStartTime: i64,
    pub LimitedEndTime: i64,
    pub MilestoneItemNum: i32,
}

impl PbMessage for FishingActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LimitActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        if !self.MilestoneRewarded.is_empty() {
            crate::wire::write_map(out, 2, &self.MilestoneRewarded);
        }
        if self.LimitedStartTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedStartTime);
        }
        if self.LimitedEndTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedEndTime);
        }
        if self.MilestoneItemNum != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MilestoneItemNum);
        }
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
                self.LimitActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_map_entry(r, &mut self.MilestoneRewarded)?,
            3 => self.LimitedStartTime = r.read_varint_i64()?,
            4 => self.LimitedEndTime = r.read_varint_i64()?,
            5 => self.MilestoneItemNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityLimitRewardRequest {
    pub TaskId: i32,
}

impl FishingActivityLimitRewardRequest {
    pub const MSG_ID: u16 = 26494;
}

impl PbMessage for FishingActivityLimitRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityLimitRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingActivityLimitRewardResponse {
    pub const MSG_ID: u16 = 23099;
}

impl PbMessage for FishingActivityLimitRewardResponse {
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
pub struct FishingActivityLimitTaskNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl FishingActivityLimitTaskNotify {
    pub const MSG_ID: u16 = 28644;
}

impl PbMessage for FishingActivityLimitTaskNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityMileStoneNotify {
    pub MilestoneRewarded: BTreeMap<i32, SignState>,
}

impl FishingActivityMileStoneNotify {
    pub const MSG_ID: u16 = 19138;
}

impl PbMessage for FishingActivityMileStoneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.MilestoneRewarded.is_empty() {
            crate::wire::write_map(out, 15, &self.MilestoneRewarded);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => crate::wire::decode_map_entry(r, &mut self.MilestoneRewarded)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityMilestoneItemNumNotify {
    pub MilestoneItemNum: i32,
}

impl FishingActivityMilestoneItemNumNotify {
    pub const MSG_ID: u16 = 24887;
}

impl PbMessage for FishingActivityMilestoneItemNumNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MilestoneItemNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MilestoneItemNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.MilestoneItemNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityMilestoneRewardRequest {
    pub ActivityId: i32,
    pub MilestoneId: Vec<i32>,
}

impl FishingActivityMilestoneRewardRequest {
    pub const MSG_ID: u16 = 25523;
}

impl PbMessage for FishingActivityMilestoneRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.MilestoneId {
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
            4 => self.ActivityId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MilestoneId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingActivityMilestoneRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingActivityMilestoneRewardResponse {
    pub const MSG_ID: u16 = 21498;
}

impl PbMessage for FishingActivityMilestoneRewardResponse {
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
pub struct FishingBoatDieNotify {
    pub EntityId: i64,
    pub Rot: Option<Rotator>,
    pub Pos: Option<Vector>,
}

impl FishingBoatDieNotify {
    pub const MSG_ID: u16 = 19589;
}

impl PbMessage for FishingBoatDieNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Pos {
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
            2 => self.EntityId = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
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
pub struct FishingCageInfoRequest {
    pub Id: i32,
}

impl FishingCageInfoRequest {
    pub const MSG_ID: u16 = 26932;
}

impl PbMessage for FishingCageInfoRequest {
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
pub struct FishingCageInfoResponse {
    pub FishCageInfo: Option<OneFishCage>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingCageInfoResponse {
    pub const MSG_ID: u16 = 16693;
}

impl PbMessage for FishingCageInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishCageInfo {
            crate::wire::write_submessage(out, 2, v);
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
                self.FishCageInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
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
pub struct FishingCompleteRequest {
    pub EntityId: i64,
    pub InteractId: i32,
}

impl FishingCompleteRequest {
    pub const MSG_ID: u16 = 21375;
}

impl PbMessage for FishingCompleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InteractId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InteractId);
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
            11 => self.InteractId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingCompleteResponse {
    pub ExtraItems: Vec<FishingItemInfo>,
    pub Items: Vec<FishingItemInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub DoubleItems: Vec<FishingItemInfo>,
}

impl FishingCompleteResponse {
    pub const MSG_ID: u16 = 23801;
}

impl PbMessage for FishingCompleteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ExtraItems {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.Items {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.DoubleItems {
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
                self.ExtraItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.DoubleItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustAcceptRequest {
    pub Accept: bool,
    pub EntrustId: i32,
}

impl FishingEntrustAcceptRequest {
    pub const MSG_ID: u16 = 28651;
}

impl PbMessage for FishingEntrustAcceptRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Accept {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            10 => self.Accept = r.read_bool()?,
            12 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustAcceptResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingEntrustAcceptResponse {
    pub const MSG_ID: u16 = 18952;
}

impl PbMessage for FishingEntrustAcceptResponse {
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
pub struct FishingEntrustHandInRequest {
    pub HandInItem: Vec<FishingHandInItem>,
    pub EntrustId: i32,
}

impl FishingEntrustHandInRequest {
    pub const MSG_ID: u16 = 19706;
}

impl PbMessage for FishingEntrustHandInRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HandInItem {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.HandInItem
                    .push(<FishingHandInItem as PbMessage>::decode(bytes)?);
            }
            10 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustHandInResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingEntrustHandInResponse {
    pub const MSG_ID: u16 = 28305;
}

impl PbMessage for FishingEntrustHandInResponse {
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
pub struct FishingEntrustRefreshRequest {
    pub EntrustId: i32,
}

impl FishingEntrustRefreshRequest {
    pub const MSG_ID: u16 = 22223;
}

impl PbMessage for FishingEntrustRefreshRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustRefreshResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EntrustId: i32,
}

impl FishingEntrustRefreshResponse {
    pub const MSG_ID: u16 = 16187;
}

impl PbMessage for FishingEntrustRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustTraceRequest {
    pub EntrustId: i32,
}

impl FishingEntrustTraceRequest {
    pub const MSG_ID: u16 = 21717;
}

impl PbMessage for FishingEntrustTraceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.EntrustId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingEntrustTraceResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingEntrustTraceResponse {
    pub const MSG_ID: u16 = 29628;
}

impl PbMessage for FishingEntrustTraceResponse {
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
pub struct FishingEntrustUpdateNotify {
    pub Entrusts: BTreeMap<i32, FishingEntrustStatus>,
    pub EntrustRefreshRatio: i32,
    pub TraceEntrusts: i32,
}

impl FishingEntrustUpdateNotify {
    pub const MSG_ID: u16 = 28710;
}

impl PbMessage for FishingEntrustUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Entrusts.is_empty() {
            crate::wire::write_map(out, 1, &self.Entrusts);
        }
        if self.EntrustRefreshRatio != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustRefreshRatio);
        }
        if self.TraceEntrusts != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TraceEntrusts);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.Entrusts)?,
            2 => self.EntrustRefreshRatio = r.read_varint_i32()?,
            5 => self.TraceEntrusts = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingExitRequest {}

impl FishingExitRequest {
    pub const MSG_ID: u16 = 16269;
}

impl PbMessage for FishingExitRequest {
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
pub struct FishingExitResponse {}

impl FishingExitResponse {
    pub const MSG_ID: u16 = 15747;
}

impl PbMessage for FishingExitResponse {
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
pub struct FishingGhostStateNotify {
    pub Open: bool,
}

impl FishingGhostStateNotify {
    pub const MSG_ID: u16 = 15355;
}

impl PbMessage for FishingGhostStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Open {
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
            15 => self.Open = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingHandInData {
    pub HandInId: i32,
}

impl PbMessage for FishingHandInData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandInId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandInId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HandInId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingHandInItem {
    pub ItemId: i32,
    pub IncrId: i32,
    pub Count: i32,
}

impl PbMessage for FishingHandInItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncrId);
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
            2 => self.IncrId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingHandInRequest {
    pub ActionIncrId: i32,
    pub RightFishingItems: Vec<FishingItemInfo>,
    pub RemoveIncrId: i32,
    pub LeftFishingItems: Vec<FishingItemInfo>,
    pub Id: i32,
}

impl FishingHandInRequest {
    pub const MSG_ID: u16 = 17299;
}

impl PbMessage for FishingHandInRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActionIncrId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionIncrId);
        }
        for item in &self.RightFishingItems {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.RemoveIncrId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemoveIncrId);
        }
        for item in &self.LeftFishingItems {
            crate::wire::write_submessage(out, 14, item);
        }
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
            2 => self.ActionIncrId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RightFishingItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            7 => self.RemoveIncrId = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.LeftFishingItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingHandInResponse {
    pub HandInComplete: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingHandInResponse {
    pub const MSG_ID: u16 = 18677;
}

impl PbMessage for FishingHandInResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HandInComplete {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            13 => self.HandInComplete = r.read_bool()?,
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
pub struct FishingIllustratedInfoUpdateNotify {
    pub UnlockDetections: Vec<i32>,
    pub IllustratedInfo: Option<OneFishingIllustratedInfo>,
}

impl FishingIllustratedInfoUpdateNotify {
    pub const MSG_ID: u16 = 28244;
}

impl PbMessage for FishingIllustratedInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockDetections {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.IllustratedInfo {
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
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockDetections,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.IllustratedInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingIllustratedRewardInfoUpdateNotify {
    pub RewardInfo: Option<FishingIllustratedRewardInfo>,
}

impl FishingIllustratedRewardInfoUpdateNotify {
    pub const MSG_ID: u16 = 26235;
}

impl PbMessage for FishingIllustratedRewardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RewardInfo {
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
pub struct FishingIllustratedRewardRequest {
    pub RewardId: i32,
}

impl FishingIllustratedRewardRequest {
    pub const MSG_ID: u16 = 26334;
}

impl PbMessage for FishingIllustratedRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.RewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingIllustratedRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingIllustratedRewardResponse {
    pub const MSG_ID: u16 = 27309;
}

impl PbMessage for FishingIllustratedRewardResponse {
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
pub struct FishingInfo {
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
    pub UnlockSkins: Vec<i32>,
    pub EntrustRefreshRatio: i32,
}

impl PbMessage for FishingInfo {
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
        for item in &self.UnlockSkins {
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
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockSkins, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            14 => self.EntrustRefreshRatio = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingInfoRequest {}

impl FishingInfoRequest {
    pub const MSG_ID: u16 = 18438;
}

impl PbMessage for FishingInfoRequest {
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
pub struct FishingInfoResponse {
    pub FishingInfo: Option<FishingInfo>,
}

impl FishingInfoResponse {
    pub const MSG_ID: u16 = 22673;
}

impl PbMessage for FishingInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishingInfo {
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
                self.FishingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingInfoUpdateNotify {
    pub FishingInfo: Option<FishingInfo>,
}

impl FishingInfoUpdateNotify {
    pub const MSG_ID: u16 = 19875;
}

impl PbMessage for FishingInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishingInfo {
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
                self.FishingInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingItemAddNotify {
    pub NetCabinItem: Vec<FishingItemInfo>,
    pub TempCabinItem: Vec<FishingItemInfo>,
}

impl FishingItemAddNotify {
    pub const MSG_ID: u16 = 22817;
}

impl PbMessage for FishingItemAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NetCabinItem {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.TempCabinItem {
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
                self.NetCabinItem
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.TempCabinItem
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
