use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryResidentTaskRewardRequest {
    pub ActivityId: i32,
    pub ResidentTaskIds: Vec<i32>,
}

impl HonamiStoryResidentTaskRewardRequest {
    pub const MSG_ID: u16 = 27298;
}

impl PbMessage for HonamiStoryResidentTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.ResidentTaskIds {
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
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ResidentTaskIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryResidentTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryResidentTaskRewardResponse {
    pub const MSG_ID: u16 = 15037;
}

impl PbMessage for HonamiStoryResidentTaskRewardResponse {
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
pub struct HonamiStoryRoleUpdateNotify {
    pub RoleId: Vec<i32>,
    pub ActivityId: i32,
}

impl HonamiStoryRoleUpdateNotify {
    pub const MSG_ID: u16 = 17089;
}

impl PbMessage for HonamiStoryRoleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStorySafeLeaveNotify {
    pub CanSafeLeave: bool,
    pub ActivityId: i32,
}

impl HonamiStorySafeLeaveNotify {
    pub const MSG_ID: u16 = 17990;
}

impl PbMessage for HonamiStorySafeLeaveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanSafeLeave {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            1 => self.CanSafeLeave = r.read_bool()?,
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryScoreRewardInfoUpdateNotify {
    pub HonamiStoryScoreRewardInfos: Vec<HonamiStoryScoreRewardInfo>,
}

impl HonamiStoryScoreRewardInfoUpdateNotify {
    pub const MSG_ID: u16 = 18607;
}

impl PbMessage for HonamiStoryScoreRewardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryScoreRewardInfos {
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
                self.HonamiStoryScoreRewardInfos
                    .push(<HonamiStoryScoreRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryScoreRewardRequest {
    pub ScoreRewardId: Vec<i32>,
    pub ActivityId: i32,
}

impl HonamiStoryScoreRewardRequest {
    pub const MSG_ID: u16 = 16084;
}

impl PbMessage for HonamiStoryScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreRewardId {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreRewardId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryScoreRewardResponse {
    pub ScoreRewardId: Vec<i32>,
    pub ActivityId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryScoreRewardResponse {
    pub const MSG_ID: u16 = 27500;
}

impl PbMessage for HonamiStoryScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreRewardId {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreRewardId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => self.ActivityId = r.read_varint_i32()?,
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
pub struct HonamiStorySellItemRequest {
    pub HonamiStoryItemUidInfos: BTreeMap<i32, HonamiStoryItemUidInfo>,
    pub ActivityId: i32,
}

impl HonamiStorySellItemRequest {
    pub const MSG_ID: u16 = 16688;
}

impl PbMessage for HonamiStorySellItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.HonamiStoryItemUidInfos.is_empty() {
            crate::wire::write_map(out, 9, &self.HonamiStoryItemUidInfos);
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
            9 => crate::wire::decode_map_entry(r, &mut self.HonamiStoryItemUidInfos)?,
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStorySellItemResponse {
    pub ActivityId: i32,
    pub HonamiStoryItemUidInfos: BTreeMap<i32, HonamiStoryItemUidInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStorySellItemResponse {
    pub const MSG_ID: u16 = 15708;
}

impl PbMessage for HonamiStorySellItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if !self.HonamiStoryItemUidInfos.is_empty() {
            crate::wire::write_map(out, 11, &self.HonamiStoryItemUidInfos);
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
            2 => self.ActivityId = r.read_varint_i32()?,
            11 => crate::wire::decode_map_entry(r, &mut self.HonamiStoryItemUidInfos)?,
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
pub struct HonamiStorySetFormationNotify {
    pub EquipRack: Option<HonamiStoryBagInfo>,
    pub HonamiStoryRackInfos: Vec<HonamiStoryRackInfo>,
}

impl HonamiStorySetFormationNotify {
    pub const MSG_ID: u16 = 18653;
}

impl PbMessage for HonamiStorySetFormationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EquipRack {
            crate::wire::write_submessage(out, 11, v);
        }
        for item in &self.HonamiStoryRackInfos {
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
            11 => {
                let bytes = r.read_len_delimited()?;
                self.EquipRack
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryRackInfos
                    .push(<HonamiStoryRackInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStorySlotInfo {
    pub SlotId: i32,
    pub IsUnlock: bool,
}

impl PbMessage for HonamiStorySlotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.IsUnlock {
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
            1 => self.SlotId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryTalentInfo {
    pub TalentId: i32,
    pub Status: i32,
}

impl PbMessage for HonamiStoryTalentInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentId);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.TalentId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryTalentInfoUpdateNotify {
    pub HonamiStoryTalentInfos: Vec<HonamiStoryTalentInfo>,
}

impl HonamiStoryTalentInfoUpdateNotify {
    pub const MSG_ID: u16 = 19936;
}

impl PbMessage for HonamiStoryTalentInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryTalentInfos {
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
                self.HonamiStoryTalentInfos
                    .push(<HonamiStoryTalentInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryTopInfo {
    pub DangerLevel: i32,
    pub MaxFloor: i32,
}

impl PbMessage for HonamiStoryTopInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangerLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangerLevel);
        }
        if self.MaxFloor != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFloor);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.DangerLevel = r.read_varint_i32()?,
            2 => self.MaxFloor = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryTopInfoNotify {
    pub HonamiStoryTopInfos: Vec<HonamiStoryTopInfo>,
    pub ActivityId: i32,
}

impl HonamiStoryTopInfoNotify {
    pub const MSG_ID: u16 = 26555;
}

impl PbMessage for HonamiStoryTopInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryTopInfos {
            crate::wire::write_submessage(out, 12, item);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryTopInfos
                    .push(<HonamiStoryTopInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryTotalRevenueNotify {
    pub TotalRevenue: i32,
}

impl HonamiStoryTotalRevenueNotify {
    pub const MSG_ID: u16 = 23885;
}

impl PbMessage for HonamiStoryTotalRevenueNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalRevenue != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalRevenue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.TotalRevenue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryUnlockSlotRequest {
    pub RackIndex: i32,
    pub SlotId: i32,
}

impl HonamiStoryUnlockSlotRequest {
    pub const MSG_ID: u16 = 26265;
}

impl PbMessage for HonamiStoryUnlockSlotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RackIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RackIndex);
        }
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            4 => self.RackIndex = r.read_varint_i32()?,
            14 => self.SlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryUnlockSlotResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RackIndex: i32,
    pub SlotId: i32,
}

impl HonamiStoryUnlockSlotResponse {
    pub const MSG_ID: u16 = 16925;
}

impl PbMessage for HonamiStoryUnlockSlotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RackIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RackIndex);
        }
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.RackIndex = r.read_varint_i32()?,
            13 => self.SlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryWeaponDressRequest {
    pub WeaponItemId: i32,
    pub RackIndex: i32,
    pub ActivityId: i32,
}

impl HonamiStoryWeaponDressRequest {
    pub const MSG_ID: u16 = 17346;
}

impl PbMessage for HonamiStoryWeaponDressRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponItemId);
        }
        if self.RackIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RackIndex);
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
            1 => self.WeaponItemId = r.read_varint_i32()?,
            10 => self.RackIndex = r.read_varint_i32()?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryWeaponDressResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RackIndex: i32,
    pub WeaponItemId: i32,
}

impl HonamiStoryWeaponDressResponse {
    pub const MSG_ID: u16 = 26893;
}

impl PbMessage for HonamiStoryWeaponDressResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RackIndex != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RackIndex);
        }
        if self.WeaponItemId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponItemId);
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
            11 => self.RackIndex = r.read_varint_i32()?,
            15 => self.WeaponItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockBatchCollectNotify {
    pub SuccessConfigIds: Vec<i32>,
    pub PlayerId: i32,
    pub EntityConfigId: i32,
}

impl HookLockBatchCollectNotify {
    pub const MSG_ID: u16 = 26841;
}

impl PbMessage for HookLockBatchCollectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SuccessConfigIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
            11 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SuccessConfigIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            13 => self.PlayerId = r.read_varint_i32()?,
            14 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockBatchCollectRequest {
    pub EntityId: i64,
    pub InstId: i32,
    pub BatchIds: Vec<i64>,
}

impl HookLockBatchCollectRequest {
    pub const MSG_ID: u16 = 26268;
}

impl PbMessage for HookLockBatchCollectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.BatchIds {
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
            1 => self.EntityId = r.read_varint_i64()?,
            4 => self.InstId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BatchIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockBatchCollectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub EntityConfigId: i32,
    pub SuccessConfigIds: Vec<i32>,
}

impl HookLockBatchCollectResponse {
    pub const MSG_ID: u16 = 26873;
}

impl PbMessage for HookLockBatchCollectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        for item in &self.SuccessConfigIds {
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
            6 => self.EntityConfigId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SuccessConfigIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockPointComponentPb {
    pub IsDisabled: bool,
}

impl PbMessage for HookLockPointComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsDisabled {
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
            1 => self.IsDisabled = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockPointExitRequest {
    pub EntityId: i64,
    pub HookLockPointExitWay: Option<HookLockPointExitWay>,
}

impl HookLockPointExitRequest {
    pub const MSG_ID: u16 = 15425;
}

impl PbMessage for HookLockPointExitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if let Some(v) = &self.HookLockPointExitWay {
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
            3 => self.EntityId = r.read_varint_i64()?,
            4 => {
                self.HookLockPointExitWay = Some(
                    <HookLockPointExitWay as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct HookLockPointExitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HookLockPointExitResponse {
    pub const MSG_ID: u16 = 28458;
}

impl PbMessage for HookLockPointExitResponse {
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
pub struct HookLockPointFinishRequest {
    pub EntityId: i64,
}

impl HookLockPointFinishRequest {
    pub const MSG_ID: u16 = 15520;
}

impl PbMessage for HookLockPointFinishRequest {
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
pub struct HookLockPointFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HookLockPointFinishResponse {
    pub const MSG_ID: u16 = 27227;
}

impl PbMessage for HookLockPointFinishResponse {
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
pub struct HookLockPointInfoNotify {
    pub IsDisabled: bool,
    pub HookLockPointEntityId: i64,
}

impl HookLockPointInfoNotify {
    pub const MSG_ID: u16 = 21402;
}

impl PbMessage for HookLockPointInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsDisabled {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.HookLockPointEntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HookLockPointEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.IsDisabled = r.read_bool()?,
            11 => self.HookLockPointEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HookLockPointRequest {
    pub EntityId: i64,
}

impl HookLockPointRequest {
    pub const MSG_ID: u16 = 16301;
}

impl PbMessage for HookLockPointRequest {
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
pub struct HookLockPointResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HookLockPointResponse {
    pub const MSG_ID: u16 = 24512;
}

impl PbMessage for HookLockPointResponse {
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
pub struct HostFogIdUnlockNotify {
    pub HostPlayerId: i32,
    pub HostFogId: i32,
}

impl HostFogIdUnlockNotify {
    pub const MSG_ID: u16 = 21721;
}

impl PbMessage for HostFogIdUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if self.HostFogId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostFogId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.HostPlayerId = r.read_varint_i32()?,
            7 => self.HostFogId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IllustratedUnlockNotify {
    pub IsNew: bool,
    pub Entry: Option<IllustratedEntry>,
    pub Type: Option<IllustratedType>,
}

impl IllustratedUnlockNotify {
    pub const MSG_ID: u16 = 29822;
}

impl PbMessage for IllustratedUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNew {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Entry {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Type {
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
            1 => self.IsNew = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Entry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
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
pub struct InfluenceInfoUpdateNotify {
    pub InfluenceInfos: Vec<OneInfluenceInfo>,
}

impl InfluenceInfoUpdateNotify {
    pub const MSG_ID: u16 = 26930;
}

impl PbMessage for InfluenceInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InfluenceInfos {
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
                self.InfluenceInfos
                    .push(<OneInfluenceInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfluenceRewardRequest {
    pub InfluenceId: i32,
}

impl InfluenceRewardRequest {
    pub const MSG_ID: u16 = 19523;
}

impl PbMessage for InfluenceRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InfluenceId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InfluenceId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.InfluenceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfluenceRewardResponse {
    pub RewardIndex: i32,
    pub RewardItems: BTreeMap<i32, i32>,
    pub InfluenceId: i32,
    pub Code: Option<ErrorCode>,
}

impl InfluenceRewardResponse {
    pub const MSG_ID: u16 = 21130;
}

impl PbMessage for InfluenceRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 5, &self.RewardItems);
        }
        if self.InfluenceId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InfluenceId);
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
            2 => self.RewardIndex = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            7 => self.InfluenceId = r.read_varint_i32()?,
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
pub struct InfrArchiveReadRequest {
    pub ArchiveId: Vec<i32>,
}

impl InfrArchiveReadRequest {
    pub const MSG_ID: u16 = 26051;
}

impl PbMessage for InfrArchiveReadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ArchiveId {
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
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ArchiveId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrArchiveReadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrArchiveReadResponse {
    pub const MSG_ID: u16 = 19607;
}

impl PbMessage for InfrArchiveReadResponse {
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
pub struct InfrArchiveTaskRewardRequest {
    pub TargetIds: Vec<i32>,
}

impl InfrArchiveTaskRewardRequest {
    pub const MSG_ID: u16 = 18945;
}

impl PbMessage for InfrArchiveTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TargetIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrArchiveTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrArchiveTaskRewardResponse {
    pub const MSG_ID: u16 = 28194;
}

impl PbMessage for InfrArchiveTaskRewardResponse {
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
pub struct InfrFireAddNotify {
    pub Add: i32,
    pub FireInfo: Option<InfrFirePb>,
}

impl InfrFireAddNotify {
    pub const MSG_ID: u16 = 18395;
}

impl PbMessage for InfrFireAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Add != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Add);
        }
        if let Some(v) = &self.FireInfo {
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
            12 => self.Add = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.FireInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrFireNoticeRequest {}

impl InfrFireNoticeRequest {
    pub const MSG_ID: u16 = 29298;
}

impl PbMessage for InfrFireNoticeRequest {
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
pub struct InfrFireNoticeResponse {
    pub Notices: Vec<InfrNotice>,
}

impl InfrFireNoticeResponse {
    pub const MSG_ID: u16 = 27191;
}

impl PbMessage for InfrFireNoticeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Notices {
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
                self.Notices.push(<InfrNotice as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrFireUpdateNotify {
    pub FireInfo: Option<InfrFirePb>,
}

impl InfrFireUpdateNotify {
    pub const MSG_ID: u16 = 18262;
}

impl PbMessage for InfrFireUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FireInfo {
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
                self.FireInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrHandInData {
    pub Type: Option<ActionInfrastructureItemDeliveryType>,
    pub Id: i32,
}

impl PbMessage for InfrHandInData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => self.Type = Some(
                <ActionInfrastructureItemDeliveryType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ),
            ),
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrLevelUpRequest {}

impl InfrLevelUpRequest {
    pub const MSG_ID: u16 = 18803;
}

impl PbMessage for InfrLevelUpRequest {
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
pub struct InfrLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrLevelUpResponse {
    pub const MSG_ID: u16 = 17552;
}

impl PbMessage for InfrLevelUpResponse {
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
pub struct InfrLibraryTaskUpdateNotify {
    pub ArchiveTasks: Vec<InfrTaskPb>,
    pub PhoneTasks: Vec<InfrTaskPb>,
}

impl InfrLibraryTaskUpdateNotify {
    pub const MSG_ID: u16 = 15668;
}

impl PbMessage for InfrLibraryTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ArchiveTasks {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.PhoneTasks {
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ArchiveTasks
                    .push(<InfrTaskPb as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhoneTasks
                    .push(<InfrTaskPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrManualCancelTraceRoadRequest {}

impl InfrManualCancelTraceRoadRequest {
    pub const MSG_ID: u16 = 17676;
}

impl PbMessage for InfrManualCancelTraceRoadRequest {
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
pub struct InfrManualCancelTraceRoadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrManualCancelTraceRoadResponse {
    pub const MSG_ID: u16 = 25842;
}

impl PbMessage for InfrManualCancelTraceRoadResponse {
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
pub struct InfrManualSwitchTraceRoadRequest {
    pub RoadId: i32,
}

impl InfrManualSwitchTraceRoadRequest {
    pub const MSG_ID: u16 = 20148;
}

impl PbMessage for InfrManualSwitchTraceRoadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoadId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoadId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.RoadId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrManualSwitchTraceRoadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrManualSwitchTraceRoadResponse {
    pub const MSG_ID: u16 = 19643;
}

impl PbMessage for InfrManualSwitchTraceRoadResponse {
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
pub struct InfrPhoneTaskRewardRequest {
    pub TargetIds: Vec<i32>,
}

impl InfrPhoneTaskRewardRequest {
    pub const MSG_ID: u16 = 16037;
}

impl PbMessage for InfrPhoneTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TargetIds {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrPhoneTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrPhoneTaskRewardResponse {
    pub const MSG_ID: u16 = 18459;
}

impl PbMessage for InfrPhoneTaskRewardResponse {
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
pub struct InfrRoadBuildRequest {
    pub RoadId: i32,
}

impl InfrRoadBuildRequest {
    pub const MSG_ID: u16 = 20207;
}

impl PbMessage for InfrRoadBuildRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoadId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoadId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.RoadId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrRoadBuildResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrRoadBuildResponse {
    pub const MSG_ID: u16 = 22175;
}

impl PbMessage for InfrRoadBuildResponse {
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
pub struct InfrThemeActivityAwardRequest {
    pub TaskId: Vec<i32>,
}

impl InfrThemeActivityAwardRequest {
    pub const MSG_ID: u16 = 20474;
}

impl PbMessage for InfrThemeActivityAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrThemeActivityAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrThemeActivityAwardResponse {
    pub const MSG_ID: u16 = 21087;
}

impl PbMessage for InfrThemeActivityAwardResponse {
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
pub struct InfrThemeActivityTaskDataUpdateNotify {
    pub ActivityTask: Option<ActivityTask>,
}

impl InfrThemeActivityTaskDataUpdateNotify {
    pub const MSG_ID: u16 = 20438;
}

impl PbMessage for InfrThemeActivityTaskDataUpdateNotify {
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
pub struct InfrV2FireUpdateNotify {
    pub FireInfo: Option<InfrV2FirePb>,
}

impl InfrV2FireUpdateNotify {
    pub const MSG_ID: u16 = 15857;
}

impl PbMessage for InfrV2FireUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FireInfo {
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
                self.FireInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2HandInData {
    pub Type: Option<ActionInfrastructureV2ItemDeliveryType>,
    pub Id: i32,
}

impl PbMessage for InfrV2HandInData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => self.Type = Some(<ActionInfrastructureV2ItemDeliveryType as crate::wire::PbEnum>::from_i32_or_default(r.read_varint_i32()?)),
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2InfoNotify {
    pub InfrInfo: Option<InfrV2Pb>,
}

impl InfrV2InfoNotify {
    pub const MSG_ID: u16 = 17182;
}

impl PbMessage for InfrV2InfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InfrInfo {
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
                self.InfrInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2LevelUpRequest {}

impl InfrV2LevelUpRequest {
    pub const MSG_ID: u16 = 26367;
}

impl PbMessage for InfrV2LevelUpRequest {
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
pub struct InfrV2LevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrV2LevelUpResponse {
    pub const MSG_ID: u16 = 29561;
}

impl PbMessage for InfrV2LevelUpResponse {
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
pub struct InfrV2ManualCancelTraceTreeRequest {}

impl InfrV2ManualCancelTraceTreeRequest {
    pub const MSG_ID: u16 = 25977;
}

impl PbMessage for InfrV2ManualCancelTraceTreeRequest {
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
pub struct InfrV2ManualCancelTraceTreeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrV2ManualCancelTraceTreeResponse {
    pub const MSG_ID: u16 = 16976;
}

impl PbMessage for InfrV2ManualCancelTraceTreeResponse {
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
pub struct InfrV2ManualSwitchTraceTreeRequest {
    pub TreeId: i32,
}

impl InfrV2ManualSwitchTraceTreeRequest {
    pub const MSG_ID: u16 = 24316;
}

impl PbMessage for InfrV2ManualSwitchTraceTreeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.TreeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2ManualSwitchTraceTreeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrV2ManualSwitchTraceTreeResponse {
    pub const MSG_ID: u16 = 26046;
}

impl PbMessage for InfrV2ManualSwitchTraceTreeResponse {
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
pub struct InfrV2ScoreRewardRequest {
    pub ScoreIds: Vec<i32>,
}

impl InfrV2ScoreRewardRequest {
    pub const MSG_ID: u16 = 29142;
}

impl PbMessage for InfrV2ScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreIds {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2ScoreRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl InfrV2ScoreRewardResponse {
    pub const MSG_ID: u16 = 27040;
}

impl PbMessage for InfrV2ScoreRewardResponse {
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
pub struct InfrV2TaskDataUpdateNotify {
    pub Task: Option<ConditionTask>,
}

impl InfrV2TaskDataUpdateNotify {
    pub const MSG_ID: u16 = 20489;
}

impl PbMessage for InfrV2TaskDataUpdateNotify {
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
pub struct InfrV2TaskRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl InfrV2TaskRewardRequest {
    pub const MSG_ID: u16 = 17782;
}

impl PbMessage for InfrV2TaskRewardRequest {
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
pub struct InfrV2TaskRewardResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl InfrV2TaskRewardResponse {
    pub const MSG_ID: u16 = 16735;
}

impl PbMessage for InfrV2TaskRewardResponse {
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
pub struct InfrV2TreeBuildRequest {
    pub Id: i32,
}

impl InfrV2TreeBuildRequest {
    pub const MSG_ID: u16 = 17207;
}

impl PbMessage for InfrV2TreeBuildRequest {
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
pub struct InfrV2TreeBuildResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InfrV2TreeBuildResponse {
    pub const MSG_ID: u16 = 28858;
}

impl PbMessage for InfrV2TreeBuildResponse {
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
pub struct InfrV2TreeFinishCondUpdateNotify {
    pub Cond: Vec<i32>,
}

impl InfrV2TreeFinishCondUpdateNotify {
    pub const MSG_ID: u16 = 21763;
}

impl PbMessage for InfrV2TreeFinishCondUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Cond {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Cond, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InfrV2TreeUpdateNotify {
    pub TreeInfo: Option<InfrV2TreePb>,
}

impl InfrV2TreeUpdateNotify {
    pub const MSG_ID: u16 = 19674;
}

impl PbMessage for InfrV2TreeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TreeInfo {
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
                self.TreeInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputSettingUpdateNotify {
    pub InputSettingData: Option<InputSettingData>,
}

impl InputSettingUpdateNotify {
    pub const MSG_ID: u16 = 25153;
}

impl PbMessage for InputSettingUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InputSettingData {
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
                self.InputSettingData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InspirationUnlockChoiceRequest {
    pub ChoiceId: i32,
    pub ChapterId: i32,
    pub ActivityId: i32,
}

impl InspirationUnlockChoiceRequest {
    pub const MSG_ID: u16 = 19608;
}

impl PbMessage for InspirationUnlockChoiceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChoiceId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChoiceId);
        }
        if self.ChapterId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChapterId);
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
            2 => self.ChoiceId = r.read_varint_i32()?,
            4 => self.ChapterId = r.read_varint_i32()?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InspirationUnlockChoiceResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InspirationUnlockChoiceResponse {
    pub const MSG_ID: u16 = 23610;
}

impl PbMessage for InspirationUnlockChoiceResponse {
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
pub struct InspirationUpdateNotify {
    pub InspirationData: Option<CiacconaInspirationPbData>,
    pub ActivityId: i32,
}

impl InspirationUpdateNotify {
    pub const MSG_ID: u16 = 20356;
}

impl PbMessage for InspirationUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InspirationData {
            crate::wire::write_submessage(out, 4, v);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InspirationData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstDetailEntry {
    pub InstId: i32,
    pub ResetTime: i64,
    pub Count: i32,
    pub ArchiveType: Option<ArchiveType>,
}

impl PbMessage for InstDetailEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.ResetTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ResetTime);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if let Some(v) = &self.ArchiveType {
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.ResetTime = r.read_varint_i64()?,
            3 => self.Count = r.read_varint_i32()?,
            4 => {
                self.ArchiveType = Some(<ArchiveType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstEntityTeleportRequest {
    pub EntityConfigId: i32,
    pub InstId: i32,
}

impl InstEntityTeleportRequest {
    pub const MSG_ID: u16 = 20901;
}

impl PbMessage for InstEntityTeleportRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            5 => self.EntityConfigId = r.read_varint_i32()?,
            8 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstEntityTeleportResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InstEntityTeleportResponse {
    pub const MSG_ID: u16 = 25123;
}

impl PbMessage for InstEntityTeleportResponse {
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
pub struct InstEntranceDetailRequest {
    pub EntranceId: i32,
}

impl InstEntranceDetailRequest {
    pub const MSG_ID: u16 = 19282;
}

impl PbMessage for InstEntranceDetailRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntranceId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            11 => self.EntranceId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstEntranceDetailResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LimitEndTime: i64,
    pub EntranceId: i32,
    pub InstDetailEntryList: Vec<InstDetailEntry>,
}

impl InstEntranceDetailResponse {
    pub const MSG_ID: u16 = 26537;
}

impl PbMessage for InstEntranceDetailResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LimitEndTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitEndTime);
        }
        if self.EntranceId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntranceId);
        }
        for item in &self.InstDetailEntryList {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => self.LimitEndTime = r.read_varint_i64()?,
            3 => self.EntranceId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.InstDetailEntryList
                    .push(<InstDetailEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstFightEndPush {
    pub AccShieldDamage: i64,
    pub DodgeSuccTimes: i32,
    pub AccDamage: i64,
    pub DamageMax: i32,
    pub FightId: Vec<u8>,
    pub StopTimes: i32,
    pub AccSelfDamage: i64,
    pub AccSkillHeal: i64,
    pub InstId: i32,
    pub ComboMax: i32,
    pub CostTime: i32,
    pub AccDodgaTimes: i32,
    pub AccItemHeal: i64,
}

impl InstFightEndPush {
    pub const MSG_ID: u16 = 27687;
}

impl PbMessage for InstFightEndPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AccShieldDamage != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccShieldDamage);
        }
        if self.DodgeSuccTimes != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DodgeSuccTimes);
        }
        if self.AccDamage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccDamage);
        }
        if self.DamageMax != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DamageMax);
        }
        if !self.FightId.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FightId);
        }
        if self.StopTimes != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StopTimes);
        }
        if self.AccSelfDamage != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccSelfDamage);
        }
        if self.AccSkillHeal != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccSkillHeal);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.ComboMax != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ComboMax);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostTime);
        }
        if self.AccDodgaTimes != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccDodgaTimes);
        }
        if self.AccItemHeal != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccItemHeal);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AccShieldDamage = r.read_varint_i64()?,
            2 => self.DodgeSuccTimes = r.read_varint_i32()?,
            3 => self.AccDamage = r.read_varint_i64()?,
            5 => self.DamageMax = r.read_varint_i32()?,
            7 => self.FightId = r.read_len_delimited()?.to_vec(),
            8 => self.StopTimes = r.read_varint_i32()?,
            9 => self.AccSelfDamage = r.read_varint_i64()?,
            10 => self.AccSkillHeal = r.read_varint_i64()?,
            11 => self.InstId = r.read_varint_i32()?,
            12 => self.ComboMax = r.read_varint_i32()?,
            13 => self.CostTime = r.read_varint_i32()?,
            14 => self.AccDodgaTimes = r.read_varint_i32()?,
            15 => self.AccItemHeal = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstPlayDataNotify {
    pub Id: i32,
}

impl InstPlayDataNotify {
    pub const MSG_ID: u16 = 18768;
}

impl PbMessage for InstPlayDataNotify {
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
pub struct InstResultView {
    pub InstId: i32,
    pub IsSucc: bool,
    pub Score: i32,
    pub EventCount: i32,
    pub SumMoodConsume: i32,
    pub SumGoldAdd: i32,
    pub EventInfos: Vec<EventInfo>,
    pub TeamLevel: i32,
    pub Roles: Vec<RogueResGainData>,
    pub RoleBondInfos: Vec<RoleBondInfo>,
    pub ShopItemCount: i32,
    pub ShopItemMax: bool,
    pub SkillPoint: i32,
    pub SkillPointMax: bool,
    pub NewIllustrationCount: i32,
    pub IllustrationMax: bool,
    pub EndingId: i32,
    pub UseTime: i32,
    pub IsNewRecord: bool,
}

impl PbMessage for InstResultView {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.IsSucc {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.EventCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventCount);
        }
        if self.SumMoodConsume != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SumMoodConsume);
        }
        if self.SumGoldAdd != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SumGoldAdd);
        }
        for item in &self.EventInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.TeamLevel != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeamLevel);
        }
        for item in &self.Roles {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.RoleBondInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.ShopItemCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopItemCount);
        }
        if self.ShopItemMax {
            crate::wire::write_tag(out, 13, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SkillPoint != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillPoint);
        }
        if self.SkillPointMax {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NewIllustrationCount != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewIllustrationCount);
        }
        if self.IllustrationMax {
            crate::wire::write_tag(out, 17, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EndingId != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EndingId);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        if self.IsNewRecord {
            crate::wire::write_tag(out, 20, WireType::Varint);
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.IsSucc = r.read_bool()?,
            4 => self.Score = r.read_varint_i32()?,
            5 => self.EventCount = r.read_varint_i32()?,
            6 => self.SumMoodConsume = r.read_varint_i32()?,
            7 => self.SumGoldAdd = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.EventInfos
                    .push(<EventInfo as PbMessage>::decode(bytes)?);
            }
            9 => self.TeamLevel = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .push(<RogueResGainData as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.RoleBondInfos
                    .push(<RoleBondInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.ShopItemCount = r.read_varint_i32()?,
            13 => self.ShopItemMax = r.read_bool()?,
            14 => self.SkillPoint = r.read_varint_i32()?,
            15 => self.SkillPointMax = r.read_bool()?,
            16 => self.NewIllustrationCount = r.read_varint_i32()?,
            17 => self.IllustrationMax = r.read_bool()?,
            18 => self.EndingId = r.read_varint_i32()?,
            19 => self.UseTime = r.read_varint_i32()?,
            20 => self.IsNewRecord = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstSaveDataNotify {
    pub Ids: Vec<i64>,
}

impl InstSaveDataNotify {
    pub const MSG_ID: u16 = 21061;
}

impl PbMessage for InstSaveDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstTeleportToResetPointRequest {}

impl InstTeleportToResetPointRequest {
    pub const MSG_ID: u16 = 19204;
}

impl PbMessage for InstTeleportToResetPointRequest {
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
pub struct InstTeleportToResetPointResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InstTeleportToResetPointResponse {
    pub const MSG_ID: u16 = 16034;
}

impl PbMessage for InstTeleportToResetPointResponse {
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
pub struct InstTimeoutNotify {
    pub Timestamp: i64,
}

impl InstTimeoutNotify {
    pub const MSG_ID: u16 = 27732;
}

impl PbMessage for InstTimeoutNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Timestamp != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Timestamp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Timestamp = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstanceOwnerInfo {
    pub IsFinishMingSuGen: bool,
}

impl PbMessage for InstanceOwnerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFinishMingSuGen {
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
            1 => self.IsFinishMingSuGen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstanceTimer {
    pub Status: i32,
    pub RemainTick: i32,
}

impl PbMessage for InstanceTimer {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Status != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
        if self.RemainTick != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainTick);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Status = r.read_varint_i32()?,
            2 => self.RemainTick = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InstanceTimerNotify {
    pub InstanceTimers: BTreeMap<i32, InstanceTimer>,
}

impl InstanceTimerNotify {
    pub const MSG_ID: u16 = 15453;
}

impl PbMessage for InstanceTimerNotify {
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
pub struct InteractiveUpdateRequest {
    pub InteractiveId: i32,
}

impl InteractiveUpdateRequest {
    pub const MSG_ID: u16 = 23869;
}

impl PbMessage for InteractiveUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InteractiveId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InteractiveId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.InteractiveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InteractiveUpdateResponse {
    pub Code: Option<ErrorCode>,
    pub InteractiveId: i32,
}

impl InteractiveUpdateResponse {
    pub const MSG_ID: u16 = 23737;
}

impl PbMessage for InteractiveUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.InteractiveId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InteractiveId);
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
            13 => self.InteractiveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InvestRequest {
    pub EntrustId: i32,
    pub Amount: i32,
}

impl InvestRequest {
    pub const MSG_ID: u16 = 16522;
}

impl PbMessage for InvestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntrustId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntrustId);
        }
        if self.Amount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Amount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.EntrustId = r.read_varint_i32()?,
            13 => self.Amount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InvestResponse {
    pub Heat: i32,
    pub RoleProperSettle: Vec<RoleProperSettle>,
    pub EvaluationLevel: i32,
    pub BaseGold: i32,
    pub Gold: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub Success: bool,
}

impl InvestResponse {
    pub const MSG_ID: u16 = 17799;
}

impl PbMessage for InvestResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Heat != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Heat);
        }
        for item in &self.RoleProperSettle {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.EvaluationLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EvaluationLevel);
        }
        if self.BaseGold != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BaseGold);
        }
        if self.Gold != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Gold);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Success {
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
            1 => self.Heat = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleProperSettle
                    .push(<RoleProperSettle as PbMessage>::decode(bytes)?);
            }
            8 => self.EvaluationLevel = r.read_varint_i32()?,
            9 => self.BaseGold = r.read_varint_i32()?,
            10 => self.Gold = r.read_varint_i32()?,
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.Success = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InviteRechallengeNotify {
    pub InviterPlayerId: i32,
}

impl InviteRechallengeNotify {
    pub const MSG_ID: u16 = 28944;
}

impl PbMessage for InviteRechallengeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InviterPlayerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InviterPlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.InviterPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct InviteRechallengeRequest {}

impl InviteRechallengeRequest {
    pub const MSG_ID: u16 = 27109;
}

impl PbMessage for InviteRechallengeRequest {
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
pub struct InviteRechallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl InviteRechallengeResponse {
    pub const MSG_ID: u16 = 15563;
}

impl PbMessage for InviteRechallengeResponse {
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
pub struct ItemChangeSyncDataPb {
    pub Item: Option<ItemDeltaPb>,
}

impl PbMessage for ItemChangeSyncDataPb {
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
pub struct ItemDecomposePreviewRequest {
    pub ItemList: Vec<DecomposeItemInfo>,
}

impl ItemDecomposePreviewRequest {
    pub const MSG_ID: u16 = 26576;
}

impl PbMessage for ItemDecomposePreviewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
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
                self.ItemList
                    .push(<DecomposeItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDecomposePreviewResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ItemDecomposePreviewResponse {
    pub const MSG_ID: u16 = 16064;
}

impl PbMessage for ItemDecomposePreviewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 3, &self.ItemMap);
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
            3 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct ItemDecomposeRequest {
    pub ItemList: Vec<DecomposeItemInfo>,
}

impl ItemDecomposeRequest {
    pub const MSG_ID: u16 = 17570;
}

impl PbMessage for ItemDecomposeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemList {
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
                self.ItemList
                    .push(<DecomposeItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDecomposeResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ItemDecomposeResponse {
    pub const MSG_ID: u16 = 24230;
}

impl PbMessage for ItemDecomposeResponse {
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
pub struct ItemDeltaPb {
    pub ItemId: i32,
    pub ItemCost: i32,
    pub ItemAdd: i32,
}

impl PbMessage for ItemDeltaPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.ItemCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemCost);
        }
        if self.ItemAdd != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemAdd);
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
            3 => self.ItemCost = r.read_varint_i32()?,
            4 => self.ItemAdd = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDisuseRequest {
    pub Oper: i32,
    pub IncrId: i32,
}

impl ItemDisuseRequest {
    pub const MSG_ID: u16 = 20767;
}

impl PbMessage for ItemDisuseRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Oper != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Oper);
        }
        if self.IncrId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            1 => self.Oper = r.read_varint_i32()?,
            12 => self.IncrId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemDisuseResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl ItemDisuseResponse {
    pub const MSG_ID: u16 = 28866;
}

impl PbMessage for ItemDisuseResponse {
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
pub struct ItemGiftUseNotify {
    pub Infos: Vec<AddCountItemInfo>,
    pub Id: i32,
}

impl ItemGiftUseNotify {
    pub const MSG_ID: u16 = 18287;
}

impl PbMessage for ItemGiftUseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 4, item);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemGiftUseRequest {
    pub SelectedId: Vec<i32>,
    pub ItemId: i32,
    pub Count: i32,
}

impl ItemGiftUseRequest {
    pub const MSG_ID: u16 = 25405;
}

impl PbMessage for ItemGiftUseRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectedId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectedId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.ItemId = r.read_varint_i32()?,
            15 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemGiftUseResponse {
    pub Infos: Vec<AddCountItemInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl ItemGiftUseResponse {
    pub const MSG_ID: u16 = 25639;
}

impl PbMessage for ItemGiftUseResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
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
pub struct ItemObtainNotify {
    pub Items: Vec<AddCountItemInfo>,
    pub RewardItemPackages: Vec<RewardItemPackage>,
    pub Reason: i32,
}

impl ItemObtainNotify {
    pub const MSG_ID: u16 = 18089;
}

impl PbMessage for ItemObtainNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.RewardItemPackages {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.Reason != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItemPackages
                    .push(<RewardItemPackage as PbMessage>::decode(bytes)?);
            }
            14 => self.Reason = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemPkgFullNotify {
    pub PkgType: i32,
}

impl ItemPkgFullNotify {
    pub const MSG_ID: u16 = 15052;
}

impl PbMessage for ItemPkgFullNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PkgType != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PkgType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.PkgType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemRelateFoundationNotify {
    pub FoundationEntityId: i64,
    pub InstId: i32,
    pub Reason: Option<ItemRelateFoundationReason>,
    pub MatchIndex: i32,
    pub ItemEntityId: i64,
}

impl ItemRelateFoundationNotify {
    pub const MSG_ID: u16 = 19604;
}

impl PbMessage for ItemRelateFoundationNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FoundationEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FoundationEntityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MatchIndex != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MatchIndex);
        }
        if self.ItemEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ItemEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.FoundationEntityId = r.read_varint_i64()?,
            8 => self.InstId = r.read_varint_i32()?,
            10 => {
                self.Reason = Some(
                    <ItemRelateFoundationReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.MatchIndex = r.read_varint_i32()?,
            15 => self.ItemEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JinzhouFlyData {
    pub QuestId: i32,
}

impl PbMessage for JinzhouFlyData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JoinInfluenceSyncDataPb {
    pub InfluenceId: Vec<i32>,
}

impl PbMessage for JoinInfluenceSyncDataPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InfluenceId {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InfluenceId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JoinSceneNotifyConfirmRequest {
    pub SceneTraceId: i64,
}

impl JoinSceneNotifyConfirmRequest {
    pub const MSG_ID: u16 = 22643;
}

impl PbMessage for JoinSceneNotifyConfirmRequest {
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
pub struct JoinSceneNotifyConfirmResponse {}

impl JoinSceneNotifyConfirmResponse {
    pub const MSG_ID: u16 = 17440;
}

impl PbMessage for JoinSceneNotifyConfirmResponse {
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
pub struct JoinWorldTeamNotify {
    pub PlayerInfos: Vec<WorldTeamPlayerInfo>,
    pub OwnerId: i32,
}

impl JoinWorldTeamNotify {
    pub const MSG_ID: u16 = 17115;
}

impl PbMessage for JoinWorldTeamNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.OwnerId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfos
                    .push(<WorldTeamPlayerInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.OwnerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessCard {
    pub CardIds: Vec<i32>,
}

impl PbMessage for JokerGuessCard {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CardIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessDisPlayCardAction {
    pub JokerSkillId: i32,
    pub Owner: Option<JokerGuessActor>,
    pub CardId: i32,
}

impl PbMessage for JokerGuessDisPlayCardAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.JokerSkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JokerSkillId);
        }
        if let Some(v) = &self.Owner {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CardId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.JokerSkillId = r.read_varint_i32()?,
            2 => {
                self.Owner = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessDrawCardAction {
    pub Drawer: Option<JokerGuessActor>,
    pub DrawCard: i32,
    pub PreSkill: i32,
}

impl PbMessage for JokerGuessDrawCardAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Drawer {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.DrawCard != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DrawCard);
        }
        if self.PreSkill != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreSkill);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Drawer = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.DrawCard = r.read_varint_i32()?,
            3 => self.PreSkill = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessDrawCardRequest {
    pub TargetCardId: i32,
}

impl JokerGuessDrawCardRequest {
    pub const MSG_ID: u16 = 29650;
}

impl PbMessage for JokerGuessDrawCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetCardId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetCardId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.TargetCardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessDrawCardResponse {
    pub DrawCardResult: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub Cmd: Vec<JokerGuessOp>,
}

impl JokerGuessDrawCardResponse {
    pub const MSG_ID: u16 = 27976;
}

impl PbMessage for JokerGuessDrawCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DrawCardResult != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DrawCardResult);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Cmd {
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
            1 => self.DrawCardResult = r.read_varint_i32()?,
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Cmd.push(<JokerGuessOp as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessHpAction {
    pub Actor: Option<JokerGuessActor>,
    pub HealthHp: i32,
}

impl PbMessage for JokerGuessHpAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Actor {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HealthHp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HealthHp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Actor = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.HealthHp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessInitGame {
    pub AiHandCards: Option<JokerGuessCard>,
    pub PlayerHandCards: Option<JokerGuessCard>,
    pub FirstActor: Option<JokerGuessActor>,
}

impl PbMessage for JokerGuessInitGame {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AiHandCards {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.PlayerHandCards {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.FirstActor {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.AiHandCards
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerHandCards
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                self.FirstActor = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct JokerGuessOp {
    pub JokerGuessLevelId: i32,
    pub OpType: Option<JokerActionType>,
    pub DrawAction: Option<JokerGuessDrawCardAction>,
    pub PlayAction: Option<JokerGuessPlayCardAction>,
    pub NewRoundAction: Option<JokerGuessRoundInfo>,
    pub SkillTriggerAction: Option<JokerGuessSkillTriggerAction>,
    pub SkillExecAction: Option<JokerGuessSkillExecAction>,
    pub Winner: Option<JokerGuessActor>,
    pub HpRefreshAction: Option<JokerGuessHpAction>,
    pub DisPlayAction: Option<JokerGuessDisPlayCardAction>,
}

impl PbMessage for JokerGuessOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.JokerGuessLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JokerGuessLevelId);
        }
        if let Some(v) = &self.OpType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.DrawAction {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.PlayAction {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.NewRoundAction {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.SkillTriggerAction {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.SkillExecAction {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.Winner {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.HpRefreshAction {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.DisPlayAction {
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
            1 => self.JokerGuessLevelId = r.read_varint_i32()?,
            2 => {
                self.OpType = Some(
                    <JokerActionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.DrawAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PlayAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.NewRoundAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SkillTriggerAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SkillExecAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                self.Winner = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.HpRefreshAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.DisPlayAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessPlayCardAction {
    pub PlayActor: Option<JokerGuessActor>,
    pub PlayCard: Option<JokerGuessCard>,
}

impl PbMessage for JokerGuessPlayCardAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlayActor {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PlayCard {
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
                self.PlayActor = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PlayCard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessPlayCardRequest {
    pub PlayCard: Option<JokerGuessCard>,
}

impl JokerGuessPlayCardRequest {
    pub const MSG_ID: u16 = 18401;
}

impl PbMessage for JokerGuessPlayCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlayCard {
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
                self.PlayCard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessPlayCardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Cmd: Vec<JokerGuessOp>,
}

impl JokerGuessPlayCardResponse {
    pub const MSG_ID: u16 = 19352;
}

impl PbMessage for JokerGuessPlayCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Cmd {
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
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Cmd.push(<JokerGuessOp as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessRewardRequest {
    pub LevelId: i32,
}

impl JokerGuessRewardRequest {
    pub const MSG_ID: u16 = 26198;
}

impl PbMessage for JokerGuessRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl JokerGuessRewardResponse {
    pub const MSG_ID: u16 = 20099;
}

impl PbMessage for JokerGuessRewardResponse {
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
pub struct JokerGuessRoundInfo {
    pub RoundNum: i32,
    pub ChangeResult: i32,
}

impl PbMessage for JokerGuessRoundInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
        }
        if self.ChangeResult != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeResult);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoundNum = r.read_varint_i32()?,
            2 => self.ChangeResult = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessSkillExecAction {
    pub JokerSkillId: i32,
    pub Owner: Option<JokerGuessActor>,
    pub Result: Option<JokerGuessSkillOp>,
    pub AiHandCards: Option<JokerGuessCard>,
    pub PlayerHandCards: Option<JokerGuessCard>,
}

impl PbMessage for JokerGuessSkillExecAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.JokerSkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JokerSkillId);
        }
        if let Some(v) = &self.Owner {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Result {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.AiHandCards {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.PlayerHandCards {
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
            1 => self.JokerSkillId = r.read_varint_i32()?,
            2 => {
                self.Owner = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                self.Result = Some(
                    <JokerGuessSkillOp as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AiHandCards
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerHandCards
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessSkillTriggerAction {
    pub JokerSkillId: i32,
    pub Owner: Option<JokerGuessActor>,
    pub RemainingSkillNum: i32,
}

impl PbMessage for JokerGuessSkillTriggerAction {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.JokerSkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.JokerSkillId);
        }
        if let Some(v) = &self.Owner {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RemainingSkillNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainingSkillNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.JokerSkillId = r.read_varint_i32()?,
            2 => {
                self.Owner = Some(
                    <JokerGuessActor as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.RemainingSkillNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessStartNewRequest {
    pub TraceId: Vec<u8>,
    pub LevelId: i32,
}

impl JokerGuessStartNewRequest {
    pub const MSG_ID: u16 = 16872;
}

impl PbMessage for JokerGuessStartNewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TraceId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TraceId);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            2 => self.TraceId = r.read_len_delimited()?.to_vec(),
            10 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JokerGuessStartNewResponse {
    pub Cmd: Vec<JokerGuessOp>,
    pub ErrorCode: Option<ErrorCode>,
    pub InitData: Option<JokerGuessInitGame>,
}

impl JokerGuessStartNewResponse {
    pub const MSG_ID: u16 = 18171;
}

impl PbMessage for JokerGuessStartNewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Cmd {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.InitData {
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Cmd.push(<JokerGuessOp as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
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
pub struct JokerGuessUseSkillRequest {
    pub SkillId: i32,
    pub Op: Option<JokerGuessSkillOp>,
}

impl JokerGuessUseSkillRequest {
    pub const MSG_ID: u16 = 18807;
}

impl PbMessage for JokerGuessUseSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if let Some(v) = &self.Op {
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
            4 => self.SkillId = r.read_varint_i32()?,
            6 => {
                self.Op = Some(
                    <JokerGuessSkillOp as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct JokerGuessUseSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Cmd: Vec<JokerGuessOp>,
}

impl JokerGuessUseSkillResponse {
    pub const MSG_ID: u16 = 28232;
}

impl PbMessage for JokerGuessUseSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Cmd {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Cmd.push(<JokerGuessOp as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KcpConvRequest {
    pub LoginTraceId: Vec<u8>,
    pub DeviceId: Vec<u8>,
    pub UdpPort: i32,
    pub Token: Vec<u8>,
    pub DisableCrc: bool,
}

impl KcpConvRequest {
    pub const MSG_ID: u16 = 3728;
}

impl PbMessage for KcpConvRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.LoginTraceId.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LoginTraceId);
        }
        if !self.DeviceId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.DeviceId);
        }
        if self.UdpPort != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UdpPort);
        }
        if !self.Token.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Token);
        }
        if self.DisableCrc {
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
            1 => self.LoginTraceId = r.read_len_delimited()?.to_vec(),
            2 => self.DeviceId = r.read_len_delimited()?.to_vec(),
            3 => self.UdpPort = r.read_varint_i32()?,
            4 => self.Token = r.read_len_delimited()?.to_vec(),
            5 => self.DisableCrc = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KcpConvResponse {
    pub Code: Option<ErrorCode>,
    pub Conv: u32,
    pub CrcCheck: i32,
    pub KcpMtu: i32,
}

impl KcpConvResponse {
    pub const MSG_ID: u16 = 3729;
}

impl PbMessage for KcpConvResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Conv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.Conv));
        }
        if self.CrcCheck != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CrcCheck);
        }
        if self.KcpMtu != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KcpMtu);
        }
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
            2 => self.Conv = r.read_varint_u32()?,
            3 => self.CrcCheck = r.read_varint_i32()?,
            4 => self.KcpMtu = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KickMatchTeamPlayerRequest {
    pub PlayerId: i32,
}

impl KickMatchTeamPlayerRequest {
    pub const MSG_ID: u16 = 24963;
}

impl PbMessage for KickMatchTeamPlayerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            13 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KickMatchTeamPlayerResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KickMatchTeamPlayerResponse {
    pub const MSG_ID: u16 = 21595;
}

impl PbMessage for KickMatchTeamPlayerResponse {
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
pub struct KickWorldTeamRequest {
    pub PlayerId: i32,
}

impl KickWorldTeamRequest {
    pub const MSG_ID: u16 = 27061;
}

impl PbMessage for KickWorldTeamRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            5 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KickWorldTeamResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KickWorldTeamResponse {
    pub const MSG_ID: u16 = 26682;
}

impl PbMessage for KickWorldTeamResponse {
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
pub struct KujiAwardGroupPb {
    pub Id: i32,
    pub GroupRank: i32,
    pub RewardedAmount: i32,
    pub AllAmount: i32,
    pub PreItemMap: BTreeMap<i32, i32>,
}

impl PbMessage for KujiAwardGroupPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.GroupRank != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupRank);
        }
        if self.RewardedAmount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardedAmount);
        }
        if self.AllAmount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AllAmount);
        }
        if !self.PreItemMap.is_empty() {
            crate::wire::write_map(out, 5, &self.PreItemMap);
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
            2 => self.GroupRank = r.read_varint_i32()?,
            3 => self.RewardedAmount = r.read_varint_i32()?,
            4 => self.AllAmount = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.PreItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoAllItemUpdateNotify {
    pub PanelPbData: Option<KurotatoItemPanelPbData>,
}

impl KurotatoAllItemUpdateNotify {
    pub const MSG_ID: u16 = 28952;
}

impl PbMessage for KurotatoAllItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoChestRewardNotify {
    pub ItemId: i32,
    pub RewardIndex: i32,
    pub SoldPrice: i32,
    pub TotalRewardCount: i32,
}

impl KurotatoChestRewardNotify {
    pub const MSG_ID: u16 = 24115;
}

impl PbMessage for KurotatoChestRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if self.SoldPrice != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SoldPrice);
        }
        if self.TotalRewardCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalRewardCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.ItemId = r.read_varint_i32()?,
            6 => self.RewardIndex = r.read_varint_i32()?,
            10 => self.SoldPrice = r.read_varint_i32()?,
            14 => self.TotalRewardCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoChestRewardSelectRequest {
    pub WillSold: bool,
}

impl KurotatoChestRewardSelectRequest {
    pub const MSG_ID: u16 = 16722;
}

impl PbMessage for KurotatoChestRewardSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WillSold {
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
            12 => self.WillSold = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoChestRewardSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoChestRewardSelectResponse {
    pub const MSG_ID: u16 = 22900;
}

impl PbMessage for KurotatoChestRewardSelectResponse {
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
pub struct KurotatoChestRewardStepPbData {}

impl PbMessage for KurotatoChestRewardStepPbData {
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
pub struct KurotatoCombatStepPbData {
    pub WaveDuration: i32,
}

impl PbMessage for KurotatoCombatStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveDuration != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WaveDuration);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.WaveDuration = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoCtx {
    pub Continue: bool,
    pub LevelId: i32,
    pub RoleId: i32,
}

impl PbMessage for KurotatoCtx {
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
pub struct KurotatoDieNotify {}

impl KurotatoDieNotify {
    pub const MSG_ID: u16 = 24258;
}

impl PbMessage for KurotatoDieNotify {
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
pub struct KurotatoDropAbsorbedCtxPb {}

impl PbMessage for KurotatoDropAbsorbedCtxPb {
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
pub struct KurotatoEndShopStepRequest {
    pub IsEndLess: bool,
}

impl KurotatoEndShopStepRequest {
    pub const MSG_ID: u16 = 26067;
}

impl PbMessage for KurotatoEndShopStepRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEndLess {
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
            9 => self.IsEndLess = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoEndShopStepResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoEndShopStepResponse {
    pub const MSG_ID: u16 = 23609;
}

impl PbMessage for KurotatoEndShopStepResponse {
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
pub struct KurotatoEndSpecialWaveCombatRequest {}

impl KurotatoEndSpecialWaveCombatRequest {
    pub const MSG_ID: u16 = 16538;
}

impl PbMessage for KurotatoEndSpecialWaveCombatRequest {
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
pub struct KurotatoEndSpecialWaveCombatResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoEndSpecialWaveCombatResponse {
    pub const MSG_ID: u16 = 21746;
}

impl PbMessage for KurotatoEndSpecialWaveCombatResponse {
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
pub struct KurotatoEndStepPbData {}

impl PbMessage for KurotatoEndStepPbData {
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
pub struct KurotatoGoldAddRequest {
    pub GoldCount: i32,
}

impl KurotatoGoldAddRequest {
    pub const MSG_ID: u16 = 18249;
}

impl PbMessage for KurotatoGoldAddRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GoldCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoldCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.GoldCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoGoldAddResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoGoldAddResponse {
    pub const MSG_ID: u16 = 25569;
}

impl PbMessage for KurotatoGoldAddResponse {
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
pub struct KurotatoItemRewardPanelPbData {
    pub RewardItemDatas: Vec<KurotatoItemRewardPbData>,
    pub RefreshCost: i32,
    pub RewardIndex: i32,
    pub TotalRewardCount: i32,
}

impl PbMessage for KurotatoItemRewardPanelPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardItemDatas {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCost);
        }
        if self.RewardIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardIndex);
        }
        if self.TotalRewardCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalRewardCount);
        }
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
                self.RewardItemDatas
                    .push(<KurotatoItemRewardPbData as PbMessage>::decode(bytes)?);
            }
            2 => self.RefreshCost = r.read_varint_i32()?,
            3 => self.RewardIndex = r.read_varint_i32()?,
            4 => self.TotalRewardCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoItemRewardPbData {
    pub ItemId: i32,
    pub SelectionId: i32,
}

impl PbMessage for KurotatoItemRewardPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.SelectionId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectionId);
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
            2 => self.SelectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoItemUnlockNotify {
    pub UnlockItems: Vec<i32>,
}

impl KurotatoItemUnlockNotify {
    pub const MSG_ID: u16 = 20943;
}

impl PbMessage for KurotatoItemUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockItems {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoItemUpdateNotify {
    pub IsAdd: bool,
    pub ItemPbData: Option<KurotatoItemPbData>,
}

impl KurotatoItemUpdateNotify {
    pub const MSG_ID: u16 = 15702;
}

impl PbMessage for KurotatoItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAdd {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ItemPbData {
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
            1 => self.IsAdd = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ItemPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoLevelDataUpdateNotify {
    pub KurotatoLevelInfos: Vec<KurotatoLevelInfo>,
}

impl KurotatoLevelDataUpdateNotify {
    pub const MSG_ID: u16 = 17855;
}

impl PbMessage for KurotatoLevelDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.KurotatoLevelInfos {
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
                self.KurotatoLevelInfos
                    .push(<KurotatoLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoLevelRecordDeleteRequest {
    pub LevelId: i32,
}

impl KurotatoLevelRecordDeleteRequest {
    pub const MSG_ID: u16 = 20893;
}

impl PbMessage for KurotatoLevelRecordDeleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoLevelRecordDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoLevelRecordDeleteResponse {
    pub const MSG_ID: u16 = 27741;
}

impl PbMessage for KurotatoLevelRecordDeleteResponse {
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
pub struct KurotatoLimitTaskReceiveRequest {
    pub TaskIds: Vec<i32>,
}

impl KurotatoLimitTaskReceiveRequest {
    pub const MSG_ID: u16 = 19700;
}

impl PbMessage for KurotatoLimitTaskReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoLimitTaskReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoLimitTaskReceiveResponse {
    pub const MSG_ID: u16 = 20933;
}

impl PbMessage for KurotatoLimitTaskReceiveResponse {
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
pub struct KurotatoLimitTaskUpdateNotify {
    pub LimitTasks: Vec<ConditionTask>,
}

impl KurotatoLimitTaskUpdateNotify {
    pub const MSG_ID: u16 = 29083;
}

impl PbMessage for KurotatoLimitTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LimitTasks {
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
                self.LimitTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoMonsterKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for KurotatoMonsterKilledCtxPb {
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
pub struct KurotatoNextWaveTypeNotify {
    pub NextWaveId: i32,
    pub NextWaveType: Option<KurotatoNextWaveType>,
}

impl KurotatoNextWaveTypeNotify {
    pub const MSG_ID: u16 = 27025;
}

impl PbMessage for KurotatoNextWaveTypeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NextWaveId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextWaveId);
        }
        if let Some(v) = &self.NextWaveType {
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
            7 => self.NextWaveId = r.read_varint_i32()?,
            12 => {
                self.NextWaveType = Some(
                    <KurotatoNextWaveType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct KurotatoPrepareStepPbData {}

impl PbMessage for KurotatoPrepareStepPbData {
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
pub struct KurotatoPropertyUpdateNotify {
    pub LockedPropertyValues: BTreeMap<i32, i32>,
    pub PropertyValues: BTreeMap<i32, i32>,
}

impl KurotatoPropertyUpdateNotify {
    pub const MSG_ID: u16 = 22357;
}

impl PbMessage for KurotatoPropertyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.LockedPropertyValues.is_empty() {
            crate::wire::write_map(out, 3, &self.LockedPropertyValues);
        }
        if !self.PropertyValues.is_empty() {
            crate::wire::write_map(out, 4, &self.PropertyValues);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_map_entry(r, &mut self.LockedPropertyValues)?,
            4 => crate::wire::decode_map_entry(r, &mut self.PropertyValues)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoReChallengeCountNotify {
    pub UsedReChallengeCount: i32,
    pub MaxReChallengeCount: i32,
}

impl KurotatoReChallengeCountNotify {
    pub const MSG_ID: u16 = 19047;
}

impl PbMessage for KurotatoReChallengeCountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UsedReChallengeCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UsedReChallengeCount);
        }
        if self.MaxReChallengeCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReChallengeCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.UsedReChallengeCount = r.read_varint_i32()?,
            11 => self.MaxReChallengeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoReChallengeRequest {
    pub IsFromCurWave: bool,
}

impl KurotatoReChallengeRequest {
    pub const MSG_ID: u16 = 20651;
}

impl PbMessage for KurotatoReChallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsFromCurWave {
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
            10 => self.IsFromCurWave = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoReChallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoReChallengeResponse {
    pub const MSG_ID: u16 = 18670;
}

impl PbMessage for KurotatoReChallengeResponse {
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
pub struct KurotatoRefineWeaponNotify {
    pub NewWeaponIncId: i32,
    pub RefinedWeaponIncIds: Vec<i32>,
}

impl KurotatoRefineWeaponNotify {
    pub const MSG_ID: u16 = 26350;
}

impl PbMessage for KurotatoRefineWeaponNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NewWeaponIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewWeaponIncId);
        }
        for item in &self.RefinedWeaponIncIds {
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
            3 => self.NewWeaponIncId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RefinedWeaponIncIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoRefineWeaponRequest {
    pub WeaponIncId: i32,
}

impl KurotatoRefineWeaponRequest {
    pub const MSG_ID: u16 = 28420;
}

impl PbMessage for KurotatoRefineWeaponRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponIncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.WeaponIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoRefineWeaponResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PanelPbData: Option<KurotatoWeaponPanelPbData>,
}

impl KurotatoRefineWeaponResponse {
    pub const MSG_ID: u16 = 23434;
}

impl PbMessage for KurotatoRefineWeaponResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoResTaskReceiveRequest {
    pub TaskIds: Vec<i32>,
}

impl KurotatoResTaskReceiveRequest {
    pub const MSG_ID: u16 = 16881;
}

impl PbMessage for KurotatoResTaskReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoResTaskReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoResTaskReceiveResponse {
    pub const MSG_ID: u16 = 21172;
}

impl PbMessage for KurotatoResTaskReceiveResponse {
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
pub struct KurotatoResTaskUpdateNotify {
    pub ResTasks: Vec<ConditionTask>,
}

impl KurotatoResTaskUpdateNotify {
    pub const MSG_ID: u16 = 27726;
}

impl PbMessage for KurotatoResTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ResTasks {
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
                self.ResTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoRoleKilledCtxPb {}

impl PbMessage for KurotatoRoleKilledCtxPb {
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
pub struct KurotatoRoleRecordNotify {
    pub InstInfos: Vec<KurotatoInstInfo>,
    pub IsPopView: bool,
    pub KurotatoRoleSaveState: Option<KurotatoRoleSaveState>,
}

impl KurotatoRoleRecordNotify {
    pub const MSG_ID: u16 = 29827;
}

impl PbMessage for KurotatoRoleRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.InstInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.IsPopView {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.KurotatoRoleSaveState {
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
                self.InstInfos
                    .push(<KurotatoInstInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.IsPopView = r.read_bool()?,
            4 => {
                self.KurotatoRoleSaveState = Some(
                    <KurotatoRoleSaveState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct KurotatoRoleRecordRequest {}

impl KurotatoRoleRecordRequest {
    pub const MSG_ID: u16 = 25830;
}

impl PbMessage for KurotatoRoleRecordRequest {
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
pub struct KurotatoRoleRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoRoleRecordResponse {
    pub const MSG_ID: u16 = 16983;
}

impl PbMessage for KurotatoRoleRecordResponse {
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
pub struct KurotatoRoleUnlockNotify {
    pub KurotatoRoleInfos: Vec<KurotatoRoleInfo>,
}

impl KurotatoRoleUnlockNotify {
    pub const MSG_ID: u16 = 16470;
}

impl PbMessage for KurotatoRoleUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.KurotatoRoleInfos {
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
                self.KurotatoRoleInfos
                    .push(<KurotatoRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoScoreTaskReceiveRequest {
    pub TaskIds: Vec<i32>,
}

impl KurotatoScoreTaskReceiveRequest {
    pub const MSG_ID: u16 = 25395;
}

impl PbMessage for KurotatoScoreTaskReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoScoreTaskReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoScoreTaskReceiveResponse {
    pub const MSG_ID: u16 = 20122;
}

impl PbMessage for KurotatoScoreTaskReceiveResponse {
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
pub struct KurotatoSellWeaponRequest {
    pub WeaponIncId: i32,
}

impl KurotatoSellWeaponRequest {
    pub const MSG_ID: u16 = 28213;
}

impl PbMessage for KurotatoSellWeaponRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.WeaponIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoSellWeaponResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PanelPbData: Option<KurotatoWeaponPanelPbData>,
}

impl KurotatoSellWeaponResponse {
    pub const MSG_ID: u16 = 24086;
}

impl PbMessage for KurotatoSellWeaponResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PanelPbData {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoSettlementInfo {
    pub IsPass: bool,
    pub CompletionTime: i32,
    pub ItemPanelPbData: Option<KurotatoItemPanelPbData>,
    pub WeaponPanelPbData: Option<KurotatoWeaponPanelPbData>,
    pub PassWaveCount: i32,
    pub CumulativeKills: i32,
    pub PropertyMap: BTreeMap<i32, i32>,
    pub UnlockWeapons: Vec<i32>,
    pub UnlockItems: Vec<i32>,
    pub UnlockRoles: Vec<i32>,
}

impl PbMessage for KurotatoSettlementInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsPass {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CompletionTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CompletionTime);
        }
        if let Some(v) = &self.ItemPanelPbData {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.WeaponPanelPbData {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.PassWaveCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassWaveCount);
        }
        if self.CumulativeKills != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CumulativeKills);
        }
        if !self.PropertyMap.is_empty() {
            crate::wire::write_map(out, 9, &self.PropertyMap);
        }
        for item in &self.UnlockWeapons {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockItems {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockRoles {
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
            2 => self.IsPass = r.read_bool()?,
            3 => self.CompletionTime = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ItemPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.PassWaveCount = r.read_varint_i32()?,
            8 => self.CumulativeKills = r.read_varint_i32()?,
            9 => crate::wire::decode_map_entry(r, &mut self.PropertyMap)?,
            10 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockWeapons, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockItems, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockRoles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoSettlementNotify {
    pub KurotatoSettlementInfo: Option<KurotatoSettlementInfo>,
}

impl KurotatoSettlementNotify {
    pub const MSG_ID: u16 = 24835;
}

impl PbMessage for KurotatoSettlementNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.KurotatoSettlementInfo {
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
                self.KurotatoSettlementInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoSettlementRequest {}

impl KurotatoSettlementRequest {
    pub const MSG_ID: u16 = 29686;
}

impl PbMessage for KurotatoSettlementRequest {
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
pub struct KurotatoSettlementResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub KurotatoSettlementInfo: Option<KurotatoSettlementInfo>,
}

impl KurotatoSettlementResponse {
    pub const MSG_ID: u16 = 24003;
}

impl PbMessage for KurotatoSettlementResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.KurotatoSettlementInfo {
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
                self.KurotatoSettlementInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopItemProductPbData {
    pub ItemId: i32,
}

impl PbMessage for KurotatoShopItemProductPbData {
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
pub struct KurotatoShopPanelPbData {
    pub ItemPbDatas: Vec<KurotatoShopProductPbData>,
    pub RefreshCost: i32,
}

impl PbMessage for KurotatoShopPanelPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemPbDatas {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
                self.ItemPbDatas
                    .push(<KurotatoShopProductPbData as PbMessage>::decode(bytes)?);
            }
            2 => self.RefreshCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopProductPbData {
    pub SelectionId: i32,
    pub Cost: i32,
    pub OriginalCost: i32,
    pub IsLocked: bool,
    pub IsBought: bool,
    pub ItemData: Option<KurotatoShopItemProductPbData>,
    pub WeaponData: Option<KurotatoShopWeaponProductPbData>,
}

impl PbMessage for KurotatoShopProductPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectionId);
        }
        if self.Cost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cost);
        }
        if self.OriginalCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OriginalCost);
        }
        if self.IsLocked {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsBought {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ItemData {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.WeaponData {
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
            1 => self.SelectionId = r.read_varint_i32()?,
            2 => self.Cost = r.read_varint_i32()?,
            3 => self.OriginalCost = r.read_varint_i32()?,
            4 => self.IsLocked = r.read_bool()?,
            5 => self.IsBought = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ItemData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopRewardLockRequest {
    pub SelectionId: i32,
}

impl KurotatoShopRewardLockRequest {
    pub const MSG_ID: u16 = 27851;
}

impl PbMessage for KurotatoShopRewardLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectionId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.SelectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopRewardLockResponse {
    pub PanelPbData: Option<KurotatoShopPanelPbData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoShopRewardLockResponse {
    pub const MSG_ID: u16 = 15987;
}

impl PbMessage for KurotatoShopRewardLockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
            crate::wire::write_submessage(out, 12, v);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PanelPbData
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
pub struct KurotatoShopRewardNotify {
    pub PanelPbData: Option<KurotatoShopPanelPbData>,
}

impl KurotatoShopRewardNotify {
    pub const MSG_ID: u16 = 15439;
}

impl PbMessage for KurotatoShopRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopRewardRefreshRequest {}

impl KurotatoShopRewardRefreshRequest {
    pub const MSG_ID: u16 = 27770;
}

impl PbMessage for KurotatoShopRewardRefreshRequest {
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
pub struct KurotatoShopRewardRefreshResponse {
    pub PanelPbData: Option<KurotatoShopPanelPbData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoShopRewardRefreshResponse {
    pub const MSG_ID: u16 = 16518;
}

impl PbMessage for KurotatoShopRewardRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
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
pub struct KurotatoShopRewardSelectRequest {
    pub SelectionId: i32,
}

impl KurotatoShopRewardSelectRequest {
    pub const MSG_ID: u16 = 25262;
}

impl PbMessage for KurotatoShopRewardSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SelectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopRewardSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PanelPbData: Option<KurotatoShopPanelPbData>,
}

impl KurotatoShopRewardSelectResponse {
    pub const MSG_ID: u16 = 28684;
}

impl PbMessage for KurotatoShopRewardSelectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PanelPbData {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoShopStepPbData {
    pub HasEndLessModeSelection: bool,
    pub NextWaveType: Option<KurotatoNextWaveType>,
}

impl PbMessage for KurotatoShopStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasEndLessModeSelection {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.NextWaveType {
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
            1 => self.HasEndLessModeSelection = r.read_bool()?,
            2 => {
                self.NextWaveType = Some(
                    <KurotatoNextWaveType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct KurotatoShopWeaponProductPbData {
    pub WeaponId: i32,
}

impl PbMessage for KurotatoShopWeaponProductPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoStepAdvanceRequest {
    pub EndStepPbData: Option<KurotatoEndStepPbData>,
}

impl KurotatoStepAdvanceRequest {
    pub const MSG_ID: u16 = 16129;
}

impl PbMessage for KurotatoStepAdvanceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.EndStepPbData {
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
                self.EndStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoStepAdvanceResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl KurotatoStepAdvanceResponse {
    pub const MSG_ID: u16 = 15011;
}

impl PbMessage for KurotatoStepAdvanceResponse {
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
pub struct KurotatoStepUpdateNotify {
    pub ChestRewardStepPbData: Option<KurotatoChestRewardStepPbData>,
    pub ShopStepPbData: Option<KurotatoShopStepPbData>,
    pub UpdateWavePbData: Option<KurotatoUpdateWavePbData>,
    pub EndStepPbData: Option<KurotatoEndStepPbData>,
    pub IsStart: bool,
    pub PrepareStepPbData: Option<KurotatoPrepareStepPbData>,
    pub UpgradeRewardStepPbData: Option<KurotatoUpgradeRewardStepPbData>,
    pub CombatStepPbData: Option<KurotatoCombatStepPbData>,
}

impl KurotatoStepUpdateNotify {
    pub const MSG_ID: u16 = 21986;
}

impl PbMessage for KurotatoStepUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ChestRewardStepPbData {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ShopStepPbData {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.UpdateWavePbData {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.EndStepPbData {
            crate::wire::write_submessage(out, 9, v);
        }
        if self.IsStart {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.PrepareStepPbData {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.UpgradeRewardStepPbData {
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
                self.ChestRewardStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ShopStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.UpdateWavePbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.EndStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.IsStart = r.read_bool()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PrepareStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.UpgradeRewardStepPbData
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
pub struct KurotatoStructureKilledCtxPb {}

impl PbMessage for KurotatoStructureKilledCtxPb {
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
pub struct KurotatoSyncEntityDealDamagePush {
    pub EntityDealDamages: BTreeMap<i64, i32>,
}

impl KurotatoSyncEntityDealDamagePush {
    pub const MSG_ID: u16 = 23335;
}

impl PbMessage for KurotatoSyncEntityDealDamagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.EntityDealDamages.is_empty() {
            crate::wire::write_map(out, 7, &self.EntityDealDamages);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => crate::wire::decode_map_entry(r, &mut self.EntityDealDamages)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoSystemInfoNotify {
    pub VariableKeyNames: BTreeMap<Vec<u8>, Vec<u8>>,
    pub WeaponPanelPbData: Option<KurotatoWeaponPanelPbData>,
    pub ItemPanelPbData: Option<KurotatoItemPanelPbData>,
    pub LevelId: i32,
    pub CharacterId: i32,
    pub LockedPropertyValues: BTreeMap<i32, i32>,
    pub PropertyValues: BTreeMap<i32, i32>,
}

impl KurotatoSystemInfoNotify {
    pub const MSG_ID: u16 = 17800;
}

impl PbMessage for KurotatoSystemInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.VariableKeyNames.is_empty() {
            crate::wire::write_map(out, 1, &self.VariableKeyNames);
        }
        if let Some(v) = &self.WeaponPanelPbData {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.ItemPanelPbData {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.CharacterId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CharacterId);
        }
        if !self.LockedPropertyValues.is_empty() {
            crate::wire::write_map(out, 10, &self.LockedPropertyValues);
        }
        if !self.PropertyValues.is_empty() {
            crate::wire::write_map(out, 11, &self.PropertyValues);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.VariableKeyNames)?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.WeaponPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ItemPanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.LevelId = r.read_varint_i32()?,
            8 => self.CharacterId = r.read_varint_i32()?,
            10 => crate::wire::decode_map_entry(r, &mut self.LockedPropertyValues)?,
            11 => crate::wire::decode_map_entry(r, &mut self.PropertyValues)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoUpdateWavePbData {}

impl PbMessage for KurotatoUpdateWavePbData {
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
pub struct KurotatoUpgradeRewardNotify {
    pub PanelPbData: Option<KurotatoItemRewardPanelPbData>,
}

impl KurotatoUpgradeRewardNotify {
    pub const MSG_ID: u16 = 18286;
}

impl PbMessage for KurotatoUpgradeRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoUpgradeRewardRefreshRequest {}

impl KurotatoUpgradeRewardRefreshRequest {
    pub const MSG_ID: u16 = 19521;
}

impl PbMessage for KurotatoUpgradeRewardRefreshRequest {
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
pub struct KurotatoUpgradeRewardRefreshResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PanelPbData: Option<KurotatoItemRewardPanelPbData>,
}

impl KurotatoUpgradeRewardRefreshResponse {
    pub const MSG_ID: u16 = 21421;
}

impl PbMessage for KurotatoUpgradeRewardRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PanelPbData {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoUpgradeRewardSelectRequest {
    pub SelectionId: i32,
}

impl KurotatoUpgradeRewardSelectRequest {
    pub const MSG_ID: u16 = 20748;
}

impl PbMessage for KurotatoUpgradeRewardSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectionId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.SelectionId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoUpgradeRewardSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl KurotatoUpgradeRewardSelectResponse {
    pub const MSG_ID: u16 = 24364;
}

impl PbMessage for KurotatoUpgradeRewardSelectResponse {
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
pub struct KurotatoUpgradeRewardStepPbData {}

impl PbMessage for KurotatoUpgradeRewardStepPbData {
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
pub struct KurotatoWeaponUnlockNotify {
    pub UnlockWeapons: Vec<i32>,
}

impl KurotatoWeaponUnlockNotify {
    pub const MSG_ID: u16 = 17432;
}

impl PbMessage for KurotatoWeaponUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockWeapons {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockWeapons, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KurotatoWeaponUpdateNotify {
    pub PanelPbData: Option<KurotatoWeaponPanelPbData>,
}

impl KurotatoWeaponUpdateNotify {
    pub const MSG_ID: u16 = 22797;
}

impl PbMessage for KurotatoWeaponUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PanelPbData {
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
                self.PanelPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LastBigSceneMiniMapInfoRequest {}

impl LastBigSceneMiniMapInfoRequest {
    pub const MSG_ID: u16 = 28705;
}

impl PbMessage for LastBigSceneMiniMapInfoRequest {
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
pub struct LastBigSceneMiniMapInfoResponse {
    pub PosA: f32,
    pub Code: Option<ErrorCode>,
    pub PosZ: f32,
    pub PosY: f32,
    pub PosX: f32,
}

impl LastBigSceneMiniMapInfoResponse {
    pub const MSG_ID: u16 = 20403;
}

impl PbMessage for LastBigSceneMiniMapInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PosA != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosA).to_bits().to_le_bytes());
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PosZ != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosZ).to_bits().to_le_bytes());
        }
        if self.PosY != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosY).to_bits().to_le_bytes());
        }
        if self.PosX != 0.0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.PosX).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.PosA = r.read_f32()?,
            3 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.PosZ = r.read_f32()?,
            12 => self.PosY = r.read_f32()?,
            14 => self.PosX = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LeaveAoiNotify {
    pub EntityIds: Vec<i64>,
}

impl LeaveAoiNotify {
    pub const MSG_ID: u16 = 19141;
}

impl PbMessage for LeaveAoiNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EntityIds {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EntityIds, |sr| {
                Ok(sr.read_varint_i64()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LeaveLevelPlayNotify {
    pub Id: i32,
}

impl LeaveLevelPlayNotify {
    pub const MSG_ID: u16 = 25269;
}

impl PbMessage for LeaveLevelPlayNotify {
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
pub struct LeaveMatchTeamNotify {
    pub PlayerId: i32,
    pub LeaveReason: Option<MatchPlayerLeaveReason>,
}

impl LeaveMatchTeamNotify {
    pub const MSG_ID: u16 = 21476;
}

impl PbMessage for LeaveMatchTeamNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.LeaveReason {
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
            4 => self.PlayerId = r.read_varint_i32()?,
            15 => {
                self.LeaveReason = Some(
                    <MatchPlayerLeaveReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct LeaveMatchTeamRequest {}

impl LeaveMatchTeamRequest {
    pub const MSG_ID: u16 = 27683;
}

impl PbMessage for LeaveMatchTeamRequest {
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
pub struct LeaveMatchTeamResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LeaveMatchTeamResponse {
    pub const MSG_ID: u16 = 18375;
}

impl PbMessage for LeaveMatchTeamResponse {
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
pub struct LeaveQuestRecallNotify {
    pub DestroyRecallQuestIds: Vec<i32>,
}

impl LeaveQuestRecallNotify {
    pub const MSG_ID: u16 = 20978;
}

impl PbMessage for LeaveQuestRecallNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DestroyRecallQuestIds {
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
                &mut self.DestroyRecallQuestIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LeaveWorldTeamRequest {
    pub PlayerId: i32,
    pub Reason: Option<WorldTeamLeaveReason>,
}

impl LeaveWorldTeamRequest {
    pub const MSG_ID: u16 = 16004;
}

impl PbMessage for LeaveWorldTeamRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Reason {
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
            7 => self.PlayerId = r.read_varint_i32()?,
            13 => {
                self.Reason = Some(
                    <WorldTeamLeaveReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct LeaveWorldTeamResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LeaveWorldTeamResponse {
    pub const MSG_ID: u16 = 16579;
}

impl PbMessage for LeaveWorldTeamResponse {
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
pub struct LevelGridPbData {
    pub GridPlacementPbInfo: Option<GridPlacementPbInfo>,
    pub EntityGridPbData: Option<EntityGridPbData>,
    pub PollutedGridPbData: Option<PollutedGridPbData>,
}

impl PbMessage for LevelGridPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GridPlacementPbInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.EntityGridPbData {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.PollutedGridPbData {
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
                self.GridPlacementPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.EntityGridPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PollutedGridPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayFastReturnPosRequest {
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub LevelPlayId: i32,
    pub TreeIncId: i64,
}

impl LevelPlayFastReturnPosRequest {
    pub const MSG_ID: u16 = 27495;
}

impl PbMessage for LevelPlayFastReturnPosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            7 => self.LevelPlayId = r.read_varint_i32()?,
            8 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayFastReturnPosResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlayFastReturnPosResponse {
    pub const MSG_ID: u16 = 17289;
}

impl PbMessage for LevelPlayFastReturnPosResponse {
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
pub struct LevelPlayFirstNotify {
    pub Id: i32,
}

impl LevelPlayFirstNotify {
    pub const MSG_ID: u16 = 28089;
}

impl PbMessage for LevelPlayFirstNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayGainRewardInfoNotify {
    pub GainRewardInfoDict: BTreeMap<i32, LevelPlayGainRewardInfo>,
}

impl LevelPlayGainRewardInfoNotify {
    pub const MSG_ID: u16 = 25358;
}

impl PbMessage for LevelPlayGainRewardInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.GainRewardInfoDict.is_empty() {
            crate::wire::write_map(out, 10, &self.GainRewardInfoDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_map_entry(r, &mut self.GainRewardInfoDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayOpenRequest {
    pub InstId: i32,
    pub LevelPlayId: i32,
}

impl LevelPlayOpenRequest {
    pub const MSG_ID: u16 = 23577;
}

impl PbMessage for LevelPlayOpenRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
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
            7 => self.InstId = r.read_varint_i32()?,
            13 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayOpenResponse {
    pub LevelPlayId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlayOpenResponse {
    pub const MSG_ID: u16 = 15142;
}

impl PbMessage for LevelPlayOpenResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
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
            6 => self.LevelPlayId = r.read_varint_i32()?,
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
pub struct LevelPlayOpenTimeNotify {
    pub OpenTime: i64,
    pub Id: i32,
}

impl LevelPlayOpenTimeNotify {
    pub const MSG_ID: u16 = 27112;
}

impl PbMessage for LevelPlayOpenTimeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
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
            3 => self.OpenTime = r.read_varint_i64()?,
            13 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardCountResetNotify {
    pub LevelPlayIds: Vec<i32>,
}

impl LevelPlayRewardCountResetNotify {
    pub const MSG_ID: u16 = 24225;
}

impl PbMessage for LevelPlayRewardCountResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LevelPlayIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardInfo {
    pub Id: i32,
    pub GetReward: bool,
}

impl PbMessage for LevelPlayRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.GetReward {
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
            2 => self.GetReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardNotify {
    pub LevelPlayRewards: Vec<LevelPlayRewardInfo>,
}

impl LevelPlayRewardNotify {
    pub const MSG_ID: u16 = 25939;
}

impl PbMessage for LevelPlayRewardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LevelPlayRewards {
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
                self.LevelPlayRewards
                    .push(<LevelPlayRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardRequest {
    pub EntityId: i64,
    pub ExchangeCount: i32,
}

impl LevelPlayRewardRequest {
    pub const MSG_ID: u16 = 20160;
}

impl PbMessage for LevelPlayRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ExchangeCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExchangeCount);
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
            13 => self.ExchangeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlayRewardResponse {
    pub const MSG_ID: u16 = 17215;
}

impl PbMessage for LevelPlayRewardResponse {
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
pub struct LevelPlayStateNotify {
    pub State: i32,
    pub CompleteNumber: i32,
    pub Id: i32,
}

impl LevelPlayStateNotify {
    pub const MSG_ID: u16 = 19272;
}

impl PbMessage for LevelPlayStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.State != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.State);
        }
        if self.CompleteNumber != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CompleteNumber);
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
            2 => self.State = r.read_varint_i32()?,
            5 => self.CompleteNumber = r.read_varint_i32()?,
            11 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayStateRequest {
    pub ArenaId: i32,
    pub InstId: i32,
}

impl LevelPlayStateRequest {
    pub const MSG_ID: u16 = 16155;
}

impl PbMessage for LevelPlayStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ArenaId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ArenaId);
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
            3 => self.ArenaId = r.read_varint_i32()?,
            11 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayStateResponse {
    pub LevelPlayStateDict: BTreeMap<i32, LevelPlayStateMsg>,
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlayStateResponse {
    pub const MSG_ID: u16 = 29313;
}

impl PbMessage for LevelPlayStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.LevelPlayStateDict.is_empty() {
            crate::wire::write_map(out, 5, &self.LevelPlayStateDict);
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
            5 => crate::wire::decode_map_entry(r, &mut self.LevelPlayStateDict)?,
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
pub struct LevelPlaySwitchSportModeRequest {
    pub OldSportMode: Option<ELevelPlaySportMode>,
    pub NewSportMode: Option<ELevelPlaySportMode>,
}

impl LevelPlaySwitchSportModeRequest {
    pub const MSG_ID: u16 = 23163;
}

impl PbMessage for LevelPlaySwitchSportModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OldSportMode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.NewSportMode {
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
            1 => {
                self.OldSportMode = Some(
                    <ELevelPlaySportMode as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => {
                self.NewSportMode = Some(
                    <ELevelPlaySportMode as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct LevelPlaySwitchSportModeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelPlaySwitchSportModeResponse {
    pub const MSG_ID: u16 = 28737;
}

impl PbMessage for LevelPlaySwitchSportModeResponse {
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
pub struct LevelPlayVarRequest {
    pub InstId: i32,
    pub LevelPlayId: i32,
}

impl LevelPlayVarRequest {
    pub const MSG_ID: u16 = 17974;
}

impl PbMessage for LevelPlayVarRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            3 => self.InstId = r.read_varint_i32()?,
            12 => self.LevelPlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayVarResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Vars: BTreeMap<Vec<u8>, VarDefinePb>,
}

impl LevelPlayVarResponse {
    pub const MSG_ID: u16 = 21256;
}

impl PbMessage for LevelPlayVarResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Vars.is_empty() {
            crate::wire::write_map(out, 14, &self.Vars);
        }
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
            14 => crate::wire::decode_map_entry(r, &mut self.Vars)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelPlayerRewardHintNotify {
    pub Id: i32,
    pub ResetTime: i64,
}

impl LevelPlayerRewardHintNotify {
    pub const MSG_ID: u16 = 27910;
}

impl PbMessage for LevelPlayerRewardHintNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.ResetTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ResetTime);
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
            13 => self.ResetTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelSequenceFrameEventActionRequest {
    pub EntityId: i64,
    pub HostPlayerId: i32,
    pub Key: Vec<u8>,
}

impl LevelSequenceFrameEventActionRequest {
    pub const MSG_ID: u16 = 20312;
}

impl PbMessage for LevelSequenceFrameEventActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostPlayerId);
        }
        if !self.Key.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Key);
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
            8 => self.HostPlayerId = r.read_varint_i32()?,
            15 => self.Key = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LevelSequenceFrameEventActionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LevelSequenceFrameEventActionResponse {
    pub const MSG_ID: u16 = 20164;
}

impl PbMessage for LevelSequenceFrameEventActionResponse {
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
pub struct LifePointChallengeActivityData {
    pub Challenges: Vec<LifePointChallengeData>,
}

impl PbMessage for LifePointChallengeActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Challenges {
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
                self.Challenges
                    .push(<LifePointChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LifePointChallengeStartRequest {
    pub Id: i32,
}

impl LifePointChallengeStartRequest {
    pub const MSG_ID: u16 = 17900;
}

impl PbMessage for LifePointChallengeStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            3 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LifePointChallengeStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LifePointChallengeStartResponse {
    pub const MSG_ID: u16 = 22206;
}

impl PbMessage for LifePointChallengeStartResponse {
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
pub struct LifePointChallengeUpdateNotify {
    pub Challenges: Vec<LifePointChallengeData>,
}

impl LifePointChallengeUpdateNotify {
    pub const MSG_ID: u16 = 21262;
}

impl PbMessage for LifePointChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Challenges {
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
                self.Challenges
                    .push(<LifePointChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiftMoveTypeCustomSpeedPb {
    pub TotalTime: f32,
    pub TimePathCurve: Vec<u8>,
}

impl PbMessage for LiftMoveTypeCustomSpeedPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalTime != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.TotalTime).to_bits().to_le_bytes());
        }
        if !self.TimePathCurve.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TimePathCurve);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TotalTime = r.read_f32()?,
            2 => self.TimePathCurve = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiftMoveTypeDefaultPb {}

impl PbMessage for LiftMoveTypeDefaultPb {
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
pub struct LiftMoveTypePb {
    pub Type: Option<LiftMoveType>,
    pub MoveDefault: Option<LiftMoveTypeDefaultPb>,
    pub MoveTeleport: Option<LiftMoveTypeTeleportPb>,
    pub CustomSpeed: Option<LiftMoveTypeCustomSpeedPb>,
}

impl PbMessage for LiftMoveTypePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.MoveDefault {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.MoveTeleport {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.CustomSpeed {
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
                self.Type = Some(<LiftMoveType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.MoveDefault
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.MoveTeleport
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CustomSpeed
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LiftMoveTypeTeleportPb {
    pub StartTeleportOffsetDistance: f32,
    pub EndTeleportOffsetDistance: f32,
    pub SendTeleportEvent: bool,
    pub TeleportStartEventOffsetDistance: f32,
}

impl PbMessage for LiftMoveTypeTeleportPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StartTeleportOffsetDistance != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.StartTeleportOffsetDistance).to_bits().to_le_bytes());
        }
        if self.EndTeleportOffsetDistance != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.EndTeleportOffsetDistance).to_bits().to_le_bytes());
        }
        if self.SendTeleportEvent {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TeleportStartEventOffsetDistance != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(
                &(self.TeleportStartEventOffsetDistance)
                    .to_bits()
                    .to_le_bytes(),
            );
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StartTeleportOffsetDistance = r.read_f32()?,
            2 => self.EndTeleportOffsetDistance = r.read_f32()?,
            3 => self.SendTeleportEvent = r.read_bool()?,
            4 => self.TeleportStartEventOffsetDistance = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightBlockByLocationOp {
    pub ConfigId: i32,
    pub GridIndex: i32,
    pub FocusTime: i32,
    pub BackToPlayer: bool,
    pub Grids: Vec<i32>,
}

impl PbMessage for LightBlockByLocationOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.GridIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridIndex);
        }
        if self.FocusTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FocusTime);
        }
        if self.BackToPlayer {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Grids {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.GridIndex = r.read_varint_i32()?,
            3 => self.FocusTime = r.read_varint_i32()?,
            4 => self.BackToPlayer = r.read_bool()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Grids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightBlockByLocationParam {
    pub GridIndex: i32,
    pub FocusTime: i32,
    pub BackToPlayer: bool,
}

impl PbMessage for LightBlockByLocationParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GridIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridIndex);
        }
        if self.FocusTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FocusTime);
        }
        if self.BackToPlayer {
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
            1 => self.GridIndex = r.read_varint_i32()?,
            2 => self.FocusTime = r.read_varint_i32()?,
            3 => self.BackToPlayer = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineCrossActivityData {
    pub Challenges: Vec<LineCrossChallengeData>,
}

impl PbMessage for LineCrossActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Challenges {
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
                self.Challenges
                    .push(<LineCrossChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineCrossActivityUpdateNotify {
    pub Challenges: Vec<LineCrossChallengeData>,
}

impl LineCrossActivityUpdateNotify {
    pub const MSG_ID: u16 = 27261;
}

impl PbMessage for LineCrossActivityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Challenges {
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
                self.Challenges
                    .push(<LineCrossChallengeData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineCrossChallengeStartRequest {
    pub Id: i32,
}

impl LineCrossChallengeStartRequest {
    pub const MSG_ID: u16 = 26440;
}

impl PbMessage for LineCrossChallengeStartRequest {
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
pub struct LineCrossChallengeStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LineCrossChallengeStartResponse {
    pub const MSG_ID: u16 = 21987;
}

impl PbMessage for LineCrossChallengeStartResponse {
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
pub struct LinkExitNotify {
    pub Reason: Option<ELinkExitReason>,
    pub PlayerId: i32,
    pub Step: i32,
}

impl LinkExitNotify {
    pub const MSG_ID: u16 = 16547;
}

impl PbMessage for LinkExitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.Step != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Step);
        }
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
                    <ELinkExitReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => self.PlayerId = r.read_varint_i32()?,
            14 => self.Step = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LinkageCheckInRewardRequest {
    pub TaskIds: Vec<i32>,
    pub Type: Option<LinkageCheckInType>,
}

impl LinkageCheckInRewardRequest {
    pub const MSG_ID: u16 = 17237;
}

impl PbMessage for LinkageCheckInRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Type {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => {
                self.Type = Some(
                    <LinkageCheckInType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct LinkageCheckInRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LinkageCheckInRewardResponse {
    pub const MSG_ID: u16 = 19375;
}

impl PbMessage for LinkageCheckInRewardResponse {
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
pub struct LinkageRewardRequest {
    pub ActivityPageId: i32,
    pub ActivityId: i32,
}

impl LinkageRewardRequest {
    pub const MSG_ID: u16 = 25785;
}

impl PbMessage for LinkageRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityPageId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityPageId);
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
            10 => self.ActivityPageId = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LinkageRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LinkageRewardResponse {
    pub const MSG_ID: u16 = 23303;
}

impl PbMessage for LinkageRewardResponse {
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
pub struct LinkingStateNotify {
    pub Step: i32,
    pub TimeStamp: i64,
    pub ContextId: i64,
    pub PlayerId: i32,
}

impl LinkingStateNotify {
    pub const MSG_ID: u16 = 27470;
}

impl PbMessage for LinkingStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Step != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Step);
        }
        if self.TimeStamp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TimeStamp);
        }
        if self.ContextId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ContextId);
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
            4 => self.Step = r.read_varint_i32()?,
            5 => self.TimeStamp = r.read_varint_i64()?,
            7 => self.ContextId = r.read_varint_i64()?,
            13 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LivenessRefreshNotify {
    pub LivenessInfo: Option<LivenessInfo>,
}

impl LivenessRefreshNotify {
    pub const MSG_ID: u16 = 17669;
}

impl PbMessage for LivenessRefreshNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LivenessInfo {
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
pub struct LivingState {
    pub Group: i32,
    pub LivingStatus: Option<LivingStatus>,
}

impl PbMessage for LivingState {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Group != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Group);
        }
        if let Some(v) = &self.LivingStatus {
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
            1 => self.Group = r.read_varint_i32()?,
            2 => {
                self.LivingStatus =
                    Some(<LivingStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadCalabashSkinInfoRequest {}

impl LoadCalabashSkinInfoRequest {
    pub const MSG_ID: u16 = 20817;
}

impl PbMessage for LoadCalabashSkinInfoRequest {
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
pub struct LoadCalabashSkinInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SkinItemIds: Vec<i32>,
    pub EquipSkinId: i32,
}

impl LoadCalabashSkinInfoResponse {
    pub const MSG_ID: u16 = 18646;
}

impl PbMessage for LoadCalabashSkinInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SkinItemIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EquipSkinId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipSkinId);
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinItemIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.EquipSkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadEquipInfoRequest {}

impl LoadEquipInfoRequest {
    pub const MSG_ID: u16 = 28824;
}

impl PbMessage for LoadEquipInfoRequest {
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
pub struct LoadEquipInfoResponse {
    pub EquipList: Vec<RoleSkinEquipData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl LoadEquipInfoResponse {
    pub const MSG_ID: u16 = 29155;
}

impl PbMessage for LoadEquipInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipList {
            crate::wire::write_submessage(out, 5, item);
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
                self.EquipList
                    .push(<RoleSkinEquipData as PbMessage>::decode(bytes)?);
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
pub struct LoadingAreaConfig {
    pub Id: i32,
    pub BeginTime: i64,
    pub EndTime: i64,
}

impl PbMessage for LoadingAreaConfig {
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
pub struct LoadingLevelAreaRequest {}

impl LoadingLevelAreaRequest {
    pub const MSG_ID: u16 = 15203;
}

impl PbMessage for LoadingLevelAreaRequest {
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
pub struct LoadingLevelAreaResponse {
    pub LimitConfigs: Vec<LoadingAreaConfig>,
}

impl LoadingLevelAreaResponse {
    pub const MSG_ID: u16 = 20507;
}

impl PbMessage for LoadingLevelAreaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LimitConfigs {
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
                self.LimitConfigs
                    .push(<LoadingAreaConfig as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LobbyQueryPlayersRequest {
    pub PlayerId: i32,
}

impl LobbyQueryPlayersRequest {
    pub const MSG_ID: u16 = 27890;
}

impl PbMessage for LobbyQueryPlayersRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            13 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LobbyQueryPlayersResponse {
    pub Item: Option<PlayerDetails>,
    pub ErrorCode: Option<ErrorCode>,
}

impl LobbyQueryPlayersResponse {
    pub const MSG_ID: u16 = 25999;
}

impl PbMessage for LobbyQueryPlayersResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Item {
            crate::wire::write_submessage(out, 14, v);
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Item
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
pub struct LoginNotify {
    pub Id: i32,
}

impl LoginNotify {
    pub const MSG_ID: u16 = 109;
}

impl PbMessage for LoginNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongPressInteractTeleportPb {
    pub ActionCtx: Option<GameCtxPb>,
    pub Index: i32,
    pub StreamingSourceConditionIndex: i32,
}

impl PbMessage for LongPressInteractTeleportPb {
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
pub struct LongShanActivity {
    pub Stages: Vec<LongShanStageInfo>,
    pub ScoreRewardedIds: Vec<i32>,
}

impl PbMessage for LongShanActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Stages {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ScoreRewardedIds {
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
                    .push(<LongShanStageInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ScoreRewardedIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanScoreRewardRequest {
    pub Ids: Vec<i32>,
    pub ActivityId: i32,
}

impl LongShanScoreRewardRequest {
    pub const MSG_ID: u16 = 15312;
}

impl PbMessage for LongShanScoreRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanScoreRewardResponse {
    pub Error: Option<ErrorCode>,
    pub ScoreRewardedIds: Vec<i32>,
}

impl LongShanScoreRewardResponse {
    pub const MSG_ID: u16 = 29191;
}

impl PbMessage for LongShanScoreRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ScoreRewardedIds {
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
            2 => {
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ScoreRewardedIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanStageInfo {
    pub Id: i32,
    pub Tasks: Vec<LongShanTaskInfo>,
    pub Unlock: bool,
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
}

impl PbMessage for LongShanStageInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Tasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
        if self.EndOpenTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndOpenTime);
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
                self.Tasks
                    .push(<LongShanTaskInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.Unlock = r.read_bool()?,
            4 => self.BeginOpenTime = r.read_varint_i64()?,
            5 => self.EndOpenTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanStageUpdateNotify {
    pub Stages: Vec<LongShanStageInfo>,
}

impl LongShanStageUpdateNotify {
    pub const MSG_ID: u16 = 22723;
}

impl PbMessage for LongShanStageUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Stages {
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
                self.Stages
                    .push(<LongShanStageInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanTaskInfo {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub IsFinished: bool,
    pub IsTaken: bool,
    pub Unlock: bool,
    pub FinishConditions: Vec<i32>,
    pub ConditionId: i32,
    pub UnlockConditionId: i32,
    pub UnlockConditionFinish: bool,
}

impl PbMessage for LongShanTaskInfo {
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
        if self.IsFinished {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsTaken {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.FinishConditions {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.UnlockConditionId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockConditionId);
        }
        if self.UnlockConditionFinish {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Current = r.read_varint_i32()?,
            3 => self.Target = r.read_varint_i32()?,
            4 => self.IsFinished = r.read_bool()?,
            5 => self.IsTaken = r.read_bool()?,
            6 => self.Unlock = r.read_bool()?,
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishConditions,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            8 => self.ConditionId = r.read_varint_i32()?,
            9 => self.UnlockConditionId = r.read_varint_i32()?,
            10 => self.UnlockConditionFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanTaskRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl LongShanTaskRewardRequest {
    pub const MSG_ID: u16 = 26202;
}

impl PbMessage for LongShanTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LongShanTaskRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl LongShanTaskRewardResponse {
    pub const MSG_ID: u16 = 25642;
}

impl PbMessage for LongShanTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
pub struct LoopTeleportVolumeActionRequest {
    pub VolumeId: i32,
}

impl LoopTeleportVolumeActionRequest {
    pub const MSG_ID: u16 = 22884;
}

impl PbMessage for LoopTeleportVolumeActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VolumeId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VolumeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.VolumeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoopTeleportVolumeActionResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ConditionMetIds: Vec<i32>,
}

impl LoopTeleportVolumeActionResponse {
    pub const MSG_ID: u16 = 25898;
}

impl PbMessage for LoopTeleportVolumeActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ConditionMetIds {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ConditionMetIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LowMemoryChangeSceneFinishRequest {}

impl LowMemoryChangeSceneFinishRequest {
    pub const MSG_ID: u16 = 20606;
}

impl PbMessage for LowMemoryChangeSceneFinishRequest {
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
pub struct LowMemoryChangeSceneFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl LowMemoryChangeSceneFinishResponse {
    pub const MSG_ID: u16 = 18214;
}

impl PbMessage for LowMemoryChangeSceneFinishResponse {
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
pub struct LowMemoryChangeSceneStartNotify {
    pub IsSwitchHLOD: bool,
    pub AssignLoadingBackgroundImagePath: Vec<u8>,
}

impl LowMemoryChangeSceneStartNotify {
    pub const MSG_ID: u16 = 25971;
}

impl PbMessage for LowMemoryChangeSceneStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsSwitchHLOD {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.AssignLoadingBackgroundImagePath.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AssignLoadingBackgroundImagePath);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.IsSwitchHLOD = r.read_bool()?,
            4 => self.AssignLoadingBackgroundImagePath = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LstBattleRecordRequest {}

impl LstBattleRecordRequest {
    pub const MSG_ID: u16 = 29144;
}

impl PbMessage for LstBattleRecordRequest {
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
pub struct LstBattleRecordResponse {
    pub LastCanRewardCount: i32,
    pub LstLevelInfos: Vec<LstLevelInfo>,
    pub LastTotalRewardCount: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl LstBattleRecordResponse {
    pub const MSG_ID: u16 = 21300;
}

impl PbMessage for LstBattleRecordResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LastCanRewardCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastCanRewardCount);
        }
        for item in &self.LstLevelInfos {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.LastTotalRewardCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastTotalRewardCount);
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
            5 => self.LastCanRewardCount = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.LstLevelInfos
                    .push(<LstLevelInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.LastTotalRewardCount = r.read_varint_i32()?,
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
pub struct LstLevelInfo {
    pub LevelId: i32,
    pub Score: i32,
    pub EasyPassFlag: bool,
}

impl PbMessage for LstLevelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.EasyPassFlag {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            3 => self.EasyPassFlag = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailAddNotify {
    pub NewMail: Option<PbMailInfo>,
    pub Reason: Option<MailAddReason>,
}

impl MailAddNotify {
    pub const MSG_ID: u16 = 21273;
}

impl PbMessage for MailAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewMail {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Reason {
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
                self.NewMail
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.Reason = Some(<MailAddReason as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailBindInfo {
    pub IsBind: bool,
    pub IsReward: bool,
    pub CloseTime: i64,
}

impl PbMessage for MailBindInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsBind {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsReward {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CloseTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CloseTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsBind = r.read_bool()?,
            2 => self.IsReward = r.read_bool()?,
            3 => self.CloseTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailBindInfoNotify {
    pub MailBindInfo: Option<MailBindInfo>,
}

impl MailBindInfoNotify {
    pub const MSG_ID: u16 = 21177;
}

impl PbMessage for MailBindInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MailBindInfo {
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
                self.MailBindInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailBindRequest {}

impl MailBindRequest {
    pub const MSG_ID: u16 = 22555;
}

impl PbMessage for MailBindRequest {
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
pub struct MailBindResponse {}

impl MailBindResponse {
    pub const MSG_ID: u16 = 28055;
}

impl PbMessage for MailBindResponse {
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
pub struct MailBindRewardRequest {}

impl MailBindRewardRequest {
    pub const MSG_ID: u16 = 24034;
}

impl PbMessage for MailBindRewardRequest {
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
pub struct MailBindRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MailBindRewardResponse {
    pub const MSG_ID: u16 = 27161;
}

impl PbMessage for MailBindRewardResponse {
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
pub struct MailDeleteNotify {
    pub Reason: Option<MailDeleteReason>,
    pub Id: Vec<u8>,
}

impl MailDeleteNotify {
    pub const MSG_ID: u16 = 29079;
}

impl PbMessage for MailDeleteNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
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
            4 => {
                self.Reason = Some(
                    <MailDeleteReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.Id = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailDeleteRequest {
    pub MailIds: Vec<Vec<u8>>,
}

impl MailDeleteRequest {
    pub const MSG_ID: u16 = 23960;
}

impl PbMessage for MailDeleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MailIds {
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
            12 => self.MailIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SuccessIds: Vec<Vec<u8>>,
}

impl MailDeleteResponse {
    pub const MSG_ID: u16 = 22089;
}

impl PbMessage for MailDeleteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SuccessIds {
            crate::wire::write_tag(out, 11, crate::wire::WireType::LengthDelimited);
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
            11 => self.SuccessIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailFavoriteRequest {
    pub Id: Vec<u8>,
    pub IsFavorite: bool,
}

impl MailFavoriteRequest {
    pub const MSG_ID: u16 = 20816;
}

impl PbMessage for MailFavoriteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Id);
        }
        if self.IsFavorite {
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
            7 => self.Id = r.read_len_delimited()?.to_vec(),
            8 => self.IsFavorite = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailFavoriteResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ExpiryTime: i64,
    pub Id: Vec<u8>,
    pub State: i32,
}

impl MailFavoriteResponse {
    pub const MSG_ID: u16 = 17919;
}

impl PbMessage for MailFavoriteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ExpiryTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExpiryTime);
        }
        if !self.Id.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Id);
        }
        if self.State != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => self.ExpiryTime = r.read_varint_i64()?,
            10 => self.Id = r.read_len_delimited()?.to_vec(),
            12 => self.State = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailGetAttachmentRequest {
    pub MailIds: Vec<Vec<u8>>,
}

impl MailGetAttachmentRequest {
    pub const MSG_ID: u16 = 15409;
}

impl PbMessage for MailGetAttachmentRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MailIds {
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
            1 => self.MailIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MailGetAttachmentResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SuccessIdMap: BTreeMap<Vec<u8>, i32>,
}

impl MailGetAttachmentResponse {
    pub const MSG_ID: u16 = 26747;
}

impl PbMessage for MailGetAttachmentResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.SuccessIdMap.is_empty() {
            crate::wire::write_map(out, 5, &self.SuccessIdMap);
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
            5 => crate::wire::decode_map_entry(r, &mut self.SuccessIdMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MainPhantomRecommendRequest {
    pub RoleId: i32,
}

impl MainPhantomRecommendRequest {
    pub const MSG_ID: u16 = 15154;
}

impl PbMessage for MainPhantomRecommendRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            9 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MainPhantomRecommendResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MainPhantomRecommendInfos: Vec<MainPhantomRecommendInfo>,
}

impl MainPhantomRecommendResponse {
    pub const MSG_ID: u16 = 26732;
}

impl PbMessage for MainPhantomRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.MainPhantomRecommendInfos {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.MainPhantomRecommendInfos
                    .push(<MainPhantomRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapCancelTraceNotify {
    pub MarkId: i32,
}

impl MapCancelTraceNotify {
    pub const MSG_ID: u16 = 17682;
}

impl PbMessage for MapCancelTraceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapCustomMarkUpdateNotify {
    pub MarkId: i32,
    pub IsDisable: bool,
    pub State: Option<MarkState>,
}

impl MapCustomMarkUpdateNotify {
    pub const MSG_ID: u16 = 24503;
}

impl PbMessage for MapCustomMarkUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if self.IsDisable {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            10 => self.MarkId = r.read_varint_i32()?,
            12 => self.IsDisable = r.read_bool()?,
            13 => {
                self.State = Some(<MarkState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkInfoRequest {}

impl MapMarkInfoRequest {
    pub const MSG_ID: u16 = 29776;
}

impl PbMessage for MapMarkInfoRequest {
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
pub struct MapMarkInfoResponse {
    pub ShowMarkIds: Vec<MapMarkShowInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub InfoList: Vec<MarkPointInfo>,
}

impl MapMarkInfoResponse {
    pub const MSG_ID: u16 = 19613;
}

impl PbMessage for MapMarkInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ShowMarkIds {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.InfoList {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.ShowMarkIds
                    .push(<MapMarkShowInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.InfoList
                    .push(<MarkPointInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapMarkInfoUpdateRequest {
    pub MarkId: i32,
    pub Pos: Option<Vector>,
}

impl MapMarkInfoUpdateRequest {
    pub const MSG_ID: u16 = 26212;
}

impl PbMessage for MapMarkInfoUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
        }
        if let Some(v) = &self.Pos {
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
            1 => self.MarkId = r.read_varint_i32()?,
            13 => {
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
pub struct MapMarkInfoUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MapMarkInfoUpdateResponse {
    pub const MSG_ID: u16 = 26030;
}

impl PbMessage for MapMarkInfoUpdateResponse {
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
pub struct MapRemoveAllMarkRequest {
    pub MarkType: i32,
}

impl MapRemoveAllMarkRequest {
    pub const MSG_ID: u16 = 26361;
}

impl PbMessage for MapRemoveAllMarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.MarkType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapRemoveAllMarkResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MarkIdList: Vec<i32>,
}

impl MapRemoveAllMarkResponse {
    pub const MSG_ID: u16 = 23743;
}

impl PbMessage for MapRemoveAllMarkResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.MarkIdList {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapRemoveMarkRequest {
    pub MarkIds: Vec<i32>,
}

impl MapRemoveMarkRequest {
    pub const MSG_ID: u16 = 16284;
}

impl PbMessage for MapRemoveMarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkIds {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapRemoveMarkResponse {
    pub MarkIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl MapRemoveMarkResponse {
    pub const MSG_ID: u16 = 29212;
}

impl PbMessage for MapRemoveMarkResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MarkIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MarkIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct MapReplaceMarkRequest {
    pub ConfigId: i32,
    pub MarkId: i32,
}

impl MapReplaceMarkRequest {
    pub const MSG_ID: u16 = 28254;
}

impl PbMessage for MapReplaceMarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            6 => self.ConfigId = r.read_varint_i32()?,
            10 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapReplaceMarkResponse {
    pub ConfigId: i32,
    pub MarkId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl MapReplaceMarkResponse {
    pub const MSG_ID: u16 = 25551;
}

impl PbMessage for MapReplaceMarkResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MarkId);
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
            2 => self.ConfigId = r.read_varint_i32()?,
            7 => self.MarkId = r.read_varint_i32()?,
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
pub struct MapTeleportByLocationOp {
    pub ConfigId: i32,
    pub GridIndex: i32,
    pub Grids: Vec<i32>,
}

impl PbMessage for MapTeleportByLocationOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.GridIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridIndex);
        }
        for item in &self.Grids {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.GridIndex = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Grids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTraceNotify {
    pub MarkId: i32,
}

impl MapTraceNotify {
    pub const MSG_ID: u16 = 26739;
}

impl PbMessage for MapTraceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MarkId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.MarkId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTravelActivityInfo {
    pub ActivityTasks: Vec<ActivityTask>,
    pub MonsterGain: Vec<i32>,
    pub GetFullReward: bool,
    pub MapTravelLevel: i32,
    pub UnLockAreas: Vec<i32>,
    pub SoarLevels: Vec<SoarLevelPlayInfo>,
}

impl PbMessage for MapTravelActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.MonsterGain {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.GetFullReward {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MapTravelLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapTravelLevel);
        }
        for item in &self.UnLockAreas {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SoarLevels {
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
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MonsterGain, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.GetFullReward = r.read_bool()?,
            4 => self.MapTravelLevel = r.read_varint_i32()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnLockAreas, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SoarLevels
                    .push(<SoarLevelPlayInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTravelFullRewardRequest {}

impl MapTravelFullRewardRequest {
    pub const MSG_ID: u16 = 21062;
}

impl PbMessage for MapTravelFullRewardRequest {
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
pub struct MapTravelFullRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MapTravelFullRewardResponse {
    pub const MSG_ID: u16 = 18348;
}

impl PbMessage for MapTravelFullRewardResponse {
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
pub struct MapTravelInfoUpdateNotify {
    pub SoarLevel: Option<SoarLevelPlayInfo>,
    pub UnLockArea: i32,
    pub ActivityTask: Option<ActivityTask>,
    pub UnLockMonsterId: i32,
}

impl MapTravelInfoUpdateNotify {
    pub const MSG_ID: u16 = 23579;
}

impl PbMessage for MapTravelInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SoarLevel {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.UnLockArea != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockArea);
        }
        if let Some(v) = &self.ActivityTask {
            crate::wire::write_submessage(out, 12, v);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.SoarLevel
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.UnLockArea = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.UnLockMonsterId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTravelLevelUpRequest {}

impl MapTravelLevelUpRequest {
    pub const MSG_ID: u16 = 24973;
}

impl PbMessage for MapTravelLevelUpRequest {
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
pub struct MapTravelLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MapTravelLevelUpResponse {
    pub const MSG_ID: u16 = 26622;
}

impl PbMessage for MapTravelLevelUpResponse {
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
pub struct MapTravelSoarRewardRequest {
    pub TaskId: i32,
}

impl MapTravelSoarRewardRequest {
    pub const MSG_ID: u16 = 15351;
}

impl PbMessage for MapTravelSoarRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTravelSoarRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MapTravelSoarRewardResponse {
    pub const MSG_ID: u16 = 24818;
}

impl PbMessage for MapTravelSoarRewardResponse {
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
pub struct MapTravelTaskRewardRequest {
    pub TaskId: i32,
}

impl MapTravelTaskRewardRequest {
    pub const MSG_ID: u16 = 24943;
}

impl PbMessage for MapTravelTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapTravelTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MapTravelTaskRewardResponse {
    pub const MSG_ID: u16 = 28036;
}

impl PbMessage for MapTravelTaskRewardResponse {
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
pub struct MatchChangePlayerUiStateNotify {
    pub MatchUiState: Option<EMatchPlayerUiState>,
    pub PlayerId: i32,
}

impl MatchChangePlayerUiStateNotify {
    pub const MSG_ID: u16 = 16156;
}

impl PbMessage for MatchChangePlayerUiStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MatchUiState {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => {
                self.MatchUiState = Some(
                    <EMatchPlayerUiState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangePlayerUiStateRequest {
    pub MatchUiState: Option<EMatchPlayerUiState>,
}

impl MatchChangePlayerUiStateRequest {
    pub const MSG_ID: u16 = 29714;
}

impl PbMessage for MatchChangePlayerUiStateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.MatchUiState {
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
                self.MatchUiState = Some(
                    <EMatchPlayerUiState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct MatchChangePlayerUiStateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MatchChangePlayerUiStateResponse {
    pub const MSG_ID: u16 = 20111;
}

impl PbMessage for MatchChangePlayerUiStateResponse {
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
pub struct MatchChangeReadyNotify {
    pub PlayerId: i32,
    pub IsReady: bool,
}

impl MatchChangeReadyNotify {
    pub const MSG_ID: u16 = 17363;
}

impl PbMessage for MatchChangeReadyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if self.IsReady {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            15 => self.IsReady = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangeReadyRequest {
    pub IsReady: bool,
}

impl MatchChangeReadyRequest {
    pub const MSG_ID: u16 = 26852;
}

impl PbMessage for MatchChangeReadyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsReady {
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
            15 => self.IsReady = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangeReadyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MatchChangeReadyResponse {
    pub const MSG_ID: u16 = 20725;
}

impl PbMessage for MatchChangeReadyResponse {
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
pub struct MatchChangeRoleContext {
    pub AbyssCtx: Option<AbyssMatchChangeRoleContext>,
}

impl PbMessage for MatchChangeRoleContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AbyssCtx {
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
                self.AbyssCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangeRoleNotify {
    pub RoleInfo: Vec<MatchRoleInfo>,
    pub PlayerId: i32,
}

impl MatchChangeRoleNotify {
    pub const MSG_ID: u16 = 24116;
}

impl PbMessage for MatchChangeRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleInfo {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleInfo
                    .push(<MatchRoleInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangeRoleRequest {
    pub RoleId: Vec<i32>,
    pub TowerDefencePhantomId: Vec<i32>,
    pub SkillBranchId: Vec<i32>,
    pub RoleContext: Option<MatchChangeRoleContext>,
}

impl MatchChangeRoleRequest {
    pub const MSG_ID: u16 = 21491;
}

impl PbMessage for MatchChangeRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.TowerDefencePhantomId {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.SkillBranchId {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.RoleContext {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.TowerDefencePhantomId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RoleContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchChangeRoleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MatchChangeRoleResponse {
    pub const MSG_ID: u16 = 21111;
}

impl PbMessage for MatchChangeRoleResponse {
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
pub struct MatchConfirmNotify {
    pub ConfirmId: i32,
}

impl MatchConfirmNotify {
    pub const MSG_ID: u16 = 29850;
}

impl PbMessage for MatchConfirmNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfirmId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfirmId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.ConfirmId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchConfirmRequest {
    pub IsAccept: bool,
}

impl MatchConfirmRequest {
    pub const MSG_ID: u16 = 17484;
}

impl PbMessage for MatchConfirmRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsAccept {
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
            8 => self.IsAccept = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchConfirmResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl MatchConfirmResponse {
    pub const MSG_ID: u16 = 15699;
}

impl PbMessage for MatchConfirmResponse {
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
pub struct MatchFailNotify {
    pub Reason: Option<MatchFailReason>,
}

impl MatchFailNotify {
    pub const MSG_ID: u16 = 23326;
}

impl PbMessage for MatchFailNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Reason {
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
                self.Reason = Some(
                    <MatchFailReason as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct MatchPlayerInfo {
    pub PlayerId: i32,
    pub RoleInfo: Vec<MatchRoleInfo>,
    pub PlayerName: Vec<u8>,
    pub IsConfirm: bool,
    pub IsReady: bool,
    pub MatchUiState: Option<EMatchPlayerUiState>,
    pub PsnOnlineId: Vec<u8>,
    pub AbyssLikeCount: i32,
    pub XboxOnlineId: Vec<u8>,
    pub XboxUserId: i32,
}

impl PbMessage for MatchPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.RoleInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        if !self.PlayerName.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PlayerName);
        }
        if self.IsConfirm {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsReady {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.MatchUiState {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.PsnOnlineId.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnOnlineId);
        }
        if self.AbyssLikeCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AbyssLikeCount);
        }
        if !self.XboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxOnlineId);
        }
        if self.XboxUserId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.XboxUserId);
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
                self.RoleInfo
                    .push(<MatchRoleInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.PlayerName = r.read_len_delimited()?.to_vec(),
            4 => self.IsConfirm = r.read_bool()?,
            5 => self.IsReady = r.read_bool()?,
            6 => {
                self.MatchUiState = Some(
                    <EMatchPlayerUiState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.PsnOnlineId = r.read_len_delimited()?.to_vec(),
            8 => self.AbyssLikeCount = r.read_varint_i32()?,
            9 => self.XboxOnlineId = r.read_len_delimited()?.to_vec(),
            10 => self.XboxUserId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchResult {
    pub ActivityId: i32,
    pub LegMatchId: i32,
    pub DangoRank: Vec<i32>,
    pub PromoteNum: i32,
}

impl PbMessage for MatchResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LegMatchId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LegMatchId);
        }
        for item in &self.DangoRank {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PromoteNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PromoteNum);
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
            2 => self.LegMatchId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DangoRank, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.PromoteNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchRoleContext {
    pub AbyssCtx: Option<AbyssMatchRoleContext>,
}

impl PbMessage for MatchRoleContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AbyssCtx {
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
                self.AbyssCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MatchRoleInfo {
    pub RoleId: i32,
    pub RoleLevel: i32,
    pub TowerDefencePhantomId: i32,
    pub RoleSkinId: i32,
    pub SkillBranchId: i32,
    pub RoleContext: Option<MatchRoleContext>,
    pub WeaponId: i32,
    pub WeaponBreachLevel: i32,
    pub WeaponSkinId: i32,
    pub DressList: Vec<i32>,
}

impl PbMessage for MatchRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.TowerDefencePhantomId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerDefencePhantomId);
        }
        if self.RoleSkinId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinId);
        }
        if self.SkillBranchId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillBranchId);
        }
        if let Some(v) = &self.RoleContext {
            crate::wire::write_submessage(out, 6, v);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.RoleLevel = r.read_varint_i32()?,
            3 => self.TowerDefencePhantomId = r.read_varint_i32()?,
            4 => self.RoleSkinId = r.read_varint_i32()?,
            5 => self.SkillBranchId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RoleContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct MatchTeamInfo {
    pub HostId: i32,
    pub PlayerInfos: Vec<MatchPlayerInfo>,
    pub TeamState: Option<MatchTeamState>,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
}

impl PbMessage for MatchTeamInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HostId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HostId);
        }
        for item in &self.PlayerInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.TeamState {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Rotation {
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
            1 => self.HostId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerInfos
                    .push(<MatchPlayerInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                self.TeamState = Some(
                    <MatchTeamState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
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
pub struct MatchTeamNotify {
    pub TeamInfo: Option<MatchTeamInfo>,
}

impl MatchTeamNotify {
    pub const MSG_ID: u16 = 25195;
}

impl PbMessage for MatchTeamNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TeamInfo {
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
                self.TeamInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
