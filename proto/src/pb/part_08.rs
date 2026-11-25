use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingNetCabinUpdateNotify {
    pub NetCabinItems: Vec<FishingItemInfo>,
}

impl FishingNetCabinUpdateNotify {
    pub const MSG_ID: u16 = 20945;
}

impl PbMessage for FishingNetCabinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NetCabinItems {
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
                self.NetCabinItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingNoticeUpdateNotify {
    pub CompleteNoticeIds: Vec<i32>,
    pub NoticeIds: Vec<i32>,
}

impl FishingNoticeUpdateNotify {
    pub const MSG_ID: u16 = 15366;
}

impl PbMessage for FishingNoticeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CompleteNoticeIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.NoticeIds {
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
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CompleteNoticeIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.NoticeIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingPointInfoRequest {
    pub Id: i32,
}

impl FishingPointInfoRequest {
    pub const MSG_ID: u16 = 22194;
}

impl PbMessage for FishingPointInfoRequest {
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
pub struct FishingPointInfoResponse {
    pub FishPointInfo: Option<OneFishPointInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingPointInfoResponse {
    pub const MSG_ID: u16 = 19378;
}

impl PbMessage for FishingPointInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishPointInfo {
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
                self.FishPointInfo
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
pub struct FishingPortUnlockNotify {
    pub UnlockPortId: Vec<i32>,
}

impl FishingPortUnlockNotify {
    pub const MSG_ID: u16 = 23503;
}

impl PbMessage for FishingPortUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockPortId {
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
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockPortId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingPortUpdateNotify {
    pub PortId: i32,
    pub IsInPort: bool,
    pub LastPortId: i32,
}

impl FishingPortUpdateNotify {
    pub const MSG_ID: u16 = 17589;
}

impl PbMessage for FishingPortUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PortId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PortId);
        }
        if self.IsInPort {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LastPortId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            4 => self.PortId = r.read_varint_i32()?,
            8 => self.IsInPort = r.read_bool()?,
            10 => self.LastPortId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingQuickSellPreviewRequest {}

impl FishingQuickSellPreviewRequest {
    pub const MSG_ID: u16 = 29724;
}

impl PbMessage for FishingQuickSellPreviewRequest {
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
pub struct FishingQuickSellPreviewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RewardItems: BTreeMap<i32, i32>,
    pub SellItems: Vec<FishingItemInfo>,
}

impl FishingQuickSellPreviewResponse {
    pub const MSG_ID: u16 = 25935;
}

impl PbMessage for FishingQuickSellPreviewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 8, &self.RewardItems);
        }
        for item in &self.SellItems {
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
            8 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SellItems
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSailingRequest {
    pub Fix: bool,
    pub PortId: i32,
}

impl FishingSailingRequest {
    pub const MSG_ID: u16 = 21616;
}

impl PbMessage for FishingSailingRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Fix {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PortId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PortId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Fix = r.read_bool()?,
            4 => self.PortId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSailingResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingSailingResponse {
    pub const MSG_ID: u16 = 20883;
}

impl PbMessage for FishingSailingResponse {
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
pub struct FishingSceneFishCageInfoUpdateNotify {
    pub SceneConfigId: i32,
    pub Cage: Option<OneFishCage>,
}

impl FishingSceneFishCageInfoUpdateNotify {
    pub const MSG_ID: u16 = 25515;
}

impl PbMessage for FishingSceneFishCageInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SceneConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneConfigId);
        }
        if let Some(v) = &self.Cage {
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
            11 => self.SceneConfigId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Cage
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSellPreviewRequest {
    pub IncrIds: Vec<i32>,
}

impl FishingSellPreviewRequest {
    pub const MSG_ID: u16 = 25932;
}

impl PbMessage for FishingSellPreviewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.IncrIds {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncrIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSellPreviewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RewardItems: BTreeMap<i32, i32>,
}

impl FishingSellPreviewResponse {
    pub const MSG_ID: u16 = 23795;
}

impl PbMessage for FishingSellPreviewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.RewardItems.is_empty() {
            crate::wire::write_map(out, 15, &self.RewardItems);
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
            15 => crate::wire::decode_map_entry(r, &mut self.RewardItems)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingShipFixRequest {}

impl FishingShipFixRequest {
    pub const MSG_ID: u16 = 15255;
}

impl PbMessage for FishingShipFixRequest {
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
pub struct FishingShipFixResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingShipFixResponse {
    pub const MSG_ID: u16 = 21934;
}

impl PbMessage for FishingShipFixResponse {
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
pub struct FishingShipForceBackNotify {
    pub ShipInfo: Option<FishingShipInfo>,
}

impl FishingShipForceBackNotify {
    pub const MSG_ID: u16 = 28002;
}

impl PbMessage for FishingShipForceBackNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ShipInfo {
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
                self.ShipInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingShipInfoUpdateNotify {
    pub ShipInfo: Option<FishingShipInfo>,
}

impl FishingShipInfoUpdateNotify {
    pub const MSG_ID: u16 = 16082;
}

impl PbMessage for FishingShipInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ShipInfo {
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
                self.ShipInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingShipSkinChangeRequest {
    pub SkinId: i32,
    pub PortId: i32,
}

impl FishingShipSkinChangeRequest {
    pub const MSG_ID: u16 = 17127;
}

impl PbMessage for FishingShipSkinChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.PortId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PortId);
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
            5 => self.PortId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingShipSkinChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingShipSkinChangeResponse {
    pub const MSG_ID: u16 = 21884;
}

impl PbMessage for FishingShipSkinChangeResponse {
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
pub struct FishingSkillBaitRequest {
    pub ItemId: i32,
}

impl FishingSkillBaitRequest {
    pub const MSG_ID: u16 = 28647;
}

impl PbMessage for FishingSkillBaitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSkillBaitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingSkillBaitResponse {
    pub const MSG_ID: u16 = 27026;
}

impl PbMessage for FishingSkillBaitResponse {
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
pub struct FishingSkillBombRequest {
    pub ItemId: i32,
}

impl FishingSkillBombRequest {
    pub const MSG_ID: u16 = 23767;
}

impl PbMessage for FishingSkillBombRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSkillBombResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Items: Vec<FishingItemInfo>,
}

impl FishingSkillBombResponse {
    pub const MSG_ID: u16 = 16903;
}

impl PbMessage for FishingSkillBombResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Items {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<FishingItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingSkillTeleportRequest {}

impl FishingSkillTeleportRequest {
    pub const MSG_ID: u16 = 18398;
}

impl PbMessage for FishingSkillTeleportRequest {
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
pub struct FishingSkillTeleportResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingSkillTeleportResponse {
    pub const MSG_ID: u16 = 18913;
}

impl PbMessage for FishingSkillTeleportResponse {
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
pub struct FishingSkinUnlockNotify {
    pub UnlockSkins: Vec<i32>,
}

impl FishingSkinUnlockNotify {
    pub const MSG_ID: u16 = 18702;
}

impl PbMessage for FishingSkinUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockSkins {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockSkins, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingTechInfoUpdateNotify {
    pub TechInfo: Option<FishingTechInfo>,
}

impl FishingTechInfoUpdateNotify {
    pub const MSG_ID: u16 = 29078;
}

impl PbMessage for FishingTechInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TechInfo {
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
                self.TechInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FishingTeleportToBoatRequest {}

impl FishingTeleportToBoatRequest {
    pub const MSG_ID: u16 = 23911;
}

impl PbMessage for FishingTeleportToBoatRequest {
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
pub struct FishingTeleportToBoatResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FishingTeleportToBoatResponse {
    pub const MSG_ID: u16 = 21519;
}

impl PbMessage for FishingTeleportToBoatResponse {
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
pub struct FixToolRequest {
    pub EntityId: i64,
    pub FixTool: i32,
}

impl FixToolRequest {
    pub const MSG_ID: u16 = 16354;
}

impl PbMessage for FixToolRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.FixTool != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FixTool);
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
            12 => self.FixTool = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FixToolResponse {
    pub Code: Option<ErrorCode>,
    pub EntityId: i64,
}

impl FixToolResponse {
    pub const MSG_ID: u16 = 20809;
}

impl PbMessage for FixToolResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            9 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeBoxExpNotify {
    pub BoxExp: i32,
    pub ChallengeLevelId: i32,
}

impl FlagChallengeBoxExpNotify {
    pub const MSG_ID: u16 = 22380;
}

impl PbMessage for FlagChallengeBoxExpNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BoxExp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxExp);
        }
        if self.ChallengeLevelId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.BoxExp = r.read_varint_i32()?,
            6 => self.ChallengeLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeCtx {
    pub FlagStrongholdId: i32,
}

impl PbMessage for FlagChallengeCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlagStrongholdId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlagStrongholdId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FlagStrongholdId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeInstNotify {
    pub ChallengeLevelId: i32,
    pub FlagChallengePerRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
    pub BoxExp: i32,
    pub FlagChallengeTempRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
}

impl FlagChallengeInstNotify {
    pub const MSG_ID: u16 = 28262;
}

impl PbMessage for FlagChallengeInstNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeLevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeLevelId);
        }
        if let Some(v) = &self.FlagChallengePerRoleLevelInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.BoxExp != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxExp);
        }
        if let Some(v) = &self.FlagChallengeTempRoleLevelInfo {
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
            2 => self.ChallengeLevelId = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengePerRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.BoxExp = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengeTempRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeLevelInfoUpdateNotify {
    pub FlagChallengeLevelInfo: Vec<FlagChallengeLevelInfo>,
}

impl FlagChallengeLevelInfoUpdateNotify {
    pub const MSG_ID: u16 = 27954;
}

impl PbMessage for FlagChallengeLevelInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlagChallengeLevelInfo {
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
                self.FlagChallengeLevelInfo
                    .push(<FlagChallengeLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengePerRoleLevelNotify {
    pub ActivityId: i32,
    pub FlagChallengeRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
}

impl FlagChallengePerRoleLevelNotify {
    pub const MSG_ID: u16 = 25423;
}

impl PbMessage for FlagChallengePerRoleLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.FlagChallengeRoleLevelInfo {
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
            11 => self.ActivityId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengeRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeRewardRequest {
    pub FlagChallengeTaskIds: Vec<i32>,
    pub ActivityId: i32,
}

impl FlagChallengeRewardRequest {
    pub const MSG_ID: u16 = 19079;
}

impl PbMessage for FlagChallengeRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlagChallengeTaskIds {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FlagChallengeTaskIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlagChallengeRewardResponse {
    pub const MSG_ID: u16 = 20586;
}

impl PbMessage for FlagChallengeRewardResponse {
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
pub struct FlagChallengeSettleNotify {
    pub ChallengeLevelId: i32,
    pub FlagChallengePerRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
}

impl FlagChallengeSettleNotify {
    pub const MSG_ID: u16 = 26883;
}

impl PbMessage for FlagChallengeSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeLevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeLevelId);
        }
        if let Some(v) = &self.FlagChallengePerRoleLevelInfo {
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
            2 => self.ChallengeLevelId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.FlagChallengePerRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeTaskInfoUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl FlagChallengeTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 28118;
}

impl PbMessage for FlagChallengeTaskInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeTempRoleLevelNotify {
    pub FlagChallengeRoleLevelInfo: Option<FlagChallengeRoleLevelInfo>,
    pub ChallengeLevelId: i32,
}

impl FlagChallengeTempRoleLevelNotify {
    pub const MSG_ID: u16 = 20785;
}

impl PbMessage for FlagChallengeTempRoleLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FlagChallengeRoleLevelInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.ChallengeLevelId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeLevelId);
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
                self.FlagChallengeRoleLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.ChallengeLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagChallengeUnlockTeleporterNotify {
    pub TeleporterId: Vec<i32>,
}

impl FlagChallengeUnlockTeleporterNotify {
    pub const MSG_ID: u16 = 27289;
}

impl PbMessage for FlagChallengeUnlockTeleporterNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TeleporterId {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TeleporterId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagStrongholdInfoUpdateNotify {
    pub FlagStrongholdInfo: Vec<FlagStrongholdInfo>,
}

impl FlagStrongholdInfoUpdateNotify {
    pub const MSG_ID: u16 = 22966;
}

impl PbMessage for FlagStrongholdInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FlagStrongholdInfo {
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
                self.FlagStrongholdInfo
                    .push(<FlagStrongholdInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlagToBoxInfo {
    pub FlagId: i32,
    pub BoxGetCount: i32,
    pub TotalCount: i32,
}

impl PbMessage for FlagToBoxInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlagId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlagId);
        }
        if self.BoxGetCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BoxGetCount);
        }
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.FlagId = r.read_varint_i32()?,
            2 => self.BoxGetCount = r.read_varint_i32()?,
            3 => self.TotalCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloorSettingsPb {
    pub MeshPath: Vec<u8>,
    pub MaterialPath: Vec<u8>,
    pub ScaleX: f32,
    pub ScaleY: f32,
    pub ShowTime: f32,
    pub DisappearTime: f32,
}

impl PbMessage for FloorSettingsPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.MeshPath.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MeshPath);
        }
        if !self.MaterialPath.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MaterialPath);
        }
        if self.ScaleX != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ScaleX).to_bits().to_le_bytes());
        }
        if self.ScaleY != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ScaleY).to_bits().to_le_bytes());
        }
        if self.ShowTime != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ShowTime).to_bits().to_le_bytes());
        }
        if self.DisappearTime != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DisappearTime).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MeshPath = r.read_len_delimited()?.to_vec(),
            2 => self.MaterialPath = r.read_len_delimited()?.to_vec(),
            3 => self.ScaleX = r.read_f32()?,
            4 => self.ScaleY = r.read_f32()?,
            5 => self.ShowTime = r.read_f32()?,
            6 => self.DisappearTime = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchAnimationRequest {
    pub ActivityId: i32,
}

impl FloroRanchAnimationRequest {
    pub const MSG_ID: u16 = 25219;
}

impl PbMessage for FloroRanchAnimationRequest {
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
pub struct FloroRanchAnimationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchAnimationResponse {
    pub const MSG_ID: u16 = 22651;
}

impl PbMessage for FloroRanchAnimationResponse {
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
pub struct FloroRanchCloseTaskRequest {
    pub SubInsId: i32,
    pub TaskIncId: i32,
    pub ActivityId: i32,
}

impl FloroRanchCloseTaskRequest {
    pub const MSG_ID: u16 = 20552;
}

impl PbMessage for FloroRanchCloseTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.TaskIncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskIncId);
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
            1 => self.SubInsId = r.read_varint_i32()?,
            10 => self.TaskIncId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchCloseTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchCloseTaskResponse {
    pub const MSG_ID: u16 = 19666;
}

impl PbMessage for FloroRanchCloseTaskResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Task {
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
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchDayStart {
    pub OnStageUnit: Vec<FloroRanchPlayUnit>,
    pub CurDay: i32,
    pub CurStage: i32,
    pub IsUnlimited: bool,
    pub RemoveUintIncId: Vec<i32>,
    pub TotalDay: i32,
    pub RemainDay: i32,
}

impl PbMessage for FloroRanchDayStart {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.OnStageUnit {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.CurDay != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurDay);
        }
        if self.CurStage != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStage);
        }
        if self.IsUnlimited {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RemoveUintIncId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.TotalDay != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDay);
        }
        if self.RemainDay != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainDay);
        }
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
                self.OnStageUnit
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            2 => self.CurDay = r.read_varint_i32()?,
            3 => self.CurStage = r.read_varint_i32()?,
            4 => self.IsUnlimited = r.read_bool()?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RemoveUintIncId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => self.TotalDay = r.read_varint_i32()?,
            7 => self.RemainDay = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchEvent {
    pub EventId: i32,
    pub Choice: Vec<i32>,
}

impl PbMessage for FloroRanchEvent {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
        }
        for item in &self.Choice {
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
            1 => self.EventId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Choice, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchEventChoiceRequest {
    pub SubInsId: i32,
    pub IncId: i32,
    pub Choice: i32,
    pub ActivityId: i32,
}

impl FloroRanchEventChoiceRequest {
    pub const MSG_ID: u16 = 25648;
}

impl PbMessage for FloroRanchEventChoiceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Choice != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Choice);
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
            1 => self.SubInsId = r.read_varint_i32()?,
            10 => self.IncId = r.read_varint_i32()?,
            12 => self.Choice = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchEventChoiceResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchEventChoiceResponse {
    pub const MSG_ID: u16 = 29916;
}

impl PbMessage for FloroRanchEventChoiceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Task {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchExecuteSkillPlayRequest {
    pub IncId: i32,
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchExecuteSkillPlayRequest {
    pub const MSG_ID: u16 = 29254;
}

impl PbMessage for FloroRanchExecuteSkillPlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            10 => self.ActivityId = r.read_varint_i32()?,
            15 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchExecuteSkillPlayResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchExecuteSkillPlayResponse {
    pub const MSG_ID: u16 = 28746;
}

impl PbMessage for FloroRanchExecuteSkillPlayResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Task {
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
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchFirstEnterRequest {
    pub SubInsId: i32,
    pub ActivityId: i32,
}

impl FloroRanchFirstEnterRequest {
    pub const MSG_ID: u16 = 27487;
}

impl PbMessage for FloroRanchFirstEnterRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            10 => self.SubInsId = r.read_varint_i32()?,
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchFirstEnterResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchFirstEnterResponse {
    pub const MSG_ID: u16 = 27776;
}

impl PbMessage for FloroRanchFirstEnterResponse {
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
pub struct FloroRanchGacha {
    pub CardIds: Vec<i32>,
    pub AllowedRefresh: bool,
    pub Cost: Vec<FloroRanchRefreshCost>,
    pub IncId: i32,
    pub FreeTimes: i32,
}

impl PbMessage for FloroRanchGacha {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CardIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.AllowedRefresh {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Cost {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.FreeTimes != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FreeTimes);
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
            2 => self.AllowedRefresh = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Cost
                    .push(<FloroRanchRefreshCost as PbMessage>::decode(bytes)?);
            }
            4 => self.IncId = r.read_varint_i32()?,
            5 => self.FreeTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchOutRequest {
    pub SubInsId: i32,
    pub ActivityId: i32,
}

impl FloroRanchOutRequest {
    pub const MSG_ID: u16 = 23261;
}

impl PbMessage for FloroRanchOutRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            3 => self.SubInsId = r.read_varint_i32()?,
            4 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchOutResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchOutResponse {
    pub const MSG_ID: u16 = 25979;
}

impl PbMessage for FloroRanchOutResponse {
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
pub struct FloroRanchPlayBuff {
    pub PlayIncId: i32,
    pub Id: i32,
    pub Point: i32,
    pub Day: i32,
    pub Plies: i32,
}

impl PbMessage for FloroRanchPlayBuff {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayIncId);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Point);
        }
        if self.Day != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Day);
        }
        if self.Plies != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Plies);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PlayIncId = r.read_varint_i32()?,
            2 => self.Id = r.read_varint_i32()?,
            3 => self.Point = r.read_varint_i32()?,
            4 => self.Day = r.read_varint_i32()?,
            5 => self.Plies = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayEvent {
    pub IncId: i32,
    pub EventId: i32,
    pub Choice: Vec<i32>,
}

impl PbMessage for FloroRanchPlayEvent {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.EventId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
        }
        for item in &self.Choice {
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
            1 => self.IncId = r.read_varint_i32()?,
            2 => self.EventId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Choice, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayEventChoiceRequest {
    pub ChoiceId: i32,
    pub SubInsId: i32,
    pub ActivityId: i32,
    pub EventId: i32,
}

impl FloroRanchPlayEventChoiceRequest {
    pub const MSG_ID: u16 = 22212;
}

impl PbMessage for FloroRanchPlayEventChoiceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChoiceId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChoiceId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.EventId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
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
            2 => self.SubInsId = r.read_varint_i32()?,
            9 => self.ActivityId = r.read_varint_i32()?,
            14 => self.EventId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayEventChoiceResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Update: Option<FloroRanchPlayInfo>,
}

impl FloroRanchPlayEventChoiceResponse {
    pub const MSG_ID: u16 = 21433;
}

impl PbMessage for FloroRanchPlayEventChoiceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Update {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Update
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayGachaRefreshRequest {
    pub IncId: i32,
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchPlayGachaRefreshRequest {
    pub const MSG_ID: u16 = 17012;
}

impl PbMessage for FloroRanchPlayGachaRefreshRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            14 => self.ActivityId = r.read_varint_i32()?,
            15 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayGachaRefreshResponse {
    pub CurDiamond: i64,
    pub CurCoin: i64,
    pub ErrorCode: Option<ErrorCode>,
    pub Task: Option<FloroRanchGacha>,
}

impl FloroRanchPlayGachaRefreshResponse {
    pub const MSG_ID: u16 = 18822;
}

impl PbMessage for FloroRanchPlayGachaRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            6 => self.CurDiamond = r.read_varint_i64()?,
            9 => self.CurCoin = r.read_varint_i64()?,
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
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
pub struct FloroRanchPlayGachaRequest {
    pub ActivityId: i32,
    pub SubInsId: i32,
    pub CardId: i32,
    pub IncId: i32,
}

impl FloroRanchPlayGachaRequest {
    pub const MSG_ID: u16 = 18280;
}

impl PbMessage for FloroRanchPlayGachaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.CardId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardId);
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
            5 => self.ActivityId = r.read_varint_i32()?,
            7 => self.SubInsId = r.read_varint_i32()?,
            10 => self.CardId = r.read_varint_i32()?,
            11 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayGachaResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GachaResult: Vec<FloroRanchPlayUnit>,
    pub Unit: Option<FloroRanchPlayUnit>,
}

impl FloroRanchPlayGachaResponse {
    pub const MSG_ID: u16 = 26388;
}

impl PbMessage for FloroRanchPlayGachaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GachaResult {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.Unit {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GachaResult
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Unit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayGmInfoUpdateNotify {
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchPlayGmInfoUpdateNotify {
    pub const MSG_ID: u16 = 26408;
}

impl PbMessage for FloroRanchPlayGmInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
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
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayInfo {
    pub SubInsId: i32,
    pub Terrains: Vec<FloroRanchPlayUnit>,
    pub PhantomList: Vec<FloroRanchPlayUnit>,
    pub ToyList: Vec<FloroRanchPlayUnit>,
    pub TotalCoin: i64,
    pub TotalDiamond: i64,
    pub CurCoin: i64,
    pub CurDiamond: i64,
    pub SkillId: i32,
    pub Stage: i32,
    pub Day: i32,
    pub StageTributeCount: i64,
    pub EnableToy: i32,
    pub Dungeon: Option<FloroRanchPlayUnit>,
    pub Role: Option<FloroRanchPlayUnit>,
    pub IsUnLimited: bool,
    pub CurStageDayNum: i32,
    pub TotalDay: i32,
    pub RemainDay: i32,
    pub Races: Vec<i32>,
    pub IsOver: bool,
}

impl PbMessage for FloroRanchPlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        for item in &self.Terrains {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.PhantomList {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.ToyList {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.TotalCoin != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCoin);
        }
        if self.TotalDiamond != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalDiamond);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
        }
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        if self.Stage != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Stage);
        }
        if self.Day != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Day);
        }
        if self.StageTributeCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StageTributeCount);
        }
        if self.EnableToy != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EnableToy);
        }
        if let Some(v) = &self.Dungeon {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.Role {
            crate::wire::write_submessage(out, 15, v);
        }
        if self.IsUnLimited {
            crate::wire::write_tag(out, 16, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurStageDayNum != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStageDayNum);
        }
        if self.TotalDay != 0 {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDay);
        }
        if self.RemainDay != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainDay);
        }
        for item in &self.Races {
            crate::wire::write_tag(out, 20, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsOver {
            crate::wire::write_tag(out, 21, WireType::Varint);
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
            1 => self.SubInsId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Terrains
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomList
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ToyList
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
            }
            5 => self.TotalCoin = r.read_varint_i64()?,
            6 => self.TotalDiamond = r.read_varint_i64()?,
            7 => self.CurCoin = r.read_varint_i64()?,
            8 => self.CurDiamond = r.read_varint_i64()?,
            9 => self.SkillId = r.read_varint_i32()?,
            10 => self.Stage = r.read_varint_i32()?,
            11 => self.Day = r.read_varint_i32()?,
            12 => self.StageTributeCount = r.read_varint_i64()?,
            13 => self.EnableToy = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Dungeon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Role
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            16 => self.IsUnLimited = r.read_bool()?,
            17 => self.CurStageDayNum = r.read_varint_i32()?,
            18 => self.TotalDay = r.read_varint_i32()?,
            19 => self.RemainDay = r.read_varint_i32()?,
            20 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Races, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            21 => self.IsOver = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayNextDayRequest {
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchPlayNextDayRequest {
    pub const MSG_ID: u16 = 20630;
}

impl PbMessage for FloroRanchPlayNextDayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            4 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayNextDayResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchPlayNextDayResponse {
    pub const MSG_ID: u16 = 26348;
}

impl PbMessage for FloroRanchPlayNextDayResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRefreshGachaRequest {
    pub SubInsId: i32,
    pub IncId: i32,
    pub ActivityId: i32,
}

impl FloroRanchPlayRefreshGachaRequest {
    pub const MSG_ID: u16 = 23703;
}

impl PbMessage for FloroRanchPlayRefreshGachaRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
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
            2 => self.SubInsId = r.read_varint_i32()?,
            7 => self.IncId = r.read_varint_i32()?,
            12 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRefreshGachaResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Gacha: Option<FloroRanchGacha>,
}

impl FloroRanchPlayRefreshGachaResponse {
    pub const MSG_ID: u16 = 29452;
}

impl PbMessage for FloroRanchPlayRefreshGachaResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Gacha {
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
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.Gacha
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRefreshShopRequest {
    pub IncId: i32,
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchPlayRefreshShopRequest {
    pub const MSG_ID: u16 = 28500;
}

impl PbMessage for FloroRanchPlayRefreshShopRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            9 => self.ActivityId = r.read_varint_i32()?,
            15 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRefreshShopResponse {
    pub Shop: Option<FloroRanchShop>,
    pub ErrorCode: Option<ErrorCode>,
    pub CurDiamond: i64,
    pub CurCoin: i64,
}

impl FloroRanchPlayRefreshShopResponse {
    pub const MSG_ID: u16 = 18191;
}

impl PbMessage for FloroRanchPlayRefreshShopResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
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
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.CurDiamond = r.read_varint_i64()?,
            13 => self.CurCoin = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRemoveUnitRequest {
    pub ActivityId: i32,
    pub SubInsId: i32,
    pub UnitIncId: i32,
}

impl FloroRanchPlayRemoveUnitRequest {
    pub const MSG_ID: u16 = 24002;
}

impl PbMessage for FloroRanchPlayRemoveUnitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.UnitIncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnitIncId);
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
            5 => self.SubInsId = r.read_varint_i32()?,
            10 => self.UnitIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayRemoveUnitResponse {
    pub Task: Vec<FloroRanchPlayTask>,
    pub ErrorCode: Option<ErrorCode>,
    pub CurDiamond: i64,
    pub CurCoin: i64,
}

impl FloroRanchPlayRemoveUnitResponse {
    pub const MSG_ID: u16 = 16556;
}

impl PbMessage for FloroRanchPlayRemoveUnitResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
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
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.CurDiamond = r.read_varint_i64()?,
            15 => self.CurCoin = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlaySelectCardGroupRequest {
    pub SubInsId: i32,
    pub CardId: Vec<i32>,
    pub ActivityId: i32,
}

impl FloroRanchPlaySelectCardGroupRequest {
    pub const MSG_ID: u16 = 20063;
}

impl PbMessage for FloroRanchPlaySelectCardGroupRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        for item in &self.CardId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.SubInsId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlaySelectCardGroupResponse {
    pub NewPhantoms: Vec<FloroRanchPlayUnit>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchPlaySelectCardGroupResponse {
    pub const MSG_ID: u16 = 19257;
}

impl PbMessage for FloroRanchPlaySelectCardGroupResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.NewPhantoms {
            crate::wire::write_submessage(out, 4, item);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.NewPhantoms
                    .push(<FloroRanchPlayUnit as PbMessage>::decode(bytes)?);
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
pub struct FloroRanchPlaySettleData {
    pub TechNum: i32,
    pub AllowedTechUp: bool,
    pub TotalDays: i32,
    pub IsNewMaxDays: bool,
    pub TotalCoin: i64,
    pub IsNewMaxCoin: bool,
    pub MaxDayCoin: i64,
    pub ChampionConfigId: i32,
    pub ChampionTotalCoinNum: i64,
    pub Win: bool,
    pub IsUnlimited: bool,
    pub EnableUnlimitedMode: bool,
    pub Reward: Vec<FloroRanchSubInsSettleReward>,
    pub ForceSettleTextKey: Vec<u8>,
    pub CurWeeklyScoreNum: i32,
    pub RewardWeeklyScoreNum: i32,
}

impl PbMessage for FloroRanchPlaySettleData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TechNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TechNum);
        }
        if self.AllowedTechUp {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TotalDays != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDays);
        }
        if self.IsNewMaxDays {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TotalCoin != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCoin);
        }
        if self.IsNewMaxCoin {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxDayCoin != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxDayCoin);
        }
        if self.ChampionConfigId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChampionConfigId);
        }
        if self.ChampionTotalCoinNum != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ChampionTotalCoinNum);
        }
        if self.Win {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsUnlimited {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.EnableUnlimitedMode {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Reward {
            crate::wire::write_submessage(out, 13, item);
        }
        if !self.ForceSettleTextKey.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ForceSettleTextKey);
        }
        if self.CurWeeklyScoreNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWeeklyScoreNum);
        }
        if self.RewardWeeklyScoreNum != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardWeeklyScoreNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TechNum = r.read_varint_i32()?,
            2 => self.AllowedTechUp = r.read_bool()?,
            3 => self.TotalDays = r.read_varint_i32()?,
            4 => self.IsNewMaxDays = r.read_bool()?,
            5 => self.TotalCoin = r.read_varint_i64()?,
            6 => self.IsNewMaxCoin = r.read_bool()?,
            7 => self.MaxDayCoin = r.read_varint_i64()?,
            8 => self.ChampionConfigId = r.read_varint_i32()?,
            9 => self.ChampionTotalCoinNum = r.read_varint_i64()?,
            10 => self.Win = r.read_bool()?,
            11 => self.IsUnlimited = r.read_bool()?,
            12 => self.EnableUnlimitedMode = r.read_bool()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .push(<FloroRanchSubInsSettleReward as PbMessage>::decode(bytes)?);
            }
            14 => self.ForceSettleTextKey = r.read_len_delimited()?.to_vec(),
            15 => self.CurWeeklyScoreNum = r.read_varint_i32()?,
            16 => self.RewardWeeklyScoreNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayShopBuyCard {
    pub NewPhantom: Option<FloroRanchPlayUnit>,
}

impl PbMessage for FloroRanchPlayShopBuyCard {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewPhantom {
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
                self.NewPhantom
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayShopBuyCardGroup {
    pub CardIds: Vec<i32>,
}

impl PbMessage for FloroRanchPlayShopBuyCardGroup {
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
pub struct FloroRanchPlayShopBuyRequest {
    pub IncId: i32,
    pub BuyId: i32,
    pub ActivityId: i32,
    pub ItemIncId: i32,
    pub SubInsId: i32,
    pub Type: Option<FloroRanchShopItemType>,
}

impl FloroRanchPlayShopBuyRequest {
    pub const MSG_ID: u16 = 23342;
}

impl PbMessage for FloroRanchPlayShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.BuyId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuyId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ItemIncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemIncId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if let Some(v) = &self.Type {
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
            1 => self.IncId = r.read_varint_i32()?,
            4 => self.BuyId = r.read_varint_i32()?,
            8 => self.ActivityId = r.read_varint_i32()?,
            9 => self.ItemIncId = r.read_varint_i32()?,
            11 => self.SubInsId = r.read_varint_i32()?,
            13 => {
                self.Type = Some(
                    <FloroRanchShopItemType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FloroRanchPlayShopBuyResponse {
    pub Type: Option<FloroRanchPlayShopBuyType>,
    pub Card: Option<FloroRanchPlayShopBuyCard>,
    pub Shop: Option<FloroRanchShop>,
    pub CardGroup: Option<FloroRanchPlayShopBuyCardGroup>,
    pub CurCoin: i64,
    pub CurDiamond: i64,
    pub Toy: Option<FloroRanchPlayShopBuyToy>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchPlayShopBuyResponse {
    pub const MSG_ID: u16 = 15480;
}

impl PbMessage for FloroRanchPlayShopBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Card {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.CardGroup {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
        }
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
        if let Some(v) = &self.Toy {
            crate::wire::write_submessage(out, 10, v);
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
                self.Type = Some(
                    <FloroRanchPlayShopBuyType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Card
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.CardGroup
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.CurCoin = r.read_varint_i64()?,
            9 => self.CurDiamond = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Toy
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
pub struct FloroRanchPlayShopBuyToy {
    pub NewToy: Option<FloroRanchPlayUnit>,
}

impl PbMessage for FloroRanchPlayShopBuyToy {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewToy {
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
                self.NewToy
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayTask {
    pub TaskType: Option<FloroRanchPlayTaskType>,
    pub DayStart: Option<FloroRanchDayStart>,
    pub Event: Option<FloroRanchEvent>,
    pub Shop: Option<FloroRanchShop>,
    pub Gacha: Option<FloroRanchGacha>,
    pub StageStart: Option<FloroRanchStageStart>,
    pub StageEnd: Option<FloroRanchStageEnd>,
    pub UnitActions: Option<FRUnitActions>,
    pub WageSettleTask: Option<FloroRanchWageSettleTask>,
    pub FloroRanchPlayEvent: Option<FloroRanchPlayEvent>,
}

impl PbMessage for FloroRanchPlayTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TaskType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.DayStart {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Event {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Shop {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.Gacha {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.StageStart {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.StageEnd {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.UnitActions {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.WageSettleTask {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.FloroRanchPlayEvent {
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
                self.TaskType = Some(
                    <FloroRanchPlayTaskType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.DayStart
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Event
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Shop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Gacha
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.StageStart
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.StageEnd
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.UnitActions
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.WageSettleTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.FloroRanchPlayEvent
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayTributeRequest {
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchPlayTributeRequest {
    pub const MSG_ID: u16 = 21163;
}

impl PbMessage for FloroRanchPlayTributeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            11 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayTributeResponse {
    pub Settle: bool,
    pub Task: Vec<FloroRanchPlayTask>,
    pub CurCoin: i64,
    pub ErrorCode: Option<ErrorCode>,
    pub RemainDay: i32,
    pub CurStage: i32,
    pub CurDiamond: i64,
}

impl FloroRanchPlayTributeResponse {
    pub const MSG_ID: u16 = 27951;
}

impl PbMessage for FloroRanchPlayTributeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Settle {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Task {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.CurCoin != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurCoin);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RemainDay != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainDay);
        }
        if self.CurStage != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStage);
        }
        if self.CurDiamond != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurDiamond);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Settle = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            7 => self.CurCoin = r.read_varint_i64()?,
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.RemainDay = r.read_varint_i32()?,
            14 => self.CurStage = r.read_varint_i32()?,
            15 => self.CurDiamond = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayUnit {
    pub PlayIncId: i32,
    pub Type: Option<FloroRanchActionUnitType>,
    pub Id: i32,
    pub Point: i32,
    pub StartStage: i32,
    pub StartDay: i32,
    pub TagId: i32,
    pub BuffList: Vec<FloroRanchPlayBuff>,
    pub Num: i32,
    pub ExtraSalary: i64,
    pub RemoveCost: i32,
    pub EvolveData: Option<FREvolveData>,
    pub SKillData: Option<FRSkillData>,
    pub Salary: i64,
    pub ToyLv: i64,
    pub SelfDefineValue: Option<FRActionSelfDefineValue>,
}

impl PbMessage for FloroRanchPlayUnit {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayIncId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Point != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Point);
        }
        if self.StartStage != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartStage);
        }
        if self.StartDay != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartDay);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagId);
        }
        for item in &self.BuffList {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.ExtraSalary != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ExtraSalary);
        }
        if self.RemoveCost != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemoveCost);
        }
        if let Some(v) = &self.EvolveData {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.SKillData {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.Salary != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Salary);
        }
        if self.ToyLv != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ToyLv);
        }
        if let Some(v) = &self.SelfDefineValue {
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
            1 => self.PlayIncId = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <FloroRanchActionUnitType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Id = r.read_varint_i32()?,
            4 => self.Point = r.read_varint_i32()?,
            5 => self.StartStage = r.read_varint_i32()?,
            6 => self.StartDay = r.read_varint_i32()?,
            7 => self.TagId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.BuffList
                    .push(<FloroRanchPlayBuff as PbMessage>::decode(bytes)?);
            }
            9 => self.Num = r.read_varint_i32()?,
            10 => self.ExtraSalary = r.read_varint_i64()?,
            11 => self.RemoveCost = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.EvolveData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.SKillData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Salary = r.read_varint_i64()?,
            15 => self.ToyLv = r.read_varint_i64()?,
            16 => {
                let bytes = r.read_len_delimited()?;
                self.SelfDefineValue
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayUnlimitedModeRequest {
    pub ActivityId: i32,
    pub SubInsId: i32,
}

impl FloroRanchPlayUnlimitedModeRequest {
    pub const MSG_ID: u16 = 25585;
}

impl PbMessage for FloroRanchPlayUnlimitedModeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            7 => self.SubInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayUnlimitedModeResponse {
    pub Task: Vec<FloroRanchPlayTask>,
    pub ErrorCode: Option<ErrorCode>,
    pub CurStage: i32,
}

impl FloroRanchPlayUnlimitedModeResponse {
    pub const MSG_ID: u16 = 20504;
}

impl PbMessage for FloroRanchPlayUnlimitedModeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
            crate::wire::write_submessage(out, 12, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurStage != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStage);
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
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.CurStage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayWeeklyEndNotify {
    pub SubInsId: i32,
    pub ActivityId: i32,
}

impl FloroRanchPlayWeeklyEndNotify {
    pub const MSG_ID: u16 = 19055;
}

impl PbMessage for FloroRanchPlayWeeklyEndNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            8 => self.SubInsId = r.read_varint_i32()?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchPlayWeeklyNotify {
    pub SubInsId: i32,
    pub ActivityId: i32,
    pub PlayerInfo: Option<FloroRanchPlayerInfo>,
}

impl FloroRanchPlayWeeklyNotify {
    pub const MSG_ID: u16 = 27614;
}

impl PbMessage for FloroRanchPlayWeeklyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.PlayerInfo {
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
            3 => self.SubInsId = r.read_varint_i32()?,
            4 => self.ActivityId = r.read_varint_i32()?,
            15 => {
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
pub struct FloroRanchPlayerInfo {
    pub UnlockCards: Vec<FloroUnLockInfo>,
    pub UnlockTech: Vec<i32>,
    pub UnlockToys: Vec<FloroUnLockInfo>,
    pub UnlockSkillIds: Vec<FloroUnLockInfo>,
    pub RewardMilestones: Vec<i32>,
    pub RaceUnLockInfo: Vec<FloroUnLockInfo>,
    pub FloroSubInsInfo: Vec<FloroSubInsInfo>,
    pub ConditionTasks: Vec<ConditionTask>,
    pub SubInsHistory: Vec<FloroRanchSubInsRecord>,
    pub ReadSubInsId: Vec<i32>,
    pub Animation: bool,
    pub LimitOpenTime: i64,
    pub LimitCloseTime: i64,
    pub InsUnLockCondition: Vec<FloroUnLockInfo>,
}

impl PbMessage for FloroRanchPlayerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockCards {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.UnlockTech {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockToys {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.UnlockSkillIds {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RewardMilestones {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RaceUnLockInfo {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.FloroSubInsInfo {
            crate::wire::write_submessage(out, 10, item);
        }
        for item in &self.ConditionTasks {
            crate::wire::write_submessage(out, 11, item);
        }
        for item in &self.SubInsHistory {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.ReadSubInsId {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Animation {
            crate::wire::write_tag(out, 15, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LimitOpenTime != 0 {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitOpenTime);
        }
        if self.LimitCloseTime != 0 {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitCloseTime);
        }
        for item in &self.InsUnLockCondition {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockCards
                    .push(<FloroUnLockInfo as PbMessage>::decode(bytes)?);
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockTech, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockToys
                    .push(<FloroUnLockInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockSkillIds
                    .push(<FloroUnLockInfo as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardMilestones,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RaceUnLockInfo
                    .push(<FloroUnLockInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.FloroSubInsInfo
                    .push(<FloroSubInsInfo as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.SubInsHistory
                    .push(<FloroRanchSubInsRecord as PbMessage>::decode(bytes)?);
            }
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReadSubInsId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            15 => self.Animation = r.read_bool()?,
            16 => self.LimitOpenTime = r.read_varint_i64()?,
            17 => self.LimitCloseTime = r.read_varint_i64()?,
            18 => {
                let bytes = r.read_len_delimited()?;
                self.InsUnLockCondition
                    .push(<FloroUnLockInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchRefreshCost {
    pub Type: Option<FloroRanchPlayResourcesType>,
    pub Count: i32,
}

impl PbMessage for FloroRanchRefreshCost {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            1 => {
                self.Type = Some(
                    <FloroRanchPlayResourcesType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Count = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchRewardMilestoneRequest {
    pub ActivityId: i32,
    pub MilestoneIds: Vec<i32>,
}

impl FloroRanchRewardMilestoneRequest {
    pub const MSG_ID: u16 = 25505;
}

impl PbMessage for FloroRanchRewardMilestoneRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.MilestoneIds {
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
            10 => self.ActivityId = r.read_varint_i32()?,
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MilestoneIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchRewardMilestoneResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchRewardMilestoneResponse {
    pub const MSG_ID: u16 = 24536;
}

impl PbMessage for FloroRanchRewardMilestoneResponse {
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
pub struct FloroRanchRewardTaskRequest {
    pub ActivityId: i32,
    pub TaskIds: Vec<i32>,
}

impl FloroRanchRewardTaskRequest {
    pub const MSG_ID: u16 = 27434;
}

impl PbMessage for FloroRanchRewardTaskRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.TaskIds {
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
            2 => self.ActivityId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchRewardTaskResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchRewardTaskResponse {
    pub const MSG_ID: u16 = 24696;
}

impl PbMessage for FloroRanchRewardTaskResponse {
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
pub struct FloroRanchSettleDataRequest {
    pub SubInsId: i32,
    pub ActivityId: i32,
}

impl FloroRanchSettleDataRequest {
    pub const MSG_ID: u16 = 23504;
}

impl PbMessage for FloroRanchSettleDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
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
            5 => self.SubInsId = r.read_varint_i32()?,
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSettleDataResponse {
    pub SettleData: Option<FloroRanchPlaySettleData>,
    pub ErrorCode: Option<ErrorCode>,
    pub ShowMvp: bool,
}

impl FloroRanchSettleDataResponse {
    pub const MSG_ID: u16 = 22164;
}

impl PbMessage for FloroRanchSettleDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SettleData {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ShowMvp {
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
                self.SettleData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.ShowMvp = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSettleRequest {
    pub SubInsId: i32,
    pub ActivityId: i32,
    pub Settle: bool,
}

impl FloroRanchSettleRequest {
    pub const MSG_ID: u16 = 22514;
}

impl PbMessage for FloroRanchSettleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Settle {
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
            10 => self.SubInsId = r.read_varint_i32()?,
            13 => self.ActivityId = r.read_varint_i32()?,
            15 => self.Settle = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSettleResponse {
    pub TotalCoin: i64,
    pub IsUnlimited: bool,
    pub MaxDayCoin: i64,
    pub TotalDays: i32,
    pub Reward: Vec<FloroRanchSubInsSettleReward>,
    pub Win: bool,
    pub ErrorCode: Option<ErrorCode>,
    pub IsNewMaxDays: bool,
    pub IsNewMaxCoin: bool,
    pub ChampionConfigId: i32,
    pub AllowedTechUp: bool,
    pub TechNum: i32,
    pub ChampionTotalCoinNum: i64,
    pub SettleData: Option<FloroRanchPlaySettleData>,
}

impl FloroRanchSettleResponse {
    pub const MSG_ID: u16 = 29820;
}

impl PbMessage for FloroRanchSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalCoin != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCoin);
        }
        if self.IsUnlimited {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxDayCoin != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxDayCoin);
        }
        if self.TotalDays != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDays);
        }
        for item in &self.Reward {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.Win {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsNewMaxDays {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsNewMaxCoin {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ChampionConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChampionConfigId);
        }
        if self.AllowedTechUp {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TechNum != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TechNum);
        }
        if self.ChampionTotalCoinNum != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ChampionTotalCoinNum);
        }
        if let Some(v) = &self.SettleData {
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
            1 => self.TotalCoin = r.read_varint_i64()?,
            2 => self.IsUnlimited = r.read_bool()?,
            3 => self.MaxDayCoin = r.read_varint_i64()?,
            4 => self.TotalDays = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .push(<FloroRanchSubInsSettleReward as PbMessage>::decode(bytes)?);
            }
            6 => self.Win = r.read_bool()?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.IsNewMaxDays = r.read_bool()?,
            10 => self.IsNewMaxCoin = r.read_bool()?,
            11 => self.ChampionConfigId = r.read_varint_i32()?,
            12 => self.AllowedTechUp = r.read_bool()?,
            13 => self.TechNum = r.read_varint_i32()?,
            14 => self.ChampionTotalCoinNum = r.read_varint_i64()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.SettleData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchShop {
    pub Items: Vec<FloroRanchShopItem>,
    pub AllowedRefresh: bool,
    pub Cost: Vec<FloroRanchRefreshCost>,
    pub IncId: i32,
    pub FreeTimes: i32,
    pub JustCard: bool,
}

impl PbMessage for FloroRanchShop {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.AllowedRefresh {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Cost {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.FreeTimes != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FreeTimes);
        }
        if self.JustCard {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<FloroRanchShopItem as PbMessage>::decode(bytes)?);
            }
            2 => self.AllowedRefresh = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Cost
                    .push(<FloroRanchRefreshCost as PbMessage>::decode(bytes)?);
            }
            4 => self.IncId = r.read_varint_i32()?,
            5 => self.FreeTimes = r.read_varint_i32()?,
            6 => self.JustCard = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchShopItem {
    pub Type: Option<FloroRanchShopItemType>,
    pub Id: i32,
    pub Price: i32,
    pub Sold: bool,
    pub IncId: i32,
}

impl PbMessage for FloroRanchShopItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Price != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Price);
        }
        if self.Sold {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
                    <FloroRanchShopItemType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Id = r.read_varint_i32()?,
            3 => self.Price = r.read_varint_i32()?,
            4 => self.Sold = r.read_bool()?,
            5 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchStageEnd {
    pub Win: bool,
    pub TotalCoin: i64,
    pub EnableUnlimitedMode: bool,
    pub IsLastStage: bool,
    pub IsUnlimitedMode: bool,
}

impl PbMessage for FloroRanchStageEnd {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Win {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TotalCoin != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCoin);
        }
        if self.EnableUnlimitedMode {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsLastStage {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsUnlimitedMode {
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
            1 => self.Win = r.read_bool()?,
            2 => self.TotalCoin = r.read_varint_i64()?,
            3 => self.EnableUnlimitedMode = r.read_bool()?,
            4 => self.IsLastStage = r.read_bool()?,
            5 => self.IsUnlimitedMode = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchStageStart {
    pub Target: i64,
    pub DayCount: i32,
    pub Reward: i64,
}

impl PbMessage for FloroRanchStageStart {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Target != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Target);
        }
        if self.DayCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DayCount);
        }
        if self.Reward != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Reward);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Target = r.read_varint_i64()?,
            2 => self.DayCount = r.read_varint_i32()?,
            3 => self.Reward = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchStartPlayRequest {
    pub SubInsId: i32,
    pub ActivityId: i32,
    pub SkillId: i32,
    pub Races: Vec<i32>,
}

impl FloroRanchStartPlayRequest {
    pub const MSG_ID: u16 = 25857;
}

impl PbMessage for FloroRanchStartPlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.Races {
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
            5 => self.SubInsId = r.read_varint_i32()?,
            7 => self.ActivityId = r.read_varint_i32()?,
            11 => self.SkillId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Races, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchStartPlayResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayInfo: Option<FloroRanchPlayInfo>,
    pub Task: Vec<FloroRanchPlayTask>,
}

impl FloroRanchStartPlayResponse {
    pub const MSG_ID: u16 = 23655;
}

impl PbMessage for FloroRanchStartPlayResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PlayInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        for item in &self.Task {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PlayInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Task
                    .push(<FloroRanchPlayTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSubInsRecord {
    pub SubInsId: i32,
    pub MaxDay: i32,
    pub MaxCoin: i64,
}

impl PbMessage for FloroRanchSubInsRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if self.MaxDay != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxDay);
        }
        if self.MaxCoin != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.MaxCoin);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.SubInsId = r.read_varint_i32()?,
            2 => self.MaxDay = r.read_varint_i32()?,
            3 => self.MaxCoin = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchSubInsSettleReward {
    pub Type: Option<SubInsSettleRewardType>,
    pub Id: i32,
}

impl PbMessage for FloroRanchSubInsSettleReward {
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
            1 => {
                self.Type = Some(
                    <SubInsSettleRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchUnitActionMsg {
    pub UnitActionType: Option<FloroRanchUnitActionType>,
    pub Caster: i32,
    pub BuffOpGroup: Option<FRUnitBuffOperateActionGroup>,
    pub UnitOpGroup: Option<FRUnitOperateActionGroup>,
    pub ResourcesChange: Option<FRUnitResourcesChangeAction>,
    pub EatingGroup: Option<FRUnitEatingActionGroup>,
    pub EvolveUpdate: Option<FREvolveUpdateAction>,
    pub MixAction: Option<FRMixAction>,
    pub SacrificeAction: Option<FRSacrifice>,
    pub ActionInfo: Option<FRDebugActionInfo>,
    pub ChangePointAction: Option<FRChangePointAction>,
    pub ActionStop: Option<FRActionStop>,
    pub ToyLvUp: Option<FRActionToyLvUp>,
    pub ChangeTag: Option<FRActionChangeTag>,
    pub TributeChange: Option<FRActionStageTributeChange>,
    pub SelfDefineValue: Option<FRActionSelfDefineValue>,
}

impl PbMessage for FloroRanchUnitActionMsg {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.UnitActionType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Caster != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Caster);
        }
        if let Some(v) = &self.BuffOpGroup {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.UnitOpGroup {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.ResourcesChange {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.EatingGroup {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.EvolveUpdate {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.MixAction {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.SacrificeAction {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.ActionInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.ChangePointAction {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.ActionStop {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.ToyLvUp {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.ChangeTag {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.TributeChange {
            crate::wire::write_submessage(out, 15, v);
        }
        if let Some(v) = &self.SelfDefineValue {
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
            1 => {
                self.UnitActionType = Some(
                    <FloroRanchUnitActionType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Caster = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BuffOpGroup
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.UnitOpGroup
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ResourcesChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.EatingGroup
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.EvolveUpdate
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.MixAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.SacrificeAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.ActionInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ChangePointAction
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.ActionStop
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ToyLvUp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.ChangeTag
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.TributeChange
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            16 => {
                let bytes = r.read_len_delimited()?;
                self.SelfDefineValue
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchUnlockTechPointRequest {
    pub ActivityId: i32,
    pub TechPoint: i32,
}

impl FloroRanchUnlockTechPointRequest {
    pub const MSG_ID: u16 = 23148;
}

impl PbMessage for FloroRanchUnlockTechPointRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.TechPoint != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TechPoint);
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
            15 => self.TechPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchUnlockTechPointResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FloroRanchUnlockTechPointResponse {
    pub const MSG_ID: u16 = 18079;
}

impl PbMessage for FloroRanchUnlockTechPointResponse {
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
pub struct FloroRanchUpdatePlayerInfoNotify {
    pub Task: Option<ActivityTask>,
    pub FloroCfgType: Option<FloroConfigType>,
    pub UnlockToy: Option<FloroUnLockInfo>,
    pub ActivityId: i32,
    pub InsLockInfo: Option<FloroUnLockInfo>,
    pub SubInfHistory: Option<FloroRanchSubInsRecord>,
    pub UnlockCard: Option<FloroUnLockInfo>,
    pub ConditionTask: Option<ConditionTask>,
    pub UnlockSkill: Option<FloroUnLockInfo>,
    pub RaceUnLockInfo: Option<FloroUnLockInfo>,
    pub FloroSubInsInfo: Option<FloroSubInsInfo>,
}

impl FloroRanchUpdatePlayerInfoNotify {
    pub const MSG_ID: u16 = 23957;
}

impl PbMessage for FloroRanchUpdatePlayerInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Task {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.FloroCfgType {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.UnlockToy {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.InsLockInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if let Some(v) = &self.SubInfHistory {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.UnlockCard {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.ConditionTask {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.UnlockSkill {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.RaceUnLockInfo {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.FloroSubInsInfo {
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
                self.Task
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                self.FloroCfgType = Some(
                    <FloroConfigType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockToy
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.ActivityId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.InsLockInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.SubInfHistory
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockCard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.UnlockSkill
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.RaceUnLockInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.FloroSubInsInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchWageSettleTask {
    pub SettleActions: Vec<FRUnitResourcesChangeAction>,
}

impl PbMessage for FloroRanchWageSettleTask {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SettleActions {
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
                self.SettleActions
                    .push(<FRUnitResourcesChangeAction as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroRanchWeeklyRequest {}

impl FloroRanchWeeklyRequest {
    pub const MSG_ID: u16 = 18853;
}

impl PbMessage for FloroRanchWeeklyRequest {
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
pub struct FloroRanchWeeklyResponse {
    pub SubInsId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ActivityId: i32,
    pub PlayerInfo: Option<FloroRanchPlayerInfo>,
}

impl FloroRanchWeeklyResponse {
    pub const MSG_ID: u16 = 28219;
}

impl PbMessage for FloroRanchWeeklyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SubInsId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubInsId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.PlayerInfo {
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
            2 => self.SubInsId = r.read_varint_i32()?,
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.ActivityId = r.read_varint_i32()?,
            15 => {
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
pub struct FloroSubInsInfo {
    pub Key: i32,
    pub UnLockCondition: i32,
    pub UnLock: bool,
    pub Pass: bool,
}

impl PbMessage for FloroSubInsInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Key != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Key);
        }
        if self.UnLockCondition != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockCondition);
        }
        if self.UnLock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Pass {
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
            1 => self.Key = r.read_varint_i32()?,
            2 => self.UnLockCondition = r.read_varint_i32()?,
            3 => self.UnLock = r.read_bool()?,
            4 => self.Pass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FloroUnLockInfo {
    pub Key: i32,
    pub UnLockCondition: i32,
    pub UnLock: bool,
}

impl PbMessage for FloroUnLockInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Key != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Key);
        }
        if self.UnLockCondition != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockCondition);
        }
        if self.UnLock {
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
            1 => self.Key = r.read_varint_i32()?,
            2 => self.UnLockCondition = r.read_varint_i32()?,
            3 => self.UnLock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowServerSkipNotify {
    pub StateId: i32,
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub FadeOutScreen: bool,
}

impl FlowServerSkipNotify {
    pub const MSG_ID: u16 = 21217;
}

impl PbMessage for FlowServerSkipNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StateId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateId);
        }
        if !self.FlowListName.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowListName);
        }
        if self.FlowId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlowId);
        }
        if self.FadeOutScreen {
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
            7 => self.StateId = r.read_varint_i32()?,
            10 => self.FlowListName = r.read_len_delimited()?.to_vec(),
            13 => self.FlowId = r.read_varint_i32()?,
            15 => self.FadeOutScreen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowTeleportToPlotPosRequest {
    pub FlowIncId: i64,
}

impl FlowTeleportToPlotPosRequest {
    pub const MSG_ID: u16 = 20939;
}

impl PbMessage for FlowTeleportToPlotPosRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FlowIncId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.FlowIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowTeleportToPlotPosResponse {
    pub Code: Option<ErrorCode>,
}

impl FlowTeleportToPlotPosResponse {
    pub const MSG_ID: u16 = 28245;
}

impl PbMessage for FlowTeleportToPlotPosResponse {
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
pub struct FlowerPollutionRequest {
    pub SourceEntityId: i64,
    pub TargetEntityId: i64,
    pub ManagerEntityId: i64,
    pub SwitchToTargetState: Vec<u8>,
}

impl FlowerPollutionRequest {
    pub const MSG_ID: u16 = 22707;
}

impl PbMessage for FlowerPollutionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SourceEntityId);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.ManagerEntityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ManagerEntityId);
        }
        if !self.SwitchToTargetState.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.SwitchToTargetState);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SourceEntityId = r.read_varint_i64()?,
            4 => self.TargetEntityId = r.read_varint_i64()?,
            10 => self.ManagerEntityId = r.read_varint_i64()?,
            13 => self.SwitchToTargetState = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlowerPollutionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlowerPollutionResponse {
    pub const MSG_ID: u16 = 23879;
}

impl PbMessage for FlowerPollutionResponse {
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
pub struct FlowerPollutionSplinePush {
    pub ManagerEntityId: i64,
    pub IsRemovePollution: bool,
    pub SplineConfigId: i32,
}

impl FlowerPollutionSplinePush {
    pub const MSG_ID: u16 = 18370;
}

impl PbMessage for FlowerPollutionSplinePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ManagerEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ManagerEntityId);
        }
        if self.IsRemovePollution {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SplineConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            2 => self.ManagerEntityId = r.read_varint_i64()?,
            5 => self.IsRemovePollution = r.read_bool()?,
            9 => self.SplineConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlyEquipData {
    pub SkinId: i32,
    pub RoleIds: Vec<i32>,
}

impl PbMessage for FlyEquipData {
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
pub struct FlySkinAllUnLoadRequest {
    pub SkinType: i32,
}

impl FlySkinAllUnLoadRequest {
    pub const MSG_ID: u16 = 17171;
}

impl PbMessage for FlySkinAllUnLoadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.SkinType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinAllUnLoadResponse {
    pub UnLoadSkinRoles: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FlySkinAllUnLoadResponse {
    pub const MSG_ID: u16 = 25150;
}

impl PbMessage for FlySkinAllUnLoadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnLoadSkinRoles {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnLoadSkinRoles,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
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
pub struct FlySkinInfo {
    pub SkinId: i32,
    pub SkinType: i32,
}

impl PbMessage for FlySkinInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
        }
        if self.SkinType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinType);
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
            2 => self.SkinType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinUnLoadRequest {
    pub SkinId: i32,
    pub RoleId: i32,
}

impl FlySkinUnLoadRequest {
    pub const MSG_ID: u16 = 22401;
}

impl PbMessage for FlySkinUnLoadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinId);
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
            8 => self.SkinId = r.read_varint_i32()?,
            11 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlySkinUnLoadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FlySkinUnLoadResponse {
    pub const MSG_ID: u16 = 29892;
}

impl PbMessage for FlySkinUnLoadResponse {
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
pub struct FollowDetectionTargetUpdateNotify {
    pub DetectionTarget: Vec<DetectionTarget>,
}

impl FollowDetectionTargetUpdateNotify {
    pub const MSG_ID: u16 = 17471;
}

impl PbMessage for FollowDetectionTargetUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DetectionTarget {
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
                self.DetectionTarget
                    .push(<DetectionTarget as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FollowTrackRequest {
    pub EntityId: i64,
}

impl FollowTrackRequest {
    pub const MSG_ID: u16 = 25969;
}

impl PbMessage for FollowTrackRequest {
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
pub struct FollowTrackResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FollowTrackResponse {
    pub const MSG_ID: u16 = 21101;
}

impl PbMessage for FollowTrackResponse {
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
pub struct FollowerChairPresentationChangeNotify {
    pub IsShow: bool,
    pub PlayerId: i32,
    pub Rotation: Option<Rotator>,
    pub EntityId: i64,
    pub Location: Option<Vector>,
}

impl FollowerChairPresentationChangeNotify {
    pub const MSG_ID: u16 = 29665;
}

impl PbMessage for FollowerChairPresentationChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            1 => self.IsShow = r.read_bool()?,
            5 => self.PlayerId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.EntityId = r.read_varint_i64()?,
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
pub struct FollowerChairPresentationPush {
    pub IsShow: bool,
    pub Location: Option<Vector>,
    pub Rotation: Option<Rotator>,
    pub EntityId: i64,
}

impl FollowerChairPresentationPush {
    pub const MSG_ID: u16 = 24462;
}

impl PbMessage for FollowerChairPresentationPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 8, v);
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
            5 => self.IsShow = r.read_bool()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Rotation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoodProcessRequest {
    pub InteractEntityId: i64,
    pub CookCount: i32,
    pub Id: i32,
    pub PrimaryFood: Vec<SingleItemInfo>,
}

impl FoodProcessRequest {
    pub const MSG_ID: u16 = 21214;
}

impl PbMessage for FoodProcessRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
        if self.CookCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CookCount);
        }
        if self.Id != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.PrimaryFood {
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
            3 => self.InteractEntityId = r.read_varint_i64()?,
            4 => self.CookCount = r.read_varint_i32()?,
            12 => self.Id = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PrimaryFood
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoodProcessResponse {
    pub Code: Option<ErrorCode>,
    pub LockState: bool,
    pub FinalFood: Vec<SingleItemInfo>,
    pub UnlockParam: Vec<i32>,
    pub Id: i32,
}

impl FoodProcessResponse {
    pub const MSG_ID: u16 = 23327;
}

impl PbMessage for FoodProcessResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LockState {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.FinalFood {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.UnlockParam {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            1 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.LockState = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.FinalFood
                    .push(<SingleItemInfo as PbMessage>::decode(bytes)?);
            }
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockParam, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            15 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ForceClosedRollbackData {
    pub ForceClosedNodeId: i32,
}

impl PbMessage for ForceClosedRollbackData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ForceClosedNodeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ForceClosedNodeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ForceClosedNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationAutoAddRoleNotify {
    pub Roles: Vec<i32>,
    pub FormationId: i32,
}

impl FormationAutoAddRoleNotify {
    pub const MSG_ID: u16 = 28861;
}

impl PbMessage for FormationAutoAddRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Roles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.FormationId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationChangeInfo {
    pub Id: i32,
    pub Role: Vec<i32>,
}

impl PbMessage for FormationChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Role {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Role, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationChangeRequest {
    pub ChangeList: Vec<FormationChangeInfo>,
    pub FormationId: i32,
}

impl FormationChangeRequest {
    pub const MSG_ID: u16 = 16758;
}

impl PbMessage for FormationChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChangeList {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
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
                self.ChangeList
                    .push(<FormationChangeInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.FormationId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationChangeResponse {
    pub FormationId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub ChangeList: Vec<FormationChangeInfo>,
}

impl FormationChangeResponse {
    pub const MSG_ID: u16 = 18622;
}

impl PbMessage for FormationChangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ChangeList {
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
            2 => self.FormationId = r.read_varint_i32()?,
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.ChangeList
                    .push(<FormationChangeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationModifyRequest {
    pub Id: i32,
    pub Role: Vec<i32>,
}

impl FormationModifyRequest {
    pub const MSG_ID: u16 = 15057;
}

impl PbMessage for FormationModifyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.Role {
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
            3 => self.Id = r.read_varint_i32()?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Role, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationModifyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Formation: Option<Formation>,
}

impl FormationModifyResponse {
    pub const MSG_ID: u16 = 18825;
}

impl PbMessage for FormationModifyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Formation {
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
pub struct FormationRequest {}

impl FormationRequest {
    pub const MSG_ID: u16 = 22596;
}

impl PbMessage for FormationRequest {
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
pub struct FormationResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Formation: Vec<Formation>,
}

impl FormationResponse {
    pub const MSG_ID: u16 = 20142;
}

impl PbMessage for FormationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Formation {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .push(<Formation as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationSwitchCurrentRequest {
    pub Id: i32,
}

impl FormationSwitchCurrentRequest {
    pub const MSG_ID: u16 = 25699;
}

impl PbMessage for FormationSwitchCurrentRequest {
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
pub struct FormationSwitchCurrentResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CurrentId: i32,
}

impl FormationSwitchCurrentResponse {
    pub const MSG_ID: u16 = 22323;
}

impl PbMessage for FormationSwitchCurrentResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurrentId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentId);
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
            14 => self.CurrentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationSwitchRoleRequest {
    pub RoleIndex: i32,
}

impl FormationSwitchRoleRequest {
    pub const MSG_ID: u16 = 22661;
}

impl PbMessage for FormationSwitchRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.RoleIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FormationSwitchRoleResponse {
    pub RoleIndex: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl FormationSwitchRoleResponse {
    pub const MSG_ID: u16 = 27628;
}

impl PbMessage for FormationSwitchRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIndex);
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
            10 => self.RoleIndex = r.read_varint_i32()?,
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
pub struct FormationUpdateNotify {
    pub FormationId: i32,
    pub Formation: Vec<FightFormation>,
    pub RemoveFormationIds: Vec<i32>,
}

impl FormationUpdateNotify {
    pub const MSG_ID: u16 = 19741;
}

impl PbMessage for FormationUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FormationId);
        }
        for item in &self.Formation {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.RemoveFormationIds {
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
            6 => self.FormationId = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .push(<FightFormation as PbMessage>::decode(bytes)?);
            }
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RemoveFormationIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoundationCoreOperateRequest {
    pub IsTakeOut: bool,
    pub FoundationEntityIncId: i64,
}

impl FoundationCoreOperateRequest {
    pub const MSG_ID: u16 = 27836;
}

impl PbMessage for FoundationCoreOperateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsTakeOut {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FoundationEntityIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FoundationEntityIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.IsTakeOut = r.read_bool()?,
            3 => self.FoundationEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoundationCoreOperateResponse {
    pub FoundationCoreInfo: Option<FoundationCoreInfo>,
    pub FromFoundationEntityId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl FoundationCoreOperateResponse {
    pub const MSG_ID: u16 = 23558;
}

impl PbMessage for FoundationCoreOperateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FoundationCoreInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.FromFoundationEntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromFoundationEntityId);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.FoundationCoreInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.FromFoundationEntityId = r.read_varint_i32()?,
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
pub struct FoundationRequest {
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
    pub AccusedEntityId: i64,
    pub EntityId: i64,
    pub ActiveOperate: i32,
}

impl FoundationRequest {
    pub const MSG_ID: u16 = 23886;
}

impl PbMessage for FoundationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Rot {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.AccusedEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.AccusedEntityId);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.ActiveOperate != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveOperate);
        }
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Rot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.AccusedEntityId = r.read_varint_i64()?,
            9 => self.EntityId = r.read_varint_i64()?,
            12 => self.ActiveOperate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FoundationResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ActiveOperate: i32,
}

impl FoundationResponse {
    pub const MSG_ID: u16 = 26544;
}

impl PbMessage for FoundationResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ActiveOperate != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveOperate);
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
            14 => self.ActiveOperate = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendAddedNotify {
    pub Info: Option<FriendInfo>,
}

impl FriendAddedNotify {
    pub const MSG_ID: u16 = 23136;
}

impl PbMessage for FriendAddedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct FriendApplyDeletedNotify {
    pub Id: i32,
}

impl FriendApplyDeletedNotify {
    pub const MSG_ID: u16 = 18666;
}

impl PbMessage for FriendApplyDeletedNotify {
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
pub struct FriendApplyHandleRequest {
    pub Ids: Vec<i32>,
    pub Operator: Option<FriendApplyOperator>,
}

impl FriendApplyHandleRequest {
    pub const MSG_ID: u16 = 24610;
}

impl PbMessage for FriendApplyHandleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Ids {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Operator {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                self.Operator = Some(
                    <FriendApplyOperator as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct FriendApplyHandleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub HandledMap: BTreeMap<i32, ErrorCode>,
}

impl FriendApplyHandleResponse {
    pub const MSG_ID: u16 = 20998;
}

impl PbMessage for FriendApplyHandleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.HandledMap.is_empty() {
            crate::wire::write_map(out, 8, &self.HandledMap);
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
            8 => crate::wire::decode_map_entry(r, &mut self.HandledMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendApplyReceivedNotify {
    pub FriendApply: Option<FriendApply>,
}

impl FriendApplyReceivedNotify {
    pub const MSG_ID: u16 = 17101;
}

impl PbMessage for FriendApplyReceivedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FriendApply {
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
                self.FriendApply
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendApplySendRequest {
    pub Way: Option<FriendApplyWay>,
    pub Id: i32,
}

impl FriendApplySendRequest {
    pub const MSG_ID: u16 = 19323;
}

impl PbMessage for FriendApplySendRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Way {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            2 => {
                self.Way = Some(
                    <FriendApplyWay as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendApplySendResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FriendApplySendResponse {
    pub const MSG_ID: u16 = 15562;
}

impl PbMessage for FriendApplySendResponse {
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
pub struct FriendDeleteRequest {
    pub Id: i32,
}

impl FriendDeleteRequest {
    pub const MSG_ID: u16 = 27693;
}

impl PbMessage for FriendDeleteRequest {
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
pub struct FriendDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FriendDeleteResponse {
    pub const MSG_ID: u16 = 17425;
}

impl PbMessage for FriendDeleteResponse {
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
pub struct FriendDeletedNotify {
    pub Id: i32,
}

impl FriendDeletedNotify {
    pub const MSG_ID: u16 = 15578;
}

impl PbMessage for FriendDeletedNotify {
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
pub struct FriendRecentlyTeamRequest {}

impl FriendRecentlyTeamRequest {
    pub const MSG_ID: u16 = 22709;
}

impl PbMessage for FriendRecentlyTeamRequest {
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
pub struct FriendRecentlyTeamResponse {
    pub Infos: Vec<RecentlyTeamInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl FriendRecentlyTeamResponse {
    pub const MSG_ID: u16 = 24249;
}

impl PbMessage for FriendRecentlyTeamResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Infos {
            crate::wire::write_submessage(out, 6, item);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Infos
                    .push(<RecentlyTeamInfo as PbMessage>::decode(bytes)?);
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
pub struct FriendRecentlyTeamUpdateNotify {
    pub TimeUpdate: BTreeMap<i32, i64>,
    pub AddInfos: Vec<RecentlyTeamInfo>,
    pub RemoveIds: Vec<i32>,
}

impl FriendRecentlyTeamUpdateNotify {
    pub const MSG_ID: u16 = 26784;
}

impl PbMessage for FriendRecentlyTeamUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.TimeUpdate.is_empty() {
            crate::wire::write_map(out, 1, &self.TimeUpdate);
        }
        for item in &self.AddInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RemoveIds {
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
            1 => crate::wire::decode_map_entry(r, &mut self.TimeUpdate)?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.AddInfos
                    .push(<RecentlyTeamInfo as PbMessage>::decode(bytes)?);
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RemoveIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendRemarkRequest {
    pub Id: i32,
    pub Remark: Vec<u8>,
}

impl FriendRemarkRequest {
    pub const MSG_ID: u16 = 19201;
}

impl PbMessage for FriendRemarkRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if !self.Remark.is_empty() {
            crate::wire::write_tag(out, 12, WireType::LengthDelimited);
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
            3 => self.Id = r.read_varint_i32()?,
            12 => self.Remark = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FriendRemarkResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FriendRemarkResponse {
    pub const MSG_ID: u16 = 29476;
}

impl PbMessage for FriendRemarkResponse {
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
pub struct FunPlayActivityInfo {
    pub FunPlayChallengeInfos: Vec<FunPlayChallengeInfo>,
}

impl PbMessage for FunPlayActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FunPlayChallengeInfos {
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
                self.FunPlayChallengeInfos
                    .push(<FunPlayChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunPlayChallengeAwardRequest {
    pub ChallengeId: i32,
}

impl FunPlayChallengeAwardRequest {
    pub const MSG_ID: u16 = 25841;
}

impl PbMessage for FunPlayChallengeAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunPlayChallengeAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FunPlayChallengeAwardResponse {
    pub const MSG_ID: u16 = 24010;
}

impl PbMessage for FunPlayChallengeAwardResponse {
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
pub struct FunPlayChallengeInfo {
    pub ChallengeId: i32,
    pub UnlockTime: i64,
    pub RewardStatus: Option<FunPlayChallengeRewardStatus>,
    pub SharpCommentIds: Vec<i32>,
    pub FinishTime: i64,
}

impl PbMessage for FunPlayChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if let Some(v) = &self.RewardStatus {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SharpCommentIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FinishTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.FinishTime);
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
            2 => self.UnlockTime = r.read_varint_i64()?,
            3 => {
                self.RewardStatus = Some(
                    <FunPlayChallengeRewardStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SharpCommentIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.FinishTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunPlayChallengeInfoUpdateNotify {
    pub FunPlayChallengeInfos: Vec<FunPlayChallengeInfo>,
}

impl FunPlayChallengeInfoUpdateNotify {
    pub const MSG_ID: u16 = 28208;
}

impl PbMessage for FunPlayChallengeInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FunPlayChallengeInfos {
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
                self.FunPlayChallengeInfos
                    .push(<FunPlayChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunPlayChallengeSettleNotify {
    pub IsSuccess: bool,
    pub IsActivityOpen: bool,
    pub ChallengeId: i32,
}

impl FunPlayChallengeSettleNotify {
    pub const MSG_ID: u16 = 17763;
}

impl PbMessage for FunPlayChallengeSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsSuccess {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsActivityOpen {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            4 => self.IsSuccess = r.read_bool()?,
            9 => self.IsActivityOpen = r.read_bool()?,
            12 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FunPlayCtx {
    pub ChallengeId: i32,
}

impl PbMessage for FunPlayCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncOpenConfirmNotify {
    pub FuncInfo: Vec<u8>,
}

impl FuncOpenConfirmNotify {
    pub const MSG_ID: u16 = 19104;
}

impl PbMessage for FuncOpenConfirmNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FuncInfo.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FuncInfo);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.FuncInfo = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncOpenUpdateNotify {
    pub Silent: bool,
    pub Func: Vec<Function>,
}

impl FuncOpenUpdateNotify {
    pub const MSG_ID: u16 = 18542;
}

impl PbMessage for FuncOpenUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Silent {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Func {
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
            8 => self.Silent = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.Func.push(<Function as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncShowRequest {
    pub FuncId: Vec<i32>,
}

impl FuncShowRequest {
    pub const MSG_ID: u16 = 25127;
}

impl PbMessage for FuncShowRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FuncId {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FuncId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FuncShowResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl FuncShowResponse {
    pub const MSG_ID: u16 = 21232;
}

impl PbMessage for FuncShowResponse {
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
pub struct FurnitureDiySaveRequest {
    pub IsOneClickPreset: bool,
    pub ActivityId: i32,
    pub FurnitureDiySlotInfos: Vec<FurnitureDiySlotInfo>,
    pub AreaId: i32,
}

impl FurnitureDiySaveRequest {
    pub const MSG_ID: u16 = 21389;
}

impl PbMessage for FurnitureDiySaveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsOneClickPreset {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.FurnitureDiySlotInfos {
            crate::wire::write_submessage(out, 11, item);
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
            5 => self.IsOneClickPreset = r.read_bool()?,
            6 => self.ActivityId = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.FurnitureDiySlotInfos
                    .push(<FurnitureDiySlotInfo as PbMessage>::decode(bytes)?);
            }
            12 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FurnitureDiySaveResponse {
    pub Atmosphere: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl FurnitureDiySaveResponse {
    pub const MSG_ID: u16 = 25998;
}

impl PbMessage for FurnitureDiySaveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Atmosphere != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Atmosphere);
        }
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
            1 => self.Atmosphere = r.read_varint_i32()?,
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
pub struct FurnitureUnlockNotify {
    pub FurnitureIds: Vec<i32>,
    pub ActivityId: i32,
}

impl FurnitureUnlockNotify {
    pub const MSG_ID: u16 = 29232;
}

impl PbMessage for FurnitureUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FurnitureIds {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FurnitureIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUMonsterKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for GPUMonsterKilledCtxPb {
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
pub struct GPURoleKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for GPURoleKilledCtxPb {
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
pub struct GaPassiveSkill {
    pub SkillId: i64,
    pub ContextId: i64,
}

impl PbMessage for GaPassiveSkill {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SkillId);
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
            1 => self.SkillId = r.read_varint_i64()?,
            2 => self.ContextId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateGroupInfo {
    pub GroupId: i32,
    pub CurGachaNum: i32,
    pub GachaAccumulateRewardInfos: Vec<GachaAccumulateRewardInfo>,
}

impl PbMessage for GachaAccumulateGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.CurGachaNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurGachaNum);
        }
        for item in &self.GachaAccumulateRewardInfos {
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
            1 => self.GroupId = r.read_varint_i32()?,
            2 => self.CurGachaNum = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.GachaAccumulateRewardInfos
                    .push(<GachaAccumulateRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateInfo {
    pub GachaAccumulateId: i32,
    pub GachaAccumulateGroupInfo: Option<GachaAccumulateGroupInfo>,
}

impl PbMessage for GachaAccumulateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaAccumulateId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaAccumulateId);
        }
        if let Some(v) = &self.GachaAccumulateGroupInfo {
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
            1 => self.GachaAccumulateId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GachaAccumulateGroupInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateInfoNotify {
    pub GachaAccumulateInfo: Option<GachaAccumulateInfo>,
}

impl GachaAccumulateInfoNotify {
    pub const MSG_ID: u16 = 21430;
}

impl PbMessage for GachaAccumulateInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GachaAccumulateInfo {
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
                self.GachaAccumulateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateInfoRequest {
    pub GachaAccumulateId: i32,
}

impl GachaAccumulateInfoRequest {
    pub const MSG_ID: u16 = 28411;
}

impl PbMessage for GachaAccumulateInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaAccumulateId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaAccumulateId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.GachaAccumulateId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GachaAccumulateInfo: Option<GachaAccumulateInfo>,
}

impl GachaAccumulateInfoResponse {
    pub const MSG_ID: u16 = 21629;
}

impl PbMessage for GachaAccumulateInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.GachaAccumulateInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.GachaAccumulateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateRewardInfo {
    pub Id: i32,
    pub GroupId: i32,
    pub GachaNum: i32,
    pub RewardContent: BTreeMap<i32, i32>,
    pub Status: i32,
    pub IsBigReward: bool,
    pub CycleCount: i32,
    pub CycleRewardedTimes: i32,
    pub IsPreView: bool,
}

impl PbMessage for GachaAccumulateRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.GachaNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaNum);
        }
        if !self.RewardContent.is_empty() {
            crate::wire::write_map(out, 4, &self.RewardContent);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
        if self.IsBigReward {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CycleCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleCount);
        }
        if self.CycleRewardedTimes != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleRewardedTimes);
        }
        if self.IsPreView {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.GroupId = r.read_varint_i32()?,
            3 => self.GachaNum = r.read_varint_i32()?,
            4 => crate::wire::decode_map_entry(r, &mut self.RewardContent)?,
            5 => self.Status = r.read_varint_i32()?,
            6 => self.IsBigReward = r.read_bool()?,
            7 => self.CycleCount = r.read_varint_i32()?,
            8 => self.CycleRewardedTimes = r.read_varint_i32()?,
            9 => self.IsPreView = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateRewardRequest {
    pub RewardId: Vec<i32>,
    pub GachaAccumulateId: i32,
    pub SelectedId: Vec<i32>,
}

impl GachaAccumulateRewardRequest {
    pub const MSG_ID: u16 = 21085;
}

impl PbMessage for GachaAccumulateRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardId {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.GachaAccumulateId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaAccumulateId);
        }
        for item in &self.SelectedId {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.GachaAccumulateId = r.read_varint_i32()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SelectedId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaAccumulateRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GachaAccumulateRewardResponse {
    pub const MSG_ID: u16 = 20196;
}

impl PbMessage for GachaAccumulateRewardResponse {
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
pub struct GachaDiscountNotify {
    pub GachaId: i32,
    pub GachaDiscountInfos: Vec<GachaDiscountInfo>,
}

impl GachaDiscountNotify {
    pub const MSG_ID: u16 = 15918;
}

impl PbMessage for GachaDiscountNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaId);
        }
        for item in &self.GachaDiscountInfos {
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
            6 => self.GachaId = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.GachaDiscountInfos
                    .push(<GachaDiscountInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaNewNotify {}

impl GachaNewNotify {
    pub const MSG_ID: u16 = 15278;
}

impl PbMessage for GachaNewNotify {
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
pub struct GachaPoolDrop {
    pub ItemId: i32,
    pub IsUp: bool,
}

impl PbMessage for GachaPoolDrop {
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
pub struct GachaResultNotify {
    pub GachaResults: Vec<GachaResult>,
}

impl GachaResultNotify {
    pub const MSG_ID: u16 = 20485;
}

impl PbMessage for GachaResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GachaResults {
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
                self.GachaResults
                    .push(<GachaResult as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GachaRoleDevelopInsCtx {
    pub GachaRoleDevelopInsId: i32,
}

impl PbMessage for GachaRoleDevelopInsCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GachaRoleDevelopInsId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GachaRoleDevelopInsId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GachaRoleDevelopInsId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayTagChangedNotify {
    pub Tags: Vec<GameplayTagData>,
    pub Id: i64,
}

impl GameplayTagChangedNotify {
    pub const MSG_ID: u16 = 17995;
}

impl PbMessage for GameplayTagChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Tags {
            crate::wire::write_submessage(out, 5, item);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Tags
                    .push(<GameplayTagData as PbMessage>::decode(bytes)?);
            }
            15 => self.Id = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayTagInitNotify {
    pub Id: i64,
    pub Tags: Vec<GameplayTagData>,
}

impl GameplayTagInitNotify {
    pub const MSG_ID: u16 = 17191;
}

impl PbMessage for GameplayTagInitNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Id);
        }
        for item in &self.Tags {
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
            10 => self.Id = r.read_varint_i64()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Tags
                    .push(<GameplayTagData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayTagPush {
    pub TagCount: i32,
    pub TagId: i32,
}

impl GameplayTagPush {
    pub const MSG_ID: u16 = 24743;
}

impl PbMessage for GameplayTagPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TagCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TagCount);
        }
        if self.TagId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            3 => self.TagCount = r.read_varint_i32()?,
            11 => self.TagId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GameplayTagRequest {
    pub TagId: i32,
    pub TagCount: i32,
}

impl GameplayTagRequest {
    pub const MSG_ID: u16 = 17502;
}

impl PbMessage for GameplayTagRequest {
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
pub struct GameplayTagResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GameplayTagResponse {
    pub const MSG_ID: u16 = 16302;
}

impl PbMessage for GameplayTagResponse {
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
pub struct GatewayEchoRequest {
    pub Content: Vec<u8>,
}

impl GatewayEchoRequest {
    pub const MSG_ID: u16 = 29209;
}

impl PbMessage for GatewayEchoRequest {
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
pub struct GatewayEchoResponse {
    pub Content: Vec<u8>,
}

impl GatewayEchoResponse {
    pub const MSG_ID: u16 = 23722;
}

impl PbMessage for GatewayEchoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Content.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
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
            11 => self.Content = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatherActivityRewardRequest {
    pub GatherTaskId: i32,
}

impl GatherActivityRewardRequest {
    pub const MSG_ID: u16 = 22600;
}

impl PbMessage for GatherActivityRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GatherTaskId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GatherTaskId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.GatherTaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatherActivityRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GatherActivityRewardResponse {
    pub const MSG_ID: u16 = 29225;
}

impl PbMessage for GatherActivityRewardResponse {
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
pub struct GatherClueInfoNotify {
    pub UnLockClueIds: Vec<i32>,
}

impl GatherClueInfoNotify {
    pub const MSG_ID: u16 = 19010;
}

impl PbMessage for GatherClueInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnLockClueIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnLockClueIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatherItemInfo {
    pub ItemId: i32,
    pub ItemNum: i32,
}

impl PbMessage for GatherItemInfo {
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
pub struct GatherTaskRecordNotify {
    pub TaskDoneInfo: Vec<TaskDoneInfo>,
}

impl GatherTaskRecordNotify {
    pub const MSG_ID: u16 = 25663;
}

impl PbMessage for GatherTaskRecordNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskDoneInfo {
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
                self.TaskDoneInfo
                    .push(<TaskDoneInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatherTaskSubmitInfo {
    pub TaskId: i32,
    pub GatherItemInfos: Vec<GatherItemInfo>,
}

impl PbMessage for GatherTaskSubmitInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TaskId);
        }
        for item in &self.GatherItemInfos {
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
            1 => self.TaskId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GatherItemInfos
                    .push(<GatherItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetAdventureRewardRequest {
    pub Id: i32,
}

impl GetAdventureRewardRequest {
    pub const MSG_ID: u16 = 26242;
}

impl PbMessage for GetAdventureRewardRequest {
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
pub struct GetAdventureRewardResponse {
    pub Id: i32,
    pub Code: Option<ErrorCode>,
}

impl GetAdventureRewardResponse {
    pub const MSG_ID: u16 = 25347;
}

impl PbMessage for GetAdventureRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            9 => self.Id = r.read_varint_i32()?,
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
pub struct GetChapterRewardRequest {
    pub Chapter: i32,
}

impl GetChapterRewardRequest {
    pub const MSG_ID: u16 = 26818;
}

impl PbMessage for GetChapterRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Chapter != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Chapter);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetChapterRewardResponse {
    pub Chapter: i32,
    pub Code: Option<ErrorCode>,
}

impl GetChapterRewardResponse {
    pub const MSG_ID: u16 = 18554;
}

impl PbMessage for GetChapterRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Chapter != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Chapter);
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
            12 => self.Chapter = r.read_varint_i32()?,
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
pub struct GetInstExchangeRewardRequest {
    pub ExchangeCount: i32,
}

impl GetInstExchangeRewardRequest {
    pub const MSG_ID: u16 = 25904;
}

impl PbMessage for GetInstExchangeRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ExchangeCount != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            12 => self.ExchangeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetInstExchangeRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GetInstExchangeRewardResponse {
    pub const MSG_ID: u16 = 23064;
}

impl PbMessage for GetInstExchangeRewardResponse {
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
pub struct GetLevelPlayRewardsRequest {
    pub LevelPlayId: i32,
    pub InstId: i32,
}

impl GetLevelPlayRewardsRequest {
    pub const MSG_ID: u16 = 16467;
}

impl PbMessage for GetLevelPlayRewardsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
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
            9 => self.LevelPlayId = r.read_varint_i32()?,
            15 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetLevelPlayRewardsResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TreasureBoxIds: Vec<i32>,
    pub RewardNodeIds: Vec<i32>,
}

impl GetLevelPlayRewardsResponse {
    pub const MSG_ID: u16 = 19260;
}

impl PbMessage for GetLevelPlayRewardsResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TreasureBoxIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.RewardNodeIds {
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TreasureBoxIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardNodeIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetTraceQuestYuXueModeRequest {}

impl GetTraceQuestYuXueModeRequest {
    pub const MSG_ID: u16 = 27604;
}

impl PbMessage for GetTraceQuestYuXueModeRequest {
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
pub struct GetTraceQuestYuXueModeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub InYuXueMode: bool,
}

impl GetTraceQuestYuXueModeResponse {
    pub const MSG_ID: u16 = 20487;
}

impl PbMessage for GetTraceQuestYuXueModeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.InYuXueMode {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.InYuXueMode = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetYuXueModeInfoRequest {}

impl GetYuXueModeInfoRequest {
    pub const MSG_ID: u16 = 26654;
}

impl PbMessage for GetYuXueModeInfoRequest {
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
pub struct GetYuXueModeInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GetYuXueModeInfoResponse {
    pub const MSG_ID: u16 = 17403;
}

impl PbMessage for GetYuXueModeInfoResponse {
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
pub struct GiantActivityNodeActiveNotify {
    pub ActiveIds: Vec<i32>,
}

impl GiantActivityNodeActiveNotify {
    pub const MSG_ID: u16 = 18243;
}

impl PbMessage for GiantActivityNodeActiveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActiveIds {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ActiveIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantDeliveryItemRequest {
    pub GiantEcoType: Option<GiantEcoNetType>,
    pub ItemInfos: BTreeMap<i32, i32>,
}

impl GiantDeliveryItemRequest {
    pub const MSG_ID: u16 = 26525;
}

impl PbMessage for GiantDeliveryItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantEcoType {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemInfos.is_empty() {
            crate::wire::write_map(out, 15, &self.ItemInfos);
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
                self.GiantEcoType = Some(
                    <GiantEcoNetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => crate::wire::decode_map_entry(r, &mut self.ItemInfos)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantDeliveryItemResponse {
    pub HasReward: bool,
    pub ErrorCode: Option<ErrorCode>,
    pub TopQuality: i32,
    pub GiantEcoInfo: Option<GiantEcoInfoPb>,
}

impl GiantDeliveryItemResponse {
    pub const MSG_ID: u16 = 18294;
}

impl PbMessage for GiantDeliveryItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HasReward {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TopQuality != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TopQuality);
        }
        if let Some(v) = &self.GiantEcoInfo {
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
            3 => self.HasReward = r.read_bool()?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => self.TopQuality = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.GiantEcoInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantEcoInfoPb {
    pub GiantEcoType: Option<GiantEcoNetType>,
    pub Level: i32,
    pub StableValue: i32,
    pub Status: Option<GiantEcoNetStatus>,
}

impl PbMessage for GiantEcoInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantEcoType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.StableValue != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StableValue);
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
            1 => {
                self.GiantEcoType = Some(
                    <GiantEcoNetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => self.Level = r.read_varint_i32()?,
            3 => self.StableValue = r.read_varint_i32()?,
            4 => {
                self.Status = Some(
                    <GiantEcoNetStatus as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GiantEcoUnlockNotify {
    pub GiantEcoType: Option<GiantEcoNetType>,
}

impl GiantEcoUnlockNotify {
    pub const MSG_ID: u16 = 29556;
}

impl PbMessage for GiantEcoUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantEcoType {
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
                self.GiantEcoType = Some(
                    <GiantEcoNetType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GiantItemAccessPb {
    pub AccessId: i32,
    pub ItemInfos: BTreeMap<i32, i32>,
    pub AccessStatus: Option<GiantItemAccessStatus>,
    pub IsFinish: bool,
}

impl PbMessage for GiantItemAccessPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AccessId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccessId);
        }
        if !self.ItemInfos.is_empty() {
            crate::wire::write_map(out, 2, &self.ItemInfos);
        }
        if let Some(v) = &self.AccessStatus {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsFinish {
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
            1 => self.AccessId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.ItemInfos)?,
            3 => {
                self.AccessStatus = Some(
                    <GiantItemAccessStatus as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantItemAccessProgressChangeNotify {
    pub GiantItemAccessPb: Option<GiantItemAccessPb>,
}

impl GiantItemAccessProgressChangeNotify {
    pub const MSG_ID: u16 = 25359;
}

impl PbMessage for GiantItemAccessProgressChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantItemAccessPb {
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
                self.GiantItemAccessPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantItemAccessUnlockNotify {
    pub GiantItemAccessPbs: Vec<GiantItemAccessPb>,
}

impl GiantItemAccessUnlockNotify {
    pub const MSG_ID: u16 = 23920;
}

impl PbMessage for GiantItemAccessUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GiantItemAccessPbs {
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
                self.GiantItemAccessPbs
                    .push(<GiantItemAccessPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantItemUnlockNotify {
    pub UnlockItemIds: Vec<i32>,
}

impl GiantItemUnlockNotify {
    pub const MSG_ID: u16 = 25972;
}

impl PbMessage for GiantItemUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockItemIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockItemIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantMaxLevelDeliveryItemRequest {
    pub GiantEcoType: Option<GiantEcoNetType>,
    pub ItemInfos: BTreeMap<i32, i32>,
}

impl GiantMaxLevelDeliveryItemRequest {
    pub const MSG_ID: u16 = 24629;
}

impl PbMessage for GiantMaxLevelDeliveryItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantEcoType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemInfos.is_empty() {
            crate::wire::write_map(out, 9, &self.ItemInfos);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.GiantEcoType = Some(
                    <GiantEcoNetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            9 => crate::wire::decode_map_entry(r, &mut self.ItemInfos)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantMaxLevelDeliveryItemResponse {
    pub GiantEcoInfo: Option<GiantEcoInfoPb>,
    pub TopQuality: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub HasReward: bool,
}

impl GiantMaxLevelDeliveryItemResponse {
    pub const MSG_ID: u16 = 28669;
}

impl PbMessage for GiantMaxLevelDeliveryItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantEcoInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.TopQuality != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TopQuality);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HasReward {
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
                self.GiantEcoInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.TopQuality = r.read_varint_i32()?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.HasReward = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantModelEnableNotify {
    pub Enable: bool,
}

impl GiantModelEnableNotify {
    pub const MSG_ID: u16 = 17776;
}

impl PbMessage for GiantModelEnableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Enable {
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
            4 => self.Enable = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantResponseNotify {
    pub GiantIdleType: Option<GiantIdleNetType>,
    pub GiantResponseInfos: Vec<GiantResponsePb>,
}

impl GiantResponseNotify {
    pub const MSG_ID: u16 = 18571;
}

impl PbMessage for GiantResponseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantIdleType {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GiantResponseInfos {
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
            6 => {
                self.GiantIdleType = Some(
                    <GiantIdleNetType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.GiantResponseInfos
                    .push(<GiantResponsePb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantResponsePb {
    pub GiantResponseType: Option<GiantResponseType>,
    pub GiantResponsePlayAnimation: Option<GiantResponsePlayAnimationPb>,
    pub GiantResponsePlayFlow: Option<GiantResponsePlayFlowPb>,
}

impl PbMessage for GiantResponsePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GiantResponseType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.GiantResponsePlayAnimation {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.GiantResponsePlayFlow {
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
                self.GiantResponseType = Some(
                    <GiantResponseType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GiantResponsePlayAnimation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.GiantResponsePlayFlow
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantResponsePlayAnimationPb {
    pub GiantAnimationId: i32,
}

impl PbMessage for GiantResponsePlayAnimationPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GiantAnimationId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GiantAnimationId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GiantAnimationId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantResponsePlayFlowPb {
    pub FlowListName: Vec<u8>,
    pub FlowId: i32,
    pub StateId: i32,
}

impl PbMessage for GiantResponsePlayFlowPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.FlowListName.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.FlowListName);
        }
        if self.FlowId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FlowId);
        }
        if self.StateId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FlowListName = r.read_len_delimited()?.to_vec(),
            2 => self.FlowId = r.read_varint_i32()?,
            3 => self.StateId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantStableValueActivityRequest {
    pub RewardIds: Vec<i32>,
}

impl GiantStableValueActivityRequest {
    pub const MSG_ID: u16 = 20090;
}

impl PbMessage for GiantStableValueActivityRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIds {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantStableValueActivityResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub GotRewardIds: Vec<i32>,
}

impl GiantStableValueActivityResponse {
    pub const MSG_ID: u16 = 27413;
}

impl PbMessage for GiantStableValueActivityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.GotRewardIds {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GotRewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantSystemBoardData {
    pub GiantEcoType: i32,
}

impl PbMessage for GiantSystemBoardData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GiantEcoType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GiantEcoType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GiantEcoType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiantSystemInfoNotify {
    pub GiantItemAccessPbs: Vec<GiantItemAccessPb>,
    pub UnlockItemIds: Vec<i32>,
    pub DisableGiantModel: bool,
    pub GiantEcoInfos: Vec<GiantEcoInfoPb>,
    pub GiantIdleType: Option<GiantIdleNetType>,
}

impl GiantSystemInfoNotify {
    pub const MSG_ID: u16 = 29120;
}

impl PbMessage for GiantSystemInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GiantItemAccessPbs {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.UnlockItemIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.DisableGiantModel {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.GiantEcoInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        if let Some(v) = &self.GiantIdleType {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GiantItemAccessPbs
                    .push(<GiantItemAccessPb as PbMessage>::decode(bytes)?);
            }
            5 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockItemIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            11 => self.DisableGiantModel = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.GiantEcoInfos
                    .push(<GiantEcoInfoPb as PbMessage>::decode(bytes)?);
            }
            13 => {
                self.GiantIdleType = Some(
                    <GiantIdleNetType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GiantTotalStableValueUpdateNotify {
    pub TotalCount: i32,
    pub NewRedDot: bool,
}

impl GiantTotalStableValueUpdateNotify {
    pub const MSG_ID: u16 = 27023;
}

impl PbMessage for GiantTotalStableValueUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCount);
        }
        if self.NewRedDot {
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
            3 => self.TotalCount = r.read_varint_i32()?,
            13 => self.NewRedDot = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiftPackPreviewRequest {
    pub GiftPackId: i32,
}

impl GiftPackPreviewRequest {
    pub const MSG_ID: u16 = 29403;
}

impl PbMessage for GiftPackPreviewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GiftPackId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GiftPackId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.GiftPackId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GiftPackPreviewResponse {
    pub PhantomPreview: BTreeMap<i32, PhantomItem>,
}

impl GiftPackPreviewResponse {
    pub const MSG_ID: u16 = 29486;
}

impl PbMessage for GiftPackPreviewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PhantomPreview.is_empty() {
            crate::wire::write_map(out, 6, &self.PhantomPreview);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_map_entry(r, &mut self.PhantomPreview)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackInfoPb {
    pub Task: Vec<GivebackTaskPb>,
    pub TotalScore: i32,
    pub RewardedIds: Vec<i32>,
    pub ActiveDay: i32,
}

impl PbMessage for GivebackInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Task {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.TotalScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalScore);
        }
        for item in &self.RewardedIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActiveDay != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActiveDay);
        }
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
                    .push(<GivebackTaskPb as PbMessage>::decode(bytes)?);
            }
            2 => self.TotalScore = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardedIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.ActiveDay = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackInfoUpdateNotify {
    pub GivebackInfo: Option<GivebackInfoPb>,
}

impl GivebackInfoUpdateNotify {
    pub const MSG_ID: u16 = 27995;
}

impl PbMessage for GivebackInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GivebackInfo {
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
                self.GivebackInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackRewardRequest {
    pub RwdId: Vec<i32>,
    pub RewardId: i32,
}

impl GivebackRewardRequest {
    pub const MSG_ID: u16 = 23197;
}

impl PbMessage for GivebackRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RwdId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RwdId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.RewardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GivebackRewardResponse {
    pub const MSG_ID: u16 = 20769;
}

impl PbMessage for GivebackRewardResponse {
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
pub struct GivebackTaskPb {
    pub Id: i32,
    pub Score: i32,
}

impl PbMessage for GivebackTaskPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GivebackUnlockNotify {
    pub GivebackInfo: Option<GivebackInfoPb>,
}

impl GivebackUnlockNotify {
    pub const MSG_ID: u16 = 15923;
}

impl PbMessage for GivebackUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GivebackInfo {
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
                self.GivebackInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmEntityFsmGroupInfoRequest {
    pub EntityId: i64,
}

impl GmEntityFsmGroupInfoRequest {
    pub const MSG_ID: u16 = 24535;
}

impl PbMessage for GmEntityFsmGroupInfoRequest {
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
pub struct GmEntityFsmGroupInfoResponse {
    pub EntityId: i64,
    pub GmEntityFsmMachineInfos: Vec<GmEntityFsmMachineInfo>,
}

impl GmEntityFsmGroupInfoResponse {
    pub const MSG_ID: u16 = 19279;
}

impl PbMessage for GmEntityFsmGroupInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.GmEntityFsmMachineInfos {
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
            2 => self.EntityId = r.read_varint_i64()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.GmEntityFsmMachineInfos
                    .push(<GmEntityFsmMachineInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmEntityFsmMachineInfo {
    pub StateIndex: i32,
    pub GmEntityFsmStateInfos: Vec<GmEntityFsmStateInfo>,
}

impl PbMessage for GmEntityFsmMachineInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StateIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateIndex);
        }
        for item in &self.GmEntityFsmStateInfos {
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
            1 => self.StateIndex = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GmEntityFsmStateInfos
                    .push(<GmEntityFsmStateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmEntityFsmStateInfo {
    pub StateIndex: i32,
    pub GmEntityFsmTransitionInfos: Vec<GmEntityFsmTransitionInfo>,
}

impl PbMessage for GmEntityFsmStateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StateIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StateIndex);
        }
        for item in &self.GmEntityFsmTransitionInfos {
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
            1 => self.StateIndex = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.GmEntityFsmTransitionInfos
                    .push(<GmEntityFsmTransitionInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmEntityFsmTransitionInfo {
    pub ToStateIndex: i32,
    pub GmEntityFsmConditions: Vec<bool>,
}

impl PbMessage for GmEntityFsmTransitionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ToStateIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToStateIndex);
        }
        for item in &self.GmEntityFsmConditions {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(*item));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ToStateIndex = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.GmEntityFsmConditions,
                |sr| Ok(sr.read_bool()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmLevelActionRequest {
    pub JsonStr: Vec<u8>,
}

impl GmLevelActionRequest {
    pub const MSG_ID: u16 = 29805;
}

impl PbMessage for GmLevelActionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.JsonStr.is_empty() {
            crate::wire::write_tag(out, 7, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.JsonStr);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.JsonStr = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GmLevelActionResponse {
    pub Code: Option<ErrorCode>,
}

impl GmLevelActionResponse {
    pub const MSG_ID: u16 = 26699;
}

impl PbMessage for GmLevelActionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct GolemCrackUpdateRequest {
    pub ActivityId: i32,
    pub GolemCrackLevelInfo: Option<GolemCrackLevelInfo>,
}

impl GolemCrackUpdateRequest {
    pub const MSG_ID: u16 = 18010;
}

impl PbMessage for GolemCrackUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.GolemCrackLevelInfo {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.GolemCrackLevelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GolemCrackUpdateResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl GolemCrackUpdateResponse {
    pub const MSG_ID: u16 = 25396;
}

impl PbMessage for GolemCrackUpdateResponse {
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
pub struct GoodsDetail {
    pub Index: i32,
    pub IsLock: bool,
    pub IsNew: bool,
    pub IsSell: bool,
    pub SourcePrice: i32,
    pub CurPrice: i32,
    pub ItemId: i32,
    pub SurvivorsGainData: Option<SurvivorsGainData>,
}

impl PbMessage for GoodsDetail {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.IsLock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSell {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
        if let Some(v) = &self.SurvivorsGainData {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => self.IsLock = r.read_bool()?,
            3 => self.IsNew = r.read_bool()?,
            4 => self.IsSell = r.read_bool()?,
            5 => self.SourcePrice = r.read_varint_i32()?,
            6 => self.CurPrice = r.read_varint_i32()?,
            7 => self.ItemId = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SurvivorsGainData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GravityFlipComponentPb {
    pub CurGravityFlipType: Option<GravityFlipType>,
}

impl PbMessage for GravityFlipComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurGravityFlipType {
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
                self.CurGravityFlipType = Some(
                    <GravityFlipType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GravityFlipInteractRequest {
    pub GravityFlipEntityId: i64,
    pub TargetGravityFlipType: Option<GravityFlipType>,
}

impl GravityFlipInteractRequest {
    pub const MSG_ID: u16 = 24889;
}

impl PbMessage for GravityFlipInteractRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GravityFlipEntityId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.GravityFlipEntityId);
        }
        if let Some(v) = &self.TargetGravityFlipType {
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
            9 => self.GravityFlipEntityId = r.read_varint_i64()?,
            13 => {
                self.TargetGravityFlipType = Some(
                    <GravityFlipType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct GravityFlipInteractResponse {
    pub CurGravityFlipType: Option<GravityFlipType>,
    pub ErrorCode: Option<ErrorCode>,
}

impl GravityFlipInteractResponse {
    pub const MSG_ID: u16 = 19707;
}

impl PbMessage for GravityFlipInteractResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurGravityFlipType {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            8 => {
                self.CurGravityFlipType = Some(
                    <GravityFlipType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
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
pub struct GravityFlipUpdateTypeNotify {
    pub NewGravityFlipType: Option<GravityFlipType>,
    pub EntityId: i64,
}

impl GravityFlipUpdateTypeNotify {
    pub const MSG_ID: u16 = 29623;
}

impl PbMessage for GravityFlipUpdateTypeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewGravityFlipType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            4 => {
                self.NewGravityFlipType = Some(
                    <GravityFlipType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GravityGearRequest {
    pub GravityGearOperate: Option<GravityGearOperate>,
    pub EntityConfId: i32,
}

impl GravityGearRequest {
    pub const MSG_ID: u16 = 28261;
}

impl PbMessage for GravityGearRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GravityGearOperate {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EntityConfId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfId);
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
                self.GravityGearOperate = Some(
                    <GravityGearOperate as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.EntityConfId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GravityGearResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GravityGearResponse {
    pub const MSG_ID: u16 = 27144;
}

impl PbMessage for GravityGearResponse {
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
pub struct GravityHookLockPointChangeGravityRequest {
    pub EntityId: i64,
}

impl GravityHookLockPointChangeGravityRequest {
    pub const MSG_ID: u16 = 27548;
}

impl PbMessage for GravityHookLockPointChangeGravityRequest {
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
pub struct GravityHookLockPointChangeGravityResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl GravityHookLockPointChangeGravityResponse {
    pub const MSG_ID: u16 = 22328;
}

impl PbMessage for GravityHookLockPointChangeGravityResponse {
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
pub struct GreatSwordChallengeData {
    pub Id: i32,
}

impl PbMessage for GreatSwordChallengeData {
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
pub struct GridCellUpdateNotify {
    pub LevelGridData: Option<LevelGridPbData>,
    pub IsAdd: bool,
}

impl GridCellUpdateNotify {
    pub const MSG_ID: u16 = 28675;
}

impl PbMessage for GridCellUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.LevelGridData {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.IsAdd {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.LevelGridData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.IsAdd = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridEventData {
    pub GridIndex: i32,
    pub EventId: i32,
    pub State: Option<GridEventState>,
    pub IsInjured: bool,
    pub Steps: Vec<i32>,
}

impl PbMessage for GridEventData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GridIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridIndex);
        }
        if self.EventId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsInjured {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Steps {
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
            1 => self.GridIndex = r.read_varint_i32()?,
            2 => self.EventId = r.read_varint_i32()?,
            3 => {
                self.State = Some(
                    <GridEventState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.IsInjured = r.read_bool()?,
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Steps, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridEventOp {
    pub GridEventData: Option<GridEventData>,
    pub ClientStepState: Option<ClientStepState>,
    pub NoStep: i32,
    pub SpecialEffectInfo: Option<SpecialEffectInfo>,
    pub EventPlotInfo: Option<EventPlotInfo>,
}

impl PbMessage for GridEventOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.GridEventData {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ClientStepState {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.NoStep != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NoStep);
        }
        if let Some(v) = &self.SpecialEffectInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.EventPlotInfo {
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
                self.GridEventData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                self.ClientStepState = Some(
                    <ClientStepState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.NoStep = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SpecialEffectInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.EventPlotInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridTakeView {
    pub OldTeamLevel: i32,
    pub NewTeamLevel: i32,
    pub Effects: Vec<i32>,
    pub Index: i32,
    pub PreChangeMood: i32,
}

impl PbMessage for GridTakeView {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OldTeamLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OldTeamLevel);
        }
        if self.NewTeamLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewTeamLevel);
        }
        for item in &self.Effects {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.PreChangeMood != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreChangeMood);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.OldTeamLevel = r.read_varint_i32()?,
            2 => self.NewTeamLevel = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Effects, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.Index = r.read_varint_i32()?,
            5 => self.PreChangeMood = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridVision {
    pub Grids: Vec<i32>,
}

impl PbMessage for GridVision {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Grids {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Grids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuessJokerLevelInfoArray {
    pub GuessJokerLevelInfos: Vec<GuessJokerLevelInfo>,
}

impl PbMessage for GuessJokerLevelInfoArray {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GuessJokerLevelInfos {
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
                self.GuessJokerLevelInfos
                    .push(<GuessJokerLevelInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideCondDoneNotify {
    pub GroupId: i32,
}

impl GuideCondDoneNotify {
    pub const MSG_ID: u16 = 18752;
}

impl PbMessage for GuideCondDoneNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.GroupId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GuideGroupResetNotify {
    pub GroupIds: Vec<i32>,
}

impl GuideGroupResetNotify {
    pub const MSG_ID: u16 = 29638;
}

impl PbMessage for GuideGroupResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GroupIds {
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
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GroupIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct H5CircumFluenceActivityData {
    pub InviteCode: Vec<u8>,
    pub Score: i32,
    pub RedDot: bool,
    pub AllRewardClaimed: bool,
}

impl PbMessage for H5CircumFluenceActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.InviteCode.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.InviteCode);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.RedDot {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.AllRewardClaimed {
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
            1 => self.InviteCode = r.read_len_delimited()?.to_vec(),
            2 => self.Score = r.read_varint_i32()?,
            3 => self.RedDot = r.read_bool()?,
            4 => self.AllRewardClaimed = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct H5CircumFluenceActivityDataNotify {
    pub ActivityData: Option<H5CircumFluenceActivityData>,
}

impl H5CircumFluenceActivityDataNotify {
    pub const MSG_ID: u16 = 16402;
}

impl PbMessage for H5CircumFluenceActivityDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityData {
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
                self.ActivityData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct H5CircumFluenceActivityReadRequest {}

impl H5CircumFluenceActivityReadRequest {
    pub const MSG_ID: u16 = 20511;
}

impl PbMessage for H5CircumFluenceActivityReadRequest {
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
pub struct H5CircumFluenceActivityReadResponse {}

impl H5CircumFluenceActivityReadResponse {
    pub const MSG_ID: u16 = 22858;
}

impl PbMessage for H5CircumFluenceActivityReadResponse {
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
pub struct H5ViewActivityDataNotify {
    pub ActivityData: Option<H5ViewActivityData>,
}

impl H5ViewActivityDataNotify {
    pub const MSG_ID: u16 = 28624;
}

impl PbMessage for H5ViewActivityDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ActivityData {
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
                self.ActivityData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct H5ViewActivityReadRequest {
    pub ActivityId: i32,
}

impl H5ViewActivityReadRequest {
    pub const MSG_ID: u16 = 22263;
}

impl PbMessage for H5ViewActivityReadRequest {
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
pub struct H5ViewActivityReadResponse {}

impl H5ViewActivityReadResponse {
    pub const MSG_ID: u16 = 15198;
}

impl PbMessage for H5ViewActivityReadResponse {
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
pub struct HalfEndlessBattleRecord {
    pub BattleFormation: Option<BattleFormation>,
    pub MaxScore: i32,
    pub MaxRound: i32,
    pub RoleLevels: Vec<i32>,
}

impl PbMessage for HalfEndlessBattleRecord {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.BattleFormation {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.MaxRound != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxRound);
        }
        for item in &self.RoleLevels {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.BattleFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.MaxScore = r.read_varint_i32()?,
            3 => self.MaxRound = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleLevels, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInInfoUpdateNotify {
    pub HandInInfo: Option<HandInInfo>,
}

impl HandInInfoUpdateNotify {
    pub const MSG_ID: u16 = 25098;
}

impl PbMessage for HandInInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HandInInfo {
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
                self.HandInInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInItemRequest {
    pub HandInItem: Vec<PbHandInItem>,
    pub TreeOwnerId: i32,
    pub NodeId: i32,
    pub TreeIncId: i64,
}

impl HandInItemRequest {
    pub const MSG_ID: u16 = 21324;
}

impl PbMessage for HandInItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HandInItem {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.NodeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NodeId);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HandInItem
                    .push(<PbHandInItem as PbMessage>::decode(bytes)?);
            }
            8 => self.TreeOwnerId = r.read_varint_i32()?,
            13 => self.NodeId = r.read_varint_i32()?,
            15 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInItemResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl HandInItemResponse {
    pub const MSG_ID: u16 = 21737;
}

impl PbMessage for HandInItemResponse {
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
pub struct HandInMingSuRequest {
    pub InteractEntityId: i64,
    pub DragonPoolId: i32,
}

impl HandInMingSuRequest {
    pub const MSG_ID: u16 = 20726;
}

impl PbMessage for HandInMingSuRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InteractEntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.InteractEntityId);
        }
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            3 => self.InteractEntityId = r.read_varint_i64()?,
            5 => self.DragonPoolId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HandInMingSuResponse {
    pub InjectedCoreItemCount: i32,
    pub Level: i32,
    pub DragonPoolId: i32,
    pub LevelGains: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl HandInMingSuResponse {
    pub const MSG_ID: u16 = 25984;
}

impl PbMessage for HandInMingSuResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InjectedCoreItemCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InjectedCoreItemCount);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.DragonPoolId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DragonPoolId);
        }
        if self.LevelGains != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelGains);
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
            6 => self.InjectedCoreItemCount = r.read_varint_i32()?,
            8 => self.Level = r.read_varint_i32()?,
            10 => self.DragonPoolId = r.read_varint_i32()?,
            13 => self.LevelGains = r.read_varint_i32()?,
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
pub struct HardnessModeChangedNotify {
    pub EntityId: i64,
    pub HardnessModeId: i32,
}

impl HardnessModeChangedNotify {
    pub const MSG_ID: u16 = 22866;
}

impl PbMessage for HardnessModeChangedNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HardnessModeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HardnessModeId);
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
            4 => self.HardnessModeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HardnessModeChangedRequest {
    pub HardnessModeId: i32,
    pub EntityId: i64,
}

impl HardnessModeChangedRequest {
    pub const MSG_ID: u16 = 19311;
}

impl PbMessage for HardnessModeChangedRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HardnessModeId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HardnessModeId);
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
            4 => self.HardnessModeId = r.read_varint_i32()?,
            7 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HardnessModeChangedResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HardnessModeChangedResponse {
    pub const MSG_ID: u16 = 25063;
}

impl PbMessage for HardnessModeChangedResponse {
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
pub struct HarvestActivityLevelDiffRequest {
    pub ActivityId: i32,
    pub InstId: i32,
    pub Diff: i32,
}

impl HarvestActivityLevelDiffRequest {
    pub const MSG_ID: u16 = 16220;
}

impl PbMessage for HarvestActivityLevelDiffRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.Diff != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Diff);
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
            5 => self.InstId = r.read_varint_i32()?,
            7 => self.Diff = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityLevelDiffResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HarvestActivityLevelDiffResponse {
    pub const MSG_ID: u16 = 28299;
}

impl PbMessage for HarvestActivityLevelDiffResponse {
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
pub struct HarvestActivityLevelNotify {
    pub HarvestLevelRewards: Vec<HarvestLevelReward>,
    pub ActivityId: i32,
}

impl HarvestActivityLevelNotify {
    pub const MSG_ID: u16 = 22145;
}

impl PbMessage for HarvestActivityLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HarvestLevelRewards {
            crate::wire::write_submessage(out, 9, item);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.HarvestLevelRewards
                    .push(<HarvestLevelReward as PbMessage>::decode(bytes)?);
            }
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityLevelRequest {
    pub ActivityId: i32,
    pub InstId: i32,
}

impl HarvestActivityLevelRequest {
    pub const MSG_ID: u16 = 27602;
}

impl PbMessage for HarvestActivityLevelRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            6 => self.ActivityId = r.read_varint_i32()?,
            14 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityLevelResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl HarvestActivityLevelResponse {
    pub const MSG_ID: u16 = 23959;
}

impl PbMessage for HarvestActivityLevelResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 4, &self.ItemMap);
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
            4 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityPointNotify {
    pub HarvestPointReward: Vec<HarvestPointReward>,
    pub ActivityId: i32,
}

impl HarvestActivityPointNotify {
    pub const MSG_ID: u16 = 23843;
}

impl PbMessage for HarvestActivityPointNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HarvestPointReward {
            crate::wire::write_submessage(out, 2, item);
        }
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HarvestPointReward
                    .push(<HarvestPointReward as PbMessage>::decode(bytes)?);
            }
            3 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityPointRequest {
    pub Id: i32,
    pub ActivityId: i32,
}

impl HarvestActivityPointRequest {
    pub const MSG_ID: u16 = 15260;
}

impl PbMessage for HarvestActivityPointRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            6 => self.Id = r.read_varint_i32()?,
            9 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HarvestActivityPointResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl HarvestActivityPointResponse {
    pub const MSG_ID: u16 = 26658;
}

impl PbMessage for HarvestActivityPointResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 2, &self.ItemMap);
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
            2 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct HarvestActivityResultNotify {
    pub HisPoint: i32,
    pub Succ: bool,
    pub CurPoint: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl HarvestActivityResultNotify {
    pub const MSG_ID: u16 = 22031;
}

impl PbMessage for HarvestActivityResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HisPoint != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HisPoint);
        }
        if self.Succ {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurPoint != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPoint);
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
            5 => self.HisPoint = r.read_varint_i32()?,
            7 => self.Succ = r.read_bool()?,
            12 => self.CurPoint = r.read_varint_i32()?,
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
pub struct HarvestActivityResultRequest {}

impl HarvestActivityResultRequest {
    pub const MSG_ID: u16 = 22854;
}

impl PbMessage for HarvestActivityResultRequest {
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
pub struct HarvestActivityResultResponse {}

impl HarvestActivityResultResponse {
    pub const MSG_ID: u16 = 19031;
}

impl PbMessage for HarvestActivityResultResponse {
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
pub struct HeadIdUpdateNotify {
    pub HeadId: i32,
}

impl HeadIdUpdateNotify {
    pub const MSG_ID: u16 = 24865;
}

impl PbMessage for HeadIdUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            5 => self.HeadId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HoldHandRequest {
    pub IsHold: bool,
    pub ActionType: i32,
    pub TargetEntityId: i64,
    pub HandType: i32,
    pub EntityId: i64,
    pub HostPlayerId: i32,
}

impl HoldHandRequest {
    pub const MSG_ID: u16 = 15637;
}

impl PbMessage for HoldHandRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsHold {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ActionType != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActionType);
        }
        if self.TargetEntityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetEntityId);
        }
        if self.HandType != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandType);
        }
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.HostPlayerId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            1 => self.IsHold = r.read_bool()?,
            3 => self.ActionType = r.read_varint_i32()?,
            5 => self.TargetEntityId = r.read_varint_i64()?,
            11 => self.HandType = r.read_varint_i32()?,
            12 => self.EntityId = r.read_varint_i64()?,
            14 => self.HostPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HoldHandResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HoldHandResponse {
    pub const MSG_ID: u16 = 26676;
}

impl PbMessage for HoldHandResponse {
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
pub struct HoldWeaponNotify {
    pub WeaponConfId: i32,
    pub EntityId: i64,
}

impl HoldWeaponNotify {
    pub const MSG_ID: u16 = 15700;
}

impl PbMessage for HoldWeaponNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WeaponConfId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponConfId);
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
            3 => self.WeaponConfId = r.read_varint_i32()?,
            5 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HoldWeaponPush {
    pub EntityId: i64,
    pub WeaponEntityId: i64,
}

impl HoldWeaponPush {
    pub const MSG_ID: u16 = 24879;
}

impl PbMessage for HoldWeaponPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.WeaponEntityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.WeaponEntityId);
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
            8 => self.WeaponEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryActivateTalentRequest {
    pub TalentId: i32,
}

impl HonamiStoryActivateTalentRequest {
    pub const MSG_ID: u16 = 29239;
}

impl PbMessage for HonamiStoryActivateTalentRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.TalentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryActivateTalentResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TalentId: i32,
}

impl HonamiStoryActivateTalentResponse {
    pub const MSG_ID: u16 = 16479;
}

impl PbMessage for HonamiStoryActivateTalentResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.TalentId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryActivityInfo {
    pub HonamiStoryPlayerBagInfo: Option<HonamiStoryPlayerBagInfo>,
    pub ActivatedTalentId: Vec<i32>,
    pub HonamiStoryItemCollectionInfos: Vec<HonamiStoryItemCollectionInfo>,
    pub HonamiStoryMascotInfos: Vec<HonamiStoryMascotInfo>,
    pub HonamiStoryAreaInfos: Vec<HonamiStoryAreaInfo>,
    pub HonamiStoryLimitTask: Vec<ConditionTask>,
    pub AreaTask: Vec<ConditionTask>,
    pub HonamiStoryScoreRewardInfos: Vec<HonamiStoryScoreRewardInfo>,
    pub LifeSupportLevel: i32,
    pub LimitShopConsumeItemNum: i32,
    pub HonamiStoryTopInfos: Vec<HonamiStoryTopInfo>,
    pub HonamiStoryTalentInfos: Vec<HonamiStoryTalentInfo>,
    pub HonamiStoryResidentTask: Vec<ConditionTask>,
    pub TotalRevenue: i32,
}

impl PbMessage for HonamiStoryActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryPlayerBagInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ActivatedTalentId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.HonamiStoryItemCollectionInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.HonamiStoryMascotInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.HonamiStoryAreaInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.HonamiStoryLimitTask {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.AreaTask {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.HonamiStoryScoreRewardInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.LifeSupportLevel != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LifeSupportLevel);
        }
        if self.LimitShopConsumeItemNum != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitShopConsumeItemNum);
        }
        for item in &self.HonamiStoryTopInfos {
            crate::wire::write_submessage(out, 12, item);
        }
        for item in &self.HonamiStoryTalentInfos {
            crate::wire::write_submessage(out, 13, item);
        }
        for item in &self.HonamiStoryResidentTask {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.TotalRevenue != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryPlayerBagInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ActivatedTalentId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryItemCollectionInfos
                    .push(<HonamiStoryItemCollectionInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryMascotInfos
                    .push(<HonamiStoryMascotInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryAreaInfos
                    .push(<HonamiStoryAreaInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryLimitTask
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AreaTask
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryScoreRewardInfos
                    .push(<HonamiStoryScoreRewardInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.LifeSupportLevel = r.read_varint_i32()?,
            11 => self.LimitShopConsumeItemNum = r.read_varint_i32()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryTopInfos
                    .push(<HonamiStoryTopInfo as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryTalentInfos
                    .push(<HonamiStoryTalentInfo as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryResidentTask
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            15 => self.TotalRevenue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryActivityInfoRequest {
    pub ActivityId: i32,
}

impl HonamiStoryActivityInfoRequest {
    pub const MSG_ID: u16 = 20790;
}

impl PbMessage for HonamiStoryActivityInfoRequest {
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
pub struct HonamiStoryActivityInfoResponse {
    pub HonamiStoryActivityInfo: Option<HonamiStoryActivityInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryActivityInfoResponse {
    pub const MSG_ID: u16 = 24959;
}

impl PbMessage for HonamiStoryActivityInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryActivityInfo {
            crate::wire::write_submessage(out, 2, v);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryActivityInfo
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
pub struct HonamiStoryAddWeaponNotify {
    pub WeaponIds: Vec<i32>,
}

impl HonamiStoryAddWeaponNotify {
    pub const MSG_ID: u16 = 23284;
}

impl PbMessage for HonamiStoryAddWeaponNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.WeaponIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryAreaInfo {
    pub HonamiStoryAreaId: i32,
    pub Status: i32,
    pub SecreteStatus: i32,
}

impl PbMessage for HonamiStoryAreaInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HonamiStoryAreaId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HonamiStoryAreaId);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
        if self.SecreteStatus != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SecreteStatus);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.HonamiStoryAreaId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            3 => self.SecreteStatus = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryAreaInfoUpdateNotify {
    pub HonamiStoryAreaInfo: Vec<HonamiStoryAreaInfo>,
}

impl HonamiStoryAreaInfoUpdateNotify {
    pub const MSG_ID: u16 = 15296;
}

impl PbMessage for HonamiStoryAreaInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryAreaInfo {
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
                self.HonamiStoryAreaInfo
                    .push(<HonamiStoryAreaInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryAreaSecretRewardRequest {
    pub AreaSecretId: i32,
}

impl HonamiStoryAreaSecretRewardRequest {
    pub const MSG_ID: u16 = 25409;
}

impl PbMessage for HonamiStoryAreaSecretRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AreaSecretId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AreaSecretId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.AreaSecretId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryAreaSecretRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryAreaSecretRewardResponse {
    pub const MSG_ID: u16 = 21464;
}

impl PbMessage for HonamiStoryAreaSecretRewardResponse {
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
pub struct HonamiStoryAreaTaskInfoRefreshNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl HonamiStoryAreaTaskInfoRefreshNotify {
    pub const MSG_ID: u16 = 26269;
}

impl PbMessage for HonamiStoryAreaTaskInfoRefreshNotify {
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
pub struct HonamiStoryAreaTaskInfoUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl HonamiStoryAreaTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 25370;
}

impl PbMessage for HonamiStoryAreaTaskInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagInfo {
    pub BagConfigId: i32,
    pub Width: i32,
    pub BagSize: i32,
    pub HonamiStoryBagItemInfos: Vec<HonamiStoryBagItemInfo>,
}

impl PbMessage for HonamiStoryBagInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BagConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BagConfigId);
        }
        if self.Width != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Width);
        }
        if self.BagSize != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BagSize);
        }
        for item in &self.HonamiStoryBagItemInfos {
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
            1 => self.BagConfigId = r.read_varint_i32()?,
            2 => self.Width = r.read_varint_i32()?,
            3 => self.BagSize = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryBagItemInfos
                    .push(<HonamiStoryBagItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagInfoUpdateNotify {
    pub HonamiStoryBagInfos: Vec<HonamiStoryBagInfo>,
}

impl HonamiStoryBagInfoUpdateNotify {
    pub const MSG_ID: u16 = 23184;
}

impl PbMessage for HonamiStoryBagInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryBagInfos {
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
                self.HonamiStoryBagInfos
                    .push(<HonamiStoryBagInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagItemInfo {
    pub HonamiStoryItemInfo: Option<HonamiStoryItemInfo>,
    pub HonamiStoryPosInfo: Option<HonamiStoryPosInfo>,
}

impl PbMessage for HonamiStoryBagItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryItemInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.HonamiStoryPosInfo {
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
                self.HonamiStoryItemInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryPosInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagOperateRequest {
    pub ActivityId: i32,
    pub HonamiStoryBagUpdateContext: Vec<HonamiStoryBagUpdateContext>,
}

impl HonamiStoryBagOperateRequest {
    pub const MSG_ID: u16 = 19759;
}

impl PbMessage for HonamiStoryBagOperateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.HonamiStoryBagUpdateContext {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryBagUpdateContext
                    .push(<HonamiStoryBagUpdateContext as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagOperateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryBagOperateResponse {
    pub const MSG_ID: u16 = 17717;
}

impl PbMessage for HonamiStoryBagOperateResponse {
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
pub struct HonamiStoryBagSizeUpdateNotify {
    pub BagSize: BTreeMap<i32, i32>,
}

impl HonamiStoryBagSizeUpdateNotify {
    pub const MSG_ID: u16 = 24747;
}

impl PbMessage for HonamiStoryBagSizeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.BagSize.is_empty() {
            crate::wire::write_map(out, 9, &self.BagSize);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => crate::wire::decode_map_entry(r, &mut self.BagSize)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagUpdateContext {
    pub BackPackConfigId: i32,
    pub HonamiStoryBagUpdateInfo: Vec<HonamiStoryBagUpdateInfo>,
}

impl PbMessage for HonamiStoryBagUpdateContext {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BackPackConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BackPackConfigId);
        }
        for item in &self.HonamiStoryBagUpdateInfo {
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
            1 => self.BackPackConfigId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryBagUpdateInfo
                    .push(<HonamiStoryBagUpdateInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryBagUpdateInfo {
    pub ItemIncrId: i32,
    pub Type: i32,
    pub HonamiStoryPosInfo: Option<HonamiStoryPosInfo>,
    pub OriHonamiStoryPosInfo: Option<HonamiStoryPosInfo>,
}

impl PbMessage for HonamiStoryBagUpdateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemIncrId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemIncrId);
        }
        if self.Type != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Type);
        }
        if let Some(v) = &self.HonamiStoryPosInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.OriHonamiStoryPosInfo {
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
            1 => self.ItemIncrId = r.read_varint_i32()?,
            2 => self.Type = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryPosInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.OriHonamiStoryPosInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryDiscardItemRequest {
    pub HonamiStoryBagUpdateContext: Option<HonamiStoryBagUpdateContext>,
}

impl HonamiStoryDiscardItemRequest {
    pub const MSG_ID: u16 = 19973;
}

impl PbMessage for HonamiStoryDiscardItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryBagUpdateContext {
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
                self.HonamiStoryBagUpdateContext
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryDiscardItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryDiscardItemResponse {
    pub const MSG_ID: u16 = 20810;
}

impl PbMessage for HonamiStoryDiscardItemResponse {
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
pub struct HonamiStoryEnterCtx {
    pub IsUpTower: bool,
    pub DangerLevel: i32,
    pub ActivityId: i32,
    pub BuySafeLeave: bool,
}

impl PbMessage for HonamiStoryEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsUpTower {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.DangerLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangerLevel);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.BuySafeLeave {
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
            1 => self.IsUpTower = r.read_bool()?,
            2 => self.DangerLevel = r.read_varint_i32()?,
            3 => self.ActivityId = r.read_varint_i32()?,
            4 => self.BuySafeLeave = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryEquipRoleRequest {
    pub RoleId: Vec<i32>,
    pub ActivityId: i32,
}

impl HonamiStoryEquipRoleRequest {
    pub const MSG_ID: u16 = 21590;
}

impl PbMessage for HonamiStoryEquipRoleRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryEquipRoleResponse {
    pub RoleId: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
    pub ActivityId: i32,
}

impl HonamiStoryEquipRoleResponse {
    pub const MSG_ID: u16 = 27622;
}

impl PbMessage for HonamiStoryEquipRoleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ErrorCode {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct HonamiStoryInstInfoNotify {
    pub DangerLevelId: i32,
    pub CanSafeLeave: bool,
    pub PollutionGroup: i32,
    pub PollutionLevel: i32,
    pub HonamiStoryBagInfos: Vec<HonamiStoryBagInfo>,
    pub PollutionStartTime: i64,
    pub HonamiStoryAreaId: i32,
    pub ActivityId: i32,
}

impl HonamiStoryInstInfoNotify {
    pub const MSG_ID: u16 = 17000;
}

impl PbMessage for HonamiStoryInstInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DangerLevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangerLevelId);
        }
        if self.CanSafeLeave {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PollutionGroup != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PollutionGroup);
        }
        if self.PollutionLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PollutionLevel);
        }
        for item in &self.HonamiStoryBagInfos {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.PollutionStartTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PollutionStartTime);
        }
        if self.HonamiStoryAreaId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HonamiStoryAreaId);
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
            2 => self.DangerLevelId = r.read_varint_i32()?,
            5 => self.CanSafeLeave = r.read_bool()?,
            7 => self.PollutionGroup = r.read_varint_i32()?,
            8 => self.PollutionLevel = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.HonamiStoryBagInfos
                    .push(<HonamiStoryBagInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.PollutionStartTime = r.read_varint_i64()?,
            11 => self.HonamiStoryAreaId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryInstLeaveRequest {}

impl HonamiStoryInstLeaveRequest {
    pub const MSG_ID: u16 = 19853;
}

impl PbMessage for HonamiStoryInstLeaveRequest {
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
pub struct HonamiStoryInstLeaveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryInstLeaveResponse {
    pub const MSG_ID: u16 = 23019;
}

impl PbMessage for HonamiStoryInstLeaveResponse {
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
pub struct HonamiStoryInstSettleNotify {
    pub TalentAdd: i32,
    pub UseTime: i32,
    pub FailAddProportion: i32,
    pub IsDeadSettle: bool,
    pub ItemTotalValue: i32,
    pub IsSuccess: bool,
    pub TotalValue: i32,
    pub IsUpdateTotalValue: bool,
    pub InnerCoin: i32,
}

impl HonamiStoryInstSettleNotify {
    pub const MSG_ID: u16 = 23167;
}

impl PbMessage for HonamiStoryInstSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentAdd != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentAdd);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        if self.FailAddProportion != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FailAddProportion);
        }
        if self.IsDeadSettle {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ItemTotalValue != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemTotalValue);
        }
        if self.IsSuccess {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TotalValue != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalValue);
        }
        if self.IsUpdateTotalValue {
            crate::wire::write_tag(out, 14, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InnerCoin != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InnerCoin);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TalentAdd = r.read_varint_i32()?,
            6 => self.UseTime = r.read_varint_i32()?,
            7 => self.FailAddProportion = r.read_varint_i32()?,
            8 => self.IsDeadSettle = r.read_bool()?,
            9 => self.ItemTotalValue = r.read_varint_i32()?,
            10 => self.IsSuccess = r.read_bool()?,
            11 => self.TotalValue = r.read_varint_i32()?,
            14 => self.IsUpdateTotalValue = r.read_bool()?,
            15 => self.InnerCoin = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryInstTopTowerSettleNotify {
    pub UseTime: i32,
    pub PassFloor: i32,
    pub RewardItems: Vec<RewardItem>,
    pub DangerLevel: i32,
}

impl HonamiStoryInstTopTowerSettleNotify {
    pub const MSG_ID: u16 = 24069;
}

impl PbMessage for HonamiStoryInstTopTowerSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        if self.PassFloor != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PassFloor);
        }
        for item in &self.RewardItems {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.DangerLevel != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DangerLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.UseTime = r.read_varint_i32()?,
            6 => self.PassFloor = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RewardItems
                    .push(<RewardItem as PbMessage>::decode(bytes)?);
            }
            13 => self.DangerLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemCollectionInfo {
    pub HonamiStoryItemId: i32,
    pub Status: i32,
}

impl PbMessage for HonamiStoryItemCollectionInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HonamiStoryItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HonamiStoryItemId);
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
            1 => self.HonamiStoryItemId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemCollectionInfoUpdateNotify {
    pub HonamiStoryItemCollectionInfos: Vec<HonamiStoryItemCollectionInfo>,
}

impl HonamiStoryItemCollectionInfoUpdateNotify {
    pub const MSG_ID: u16 = 16250;
}

impl PbMessage for HonamiStoryItemCollectionInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryItemCollectionInfos {
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
                self.HonamiStoryItemCollectionInfos
                    .push(<HonamiStoryItemCollectionInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemCollectionRequest {
    pub HonamiStoryItemCollectionId: Vec<i32>,
    pub ActivityId: i32,
}

impl HonamiStoryItemCollectionRequest {
    pub const MSG_ID: u16 = 27262;
}

impl PbMessage for HonamiStoryItemCollectionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryItemCollectionId {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.HonamiStoryItemCollectionId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            14 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemCollectionResponse {
    pub ActivityId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub HonamiStoryItemCollectionId: Vec<i32>,
}

impl HonamiStoryItemCollectionResponse {
    pub const MSG_ID: u16 = 17176;
}

impl PbMessage for HonamiStoryItemCollectionResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.HonamiStoryItemCollectionId {
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
            1 => self.ActivityId = r.read_varint_i32()?,
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.HonamiStoryItemCollectionId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryItemUidInfo {
    pub ItemUId: Vec<i32>,
}

impl PbMessage for HonamiStoryItemUidInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ItemUId {
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ItemUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLifeSupportNotify {
    pub LifeSupportLevel: i32,
    pub ActivityId: i32,
}

impl HonamiStoryLifeSupportNotify {
    pub const MSG_ID: u16 = 22572;
}

impl PbMessage for HonamiStoryLifeSupportNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LifeSupportLevel != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LifeSupportLevel);
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
            3 => self.LifeSupportLevel = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLifeSupportUpRequest {
    pub ActivityId: i32,
}

impl HonamiStoryLifeSupportUpRequest {
    pub const MSG_ID: u16 = 24974;
}

impl PbMessage for HonamiStoryLifeSupportUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLifeSupportUpResponse {
    pub ActivityId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub LifeSupportLevel: i32,
}

impl HonamiStoryLifeSupportUpResponse {
    pub const MSG_ID: u16 = 29000;
}

impl PbMessage for HonamiStoryLifeSupportUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LifeSupportLevel != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LifeSupportLevel);
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.LifeSupportLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLimitShopConsumeNumNotify {
    pub LimitItemConsumeNum: i32,
}

impl HonamiStoryLimitShopConsumeNumNotify {
    pub const MSG_ID: u16 = 16893;
}

impl PbMessage for HonamiStoryLimitShopConsumeNumNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LimitItemConsumeNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitItemConsumeNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.LimitItemConsumeNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLimitTaskInfoUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl HonamiStoryLimitTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 22327;
}

impl PbMessage for HonamiStoryLimitTaskInfoUpdateNotify {
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
pub struct HonamiStoryLimitTaskRewardRequest {
    pub LimitTaskId: i32,
    pub ActivityId: i32,
    pub LimitTaskIds: Vec<i32>,
}

impl HonamiStoryLimitTaskRewardRequest {
    pub const MSG_ID: u16 = 26293;
}

impl PbMessage for HonamiStoryLimitTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LimitTaskId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LimitTaskId);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.LimitTaskIds {
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
            2 => self.LimitTaskId = r.read_varint_i32()?,
            10 => self.ActivityId = r.read_varint_i32()?,
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LimitTaskIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLimitTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryLimitTaskRewardResponse {
    pub const MSG_ID: u16 = 20813;
}

impl PbMessage for HonamiStoryLimitTaskRewardResponse {
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
pub struct HonamiStoryLockItemRequest {
    pub BackPackId: i32,
    pub ItemUid: i32,
    pub IsLock: bool,
}

impl HonamiStoryLockItemRequest {
    pub const MSG_ID: u16 = 23316;
}

impl PbMessage for HonamiStoryLockItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BackPackId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BackPackId);
        }
        if self.ItemUid != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUid);
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
            4 => self.BackPackId = r.read_varint_i32()?,
            7 => self.ItemUid = r.read_varint_i32()?,
            8 => self.IsLock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryLockItemResponse {
    pub ItemUid: i32,
    pub IsLock: bool,
    pub BackPackId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryLockItemResponse {
    pub const MSG_ID: u16 = 21718;
}

impl PbMessage for HonamiStoryLockItemResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemUid);
        }
        if self.IsLock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BackPackId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BackPackId);
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
            1 => self.ItemUid = r.read_varint_i32()?,
            3 => self.IsLock = r.read_bool()?,
            7 => self.BackPackId = r.read_varint_i32()?,
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
pub struct HonamiStoryMascotInfo {
    pub HonamiStoryMascotId: i32,
    pub Status: i32,
}

impl PbMessage for HonamiStoryMascotInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HonamiStoryMascotId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HonamiStoryMascotId);
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
            1 => self.HonamiStoryMascotId = r.read_varint_i32()?,
            2 => self.Status = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryMascotInfoUpdateNotify {
    pub HonamiStoryMascotInfos: Vec<HonamiStoryMascotInfo>,
}

impl HonamiStoryMascotInfoUpdateNotify {
    pub const MSG_ID: u16 = 22035;
}

impl PbMessage for HonamiStoryMascotInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HonamiStoryMascotInfos {
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
                self.HonamiStoryMascotInfos
                    .push(<HonamiStoryMascotInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryMascotRewardRequest {
    pub HonamiStoryMascotId: i32,
}

impl HonamiStoryMascotRewardRequest {
    pub const MSG_ID: u16 = 27581;
}

impl PbMessage for HonamiStoryMascotRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.HonamiStoryMascotId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HonamiStoryMascotId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.HonamiStoryMascotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryMascotRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryMascotRewardResponse {
    pub const MSG_ID: u16 = 28960;
}

impl PbMessage for HonamiStoryMascotRewardResponse {
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
pub struct HonamiStoryPickUpItemRequest {
    pub HonamiStoryBagUpdateInfo: Option<HonamiStoryBagUpdateInfo>,
}

impl HonamiStoryPickUpItemRequest {
    pub const MSG_ID: u16 = 17260;
}

impl PbMessage for HonamiStoryPickUpItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.HonamiStoryBagUpdateInfo {
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
                self.HonamiStoryBagUpdateInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryPickUpItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl HonamiStoryPickUpItemResponse {
    pub const MSG_ID: u16 = 19769;
}

impl PbMessage for HonamiStoryPickUpItemResponse {
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
pub struct HonamiStoryPollutionBoxSteadyLimitNotify {}

impl HonamiStoryPollutionBoxSteadyLimitNotify {
    pub const MSG_ID: u16 = 17818;
}

impl PbMessage for HonamiStoryPollutionBoxSteadyLimitNotify {
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
pub struct HonamiStoryPollutionUpdateNotify {
    pub PollutionStartTime: i64,
    pub PollutionLevel: i32,
}

impl HonamiStoryPollutionUpdateNotify {
    pub const MSG_ID: u16 = 21640;
}

impl PbMessage for HonamiStoryPollutionUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PollutionStartTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PollutionStartTime);
        }
        if self.PollutionLevel != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PollutionLevel);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.PollutionStartTime = r.read_varint_i64()?,
            5 => self.PollutionLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryRackInfo {
    pub RoleId: i32,
    pub HonamiStorySlotInfos: Vec<HonamiStorySlotInfo>,
    pub DressWeapon: i32,
}

impl PbMessage for HonamiStoryRackInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.HonamiStorySlotInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.DressWeapon != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DressWeapon);
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
                self.HonamiStorySlotInfos
                    .push(<HonamiStorySlotInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.DressWeapon = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct HonamiStoryResidentTaskInfoUpdateNotify {
    pub ConditionTasks: Vec<ConditionTask>,
}

impl HonamiStoryResidentTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 19128;
}

impl PbMessage for HonamiStoryResidentTaskInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ConditionTasks {
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
                self.ConditionTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}
