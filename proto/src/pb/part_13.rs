use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResRoleBondUpdateNotify {
    pub RoleBondInfos: Vec<RoleBondInfo>,
}

impl RogueResRoleBondUpdateNotify {
    pub const MSG_ID: u16 = 21609;
}

impl PbMessage for RogueResRoleBondUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleBondInfos {
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
                self.RoleBondInfos
                    .push(<RoleBondInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResRoleBuff {
    pub ConfigId: i32,
    pub IsNew: bool,
}

impl PbMessage for RogueResRoleBuff {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNew {
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
            2 => self.IsNew = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResRoomInfoNotify {
    pub SkyBoxId: i32,
    pub MaxLayer: i32,
    pub RoguelikeRoomTypeId: i32,
    pub RoguelikeRoomId: i32,
    pub CurLayer: i32,
}

impl RogueResRoomInfoNotify {
    pub const MSG_ID: u16 = 28115;
}

impl PbMessage for RogueResRoomInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkyBoxId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkyBoxId);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.RoguelikeRoomTypeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomTypeId);
        }
        if self.RoguelikeRoomId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomId);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.SkyBoxId = r.read_varint_i32()?,
            8 => self.MaxLayer = r.read_varint_i32()?,
            10 => self.RoguelikeRoomTypeId = r.read_varint_i32()?,
            13 => self.RoguelikeRoomId = r.read_varint_i32()?,
            15 => self.CurLayer = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResSeasonData {
    pub SeasonId: i32,
    pub TalentSkillDict: BTreeMap<i32, i32>,
    pub Endings: Vec<i32>,
    pub EndingRewardIds: Vec<ActivityTask>,
    pub StaticTrialRoles: Vec<i32>,
    pub DynamicTrialRoles: Vec<i32>,
    pub EndTime: i64,
    pub TotalShopItem: i32,
}

impl PbMessage for RogueResSeasonData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if !self.TalentSkillDict.is_empty() {
            crate::wire::write_map(out, 2, &self.TalentSkillDict);
        }
        for item in &self.Endings {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.EndingRewardIds {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.StaticTrialRoles {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DynamicTrialRoles {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if self.TotalShopItem != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalShopItem);
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
            2 => crate::wire::decode_map_entry(r, &mut self.TalentSkillDict)?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Endings, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.EndingRewardIds
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.StaticTrialRoles,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DynamicTrialRoles,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.EndTime = r.read_varint_i64()?,
            8 => self.TotalShopItem = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResShopToken {
    pub ConfigId: i32,
    pub IsNew: bool,
    pub IsSell: bool,
    pub ElementUnits: Vec<ElementUnit>,
    pub SourcePrice: i32,
    pub CurPrice: i32,
    pub ItemId: i32,
    pub Index: i32,
}

impl PbMessage for RogueResShopToken {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSell {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ElementUnits {
            crate::wire::write_submessage(out, 4, item);
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
        if self.Index != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IsNew = r.read_bool()?,
            3 => self.IsSell = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ElementUnits
                    .push(<ElementUnit as PbMessage>::decode(bytes)?);
            }
            5 => self.SourcePrice = r.read_varint_i32()?,
            6 => self.CurPrice = r.read_varint_i32()?,
            7 => self.ItemId = r.read_varint_i32()?,
            8 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResSubLevelData {
    pub LoadSubLevels: Vec<Vec<u8>>,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub A: f32,
}

impl PbMessage for RogueResSubLevelData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LoadSubLevels {
            crate::wire::write_tag(out, 1, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.X != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        if self.A != 0.0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Fixed32);
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
            1 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            2 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            3 => self.X = r.read_f32()?,
            4 => self.Y = r.read_f32()?,
            5 => self.Z = r.read_f32()?,
            6 => self.A = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResSwitchFormationRequest {
    pub Index: i32,
}

impl RogueResSwitchFormationRequest {
    pub const MSG_ID: u16 = 27031;
}

impl PbMessage for RogueResSwitchFormationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            12 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResSwitchFormationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResSwitchFormationResponse {
    pub const MSG_ID: u16 = 22098;
}

impl PbMessage for RogueResSwitchFormationResponse {
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
pub struct RogueResTalentSkillLevelRequest {
    pub SkillId: i32,
}

impl RogueResTalentSkillLevelRequest {
    pub const MSG_ID: u16 = 21757;
}

impl PbMessage for RogueResTalentSkillLevelRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTalentSkillLevelResponse {
    pub Level: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResTalentSkillLevelResponse {
    pub const MSG_ID: u16 = 15004;
}

impl PbMessage for RogueResTalentSkillLevelResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
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
            6 => self.Level = r.read_varint_i32()?,
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
pub struct RogueResTalentUnlockNotify {
    pub SkillId: i32,
}

impl RogueResTalentUnlockNotify {
    pub const MSG_ID: u16 = 24327;
}

impl PbMessage for RogueResTalentUnlockNotify {
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
pub struct RogueResTaskAwardRequest {
    pub ConfigId: i32,
}

impl RogueResTaskAwardRequest {
    pub const MSG_ID: u16 = 24834;
}

impl PbMessage for RogueResTaskAwardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTaskAwardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueResTaskAwardResponse {
    pub const MSG_ID: u16 = 29739;
}

impl PbMessage for RogueResTaskAwardResponse {
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
pub struct RogueResTaskUpdateNotify {
    pub RogueResBranchTaskTheme: Option<RogueResBranchTaskTheme>,
}

impl RogueResTaskUpdateNotify {
    pub const MSG_ID: u16 = 24660;
}

impl PbMessage for RogueResTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResBranchTaskTheme {
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
                self.RogueResBranchTaskTheme
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTeamLevelUpdateNotify {
    pub EventType: i32,
    pub TeamLevel: i32,
    pub RoleLevel: i32,
}

impl RogueResTeamLevelUpdateNotify {
    pub const MSG_ID: u16 = 18322;
}

impl PbMessage for RogueResTeamLevelUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventType != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventType);
        }
        if self.TeamLevel != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeamLevel);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            6 => self.EventType = r.read_varint_i32()?,
            7 => self.TeamLevel = r.read_varint_i32()?,
            15 => self.RoleLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResToken {
    pub ConfigId: i32,
    pub IsNew: bool,
    pub ElementUnits: Vec<ElementUnit>,
    pub IsSelect: bool,
}

impl PbMessage for RogueResToken {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IsNew {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ElementUnits {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.IsSelect {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.IsNew = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ElementUnits
                    .push(<ElementUnit as PbMessage>::decode(bytes)?);
            }
            4 => self.IsSelect = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueResTotalShopItemUpdateNotify {
    pub SeasonId: i32,
    pub TotalShopItem: i32,
}

impl RogueResTotalShopItemUpdateNotify {
    pub const MSG_ID: u16 = 29902;
}

impl PbMessage for RogueResTotalShopItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.TotalShopItem != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalShopItem);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SeasonId = r.read_varint_i32()?,
            6 => self.TotalShopItem = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueRoleInstData {
    pub InstId: i32,
    pub IsUnlock: bool,
    pub IsFinish: bool,
}

impl PbMessage for RogueRoleInstData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsFinish {
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueSeasonData {
    pub SeasonId: i32,
    pub StartTime: i64,
    pub EndTime: i64,
    pub RoguelikeTokenList: Vec<RogueToken>,
    pub SeasonRewardList: Vec<RogueSeasonReward>,
    pub TokenItemCount: i32,
    pub BlackFlowerUseCount: i32,
    pub BlackFlowerMaxCount: i32,
    pub TowerTrialBestClearCount: i32,
}

impl PbMessage for RogueSeasonData {
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
        if self.BlackFlowerUseCount != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlackFlowerUseCount);
        }
        if self.BlackFlowerMaxCount != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlackFlowerMaxCount);
        }
        if self.TowerTrialBestClearCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerTrialBestClearCount);
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
                    .push(<RogueToken as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.SeasonRewardList
                    .push(<RogueSeasonReward as PbMessage>::decode(bytes)?);
            }
            6 => self.TokenItemCount = r.read_varint_i32()?,
            7 => self.BlackFlowerUseCount = r.read_varint_i32()?,
            8 => self.BlackFlowerMaxCount = r.read_varint_i32()?,
            10 => self.TowerTrialBestClearCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueToken {
    pub Id: i32,
    pub IsReceive: bool,
}

impl PbMessage for RogueToken {
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
pub struct RogueTowerTrialBossInfo {
    pub Id: i32,
    pub State: Option<RogueTowerTrialBossState>,
}

impl PbMessage for RogueTowerTrialBossInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                self.State = Some(
                    <RogueTowerTrialBossState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RogueTowerTrialGiveUpTempArchiveRequest {}

impl RogueTowerTrialGiveUpTempArchiveRequest {
    pub const MSG_ID: u16 = 20363;
}

impl PbMessage for RogueTowerTrialGiveUpTempArchiveRequest {
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
pub struct RogueTowerTrialGiveUpTempArchiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueTowerTrialGiveUpTempArchiveResponse {
    pub const MSG_ID: u16 = 25299;
}

impl PbMessage for RogueTowerTrialGiveUpTempArchiveResponse {
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
pub struct RogueTowerTrialReChallengeNotify {}

impl RogueTowerTrialReChallengeNotify {
    pub const MSG_ID: u16 = 19059;
}

impl PbMessage for RogueTowerTrialReChallengeNotify {
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
pub struct RogueTowerTrialReChallengeRequest {
    pub IsReChallenge: bool,
}

impl RogueTowerTrialReChallengeRequest {
    pub const MSG_ID: u16 = 24158;
}

impl PbMessage for RogueTowerTrialReChallengeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsReChallenge {
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
            9 => self.IsReChallenge = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialReChallengeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueTowerTrialReChallengeResponse {
    pub const MSG_ID: u16 = 15271;
}

impl PbMessage for RogueTowerTrialReChallengeResponse {
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
pub struct RogueTowerTrialResultInfo {
    pub IsNewRecord: bool,
    pub Ids: Vec<i32>,
    pub ChallengingBossId: i32,
    pub MaxCount: i32,
    pub SlotId: i32,
    pub HistoryMaxKills: i32,
    pub TotalCostTime: i32,
}

impl PbMessage for RogueTowerTrialResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNewRecord {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Ids {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ChallengingBossId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengingBossId);
        }
        if self.MaxCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxCount);
        }
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotId);
        }
        if self.HistoryMaxKills != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HistoryMaxKills);
        }
        if self.TotalCostTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCostTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsNewRecord = r.read_bool()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Ids, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.ChallengingBossId = r.read_varint_i32()?,
            4 => self.MaxCount = r.read_varint_i32()?,
            5 => self.SlotId = r.read_varint_i32()?,
            6 => self.HistoryMaxKills = r.read_varint_i32()?,
            13 => self.TotalCostTime = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialResultNotify {
    pub ErrorCode: Option<ErrorCode>,
    pub RogueTowerTrialResultInfo: Option<RogueTowerTrialResultInfo>,
}

impl RogueTowerTrialResultNotify {
    pub const MSG_ID: u16 = 20375;
}

impl PbMessage for RogueTowerTrialResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RogueTowerTrialResultInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RogueTowerTrialResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialResultRequest {
    pub InstId: i32,
}

impl RogueTowerTrialResultRequest {
    pub const MSG_ID: u16 = 19883;
}

impl PbMessage for RogueTowerTrialResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialResultResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RogueTowerTrialResultInfo: Option<RogueTowerTrialResultInfo>,
}

impl RogueTowerTrialResultResponse {
    pub const MSG_ID: u16 = 19217;
}

impl PbMessage for RogueTowerTrialResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RogueTowerTrialResultInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RogueTowerTrialResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialSelectBossNotify {
    pub BossInfos: Vec<RogueTowerTrialBossInfo>,
}

impl RogueTowerTrialSelectBossNotify {
    pub const MSG_ID: u16 = 16751;
}

impl PbMessage for RogueTowerTrialSelectBossNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BossInfos {
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
                self.BossInfos
                    .push(<RogueTowerTrialBossInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialSelectBossRequest {
    pub Id: i32,
}

impl RogueTowerTrialSelectBossRequest {
    pub const MSG_ID: u16 = 22870;
}

impl PbMessage for RogueTowerTrialSelectBossRequest {
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
pub struct RogueTowerTrialSelectBossResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueTowerTrialSelectBossResponse {
    pub const MSG_ID: u16 = 28302;
}

impl PbMessage for RogueTowerTrialSelectBossResponse {
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
pub struct RogueTowerTrialStartRequest {
    pub InstId: i32,
    pub SlotId: i32,
}

impl RogueTowerTrialStartRequest {
    pub const MSG_ID: u16 = 24725;
}

impl PbMessage for RogueTowerTrialStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.SlotId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            3 => self.InstId = r.read_varint_i32()?,
            11 => self.SlotId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueTowerTrialStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueTowerTrialStartResponse {
    pub const MSG_ID: u16 = 23092;
}

impl PbMessage for RogueTowerTrialStartResponse {
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
pub struct RogueViewGroup {
    pub RogueViewUnit: Option<RogueViewUnit>,
    pub ChildRogueViewUnits: Vec<RogueViewUnit>,
}

impl PbMessage for RogueViewGroup {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueViewUnit {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ChildRogueViewUnits {
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
                self.RogueViewUnit
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ChildRogueViewUnits
                    .push(<RogueViewUnit as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueViewPackage {
    pub RogueViewGroups: Vec<RogueViewGroup>,
}

impl PbMessage for RogueViewPackage {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueViewGroups {
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
                self.RogueViewGroups
                    .push(<RogueViewGroup as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueViewUnit {
    pub RoguelikeEventGainNotify: Option<RoguelikeEventGainNotify>,
    pub ItemObtainNotify: Option<ItemObtainNotify>,
}

impl PbMessage for RogueViewUnit {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoguelikeEventGainNotify {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.ItemObtainNotify {
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
                self.RoguelikeEventGainNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ItemObtainNotify
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyArtifactInfoRequest {}

impl RogueWeeklyArtifactInfoRequest {
    pub const MSG_ID: u16 = 27043;
}

impl PbMessage for RogueWeeklyArtifactInfoRequest {
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
pub struct RogueWeeklyArtifactInfoResponse {
    pub RogueWeeklyEntrys: Vec<RogueWeeklyEntry>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyArtifactInfoResponse {
    pub const MSG_ID: u16 = 16633;
}

impl PbMessage for RogueWeeklyArtifactInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueWeeklyEntrys {
            crate::wire::write_submessage(out, 9, item);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyEntrys
                    .push(<RogueWeeklyEntry as PbMessage>::decode(bytes)?);
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
pub struct RogueWeeklyArtifactSelectRequest {
    pub CycleId: i32,
    pub Index: i32,
}

impl RogueWeeklyArtifactSelectRequest {
    pub const MSG_ID: u16 = 22776;
}

impl PbMessage for RogueWeeklyArtifactSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
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
            2 => self.CycleId = r.read_varint_i32()?,
            10 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyArtifactSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyArtifactSelectResponse {
    pub const MSG_ID: u16 = 17264;
}

impl PbMessage for RogueWeeklyArtifactSelectResponse {
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
pub struct RogueWeeklyBlackFlowerNotify {
    pub SilentAreaIds: Vec<i32>,
    pub EntityConfigId: i32,
}

impl RogueWeeklyBlackFlowerNotify {
    pub const MSG_ID: u16 = 22410;
}

impl PbMessage for RogueWeeklyBlackFlowerNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SilentAreaIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SilentAreaIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            8 => self.EntityConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyBlackFlowerRequest {
    pub EntityConfigId: i32,
    pub IsDouble: bool,
    pub Ret: bool,
    pub SilentAreaId: i32,
}

impl RogueWeeklyBlackFlowerRequest {
    pub const MSG_ID: u16 = 21004;
}

impl PbMessage for RogueWeeklyBlackFlowerRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EntityConfigId);
        }
        if self.IsDouble {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Ret {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SilentAreaId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SilentAreaId);
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
            8 => self.IsDouble = r.read_bool()?,
            10 => self.Ret = r.read_bool()?,
            15 => self.SilentAreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyBlackFlowerResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub MaxFreeCount: i32,
    pub UseFreeCount: i32,
}

impl RogueWeeklyBlackFlowerResponse {
    pub const MSG_ID: u16 = 21684;
}

impl PbMessage for RogueWeeklyBlackFlowerResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MaxFreeCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFreeCount);
        }
        if self.UseFreeCount != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseFreeCount);
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
            9 => self.MaxFreeCount = r.read_varint_i32()?,
            14 => self.UseFreeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyBuff {
    pub IsNew: bool,
}

impl PbMessage for RogueWeeklyBuff {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNew {
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
            1 => self.IsNew = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyCurrencyNotify {
    pub CurrencyDict: BTreeMap<i32, i32>,
}

impl RogueWeeklyCurrencyNotify {
    pub const MSG_ID: u16 = 25282;
}

impl PbMessage for RogueWeeklyCurrencyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CurrencyDict.is_empty() {
            crate::wire::write_map(out, 2, &self.CurrencyDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => crate::wire::decode_map_entry(r, &mut self.CurrencyDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyCurrencyUpdateNotify {
    pub CurrencyDict: BTreeMap<i32, i32>,
}

impl RogueWeeklyCurrencyUpdateNotify {
    pub const MSG_ID: u16 = 15390;
}

impl PbMessage for RogueWeeklyCurrencyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CurrencyDict.is_empty() {
            crate::wire::write_map(out, 5, &self.CurrencyDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => crate::wire::decode_map_entry(r, &mut self.CurrencyDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyData {
    pub CycleId: i32,
    pub Score: i32,
    pub RogueWeeklyLastInfo: Option<RogueWeeklyLastInfo>,
    pub RogueWeeklyAwards: Vec<RogueWeeklyAward>,
    pub MaxScore: i32,
    pub CurWorldLevel: i32,
    pub UseFreeCount: i32,
    pub MaxFreeCount: i32,
}

impl PbMessage for RogueWeeklyData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if let Some(v) = &self.RogueWeeklyLastInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.RogueWeeklyAwards {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
        if self.CurWorldLevel != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurWorldLevel);
        }
        if self.UseFreeCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseFreeCount);
        }
        if self.MaxFreeCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxFreeCount);
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
            2 => self.Score = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyLastInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyAwards
                    .push(<RogueWeeklyAward as PbMessage>::decode(bytes)?);
            }
            7 => self.MaxScore = r.read_varint_i32()?,
            8 => self.CurWorldLevel = r.read_varint_i32()?,
            9 => self.UseFreeCount = r.read_varint_i32()?,
            10 => self.MaxFreeCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyEntry {
    pub Index: i32,
    pub Type: Option<RogueWeeklyEntryType>,
    pub ConfigId: i32,
    pub RogueWeeklyBuff: Option<RogueWeeklyBuff>,
    pub RogueWeeklyGoods: Option<RogueWeeklyGoods>,
    pub IsRecommend: bool,
}

impl PbMessage for RogueWeeklyEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if let Some(v) = &self.RogueWeeklyBuff {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.RogueWeeklyGoods {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.IsRecommend {
            crate::wire::write_tag(out, 100, WireType::Varint);
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
            1 => self.Index = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <RogueWeeklyEntryType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.ConfigId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyBuff
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyGoods
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            100 => self.IsRecommend = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyGoods {
    pub IsNew: bool,
    pub IsSell: bool,
    pub SourcePrice: i32,
    pub CurPrice: i32,
}

impl PbMessage for RogueWeeklyGoods {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNew {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsSell {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SourcePrice != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourcePrice);
        }
        if self.CurPrice != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurPrice);
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
            2 => self.IsSell = r.read_bool()?,
            3 => self.SourcePrice = r.read_varint_i32()?,
            4 => self.CurPrice = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyGotoNextRoomRequest {
    pub Confirm: bool,
}

impl RogueWeeklyGotoNextRoomRequest {
    pub const MSG_ID: u16 = 24488;
}

impl PbMessage for RogueWeeklyGotoNextRoomRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Confirm {
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
            14 => self.Confirm = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyGotoNextRoomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyGotoNextRoomResponse {
    pub const MSG_ID: u16 = 29783;
}

impl PbMessage for RogueWeeklyGotoNextRoomResponse {
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
pub struct RogueWeeklyInstInfoNotify {
    pub ConfigId: i32,
    pub RogueWeeklyOptions: BTreeMap<i32, RogueWeeklyOption>,
    pub RogueWeeklyBuffs: Vec<i32>,
}

impl RogueWeeklyInstInfoNotify {
    pub const MSG_ID: u16 = 15788;
}

impl PbMessage for RogueWeeklyInstInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if !self.RogueWeeklyOptions.is_empty() {
            crate::wire::write_map(out, 10, &self.RogueWeeklyOptions);
        }
        for item in &self.RogueWeeklyBuffs {
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
            3 => self.ConfigId = r.read_varint_i32()?,
            10 => crate::wire::decode_map_entry(r, &mut self.RogueWeeklyOptions)?,
            14 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RogueWeeklyBuffs,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyInteractionResultNotify {
    pub CurScore: i32,
    pub MaxScore: i32,
}

impl RogueWeeklyInteractionResultNotify {
    pub const MSG_ID: u16 = 25625;
}

impl PbMessage for RogueWeeklyInteractionResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurScore != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurScore);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.CurScore = r.read_varint_i32()?,
            12 => self.MaxScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyLastInfoRequest {
    pub ActivityId: i32,
}

impl RogueWeeklyLastInfoRequest {
    pub const MSG_ID: u16 = 27120;
}

impl PbMessage for RogueWeeklyLastInfoRequest {
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
pub struct RogueWeeklyLastInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RogueWeeklyLastInfo: Option<RogueWeeklyLastInfo>,
}

impl RogueWeeklyLastInfoResponse {
    pub const MSG_ID: u16 = 16590;
}

impl PbMessage for RogueWeeklyLastInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RogueWeeklyLastInfo {
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
            14 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyLastInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyOption {
    pub RogueWeeklyEntrys: Vec<RogueWeeklyEntry>,
    pub Type: Option<RogueWeeklyEntryType>,
}

impl PbMessage for RogueWeeklyOption {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueWeeklyEntrys {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.Type {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWeeklyEntrys
                    .push(<RogueWeeklyEntry as PbMessage>::decode(bytes)?);
            }
            2 => {
                self.Type = Some(
                    <RogueWeeklyEntryType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RogueWeeklyOptionSelectRequest {
    pub Index: i32,
    pub BindId: i32,
}

impl RogueWeeklyOptionSelectRequest {
    pub const MSG_ID: u16 = 19544;
}

impl PbMessage for RogueWeeklyOptionSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.BindId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Index = r.read_varint_i32()?,
            15 => self.BindId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyOptionSelectResponse {
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyOptionSelectResponse {
    pub const MSG_ID: u16 = 15921;
}

impl PbMessage for RogueWeeklyOptionSelectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 13, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            13 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
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
pub struct RogueWeeklyResultNotify {
    pub MaxLayer: i32,
    pub UseRecommendRole: bool,
    pub ExtraScore: i32,
    pub Score: i32,
    pub CurLayer: i32,
}

impl RogueWeeklyResultNotify {
    pub const MSG_ID: u16 = 19127;
}

impl PbMessage for RogueWeeklyResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.UseRecommendRole {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ExtraScore != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraScore);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.MaxLayer = r.read_varint_i32()?,
            7 => self.UseRecommendRole = r.read_bool()?,
            9 => self.ExtraScore = r.read_varint_i32()?,
            10 => self.Score = r.read_varint_i32()?,
            11 => self.CurLayer = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyResultRequest {}

impl RogueWeeklyResultRequest {
    pub const MSG_ID: u16 = 28144;
}

impl PbMessage for RogueWeeklyResultRequest {
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
pub struct RogueWeeklyResultResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyResultResponse {
    pub const MSG_ID: u16 = 20631;
}

impl PbMessage for RogueWeeklyResultResponse {
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
pub struct RogueWeeklyRewardRequest {
    pub ActivityId: i32,
    pub ConfigId: i32,
}

impl RogueWeeklyRewardRequest {
    pub const MSG_ID: u16 = 16216;
}

impl PbMessage for RogueWeeklyRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            6 => self.ActivityId = r.read_varint_i32()?,
            14 => self.ConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyRewardResponse {
    pub const MSG_ID: u16 = 23902;
}

impl PbMessage for RogueWeeklyRewardResponse {
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
pub struct RogueWeeklyRoomInfoNotify {
    pub RoguelikeRoomId: i32,
    pub SkyBoxId: i32,
    pub MaxLayer: i32,
    pub CurLayer: i32,
    pub RoguelikeRoomTypeId: i32,
    pub Score: i32,
}

impl RogueWeeklyRoomInfoNotify {
    pub const MSG_ID: u16 = 18827;
}

impl PbMessage for RogueWeeklyRoomInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoguelikeRoomId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomId);
        }
        if self.SkyBoxId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkyBoxId);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.RoguelikeRoomTypeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomTypeId);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            2 => self.RoguelikeRoomId = r.read_varint_i32()?,
            4 => self.SkyBoxId = r.read_varint_i32()?,
            6 => self.MaxLayer = r.read_varint_i32()?,
            8 => self.CurLayer = r.read_varint_i32()?,
            10 => self.RoguelikeRoomTypeId = r.read_varint_i32()?,
            11 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyStartRequest {
    pub CycleId: i32,
    pub SkillBranchIds: Vec<i32>,
    pub Roles: Vec<i32>,
}

impl RogueWeeklyStartRequest {
    pub const MSG_ID: u16 = 28012;
}

impl PbMessage for RogueWeeklyStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CycleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CycleId);
        }
        for item in &self.SkillBranchIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.Roles {
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
            1 => self.CycleId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkillBranchIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Roles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWeeklyStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWeeklyStartResponse {
    pub const MSG_ID: u16 = 18980;
}

impl PbMessage for RogueWeeklyStartResponse {
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
pub struct RogueWeeklySubLevelNotify {
    pub Z: f32,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
    pub Y: f32,
    pub CurScore: i32,
    pub IsExtra: bool,
    pub A: f32,
    pub X: f32,
    pub LoadSubLevels: Vec<Vec<u8>>,
    pub MaxScore: i32,
}

impl RogueWeeklySubLevelNotify {
    pub const MSG_ID: u16 = 21204;
}

impl PbMessage for RogueWeeklySubLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.CurScore != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurScore);
        }
        if self.IsExtra {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.A != 0.0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.A).to_bits().to_le_bytes());
        }
        if self.X != 0.0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        for item in &self.LoadSubLevels {
            crate::wire::write_tag(out, 12, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.MaxScore != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxScore);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.Z = r.read_f32()?,
            2 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            3 => self.Y = r.read_f32()?,
            8 => self.CurScore = r.read_varint_i32()?,
            9 => self.IsExtra = r.read_bool()?,
            10 => self.A = r.read_f32()?,
            11 => self.X = r.read_f32()?,
            12 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            14 => self.MaxScore = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatCtx {
    pub ActivityId: i32,
    pub IsFirst: bool,
    pub Idnex: i32,
}

impl PbMessage for RogueWhiteCatCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.IsFirst {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Idnex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Idnex);
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
            2 => self.IsFirst = r.read_bool()?,
            3 => self.Idnex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatData {
    pub MaxEnergy: i32,
    pub SignStateList: Vec<SignState>,
    pub RogueRoleInstData: Vec<RogueRoleInstData>,
    pub RogueLevelPlayData: Vec<RogueLevelPlayData>,
    pub BossRewardStateList: Vec<SignState>,
    pub RogueLimitedAwards: Vec<RogueLimitedAward>,
    pub ScoreMap: BTreeMap<i32, i32>,
    pub LimitedStartTime: i64,
    pub LimitedEndTime: i64,
    pub RogueBossInstDatas: Vec<RogueBossInstData>,
    pub Record: i32,
    pub UnlockButtons: Vec<i32>,
}

impl PbMessage for RogueWhiteCatData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxEnergy != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxEnergy);
        }
        for item in &self.SignStateList {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        for item in &self.RogueRoleInstData {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RogueLevelPlayData {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.BossRewardStateList {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*item));
        }
        for item in &self.RogueLimitedAwards {
            crate::wire::write_submessage(out, 8, item);
        }
        if !self.ScoreMap.is_empty() {
            crate::wire::write_map(out, 9, &self.ScoreMap);
        }
        if self.LimitedStartTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedStartTime);
        }
        if self.LimitedEndTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LimitedEndTime);
        }
        for item in &self.RogueBossInstDatas {
            crate::wire::write_submessage(out, 12, item);
        }
        if self.Record != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Record);
        }
        for item in &self.UnlockButtons {
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
            1 => self.MaxEnergy = r.read_varint_i32()?,
            3 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SignStateList, |sr| {
                    Ok(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                })?
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RogueRoleInstData
                    .push(<RogueRoleInstData as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueLevelPlayData
                    .push(<RogueLevelPlayData as PbMessage>::decode(bytes)?);
            }
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.BossRewardStateList,
                |sr| {
                    Ok(<SignState as crate::wire::PbEnum>::from_i32_or_default(
                        sr.read_varint_i32()?,
                    ))
                },
            )?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RogueLimitedAwards
                    .push(<RogueLimitedAward as PbMessage>::decode(bytes)?);
            }
            9 => crate::wire::decode_map_entry(r, &mut self.ScoreMap)?,
            10 => self.LimitedStartTime = r.read_varint_i64()?,
            11 => self.LimitedEndTime = r.read_varint_i64()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.RogueBossInstDatas
                    .push(<RogueBossInstData as PbMessage>::decode(bytes)?);
            }
            13 => self.Record = r.read_varint_i32()?,
            14 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockButtons, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatDataUpdateNotify {
    pub ActivityId: i32,
    pub RogueWhiteCatData: Option<RogueWhiteCatData>,
}

impl RogueWhiteCatDataUpdateNotify {
    pub const MSG_ID: u16 = 27640;
}

impl PbMessage for RogueWhiteCatDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if let Some(v) = &self.RogueWhiteCatData {
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
            2 => self.ActivityId = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RogueWhiteCatData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatLimitedRewardRequest {
    pub Index: i32,
}

impl RogueWhiteCatLimitedRewardRequest {
    pub const MSG_ID: u16 = 29426;
}

impl PbMessage for RogueWhiteCatLimitedRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatLimitedRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWhiteCatLimitedRewardResponse {
    pub const MSG_ID: u16 = 25823;
}

impl PbMessage for RogueWhiteCatLimitedRewardResponse {
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
pub struct RogueWhiteCatLimitedUpdateNotify {
    pub RogueLimitedAwards: Vec<RogueLimitedAward>,
}

impl RogueWhiteCatLimitedUpdateNotify {
    pub const MSG_ID: u16 = 28734;
}

impl PbMessage for RogueWhiteCatLimitedUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueLimitedAwards {
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
                self.RogueLimitedAwards
                    .push(<RogueLimitedAward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatRecordRequest {
    pub ActivityId: i32,
    pub Value: i32,
}

impl RogueWhiteCatRecordRequest {
    pub const MSG_ID: u16 = 23990;
}

impl PbMessage for RogueWhiteCatRecordRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Value != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            3 => self.ActivityId = r.read_varint_i32()?,
            8 => self.Value = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RogueWhiteCatRecordResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RogueWhiteCatRecordResponse {
    pub const MSG_ID: u16 = 27210;
}

impl PbMessage for RogueWhiteCatRecordResponse {
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
pub struct RoguelikeActivityData {
    pub BeginOpenTime: i64,
    pub EndOpenTime: i64,
    pub RogueSeasonData: Option<RogueSeasonData>,
}

impl PbMessage for RoguelikeActivityData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BeginOpenTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginOpenTime);
        }
        if self.EndOpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndOpenTime);
        }
        if let Some(v) = &self.RogueSeasonData {
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
            1 => self.BeginOpenTime = r.read_varint_i64()?,
            2 => self.EndOpenTime = r.read_varint_i64()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RogueSeasonData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeBossInstStartRequest {
    pub ActivityId: i32,
    pub RoleList: Vec<i32>,
}

impl RoguelikeBossInstStartRequest {
    pub const MSG_ID: u16 = 28063;
}

impl PbMessage for RoguelikeBossInstStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.RoleList {
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
            3 => self.ActivityId = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeBossInstStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeBossInstStartResponse {
    pub const MSG_ID: u16 = 21669;
}

impl PbMessage for RoguelikeBossInstStartResponse {
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
pub struct RoguelikeChooseData {
    pub Index: i32,
    pub Type: Option<RoguelikeGainDataType>,
    pub MaxTime: i32,
    pub UseTime: i32,
    pub EventId: i32,
    pub RogueGainEntryList: Vec<RogueGainEntry>,
    pub Layer: i32,
    pub IsSelect: bool,
    pub NumItems: Vec<NumItem>,
}

impl PbMessage for RoguelikeChooseData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.MaxTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxTime);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        if self.EventId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventId);
        }
        for item in &self.RogueGainEntryList {
            crate::wire::write_submessage(out, 6, item);
        }
        if self.Layer != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.NumItems {
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
            1 => self.Index = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <RoguelikeGainDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.MaxTime = r.read_varint_i32()?,
            4 => self.UseTime = r.read_varint_i32()?,
            5 => self.EventId = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueGainEntryList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            7 => self.Layer = r.read_varint_i32()?,
            8 => self.IsSelect = r.read_bool()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.NumItems.push(<NumItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeChooseDataNotify {
    pub RoguelikeChooseDataList: Vec<RoguelikeChooseData>,
}

impl RoguelikeChooseDataNotify {
    pub const MSG_ID: u16 = 26262;
}

impl PbMessage for RoguelikeChooseDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoguelikeChooseDataList {
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
                self.RoguelikeChooseDataList
                    .push(<RoguelikeChooseData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeChooseDataRequest {
    pub Index: i32,
    pub Layer: i32,
}

impl RoguelikeChooseDataRequest {
    pub const MSG_ID: u16 = 25554;
}

impl PbMessage for RoguelikeChooseDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if self.Layer != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.Index = r.read_varint_i32()?,
            15 => self.Layer = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeChooseDataResponse {
    pub RoguelikeChooseData: Option<RoguelikeChooseData>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeChooseDataResponse {
    pub const MSG_ID: u16 = 17486;
}

impl PbMessage for RoguelikeChooseDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoguelikeChooseData {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 4, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeChooseData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
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
pub struct RoguelikeChooseDataResultRequest {
    pub Layer: i32,
    pub BindId: i32,
    pub Index: i32,
}

impl RoguelikeChooseDataResultRequest {
    pub const MSG_ID: u16 = 18731;
}

impl PbMessage for RoguelikeChooseDataResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Layer != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
        if self.BindId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            8 => self.Layer = r.read_varint_i32()?,
            11 => self.BindId = r.read_varint_i32()?,
            15 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeChooseDataResultResponse {
    pub ExtraRogueGainEntrys: Vec<RogueGainEntry>,
    pub IsSelect: bool,
    pub RogueViewPackage: Option<RogueViewPackage>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub RogueGainEntryList: Vec<RogueGainEntry>,
    pub RoguelikeChooseDataNotify: Option<RoguelikeChooseDataNotify>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeChooseDataResultResponse {
    pub const MSG_ID: u16 = 18902;
}

impl PbMessage for RoguelikeChooseDataResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ExtraRogueGainEntrys {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.RogueViewPackage {
            crate::wire::write_submessage(out, 6, v);
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.RogueGainEntryList {
            crate::wire::write_submessage(out, 13, item);
        }
        if let Some(v) = &self.RoguelikeChooseDataNotify {
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraRogueGainEntrys
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            2 => self.IsSelect = r.read_bool()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RogueViewPackage
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
            13 => {
                let bytes = r.read_len_delimited()?;
                self.RogueGainEntryList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeChooseDataNotify
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
pub struct RoguelikeCurrencyUpdateNotify {
    pub CurrencyUpdateDict: BTreeMap<i32, i32>,
}

impl RoguelikeCurrencyUpdateNotify {
    pub const MSG_ID: u16 = 24300;
}

impl PbMessage for RoguelikeCurrencyUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.CurrencyUpdateDict.is_empty() {
            crate::wire::write_map(out, 11, &self.CurrencyUpdateDict);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => crate::wire::decode_map_entry(r, &mut self.CurrencyUpdateDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeEventGainNotify {
    pub RogueGainEntryList: Vec<RogueGainEntry>,
    pub RogueGainEntryList2: Vec<RogueGainEntry>,
}

impl RoguelikeEventGainNotify {
    pub const MSG_ID: u16 = 17169;
}

impl PbMessage for RoguelikeEventGainNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RogueGainEntryList {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RogueGainEntryList2 {
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
                self.RogueGainEntryList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.RogueGainEntryList2
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeGainDataUpdateNotify {
    pub RogueGainEntry: Option<RogueGainEntry>,
    pub ElementDict: BTreeMap<i32, i32>,
    pub Index: i32,
    pub GainDataUpdateType: Option<GainDataUpdateType>,
}

impl RoguelikeGainDataUpdateNotify {
    pub const MSG_ID: u16 = 22913;
}

impl PbMessage for RoguelikeGainDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueGainEntry {
            crate::wire::write_submessage(out, 1, v);
        }
        if !self.ElementDict.is_empty() {
            crate::wire::write_map(out, 5, &self.ElementDict);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        if let Some(v) = &self.GainDataUpdateType {
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
                self.RogueGainEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => crate::wire::decode_map_entry(r, &mut self.ElementDict)?,
            10 => self.Index = r.read_varint_i32()?,
            11 => {
                self.GainDataUpdateType = Some(
                    <GainDataUpdateType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoguelikeGiveUpGainRequest {
    pub BindId: i32,
    pub Layer: i32,
}

impl RoguelikeGiveUpGainRequest {
    pub const MSG_ID: u16 = 29402;
}

impl PbMessage for RoguelikeGiveUpGainRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BindId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
        if self.Layer != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.BindId = r.read_varint_i32()?,
            8 => self.Layer = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeGiveUpGainResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
    pub BindId: i32,
}

impl RoguelikeGiveUpGainResponse {
    pub const MSG_ID: u16 = 21026;
}

impl PbMessage for RoguelikeGiveUpGainResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 6, &self.ItemMap);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.BindId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => self.BindId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeGotoNextRoomRequest {}

impl RoguelikeGotoNextRoomRequest {
    pub const MSG_ID: u16 = 20444;
}

impl PbMessage for RoguelikeGotoNextRoomRequest {
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
pub struct RoguelikeGotoNextRoomResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeGotoNextRoomResponse {
    pub const MSG_ID: u16 = 23489;
}

impl PbMessage for RoguelikeGotoNextRoomResponse {
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
pub struct RoguelikeInfoNotify {
    pub MiraclecreationList: Vec<RogueGainEntry>,
    pub RoleEntry: Option<RogueGainEntry>,
    pub BuffEntryList: Vec<RogueGainEntry>,
    pub ModeType: Option<RogueChallengeModeType>,
    pub ElementDict: BTreeMap<i32, i32>,
    pub RoguelikeInstInfo: Option<RoguelikeInstInfo>,
    pub PhantomEntry: Option<RogueGainEntry>,
}

impl RoguelikeInfoNotify {
    pub const MSG_ID: u16 = 15467;
}

impl PbMessage for RoguelikeInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.MiraclecreationList {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.RoleEntry {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.BuffEntryList {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.ModeType {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ElementDict.is_empty() {
            crate::wire::write_map(out, 12, &self.ElementDict);
        }
        if let Some(v) = &self.RoguelikeInstInfo {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.PhantomEntry {
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
                self.MiraclecreationList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.BuffEntryList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ModeType = Some(
                    <RogueChallengeModeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => crate::wire::decode_map_entry(r, &mut self.ElementDict)?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeInstInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeInstInfo {
    pub RoleEntry: Option<RogueGainEntry>,
    pub PhantomEntry: Option<RogueGainEntry>,
    pub BuffEntryList: Vec<RogueGainEntry>,
    pub ElementDict: BTreeMap<i32, i32>,
    pub MiraclecreationList: Vec<RogueGainEntry>,
}

impl PbMessage for RoguelikeInstInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleEntry {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.PhantomEntry {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.BuffEntryList {
            crate::wire::write_submessage(out, 3, item);
        }
        if !self.ElementDict.is_empty() {
            crate::wire::write_map(out, 4, &self.ElementDict);
        }
        for item in &self.MiraclecreationList {
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
                self.RoleEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.BuffEntryList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            4 => crate::wire::decode_map_entry(r, &mut self.ElementDict)?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.MiraclecreationList
                    .push(<RogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeLastInfo {
    pub InstId: i32,
    pub CurLayer: i32,
    pub MaxLayer: i32,
    pub ModeType: Option<RogueChallengeModeType>,
}

impl PbMessage for RoguelikeLastInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if let Some(v) = &self.ModeType {
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
            2 => self.InstId = r.read_varint_i32()?,
            3 => self.CurLayer = r.read_varint_i32()?,
            4 => self.MaxLayer = r.read_varint_i32()?,
            5 => {
                self.ModeType = Some(
                    <RogueChallengeModeType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoguelikeLastInfoRequest {}

impl RoguelikeLastInfoRequest {
    pub const MSG_ID: u16 = 17013;
}

impl PbMessage for RoguelikeLastInfoRequest {
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
pub struct RoguelikeLastInfoResponse {
    pub TempRogueArchiveInfo: Option<RogueArchiveInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub RoguelikeLastInfos: Vec<RoguelikeLastInfo>,
    pub HasData: bool,
}

impl RoguelikeLastInfoResponse {
    pub const MSG_ID: u16 = 24998;
}

impl PbMessage for RoguelikeLastInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TempRogueArchiveInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RoguelikeLastInfos {
            crate::wire::write_submessage(out, 14, item);
        }
        if self.HasData {
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
                let bytes = r.read_len_delimited()?;
                self.TempRogueArchiveInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeLastInfos
                    .push(<RoguelikeLastInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.HasData = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikePopularEntriesChangeRequest {
    pub Entries: Vec<i32>,
    pub InstId: i32,
    pub SeasonId: i32,
}

impl RoguelikePopularEntriesChangeRequest {
    pub const MSG_ID: u16 = 24745;
}

impl PbMessage for RoguelikePopularEntriesChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Entries {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Entries, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => self.InstId = r.read_varint_i32()?,
            10 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikePopularEntriesChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikePopularEntriesChangeResponse {
    pub const MSG_ID: u16 = 20560;
}

impl PbMessage for RoguelikePopularEntriesChangeResponse {
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
pub struct RoguelikePopularEntriesInfoRequest {
    pub SeasonId: i32,
    pub InstId: i32,
}

impl RoguelikePopularEntriesInfoRequest {
    pub const MSG_ID: u16 = 26192;
}

impl PbMessage for RoguelikePopularEntriesInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            4 => self.SeasonId = r.read_varint_i32()?,
            7 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikePopularEntriesInfoResponse {
    pub PopularEntrie: Option<PopularEntrie>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikePopularEntriesInfoResponse {
    pub const MSG_ID: u16 = 29994;
}

impl PbMessage for RoguelikePopularEntriesInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PopularEntrie {
            crate::wire::write_submessage(out, 9, v);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PopularEntrie
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
pub struct RoguelikeQuitRequest {}

impl RoguelikeQuitRequest {
    pub const MSG_ID: u16 = 18528;
}

impl PbMessage for RoguelikeQuitRequest {
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
pub struct RoguelikeQuitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeQuitResponse {
    pub const MSG_ID: u16 = 18512;
}

impl PbMessage for RoguelikeQuitResponse {
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
pub struct RoguelikeRefreshGainRequest {
    pub Layer: i32,
    pub BindId: i32,
}

impl RoguelikeRefreshGainRequest {
    pub const MSG_ID: u16 = 20860;
}

impl PbMessage for RoguelikeRefreshGainRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Layer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
        if self.BindId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.Layer = r.read_varint_i32()?,
            9 => self.BindId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeRefreshGainResponse {
    pub RoguelikeChooseData: Option<RoguelikeChooseData>,
    pub ErrorParams: Vec<Vec<u8>>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeRefreshGainResponse {
    pub const MSG_ID: u16 = 23711;
}

impl PbMessage for RoguelikeRefreshGainResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoguelikeChooseData {
            crate::wire::write_submessage(out, 4, v);
        }
        for item in &self.ErrorParams {
            crate::wire::write_tag(out, 6, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeChooseData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.ErrorParams.push(r.read_len_delimited()?.to_vec()),
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
pub struct RoguelikeResultInfo {
    pub IsNewRecord: bool,
    pub Time: u32,
    pub CurLayer: i32,
    pub MaxLayer: i32,
    pub KillEnemyCount: i32,
    pub GetGainCount: i32,
    pub Rewards: BTreeMap<i32, i32>,
    pub RoleEntry: Option<RogueGainEntry>,
    pub PhantomEntry: Option<RogueGainEntry>,
    pub IsDeadTrigger: bool,
    pub InstId: i32,
    pub TotalCostTime: i32,
    pub Rate: i32,
    pub SelectHotEntryGroupId: i32,
    pub RogueArchiveState: Option<RogueArchiveState>,
    pub RogueArchiveInfo: Option<RogueArchiveInfo>,
}

impl PbMessage for RoguelikeResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsNewRecord {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Time != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.Time));
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.KillEnemyCount != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.KillEnemyCount);
        }
        if self.GetGainCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GetGainCount);
        }
        if !self.Rewards.is_empty() {
            crate::wire::write_map(out, 8, &self.Rewards);
        }
        if let Some(v) = &self.RoleEntry {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.PhantomEntry {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.IsDeadTrigger {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.TotalCostTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCostTime);
        }
        if self.Rate != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rate);
        }
        if self.SelectHotEntryGroupId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectHotEntryGroupId);
        }
        if let Some(v) = &self.RogueArchiveState {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RogueArchiveInfo {
            crate::wire::write_submessage(out, 17, v);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.IsNewRecord = r.read_bool()?,
            2 => self.Time = r.read_varint_u32()?,
            3 => self.CurLayer = r.read_varint_i32()?,
            4 => self.MaxLayer = r.read_varint_i32()?,
            5 => self.KillEnemyCount = r.read_varint_i32()?,
            6 => self.GetGainCount = r.read_varint_i32()?,
            8 => crate::wire::decode_map_entry(r, &mut self.Rewards)?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.IsDeadTrigger = r.read_bool()?,
            12 => self.InstId = r.read_varint_i32()?,
            13 => self.TotalCostTime = r.read_varint_i32()?,
            14 => self.Rate = r.read_varint_i32()?,
            15 => self.SelectHotEntryGroupId = r.read_varint_i32()?,
            16 => {
                self.RogueArchiveState = Some(
                    <RogueArchiveState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            17 => {
                let bytes = r.read_len_delimited()?;
                self.RogueArchiveInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeResultNotify {
    pub RoguelikeResultInfo: Option<RoguelikeResultInfo>,
}

impl RoguelikeResultNotify {
    pub const MSG_ID: u16 = 22995;
}

impl PbMessage for RoguelikeResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoguelikeResultInfo {
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
                self.RoguelikeResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeResultRequest {
    pub InstId: i32,
}

impl RoguelikeResultRequest {
    pub const MSG_ID: u16 = 20242;
}

impl PbMessage for RoguelikeResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeResultResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RoguelikeResultInfo: Option<RoguelikeResultInfo>,
}

impl RoguelikeResultResponse {
    pub const MSG_ID: u16 = 15568;
}

impl PbMessage for RoguelikeResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.RoguelikeResultInfo {
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
            2 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RoguelikeResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeRoleInstStartRequest {
    pub ActivityId: i32,
    pub Index: i32,
}

impl RoguelikeRoleInstStartRequest {
    pub const MSG_ID: u16 = 27659;
}

impl PbMessage for RoguelikeRoleInstStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            5 => self.ActivityId = r.read_varint_i32()?,
            7 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeRoleInstStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeRoleInstStartResponse {
    pub const MSG_ID: u16 = 18259;
}

impl PbMessage for RoguelikeRoleInstStartResponse {
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
pub struct RoguelikeRoleRoomNotify {
    pub Index: i32,
    pub RoleRoomIds: Vec<i32>,
}

impl RoguelikeRoleRoomNotify {
    pub const MSG_ID: u16 = 26103;
}

impl PbMessage for RoguelikeRoleRoomNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        for item in &self.RoleRoomIds {
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
            4 => self.Index = r.read_varint_i32()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleRoomIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeRoleRoomSelectRequest {
    pub RoleRoomId: i32,
    pub Index: i32,
}

impl RoguelikeRoleRoomSelectRequest {
    pub const MSG_ID: u16 = 18165;
}

impl PbMessage for RoguelikeRoleRoomSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleRoomId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleRoomId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            1 => self.RoleRoomId = r.read_varint_i32()?,
            9 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeRoleRoomSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeRoleRoomSelectResponse {
    pub const MSG_ID: u16 = 15944;
}

impl PbMessage for RoguelikeRoleRoomSelectResponse {
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
pub struct RoguelikeRoomInfoNotify {
    pub CurLayer: i32,
    pub MaxLayer: i32,
    pub SkyBoxId: i32,
    pub RoguelikeRoomTypeId: i32,
    pub RoguelikeRoomId: i32,
}

impl RoguelikeRoomInfoNotify {
    pub const MSG_ID: u16 = 18663;
}

impl PbMessage for RoguelikeRoomInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.SkyBoxId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkyBoxId);
        }
        if self.RoguelikeRoomTypeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomTypeId);
        }
        if self.RoguelikeRoomId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoguelikeRoomId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.CurLayer = r.read_varint_i32()?,
            7 => self.MaxLayer = r.read_varint_i32()?,
            8 => self.SkyBoxId = r.read_varint_i32()?,
            10 => self.RoguelikeRoomTypeId = r.read_varint_i32()?,
            13 => self.RoguelikeRoomId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeSeasonDataRequest {
    pub SeasonId: i32,
}

impl RoguelikeSeasonDataRequest {
    pub const MSG_ID: u16 = 15218;
}

impl PbMessage for RoguelikeSeasonDataRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeSeasonDataResponse {
    pub SeasonData: Option<SeasonData>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeSeasonDataResponse {
    pub const MSG_ID: u16 = 18829;
}

impl PbMessage for RoguelikeSeasonDataResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SeasonData {
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
                self.SeasonData
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
pub struct RoguelikeSeasonRewardReceiveRequest {
    pub Index: Vec<i32>,
    pub SeasonId: i32,
}

impl RoguelikeSeasonRewardReceiveRequest {
    pub const MSG_ID: u16 = 20035;
}

impl PbMessage for RoguelikeSeasonRewardReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Index {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
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
            4 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeSeasonRewardReceiveResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeSeasonRewardReceiveResponse {
    pub const MSG_ID: u16 = 18258;
}

impl PbMessage for RoguelikeSeasonRewardReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 1, &self.ItemMap);
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
            1 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct RoguelikeStartRequest {
    pub SelectHotEntryGroupId: i32,
    pub RoleIds: Vec<i32>,
    pub InstId: i32,
    pub SeasonId: i32,
    pub ContinueLastProgress: bool,
}

impl RoguelikeStartRequest {
    pub const MSG_ID: u16 = 22052;
}

impl PbMessage for RoguelikeStartRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SelectHotEntryGroupId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectHotEntryGroupId);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
        if self.ContinueLastProgress {
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
            3 => self.SelectHotEntryGroupId = r.read_varint_i32()?,
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.InstId = r.read_varint_i32()?,
            13 => self.SeasonId = r.read_varint_i32()?,
            14 => self.ContinueLastProgress = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeStartResponse {
    pub const MSG_ID: u16 = 22991;
}

impl PbMessage for RoguelikeStartResponse {
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
pub struct RoguelikeSubLevelNotify {
    pub A: f32,
    pub X: f32,
    pub LoadSubLevel: Vec<u8>,
    pub UnLoadSubLevel: Vec<u8>,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
    pub Y: f32,
    pub Z: f32,
    pub LoadSubLevels: Vec<Vec<u8>>,
}

impl RoguelikeSubLevelNotify {
    pub const MSG_ID: u16 = 29131;
}

impl PbMessage for RoguelikeSubLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.A != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.A).to_bits().to_le_bytes());
        }
        if self.X != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if !self.LoadSubLevel.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.LoadSubLevel);
        }
        if !self.UnLoadSubLevel.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.UnLoadSubLevel);
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 11, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        for item in &self.LoadSubLevels {
            crate::wire::write_tag(out, 15, crate::wire::WireType::LengthDelimited);
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
            2 => self.A = r.read_f32()?,
            5 => self.X = r.read_f32()?,
            9 => self.LoadSubLevel = r.read_len_delimited()?.to_vec(),
            10 => self.UnLoadSubLevel = r.read_len_delimited()?.to_vec(),
            11 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            12 => self.Y = r.read_f32()?,
            14 => self.Z = r.read_f32()?,
            15 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTalentInfoRequest {
    pub SeasonId: i32,
}

impl RoguelikeTalentInfoRequest {
    pub const MSG_ID: u16 = 17548;
}

impl PbMessage for RoguelikeTalentInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTalentInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub TalentSkillDict: BTreeMap<i32, i32>,
}

impl RoguelikeTalentInfoResponse {
    pub const MSG_ID: u16 = 29814;
}

impl PbMessage for RoguelikeTalentInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.TalentSkillDict.is_empty() {
            crate::wire::write_map(out, 15, &self.TalentSkillDict);
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
            15 => crate::wire::decode_map_entry(r, &mut self.TalentSkillDict)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTalentLevelUpRequest {
    pub SkillId: i32,
}

impl RoguelikeTalentLevelUpRequest {
    pub const MSG_ID: u16 = 29178;
}

impl PbMessage for RoguelikeTalentLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            2 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTalentLevelUpResponse {
    pub Level: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeTalentLevelUpResponse {
    pub const MSG_ID: u16 = 24087;
}

impl PbMessage for RoguelikeTalentLevelUpResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Level != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
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
            10 => self.Level = r.read_varint_i32()?,
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
pub struct RoguelikeTalentUnlockNotify {
    pub SkillId: i32,
}

impl RoguelikeTalentUnlockNotify {
    pub const MSG_ID: u16 = 17875;
}

impl PbMessage for RoguelikeTalentUnlockNotify {
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
pub struct RoguelikeToken {
    pub Id: i32,
    pub IsReceive: bool,
}

impl PbMessage for RoguelikeToken {
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
pub struct RoguelikeTokenReceiveRequest {
    pub SeasonId: i32,
    pub Id: i32,
}

impl RoguelikeTokenReceiveRequest {
    pub const MSG_ID: u16 = 18099;
}

impl PbMessage for RoguelikeTokenReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
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
            3 => self.SeasonId = r.read_varint_i32()?,
            5 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTokenReceiveResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeTokenReceiveResponse {
    pub const MSG_ID: u16 = 15495;
}

impl PbMessage for RoguelikeTokenReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 5, &self.ItemMap);
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
            5 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct RoguelikeTrialRoleInfoRequest {
    pub TrialRoleIds: Vec<i32>,
    pub InstIds: Vec<i32>,
    pub SeasonId: i32,
}

impl RoguelikeTrialRoleInfoRequest {
    pub const MSG_ID: u16 = 19261;
}

impl PbMessage for RoguelikeTrialRoleInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TrialRoleIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.InstIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.SeasonId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SeasonId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TrialRoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InstIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeTrialRoleInfoResponse {
    pub AddLevel: i32,
    pub ValidTrialRoleIds: Vec<i32>,
    pub RoleLevel: i32,
    pub MaxLevel: i32,
    pub WeaponLevel: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub RogueInstRoleDatas: Vec<RogueInstRoleData>,
}

impl RoguelikeTrialRoleInfoResponse {
    pub const MSG_ID: u16 = 15508;
}

impl PbMessage for RoguelikeTrialRoleInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AddLevel != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddLevel);
        }
        for item in &self.ValidTrialRoleIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RoleLevel != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleLevel);
        }
        if self.MaxLevel != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLevel);
        }
        if self.WeaponLevel != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponLevel);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RogueInstRoleDatas {
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
            1 => self.AddLevel = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ValidTrialRoleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => self.RoleLevel = r.read_varint_i32()?,
            5 => self.MaxLevel = r.read_varint_i32()?,
            6 => self.WeaponLevel = r.read_varint_i32()?,
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.RogueInstRoleDatas
                    .push(<RogueInstRoleData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeWriteCatBossRewardRequest {
    pub ActivityId: i32,
    pub Index: i32,
}

impl RoguelikeWriteCatBossRewardRequest {
    pub const MSG_ID: u16 = 19415;
}

impl PbMessage for RoguelikeWriteCatBossRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            15 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeWriteCatBossRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeWriteCatBossRewardResponse {
    pub const MSG_ID: u16 = 23662;
}

impl PbMessage for RoguelikeWriteCatBossRewardResponse {
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
pub struct RoguelikeWriteCatLevelPlayRewardRequest {
    pub Index: i32,
    pub ActivityId: i32,
}

impl RoguelikeWriteCatLevelPlayRewardRequest {
    pub const MSG_ID: u16 = 25589;
}

impl PbMessage for RoguelikeWriteCatLevelPlayRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
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
            3 => self.Index = r.read_varint_i32()?,
            6 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeWriteCatLevelPlayRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeWriteCatLevelPlayRewardResponse {
    pub const MSG_ID: u16 = 22369;
}

impl PbMessage for RoguelikeWriteCatLevelPlayRewardResponse {
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
pub struct RoguelikeWriteCatRewardRequest {
    pub ActivityId: i32,
    pub Index: i32,
}

impl RoguelikeWriteCatRewardRequest {
    pub const MSG_ID: u16 = 26136;
}

impl PbMessage for RoguelikeWriteCatRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            6 => self.ActivityId = r.read_varint_i32()?,
            15 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoguelikeWriteCatRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoguelikeWriteCatRewardResponse {
    pub const MSG_ID: u16 = 22912;
}

impl PbMessage for RoguelikeWriteCatRewardResponse {
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
pub struct RoleBondInfo {
    pub ConfigId: i32,
    pub Level: i32,
    pub TargetStar: i32,
    pub CurStar: i32,
}

impl PbMessage for RoleBondInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.TargetStar != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetStar);
        }
        if self.CurStar != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStar);
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
            2 => self.Level = r.read_varint_i32()?,
            3 => self.TargetStar = r.read_varint_i32()?,
            4 => self.CurStar = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleBondInfoView {
    pub ParentId: i32,
    pub OldStar: i32,
    pub NewStar: i32,
    pub OldRoleBondInfos: Vec<RoleBondInfo>,
    pub RoleIdOrTrialRoleId: i32,
}

impl PbMessage for RoleBondInfoView {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ParentId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ParentId);
        }
        if self.OldStar != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OldStar);
        }
        if self.NewStar != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NewStar);
        }
        for item in &self.OldRoleBondInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        if self.RoleIdOrTrialRoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleIdOrTrialRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ParentId = r.read_varint_i32()?,
            2 => self.OldStar = r.read_varint_i32()?,
            3 => self.NewStar = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.OldRoleBondInfos
                    .push(<RoleBondInfo as PbMessage>::decode(bytes)?);
            }
            5 => self.RoleIdOrTrialRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleBuffUnlockView {
    pub RogueResRole: Option<RogueResRole>,
    pub Buffs: Vec<i32>,
}

impl PbMessage for RoleBuffUnlockView {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResRole {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.Buffs {
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
                self.RogueResRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Buffs, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleChangeNameNotify {
    pub RoleId: i32,
    pub Name: Vec<u8>,
}

impl RoleChangeNameNotify {
    pub const MSG_ID: u16 = 27785;
}

impl PbMessage for RoleChangeNameNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
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
            12 => self.RoleId = r.read_varint_i32()?,
            14 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleDevelopConfigUpdateNotify {
    pub Configs: Option<RoleDevelopConfigs>,
}

impl RoleDevelopConfigUpdateNotify {
    pub const MSG_ID: u16 = 25103;
}

impl PbMessage for RoleDevelopConfigUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Configs {
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
                self.Configs
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorAcceptQuestRequest {
    pub Chapter: i32,
    pub RoleId: i32,
    pub QuestType: Option<FavorQuestType>,
}

impl RoleFavorAcceptQuestRequest {
    pub const MSG_ID: u16 = 27516;
}

impl PbMessage for RoleFavorAcceptQuestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Chapter != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Chapter);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.QuestType {
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
            2 => self.Chapter = r.read_varint_i32()?,
            6 => self.RoleId = r.read_varint_i32()?,
            10 => {
                self.QuestType = Some(
                    <FavorQuestType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoleFavorAcceptQuestResponse {
    pub QuestType: Option<FavorQuestType>,
    pub RoleId: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl RoleFavorAcceptQuestResponse {
    pub const MSG_ID: u16 = 17922;
}

impl PbMessage for RoleFavorAcceptQuestResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.QuestType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            3 => {
                self.QuestType = Some(
                    <FavorQuestType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.RoleId = r.read_varint_i32()?,
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
pub struct RoleFavorActiveNotify {
    pub RoleFavorInfo: Option<RoleFavor>,
}

impl RoleFavorActiveNotify {
    pub const MSG_ID: u16 = 18931;
}

impl PbMessage for RoleFavorActiveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleFavorInfo {
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
                self.RoleFavorInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorExpTipsNotify {
    pub Count: i32,
    pub ExpTips: Vec<ExpTips>,
    pub ItemId: i32,
}

impl RoleFavorExpTipsNotify {
    pub const MSG_ID: u16 = 18293;
}

impl PbMessage for RoleFavorExpTipsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Count != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        for item in &self.ExpTips {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            8 => self.Count = r.read_varint_i32()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ExpTips.push(<ExpTips as PbMessage>::decode(bytes)?);
            }
            12 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorFinishConditionNotify {
    pub RoleConditionInfoMap: BTreeMap<i32, ConditionInfo>,
}

impl RoleFavorFinishConditionNotify {
    pub const MSG_ID: u16 = 25609;
}

impl PbMessage for RoleFavorFinishConditionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RoleConditionInfoMap.is_empty() {
            crate::wire::write_map(out, 1, &self.RoleConditionInfoMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => crate::wire::decode_map_entry(r, &mut self.RoleConditionInfoMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorLevelUpdateNotify {
    pub RoleId: i32,
    pub Level: i32,
    pub Exp: i32,
}

impl RoleFavorLevelUpdateNotify {
    pub const MSG_ID: u16 = 18110;
}

impl PbMessage for RoleFavorLevelUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            5 => self.Level = r.read_varint_i32()?,
            12 => self.Exp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorNewCanUnLockNotify {
    pub CanUnLockId: i32,
    pub RoleId: i32,
    pub ItemType: Option<FavorItemType>,
}

impl RoleFavorNewCanUnLockNotify {
    pub const MSG_ID: u16 = 24807;
}

impl PbMessage for RoleFavorNewCanUnLockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanUnLockId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanUnLockId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.ItemType {
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
            6 => self.CanUnLockId = r.read_varint_i32()?,
            10 => self.RoleId = r.read_varint_i32()?,
            11 => {
                self.ItemType = Some(<FavorItemType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorNewQuestUpdateNotify {
    pub FavorQuest: Option<FavorQuest>,
    pub RoleId: i32,
}

impl RoleFavorNewQuestUpdateNotify {
    pub const MSG_ID: u16 = 26296;
}

impl PbMessage for RoleFavorNewQuestUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FavorQuest {
            crate::wire::write_submessage(out, 8, v);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.FavorQuest
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorTalkScoreRequest {
    pub Score: i32,
    pub ErrorCount: i32,
}

impl RoleFavorTalkScoreRequest {
    pub const MSG_ID: u16 = 18847;
}

impl PbMessage for RoleFavorTalkScoreRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Score != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.ErrorCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ErrorCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.Score = r.read_varint_i32()?,
            9 => self.ErrorCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorTalkScoreResponse {
    pub IsComplete: bool,
    pub ErrCode: Option<ErrorCode>,
}

impl RoleFavorTalkScoreResponse {
    pub const MSG_ID: u16 = 15509;
}

impl PbMessage for RoleFavorTalkScoreResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsComplete {
            crate::wire::write_tag(out, 5, WireType::Varint);
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
            5 => self.IsComplete = r.read_bool()?,
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
pub struct RoleFavorUnLockRequest {
    pub RoleId: i32,
    pub ItemType: Option<FavorItemType>,
    pub UnLockId: i32,
}

impl RoleFavorUnLockRequest {
    pub const MSG_ID: u16 = 18650;
}

impl PbMessage for RoleFavorUnLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.ItemType {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UnLockId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.RoleId = r.read_varint_i32()?,
            9 => {
                self.ItemType = Some(<FavorItemType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.UnLockId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleFavorUnLockResponse {
    pub RoleId: i32,
    pub ItemType: Option<FavorItemType>,
    pub UnLockId: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl RoleFavorUnLockResponse {
    pub const MSG_ID: u16 = 21000;
}

impl PbMessage for RoleFavorUnLockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.ItemType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.UnLockId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockId);
        }
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
            1 => self.RoleId = r.read_varint_i32()?,
            3 => {
                self.ItemType = Some(<FavorItemType as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.UnLockId = r.read_varint_i32()?,
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
pub struct RoleFlySkinChange {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl PbMessage for RoleFlySkinChange {
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
pub struct RoleGiftRewardRequest {
    pub TaskId: i32,
}

impl RoleGiftRewardRequest {
    pub const MSG_ID: u16 = 29332;
}

impl PbMessage for RoleGiftRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleGiftRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleGiftRewardResponse {
    pub const MSG_ID: u16 = 28753;
}

impl PbMessage for RoleGiftRewardResponse {
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
pub struct RoleGoDownNotify {
    pub EntityId: i64,
}

impl RoleGoDownNotify {
    pub const MSG_ID: u16 = 19940;
}

impl PbMessage for RoleGoDownNotify {
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
pub struct RoleGood {
    pub RoleId: i32,
    pub Result: Option<RoleSettleResult>,
}

impl PbMessage for RoleGood {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if let Some(v) = &self.Result {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                self.Result = Some(
                    <RoleSettleResult as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoleIllustratedInfo {
    pub RoleId: i32,
    pub UnlockTime: i64,
}

impl PbMessage for RoleIllustratedInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.UnlockTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionActiveNotify {
    pub RoleMotionInfo: Option<RoleMotion>,
}

impl RoleMotionActiveNotify {
    pub const MSG_ID: u16 = 23591;
}

impl PbMessage for RoleMotionActiveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleMotionInfo {
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
                self.RoleMotionInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionFinishConditionNotify {
    pub RoleConditionInfoMap: BTreeMap<i32, ConditionInfo>,
}

impl RoleMotionFinishConditionNotify {
    pub const MSG_ID: u16 = 25957;
}

impl PbMessage for RoleMotionFinishConditionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.RoleConditionInfoMap.is_empty() {
            crate::wire::write_map(out, 3, &self.RoleConditionInfoMap);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => crate::wire::decode_map_entry(r, &mut self.RoleConditionInfoMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionNewCanUnLockNotify {
    pub CanUnLockId: i32,
    pub RoleId: i32,
}

impl RoleMotionNewCanUnLockNotify {
    pub const MSG_ID: u16 = 29340;
}

impl PbMessage for RoleMotionNewCanUnLockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanUnLockId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanUnLockId);
        }
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
            7 => self.CanUnLockId = r.read_varint_i32()?,
            10 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionUnLockRequest {
    pub UnLockId: i32,
    pub RoleId: i32,
}

impl RoleMotionUnLockRequest {
    pub const MSG_ID: u16 = 29130;
}

impl PbMessage for RoleMotionUnLockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UnLockId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockId);
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
            1 => self.UnLockId = r.read_varint_i32()?,
            9 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleMotionUnLockResponse {
    pub RoleId: i32,
    pub UnLockId: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl RoleMotionUnLockResponse {
    pub const MSG_ID: u16 = 24424;
}

impl PbMessage for RoleMotionUnLockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.UnLockId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnLockId);
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
            4 => self.RoleId = r.read_varint_i32()?,
            11 => self.UnLockId = r.read_varint_i32()?,
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
pub struct RoleOrWeaponUpView {
    pub SurvivorsOption: Option<SurvivorsOption>,
}

impl PbMessage for RoleOrWeaponUpView {
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
pub struct RolePropInfo {
    pub RoleId: i32,
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
    pub PhantomProp: Option<RolePhantomPropInfo>,
}

impl PbMessage for RolePropInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.BaseProp {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.AddProp {
            crate::wire::write_submessage(out, 3, item);
        }
        if let Some(v) = &self.PhantomProp {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BaseProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.AddProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomProp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RolePropRequest {
    pub RoleIds: Vec<i32>,
}

impl RolePropRequest {
    pub const MSG_ID: u16 = 29283;
}

impl PbMessage for RolePropRequest {
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
pub struct RolePropResponse {
    pub RoleProps: Vec<RolePropInfo>,
    pub Error: Option<ErrorCode>,
}

impl RolePropResponse {
    pub const MSG_ID: u16 = 16928;
}

impl PbMessage for RolePropResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleProps {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.Error {
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
                let bytes = r.read_len_delimited()?;
                self.RoleProps
                    .push(<RolePropInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
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
pub struct RoleProperSettle {
    pub RoleId: i32,
    pub PropertyA: i32,
    pub PropertyB: i32,
    pub PropertyC: i32,
    pub Level: i32,
}

impl PbMessage for RoleProperSettle {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.PropertyA != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PropertyA);
        }
        if self.PropertyB != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PropertyB);
        }
        if self.PropertyC != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PropertyC);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.PropertyA = r.read_varint_i32()?,
            3 => self.PropertyB = r.read_varint_i32()?,
            4 => self.PropertyC = r.read_varint_i32()?,
            5 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleQuestAcceptRequest {
    pub QuestId: i32,
}

impl RoleQuestAcceptRequest {
    pub const MSG_ID: u16 = 18275;
}

impl PbMessage for RoleQuestAcceptRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.QuestId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            6 => self.QuestId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleQuestAcceptResponse {
    pub ErrorId: Option<ErrorCode>,
}

impl RoleQuestAcceptResponse {
    pub const MSG_ID: u16 = 18875;
}

impl PbMessage for RoleQuestAcceptResponse {
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
pub struct RoleQuestNewUnlockNotify {
    pub QuestIds: Vec<i32>,
}

impl RoleQuestNewUnlockNotify {
    pub const MSG_ID: u16 = 16834;
}

impl PbMessage for RoleQuestNewUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.QuestIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.QuestIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleQuestPointStateRequest {}

impl RoleQuestPointStateRequest {
    pub const MSG_ID: u16 = 29691;
}

impl PbMessage for RoleQuestPointStateRequest {
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
pub struct RoleQuestPointStateResponse {
    pub CurrPoint: i32,
    pub MaxPoint: i32,
    pub LastAddPointTime: i64,
}

impl RoleQuestPointStateResponse {
    pub const MSG_ID: u16 = 24846;
}

impl PbMessage for RoleQuestPointStateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrPoint != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrPoint);
        }
        if self.MaxPoint != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxPoint);
        }
        if self.LastAddPointTime != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastAddPointTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.CurrPoint = r.read_varint_i32()?,
            5 => self.MaxPoint = r.read_varint_i32()?,
            13 => self.LastAddPointTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleQuestStateNotify {
    pub Unlock: Vec<i32>,
    pub InProgress: Vec<i32>,
    pub UnlockPoint: i32,
    pub MaxUnlockPoint: i32,
    pub LastAddPointTime: i64,
    pub Over: Vec<i32>,
}

impl RoleQuestStateNotify {
    pub const MSG_ID: u16 = 15015;
}

impl PbMessage for RoleQuestStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Unlock {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.InProgress {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.UnlockPoint != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockPoint);
        }
        if self.MaxUnlockPoint != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxUnlockPoint);
        }
        if self.LastAddPointTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.LastAddPointTime);
        }
        for item in &self.Over {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Unlock, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.InProgress, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.UnlockPoint = r.read_varint_i32()?,
            6 => self.MaxUnlockPoint = r.read_varint_i32()?,
            10 => self.LastAddPointTime = r.read_varint_i64()?,
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Over, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSceneVoiceSwitchNotify {
    pub VoiceGroupData: i32,
}

impl RoleSceneVoiceSwitchNotify {
    pub const MSG_ID: u16 = 23430;
}

impl PbMessage for RoleSceneVoiceSwitchNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VoiceGroupData != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VoiceGroupData);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.VoiceGroupData = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleShowListUpdateNotify {
    pub RoleShowList: Vec<RoleShowEntry>,
}

impl RoleShowListUpdateNotify {
    pub const MSG_ID: u16 = 27858;
}

impl PbMessage for RoleShowListUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleShowList {
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
                self.RoleShowList
                    .push(<RoleShowEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkillInfoPb {
    pub RoleEntityConfigId: i32,
    pub SkillInfoPb: Option<SkillInfoPb>,
}

impl PbMessage for RoleSkillInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleEntityConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleEntityConfigId);
        }
        if let Some(v) = &self.SkillInfoPb {
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
            1 => self.RoleEntityConfigId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.SkillInfoPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinAddNotify {
    pub SkinIds: Vec<i32>,
}

impl RoleSkinAddNotify {
    pub const MSG_ID: u16 = 20442;
}

impl PbMessage for RoleSkinAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinChangeNotify {
    pub RoleId: i32,
    pub SkinId: i32,
}

impl RoleSkinChangeNotify {
    pub const MSG_ID: u16 = 16509;
}

impl PbMessage for RoleSkinChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.SkinId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
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
            13 => self.SkinId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinEquipData {
    pub RoleID: i32,
    pub SkinItemId: i32,
}

impl PbMessage for RoleSkinEquipData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleID != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleID);
        }
        if self.SkinItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkinItemId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoleID = r.read_varint_i32()?,
            2 => self.SkinItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinRequest {}

impl RoleSkinRequest {
    pub const MSG_ID: u16 = 26601;
}

impl PbMessage for RoleSkinRequest {
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
pub struct RoleSkinResponse {
    pub SkinIds: Vec<i32>,
}

impl RoleSkinResponse {
    pub const MSG_ID: u16 = 15433;
}

impl PbMessage for RoleSkinResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinTrialEnterCtx {
    pub ActivityId: i32,
    pub RoleSkinTrialInfoId: i32,
}

impl PbMessage for RoleSkinTrialEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.RoleSkinTrialInfoId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.RoleSkinTrialInfoId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinTrialInfoActivity {
    pub RoleSkinTrialTask: Vec<RoleSkinTrialTask>,
}

impl PbMessage for RoleSkinTrialInfoActivity {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleSkinTrialTask {
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
                self.RoleSkinTrialTask
                    .push(<RoleSkinTrialTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleSkinTrialSettleNotify {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleSkinTrialSettleNotify {
    pub const MSG_ID: u16 = 25987;
}

impl PbMessage for RoleSkinTrialSettleNotify {
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
pub struct RoleSkinTrialTask {
    pub RoleSkinTrialInfoId: i32,
    pub ChallengeState: Option<ChallengeState>,
}

impl PbMessage for RoleSkinTrialTask {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleSkinTrialInfoId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleSkinTrialInfoId);
        }
        if let Some(v) = &self.ChallengeState {
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
            1 => self.RoleSkinTrialInfoId = r.read_varint_i32()?,
            2 => {
                self.ChallengeState = Some(
                    <ChallengeState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoleSkinTrialUiEndPush {}

impl RoleSkinTrialUiEndPush {
    pub const MSG_ID: u16 = 26571;
}

impl PbMessage for RoleSkinTrialUiEndPush {
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
pub struct RoleSkinUpdateNotify {
    pub SkinIds: Vec<i32>,
}

impl RoleSkinUpdateNotify {
    pub const MSG_ID: u16 = 26056;
}

impl PbMessage for RoleSkinUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTeachEnterCtx {
    pub RoleId: i32,
}

impl PbMessage for RoleTeachEnterCtx {
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
pub struct RoleTowerAttributeQueryRequest {
    pub ChallengeId: i32,
    pub RoleIds: Vec<i32>,
    pub UseInitInstData: bool,
}

impl RoleTowerAttributeQueryRequest {
    pub const MSG_ID: u16 = 26515;
}

impl PbMessage for RoleTowerAttributeQueryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.UseInitInstData {
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
            8 => self.ChallengeId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => self.UseInitInstData = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerAttributeQueryResponse {
    pub RoleInfos: Vec<RoleTowerRoleAttributeInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomLevel: i32,
}

impl RoleTowerAttributeQueryResponse {
    pub const MSG_ID: u16 = 29708;
}

impl PbMessage for RoleTowerAttributeQueryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PhantomLevel != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomLevel);
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
                self.RoleInfos
                    .push(<RoleTowerRoleAttributeInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.PhantomLevel = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerBattleStepPbData {
    pub TimeData: Option<RoleTowerTimerData>,
}

impl PbMessage for RoleTowerBattleStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TimeData {
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
                self.TimeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeSettleInfo {
    pub ActivityId: i32,
    pub ChallengeId: i32,
    pub SettleReason: Option<RoleTowerSettleReason>,
    pub CurrentRecord: Option<RoleTowerBestRecord>,
    pub BestRecord: Option<RoleTowerBestRecord>,
    pub IsBestRecordUpdated: bool,
    pub CurrentRankPercent: i32,
    pub BestRankPercent: i32,
    pub RankErrorCode: Option<ErrorCode>,
}

impl PbMessage for RoleTowerChallengeSettleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if let Some(v) = &self.SettleReason {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.CurrentRecord {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.BestRecord {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.IsBestRecordUpdated {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CurrentRankPercent != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrentRankPercent);
        }
        if self.BestRankPercent != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BestRankPercent);
        }
        if let Some(v) = &self.RankErrorCode {
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.ChallengeId = r.read_varint_i32()?,
            3 => {
                self.SettleReason = Some(
                    <RoleTowerSettleReason as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CurrentRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.BestRecord
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.IsBestRecordUpdated = r.read_bool()?,
            7 => self.CurrentRankPercent = r.read_varint_i32()?,
            8 => self.BestRankPercent = r.read_varint_i32()?,
            9 => {
                self.RankErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeSettleNotify {
    pub ChallengeSettleInfo: Option<RoleTowerChallengeSettleInfo>,
}

impl RoleTowerChallengeSettleNotify {
    pub const MSG_ID: u16 = 15085;
}

impl PbMessage for RoleTowerChallengeSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ChallengeSettleInfo {
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
                self.ChallengeSettleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeSettleRequest {
    pub ChallengeId: i32,
}

impl RoleTowerChallengeSettleRequest {
    pub const MSG_ID: u16 = 19036;
}

impl PbMessage for RoleTowerChallengeSettleRequest {
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
pub struct RoleTowerChallengeSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ChallengeSettleInfo: Option<RoleTowerChallengeSettleInfo>,
}

impl RoleTowerChallengeSettleResponse {
    pub const MSG_ID: u16 = 18185;
}

impl PbMessage for RoleTowerChallengeSettleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChallengeSettleInfo {
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
                self.ChallengeSettleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerChallengeUpdateNotify {
    pub ChallengeInfos: Vec<RoleTowerChallengeInfo>,
}

impl RoleTowerChallengeUpdateNotify {
    pub const MSG_ID: u16 = 21751;
}

impl PbMessage for RoleTowerChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeInfos {
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
                self.ChallengeInfos
                    .push(<RoleTowerChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerConsumeItemRequest {
    pub ItemId: i32,
}

impl RoleTowerConsumeItemRequest {
    pub const MSG_ID: u16 = 17781;
}

impl PbMessage for RoleTowerConsumeItemRequest {
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
pub struct RoleTowerConsumeItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerConsumeItemResponse {
    pub const MSG_ID: u16 = 16552;
}

impl PbMessage for RoleTowerConsumeItemResponse {
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
pub struct RoleTowerCtx {
    pub Param: Option<RoleTowerInstParam>,
}

impl PbMessage for RoleTowerCtx {
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
pub struct RoleTowerDieNotify {}

impl RoleTowerDieNotify {
    pub const MSG_ID: u16 = 26184;
}

impl PbMessage for RoleTowerDieNotify {
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
pub struct RoleTowerFloorSettleNotify {
    pub RemainTimeMs: i64,
    pub TotalCostTimeMs: i64,
    pub FloorId: i32,
    pub TotalGainTimeMs: i64,
}

impl RoleTowerFloorSettleNotify {
    pub const MSG_ID: u16 = 22389;
}

impl PbMessage for RoleTowerFloorSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        if self.TotalCostTimeMs != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCostTimeMs);
        }
        if self.FloorId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FloorId);
        }
        if self.TotalGainTimeMs != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalGainTimeMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.RemainTimeMs = r.read_varint_i64()?,
            8 => self.TotalCostTimeMs = r.read_varint_i64()?,
            9 => self.FloorId = r.read_varint_i32()?,
            15 => self.TotalGainTimeMs = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerInstParam {
    pub LevelId: i32,
    pub IsContinue: bool,
    pub TokenId: i32,
    pub BranchId: i32,
    pub RoleSkillBranchIds: BTreeMap<i32, i32>,
}

impl PbMessage for RoleTowerInstParam {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.IsContinue {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TokenId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenId);
        }
        if self.BranchId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchId);
        }
        if !self.RoleSkillBranchIds.is_empty() {
            crate::wire::write_map(out, 5, &self.RoleSkillBranchIds);
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
            2 => self.IsContinue = r.read_bool()?,
            3 => self.TokenId = r.read_varint_i32()?,
            4 => self.BranchId = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.RoleSkillBranchIds)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerItemUpdateNotify {
    pub Items: BTreeMap<i32, i32>,
}

impl RoleTowerItemUpdateNotify {
    pub const MSG_ID: u16 = 23300;
}

impl PbMessage for RoleTowerItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Items.is_empty() {
            crate::wire::write_map(out, 2, &self.Items);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerLoadSubLevelStepPbData {
    pub LoadSubLevels: Vec<Vec<u8>>,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
}

impl PbMessage for RoleTowerLoadSubLevelStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LoadSubLevels {
            crate::wire::write_tag(out, 7, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 8, crate::wire::WireType::LengthDelimited);
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
            7 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            8 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPhantomAddAttrsRequest {
    pub RoleIds: Vec<i32>,
    pub ChallengeId: i32,
}

impl RoleTowerPhantomAddAttrsRequest {
    pub const MSG_ID: u16 = 29253;
}

impl PbMessage for RoleTowerPhantomAddAttrsRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPhantomAddAttrsResponse {
    pub PhantomAddAttrInfos: Vec<RoleTowerPhantomAddAttr>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerPhantomAddAttrsResponse {
    pub const MSG_ID: u16 = 25433;
}

impl PbMessage for RoleTowerPhantomAddAttrsResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomAddAttrInfos {
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
                self.PhantomAddAttrInfos
                    .push(<RoleTowerPhantomAddAttr as PbMessage>::decode(bytes)?);
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
pub struct RoleTowerPhantomAddAttrsUpdateNotify {
    pub PhantomAddAttrInfos: Vec<RoleTowerPhantomAddAttr>,
    pub PhantomAddAttrs: BTreeMap<i32, i32>,
}

impl RoleTowerPhantomAddAttrsUpdateNotify {
    pub const MSG_ID: u16 = 26773;
}

impl PbMessage for RoleTowerPhantomAddAttrsUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomAddAttrInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        if !self.PhantomAddAttrs.is_empty() {
            crate::wire::write_map(out, 15, &self.PhantomAddAttrs);
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
                self.PhantomAddAttrInfos
                    .push(<RoleTowerPhantomAddAttr as PbMessage>::decode(bytes)?);
            }
            15 => crate::wire::decode_map_entry(r, &mut self.PhantomAddAttrs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPhantomAttributeInfo {
    pub PhantomId: i32,
    pub SlotIndex: i32,
    pub TotalAttrs: Option<RolePhantomPropInfo>,
}

impl PbMessage for RoleTowerPhantomAttributeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomId);
        }
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
        if let Some(v) = &self.TotalAttrs {
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
            1 => self.PhantomId = r.read_varint_i32()?,
            2 => self.SlotIndex = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.TotalAttrs
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPrepareInfo {
    pub ChallengeId: i32,
    pub CurStageId: i32,
    pub RemainTimeMs: i64,
    pub Roles: Vec<RoleTowerRoleBrief>,
    pub RoleEnergyInfos: Vec<RoleTowerRoleEnergyInfo>,
    pub TokenPanelInfo: Option<RoleTowerTokenPanelInfo>,
    pub EquipTokenId: i32,
    pub StageInfos: Vec<RoleTowerStageInfo>,
    pub NeedBranchSelect: bool,
    pub BranchFloorInfo: Option<RoleTowerFloorInfo>,
    pub TeamStarInfo: BTreeMap<i32, i32>,
    pub PhantomInfo: Option<RoleTowerPhantomInfo>,
    pub NeedOpenUi: bool,
}

impl PbMessage for RoleTowerPrepareInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CurStageId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurStageId);
        }
        if self.RemainTimeMs != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RemainTimeMs);
        }
        for item in &self.Roles {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RoleEnergyInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.TokenPanelInfo {
            crate::wire::write_submessage(out, 8, v);
        }
        if self.EquipTokenId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipTokenId);
        }
        for item in &self.StageInfos {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.NeedBranchSelect {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.BranchFloorInfo {
            crate::wire::write_submessage(out, 12, v);
        }
        if !self.TeamStarInfo.is_empty() {
            crate::wire::write_map(out, 13, &self.TeamStarInfo);
        }
        if let Some(v) = &self.PhantomInfo {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.NeedOpenUi {
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
            2 => self.ChallengeId = r.read_varint_i32()?,
            3 => self.CurStageId = r.read_varint_i32()?,
            4 => self.RemainTimeMs = r.read_varint_i64()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Roles
                    .push(<RoleTowerRoleBrief as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEnergyInfos
                    .push(<RoleTowerRoleEnergyInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.TokenPanelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.EquipTokenId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.StageInfos
                    .push(<RoleTowerStageInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.NeedBranchSelect = r.read_bool()?,
            12 => {
                let bytes = r.read_len_delimited()?;
                self.BranchFloorInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => crate::wire::decode_map_entry(r, &mut self.TeamStarInfo)?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.NeedOpenUi = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPrepareSelection {
    pub Roles: Vec<RoleTowerRoleBrief>,
    pub EquipTokenId: i32,
    pub BranchIndex: i32,
}

impl PbMessage for RoleTowerPrepareSelection {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.EquipTokenId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EquipTokenId);
        }
        if self.BranchIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BranchIndex);
        }
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
                self.Roles
                    .push(<RoleTowerRoleBrief as PbMessage>::decode(bytes)?);
            }
            2 => self.EquipTokenId = r.read_varint_i32()?,
            3 => self.BranchIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerPrepareStepPbData {
    pub PrepareInfo: Option<RoleTowerPrepareInfo>,
    pub PrepareSelection: Option<RoleTowerPrepareSelection>,
}

impl PbMessage for RoleTowerPrepareStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PrepareInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.PrepareSelection {
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
                self.PrepareInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PrepareSelection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankEntry {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub Rank: i32,
    pub RankPercent: i32,
    pub Record: Option<RoleTowerBestRecord>,
    pub Remark: Vec<u8>,
}

impl PbMessage for RoleTowerRankEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Rank != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Rank);
        }
        if self.RankPercent != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankPercent);
        }
        if let Some(v) = &self.Record {
            crate::wire::write_submessage(out, 5, v);
        }
        if !self.Remark.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            3 => self.Rank = r.read_varint_i32()?,
            4 => self.RankPercent = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Record
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.Remark = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankInfo {
    pub ChallengeId: i32,
    pub RankType: Option<RoleTowerRankType>,
    pub RewardSettleTime: i64,
    pub FriendEntries: Vec<RoleTowerRankEntry>,
    pub SelfServerEntry: Option<RoleTowerRankEntry>,
    pub CanGetServerRankReward: bool,
    pub State: Option<ConditionTaskState>,
    pub RewardPreview: Vec<RewardItem>,
    pub NextRefreshTime: i64,
}

impl PbMessage for RoleTowerRankInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if let Some(v) = &self.RankType {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.RewardSettleTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RewardSettleTime);
        }
        for item in &self.FriendEntries {
            crate::wire::write_submessage(out, 6, item);
        }
        if let Some(v) = &self.SelfServerEntry {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.CanGetServerRankReward {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.State {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RewardPreview {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.NextRefreshTime != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            2 => self.ChallengeId = r.read_varint_i32()?,
            3 => {
                self.RankType = Some(
                    <RoleTowerRankType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.RewardSettleTime = r.read_varint_i64()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.FriendEntries
                    .push(<RoleTowerRankEntry as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.SelfServerEntry
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.CanGetServerRankReward = r.read_bool()?,
            9 => {
                self.State = Some(
                    <ConditionTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.RewardPreview
                    .push(<RewardItem as PbMessage>::decode(bytes)?);
            }
            11 => self.NextRefreshTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankRequest {
    pub ActivityId: i32,
}

impl RoleTowerRankRequest {
    pub const MSG_ID: u16 = 19891;
}

impl PbMessage for RoleTowerRankRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankResponse {
    pub RankInfo: Vec<RoleTowerRankInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerRankResponse {
    pub const MSG_ID: u16 = 25174;
}

impl PbMessage for RoleTowerRankResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RankInfo {
            crate::wire::write_submessage(out, 3, item);
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
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RankInfo
                    .push(<RoleTowerRankInfo as PbMessage>::decode(bytes)?);
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
pub struct RoleTowerRankRewardReceiveRequest {
    pub ChallengeIds: Vec<i32>,
}

impl RoleTowerRankRewardReceiveRequest {
    pub const MSG_ID: u16 = 23829;
}

impl PbMessage for RoleTowerRankRewardReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChallengeIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChallengeIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRankRewardReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerRankRewardReceiveResponse {
    pub const MSG_ID: u16 = 24106;
}

impl PbMessage for RoleTowerRankRewardReceiveResponse {
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
pub struct RoleTowerRewardTaskReceiveRequest {
    pub TaskIds: Vec<i32>,
}

impl RoleTowerRewardTaskReceiveRequest {
    pub const MSG_ID: u16 = 29411;
}

impl PbMessage for RoleTowerRewardTaskReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskIds {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRewardTaskReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerRewardTaskReceiveResponse {
    pub const MSG_ID: u16 = 18563;
}

impl PbMessage for RoleTowerRewardTaskReceiveResponse {
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
pub struct RoleTowerRewardTaskUpdateNotify {
    pub LimitTasks: Vec<ConditionTask>,
}

impl RoleTowerRewardTaskUpdateNotify {
    pub const MSG_ID: u16 = 23293;
}

impl PbMessage for RoleTowerRewardTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.LimitTasks {
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
                self.LimitTasks
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerRoleAttributeInfo {
    pub RoleProp: Option<RolePropInfo>,
    pub PhantomInfos: Vec<RoleTowerPhantomAttributeInfo>,
}

impl PbMessage for RoleTowerRoleAttributeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleProp {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.PhantomInfos {
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
                self.RoleProp
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomInfos
                    .push(<RoleTowerPhantomAttributeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerSaveDeleteRequest {
    pub ChallengeId: i32,
}

impl RoleTowerSaveDeleteRequest {
    pub const MSG_ID: u16 = 29169;
}

impl PbMessage for RoleTowerSaveDeleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerSaveDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerSaveDeleteResponse {
    pub const MSG_ID: u16 = 15219;
}

impl PbMessage for RoleTowerSaveDeleteResponse {
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
pub struct RoleTowerScoreRewardReceiveRequest {
    pub RewardIds: Vec<i32>,
}

impl RoleTowerScoreRewardReceiveRequest {
    pub const MSG_ID: u16 = 20934;
}

impl PbMessage for RoleTowerScoreRewardReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RewardIds {
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
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerScoreRewardReceiveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerScoreRewardReceiveResponse {
    pub const MSG_ID: u16 = 25210;
}

impl PbMessage for RoleTowerScoreRewardReceiveResponse {
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
pub struct RoleTowerScoreTaskUpdateNotify {
    pub ScoreTasks: Vec<i32>,
}

impl RoleTowerScoreTaskUpdateNotify {
    pub const MSG_ID: u16 = 24764;
}

impl PbMessage for RoleTowerScoreTaskUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ScoreTasks {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ScoreTasks, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerScoreUpdateNotify {
    pub ActivityId: i32,
    pub Score: i32,
}

impl RoleTowerScoreUpdateNotify {
    pub const MSG_ID: u16 = 28517;
}

impl PbMessage for RoleTowerScoreUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            8 => self.ActivityId = r.read_varint_i32()?,
            10 => self.Score = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerSettleStepPbData {
    pub StageSettleInfo: Option<SettleStepPbData>,
}

impl PbMessage for RoleTowerSettleStepPbData {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.StageSettleInfo {
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
                self.StageSettleInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerStageRetryRequest {
    pub Selection: Option<RoleTowerPrepareSelection>,
    pub ChallengeId: i32,
}

impl RoleTowerStageRetryRequest {
    pub const MSG_ID: u16 = 28382;
}

impl PbMessage for RoleTowerStageRetryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Selection {
            crate::wire::write_submessage(out, 1, v);
        }
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.Selection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.ChallengeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerStageRetryResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerStageRetryResponse {
    pub const MSG_ID: u16 = 27529;
}

impl PbMessage for RoleTowerStageRetryResponse {
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
pub struct RoleTowerStageSettleRequest {}

impl RoleTowerStageSettleRequest {
    pub const MSG_ID: u16 = 23809;
}

impl PbMessage for RoleTowerStageSettleRequest {
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
pub struct RoleTowerStageSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerStageSettleResponse {
    pub const MSG_ID: u16 = 15371;
}

impl PbMessage for RoleTowerStageSettleResponse {
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
pub struct RoleTowerStepAdvanceRequest {
    pub SettleStepPbData: Option<RoleTowerSettleStepPbData>,
    pub TeamUpgradeStepPbData: Option<RoleTowerTeamUpgradeStepPbData>,
    pub BattleStepPbData: Option<RoleTowerBattleStepPbData>,
    pub LoadSubLevelStepPbData: Option<RoleTowerLoadSubLevelStepPbData>,
    pub PrepareStepPbData: Option<RoleTowerPrepareStepPbData>,
}

impl RoleTowerStepAdvanceRequest {
    pub const MSG_ID: u16 = 23562;
}

impl PbMessage for RoleTowerStepAdvanceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SettleStepPbData {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.TeamUpgradeStepPbData {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.BattleStepPbData {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.LoadSubLevelStepPbData {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.PrepareStepPbData {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.SettleStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.TeamUpgradeStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.BattleStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.LoadSubLevelStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PrepareStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerStepAdvanceResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTowerStepAdvanceResponse {
    pub const MSG_ID: u16 = 28939;
}

impl PbMessage for RoleTowerStepAdvanceResponse {
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
pub struct RoleTowerStepUpdateNotify {
    pub PrepareStepPbData: Option<RoleTowerPrepareStepPbData>,
    pub TeamUpgradeStepPbData: Option<RoleTowerTeamUpgradeStepPbData>,
    pub NeedOpenUi: bool,
    pub SettleStepPbData: Option<RoleTowerSettleStepPbData>,
    pub LoadSubLevelStepPbData: Option<RoleTowerLoadSubLevelStepPbData>,
    pub BattleStepPbData: Option<RoleTowerBattleStepPbData>,
}

impl RoleTowerStepUpdateNotify {
    pub const MSG_ID: u16 = 25704;
}

impl PbMessage for RoleTowerStepUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PrepareStepPbData {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.TeamUpgradeStepPbData {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.NeedOpenUi {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.SettleStepPbData {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.LoadSubLevelStepPbData {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.BattleStepPbData {
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
                let bytes = r.read_len_delimited()?;
                self.PrepareStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.TeamUpgradeStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => self.NeedOpenUi = r.read_bool()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.SettleStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.LoadSubLevelStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.BattleStepPbData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTeamUpgradeStepPbData {}

impl PbMessage for RoleTowerTeamUpgradeStepPbData {
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
pub struct RoleTowerTimerData {
    pub StartTime: i64,
    pub Duration: i64,
    pub PauseEndTime: i64,
    pub PauseStartTime: i64,
    pub ChangeTime: i64,
}

impl PbMessage for RoleTowerTimerData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StartTime != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.StartTime);
        }
        if self.Duration != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Duration);
        }
        if self.PauseEndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PauseEndTime);
        }
        if self.PauseStartTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PauseStartTime);
        }
        if self.ChangeTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ChangeTime);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StartTime = r.read_varint_i64()?,
            2 => self.Duration = r.read_varint_i64()?,
            3 => self.PauseEndTime = r.read_varint_i64()?,
            4 => self.PauseStartTime = r.read_varint_i64()?,
            5 => self.ChangeTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTimerDataUpdateNotify {
    pub TimeData: Option<RoleTowerTimerData>,
}

impl RoleTowerTimerDataUpdateNotify {
    pub const MSG_ID: u16 = 20667;
}

impl PbMessage for RoleTowerTimerDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TimeData {
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
                self.TimeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTimerDurationUpdateNotify {
    pub TimeData: Option<RoleTowerTimerData>,
    pub DurationUpdates: Vec<i32>,
}

impl RoleTowerTimerDurationUpdateNotify {
    pub const MSG_ID: u16 = 19870;
}

impl PbMessage for RoleTowerTimerDurationUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TimeData {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.DurationUpdates {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TimeData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DurationUpdates,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTokenInfo {
    pub TokenId: i32,
    pub ItemId: i32,
    pub Count: i32,
    pub IsRecommend: bool,
    pub RemainUseCount: i32,
    pub CdEndTimeMs: i64,
}

impl PbMessage for RoleTowerTokenInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TokenId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TokenId);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IsRecommend {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RemainUseCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainUseCount);
        }
        if self.CdEndTimeMs != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CdEndTimeMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TokenId = r.read_varint_i32()?,
            2 => self.ItemId = r.read_varint_i32()?,
            3 => self.Count = r.read_varint_i32()?,
            8 => self.IsRecommend = r.read_bool()?,
            9 => self.RemainUseCount = r.read_varint_i32()?,
            10 => self.CdEndTimeMs = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTokenPanelInfo {
    pub Tokens: Vec<RoleTowerTokenInfo>,
}

impl PbMessage for RoleTowerTokenPanelInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Tokens {
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
                self.Tokens
                    .push(<RoleTowerTokenInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerTokenUpdateNotify {
    pub TokenPanelInfo: Option<RoleTowerTokenPanelInfo>,
}

impl RoleTowerTokenUpdateNotify {
    pub const MSG_ID: u16 = 18236;
}

impl PbMessage for RoleTowerTokenUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TokenPanelInfo {
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
                self.TokenPanelInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTowerUnlockTokenUpdateNotify {
    pub UnlockTokens: Vec<i32>,
}

impl RoleTowerUnlockTokenUpdateNotify {
    pub const MSG_ID: u16 = 20373;
}

impl PbMessage for RoleTowerUnlockTokenUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockTokens {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockTokens, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrainRequest {
    pub TrainType: i32,
    pub RoleId: i32,
}

impl RoleTrainRequest {
    pub const MSG_ID: u16 = 15533;
}

impl PbMessage for RoleTrainRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TrainType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TrainType);
        }
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
            4 => self.TrainType = r.read_varint_i32()?,
            7 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrainResponse {
    pub GreatSucc: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleTrainResponse {
    pub const MSG_ID: u16 = 22364;
}

impl PbMessage for RoleTrainResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GreatSucc {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            3 => self.GreatSucc = r.read_bool()?,
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
pub struct RoleTrialEnterCtx {
    pub ActivityId: i32,
    pub RoleId: i32,
}

impl PbMessage for RoleTrialEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleTrialUiEndPush {}

impl RoleTrialUiEndPush {
    pub const MSG_ID: u16 = 29512;
}

impl PbMessage for RoleTrialUiEndPush {
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
pub struct RoleUpdateDevelopTargetNotify {
    pub FirstPhantomId: i32,
    pub RoleId: i32,
    pub PlanId: i32,
}

impl RoleUpdateDevelopTargetNotify {
    pub const MSG_ID: u16 = 29070;
}

impl PbMessage for RoleUpdateDevelopTargetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FirstPhantomId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstPhantomId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.PlanId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlanId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FirstPhantomId = r.read_varint_i32()?,
            5 => self.RoleId = r.read_varint_i32()?,
            14 => self.PlanId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleUpdateDevelopTargetRequest {
    pub RoleId: i32,
    pub FirstPhantomId: i32,
    pub Core: i32,
    pub PlanId: i32,
    pub Source: i32,
}

impl RoleUpdateDevelopTargetRequest {
    pub const MSG_ID: u16 = 29614;
}

impl PbMessage for RoleUpdateDevelopTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.FirstPhantomId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FirstPhantomId);
        }
        if self.Core != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Core);
        }
        if self.PlanId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlanId);
        }
        if self.Source != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Source);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.RoleId = r.read_varint_i32()?,
            4 => self.FirstPhantomId = r.read_varint_i32()?,
            7 => self.Core = r.read_varint_i32()?,
            9 => self.PlanId = r.read_varint_i32()?,
            11 => self.Source = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoleUpdateDevelopTargetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoleUpdateDevelopTargetResponse {
    pub const MSG_ID: u16 = 16891;
}

impl PbMessage for RoleUpdateDevelopTargetResponse {
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
pub struct RoleWeaponSkinComponentPb {
    pub WeaponSkinId: i32,
}

impl PbMessage for RoleWeaponSkinComponentPb {
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
pub struct RollBlockCloseGamePlayNotify {
    pub IncId: i32,
}

impl RollBlockCloseGamePlayNotify {
    pub const MSG_ID: u16 = 21222;
}

impl PbMessage for RollBlockCloseGamePlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
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
            11 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockExitGamePlayRequest {
    pub IncId: i32,
}

impl RollBlockExitGamePlayRequest {
    pub const MSG_ID: u16 = 28361;
}

impl PbMessage for RollBlockExitGamePlayRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            9 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockExitGamePlayResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockExitGamePlayResponse {
    pub const MSG_ID: u16 = 21367;
}

impl PbMessage for RollBlockExitGamePlayResponse {
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
pub struct RollBlockGamePlayPbInfo {
    pub IncId: i32,
    pub GroupId: i32,
    pub Difficulty: i32,
    pub IsMainControl: bool,
    pub InitVisibleEntityIds: Vec<i64>,
    pub Position: Option<Vector>,
    pub Rotation: Option<Vector>,
    pub AvailableInputs: Vec<RbInput>,
    pub GridSizeX: i32,
    pub GridSizeY: i32,
    pub ShowHintErrorInputCount: i32,
    pub SubCameraTag: i32,
    pub TotalDifficultyCount: i32,
    pub ReadyDelaySecond: f32,
    pub InitInvisibleEntityIds: Vec<i64>,
}

impl PbMessage for RollBlockGamePlayPbInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.Difficulty != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Difficulty);
        }
        if self.IsMainControl {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.InitVisibleEntityIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if let Some(v) = &self.Position {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.Rotation {
            crate::wire::write_submessage(out, 7, v);
        }
        for item in &self.AvailableInputs {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.GridSizeX != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridSizeX);
        }
        if self.GridSizeY != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GridSizeY);
        }
        if self.ShowHintErrorInputCount != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowHintErrorInputCount);
        }
        if self.SubCameraTag != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SubCameraTag);
        }
        if self.TotalDifficultyCount != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalDifficultyCount);
        }
        if self.ReadyDelaySecond != 0.0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.ReadyDelaySecond).to_bits().to_le_bytes());
        }
        for item in &self.InitInvisibleEntityIds {
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
            1 => self.IncId = r.read_varint_i32()?,
            2 => self.GroupId = r.read_varint_i32()?,
            3 => self.Difficulty = r.read_varint_i32()?,
            4 => self.IsMainControl = r.read_bool()?,
            5 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.InitVisibleEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AvailableInputs
                    .push(<RbInput as PbMessage>::decode(bytes)?);
            }
            9 => self.GridSizeX = r.read_varint_i32()?,
            10 => self.GridSizeY = r.read_varint_i32()?,
            11 => self.ShowHintErrorInputCount = r.read_varint_i32()?,
            12 => self.SubCameraTag = r.read_varint_i32()?,
            13 => self.TotalDifficultyCount = r.read_varint_i32()?,
            14 => self.ReadyDelaySecond = r.read_f32()?,
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.InitInvisibleEntityIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockGamePlayReadyRequest {
    pub IncId: i32,
}

impl RollBlockGamePlayReadyRequest {
    pub const MSG_ID: u16 = 26018;
}

impl PbMessage for RollBlockGamePlayReadyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockGamePlayReadyResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockGamePlayReadyResponse {
    pub const MSG_ID: u16 = 23212;
}

impl PbMessage for RollBlockGamePlayReadyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
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
pub struct RollBlockInputRequest {
    pub Input: Option<RbInput>,
    pub IncId: i32,
}

impl RollBlockInputRequest {
    pub const MSG_ID: u16 = 19364;
}

impl PbMessage for RollBlockInputRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Input {
            crate::wire::write_submessage(out, 6, v);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Input
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockInputResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockInputResponse {
    pub const MSG_ID: u16 = 17349;
}

impl PbMessage for RollBlockInputResponse {
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
pub struct RollBlockItemUpdateNotify {
    pub Component: Option<RbItemComponentPb>,
    pub EntityId: i64,
}

impl RollBlockItemUpdateNotify {
    pub const MSG_ID: u16 = 22864;
}

impl PbMessage for RollBlockItemUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Component {
            crate::wire::write_submessage(out, 7, v);
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
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Component
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockMovementFinishRequest {
    pub EntityId: i64,
    pub IncId: i32,
}

impl RollBlockMovementFinishRequest {
    pub const MSG_ID: u16 = 25774;
}

impl PbMessage for RollBlockMovementFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
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
            6 => self.EntityId = r.read_varint_i64()?,
            12 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockMovementFinishResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockMovementFinishResponse {
    pub const MSG_ID: u16 = 20004;
}

impl PbMessage for RollBlockMovementFinishResponse {
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
pub struct RollBlockOpenGamePlayNotify {
    pub Info: Option<RollBlockGamePlayPbInfo>,
}

impl RollBlockOpenGamePlayNotify {
    pub const MSG_ID: u16 = 15482;
}

impl PbMessage for RollBlockOpenGamePlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct RollBlockResetGamePlayNotify {
    pub Info: Option<RollBlockGamePlayPbInfo>,
}

impl RollBlockResetGamePlayNotify {
    pub const MSG_ID: u16 = 26331;
}

impl PbMessage for RollBlockResetGamePlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct RollBlockResetGamePlayRequest {
    pub IncId: i32,
}

impl RollBlockResetGamePlayRequest {
    pub const MSG_ID: u16 = 22610;
}

impl PbMessage for RollBlockResetGamePlayRequest {
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
pub struct RollBlockResetGamePlayResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockResetGamePlayResponse {
    pub const MSG_ID: u16 = 26645;
}

impl PbMessage for RollBlockResetGamePlayResponse {
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
pub struct RollBlockSwitchControlBlockRequest {
    pub IncId: i32,
}

impl RollBlockSwitchControlBlockRequest {
    pub const MSG_ID: u16 = 17791;
}

impl PbMessage for RollBlockSwitchControlBlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            14 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockSwitchControlBlockResponse {
    pub CurMainControlBlockEntityId: i64,
    pub Code: Option<ErrorCode>,
    pub PreMainControlBlockEntityId: i64,
}

impl RollBlockSwitchControlBlockResponse {
    pub const MSG_ID: u16 = 22237;
}

impl PbMessage for RollBlockSwitchControlBlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurMainControlBlockEntityId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CurMainControlBlockEntityId);
        }
        if let Some(v) = &self.Code {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PreMainControlBlockEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PreMainControlBlockEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.CurMainControlBlockEntityId = r.read_varint_i64()?,
            13 => {
                self.Code = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.PreMainControlBlockEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockSyncInfoNotify {
    pub FullInfos: Vec<RollBlockGamePlayPbInfo>,
}

impl RollBlockSyncInfoNotify {
    pub const MSG_ID: u16 = 29588;
}

impl PbMessage for RollBlockSyncInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FullInfos {
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
                self.FullInfos
                    .push(<RollBlockGamePlayPbInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockToggleHintRequest {
    pub IncId: i32,
    pub IsOpen: bool,
}

impl RollBlockToggleHintRequest {
    pub const MSG_ID: u16 = 15560;
}

impl PbMessage for RollBlockToggleHintRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.IsOpen {
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
            3 => self.IncId = r.read_varint_i32()?,
            13 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockToggleHintResponse {
    pub Code: Option<ErrorCode>,
}

impl RollBlockToggleHintResponse {
    pub const MSG_ID: u16 = 16448;
}

impl PbMessage for RollBlockToggleHintResponse {
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
pub struct RollBlockUpdateAvailableInputsNotify {
    pub IncId: i32,
    pub AvailableInputs: Vec<RbInput>,
}

impl RollBlockUpdateAvailableInputsNotify {
    pub const MSG_ID: u16 = 26586;
}

impl PbMessage for RollBlockUpdateAvailableInputsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        for item in &self.AvailableInputs {
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
            2 => self.IncId = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AvailableInputs
                    .push(<RbInput as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RollBlockUpdateGamePlayNotify {
    pub Info: Option<RollBlockGamePlayPbInfo>,
}

impl RollBlockUpdateGamePlayNotify {
    pub const MSG_ID: u16 = 24110;
}

impl PbMessage for RollBlockUpdateGamePlayNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
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
pub struct RollBuffBondLinkIdOp {
    pub RogueResOption: Option<RogueResOption>,
    pub RoleBondInfoView: Option<RoleBondInfoView>,
    pub LinkId: Vec<i32>,
}

impl PbMessage for RollBuffBondLinkIdOp {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RogueResOption {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.RoleBondInfoView {
            crate::wire::write_submessage(out, 2, v);
        }
        for item in &self.LinkId {
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
                self.RogueResOption
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleBondInfoView
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LinkId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueAttChange {
    pub AttId: i32,
    pub ChangeAttValue: i32,
}

impl PbMessage for RoverRogueAttChange {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttId);
        }
        if self.ChangeAttValue != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChangeAttValue);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.AttId = r.read_varint_i32()?,
            2 => self.ChangeAttValue = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueBlessGroupData {
    pub BlessGroupId: i32,
    pub BlessIdList: Vec<i32>,
}

impl PbMessage for RoverRogueBlessGroupData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BlessGroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlessGroupId);
        }
        for item in &self.BlessIdList {
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
            2 => self.BlessGroupId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BlessIdList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueBlessGroupSelectNotify {
    pub IsToken: bool,
    pub BlessGroupData: Vec<RoverRogueBlessGroupData>,
}

impl RoverRogueBlessGroupSelectNotify {
    pub const MSG_ID: u16 = 23210;
}

impl PbMessage for RoverRogueBlessGroupSelectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsToken {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.BlessGroupData {
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
            4 => self.IsToken = r.read_bool()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.BlessGroupData
                    .push(<RoverRogueBlessGroupData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueBlessGroupSelectRequest {
    pub BlessGroupId: i32,
    pub ActivityId: i32,
}

impl RoverRogueBlessGroupSelectRequest {
    pub const MSG_ID: u16 = 25277;
}

impl PbMessage for RoverRogueBlessGroupSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BlessGroupId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlessGroupId);
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
            5 => self.BlessGroupId = r.read_varint_i32()?,
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueBlessGroupSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueBlessGroupSelectResponse {
    pub const MSG_ID: u16 = 16971;
}

impl PbMessage for RoverRogueBlessGroupSelectResponse {
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
pub struct RoverRogueChooseData {
    pub BindId: i32,
    pub Type: Option<RoverRogueGainOperateType>,
    pub Layer: i32,
    pub IsSelect: bool,
    pub MaxTime: i32,
    pub UseTime: i32,
    pub Entries: Vec<RoverRogueGainEntry>,
    pub RefreshCost: i32,
}

impl PbMessage for RoverRogueChooseData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BindId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Layer != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
        if self.IsSelect {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.MaxTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxTime);
        }
        if self.UseTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UseTime);
        }
        for item in &self.Entries {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.RefreshCost != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            1 => self.BindId = r.read_varint_i32()?,
            2 => {
                self.Type = Some(
                    <RoverRogueGainOperateType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.Layer = r.read_varint_i32()?,
            4 => self.IsSelect = r.read_bool()?,
            5 => self.MaxTime = r.read_varint_i32()?,
            6 => self.UseTime = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Entries
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            8 => self.RefreshCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueChooseDataNotify {
    pub ChooseData: Option<RoverRogueChooseData>,
}

impl RoverRogueChooseDataNotify {
    pub const MSG_ID: u16 = 17268;
}

impl PbMessage for RoverRogueChooseDataNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ChooseData {
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
                self.ChooseData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueChooseDataResultRequest {
    pub BindId: i32,
    pub IncId: i32,
}

impl RoverRogueChooseDataResultRequest {
    pub const MSG_ID: u16 = 29940;
}

impl PbMessage for RoverRogueChooseDataResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BindId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
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
            2 => self.BindId = r.read_varint_i32()?,
            7 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueChooseDataResultResponse {
    pub IsSelect: bool,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueChooseDataResultResponse {
    pub const MSG_ID: u16 = 20137;
}

impl PbMessage for RoverRogueChooseDataResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsSelect {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
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
            2 => self.IsSelect = r.read_bool()?,
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
pub struct RoverRogueCurrencyNotify {
    pub ResurrectionRest: i32,
    pub ItemData: Vec<RoverRogueItemCurrencyData>,
}

impl RoverRogueCurrencyNotify {
    pub const MSG_ID: u16 = 23276;
}

impl PbMessage for RoverRogueCurrencyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResurrectionRest != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResurrectionRest);
        }
        for item in &self.ItemData {
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
            1 => self.ResurrectionRest = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.ItemData
                    .push(<RoverRogueItemCurrencyData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueEventNotify {
    pub EventIncId: i32,
    pub ChoiceList: Vec<i32>,
}

impl RoverRogueEventNotify {
    pub const MSG_ID: u16 = 16943;
}

impl PbMessage for RoverRogueEventNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EventIncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventIncId);
        }
        for item in &self.ChoiceList {
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
            1 => self.EventIncId = r.read_varint_i32()?,
            9 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ChoiceList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueEventSelectRequest {
    pub ChoiceId: i32,
    pub EventIncId: i32,
}

impl RoverRogueEventSelectRequest {
    pub const MSG_ID: u16 = 25790;
}

impl PbMessage for RoverRogueEventSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChoiceId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChoiceId);
        }
        if self.EventIncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EventIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.ChoiceId = r.read_varint_i32()?,
            10 => self.EventIncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueEventSelectResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueEventSelectResponse {
    pub const MSG_ID: u16 = 18651;
}

impl PbMessage for RoverRogueEventSelectResponse {
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
pub struct RoverRogueGainDataUpdateNotify {
    pub Adds: Vec<RoverRogueGainEntry>,
    pub RemoveIncIds: Vec<i32>,
    pub Type: Option<RoverRogueGainDataType>,
    pub Updates: Vec<RoverRogueGainEntry>,
}

impl RoverRogueGainDataUpdateNotify {
    pub const MSG_ID: u16 = 18551;
}

impl PbMessage for RoverRogueGainDataUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Adds {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RemoveIncIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.Updates {
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
                let bytes = r.read_len_delimited()?;
                self.Adds
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RemoveIncIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => {
                self.Type = Some(
                    <RoverRogueGainDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Updates
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGainEntry {
    pub Type: Option<RoverRogueGainDataType>,
    pub ConfigId: i32,
    pub IncId: i32,
    pub ItemRemainingRooms: i32,
    pub LootInfo: Option<RoverRogueLootInfo>,
    pub Expire: bool,
}

impl PbMessage for RoverRogueGainEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ItemRemainingRooms != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemRemainingRooms);
        }
        if let Some(v) = &self.LootInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.Expire {
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
            2 => {
                self.Type = Some(
                    <RoverRogueGainDataType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.ConfigId = r.read_varint_i32()?,
            4 => self.IncId = r.read_varint_i32()?,
            5 => self.ItemRemainingRooms = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.LootInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.Expire = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGainGetOrLoseNotify {
    pub Data: Vec<RoverRogueGetOrLoseData>,
    pub RoverRogueGainEntryIncIdLose: Vec<RoverRogueGainEntry>,
    pub RoverRogueGainEntryIncIdGet: Vec<RoverRogueGainEntry>,
}

impl RoverRogueGainGetOrLoseNotify {
    pub const MSG_ID: u16 = 26853;
}

impl PbMessage for RoverRogueGainGetOrLoseNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Data {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.RoverRogueGainEntryIncIdLose {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.RoverRogueGainEntryIncIdGet {
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
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Data
                    .push(<RoverRogueGetOrLoseData as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.RoverRogueGainEntryIncIdLose
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.RoverRogueGainEntryIncIdGet
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGetOrLoseData {
    pub EffectId: i32,
    pub RoverRogueGainEntryIncIdGet: Vec<RoverRogueGainEntry>,
    pub RoverRogueGainEntryIncIdLose: Vec<RoverRogueGainEntry>,
}

impl PbMessage for RoverRogueGetOrLoseData {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
        for item in &self.RoverRogueGainEntryIncIdGet {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.RoverRogueGainEntryIncIdLose {
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
            1 => self.EffectId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoverRogueGainEntryIncIdGet
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.RoverRogueGainEntryIncIdLose
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGiveUpGainRequest {
    pub BindId: i32,
}

impl RoverRogueGiveUpGainRequest {
    pub const MSG_ID: u16 = 28315;
}

impl PbMessage for RoverRogueGiveUpGainRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BindId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.BindId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGiveUpGainResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueGiveUpGainResponse {
    pub const MSG_ID: u16 = 19016;
}

impl PbMessage for RoverRogueGiveUpGainResponse {
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
pub struct RoverRogueGotoNextRoomCommitRequest {
    pub ActivityId: i32,
}

impl RoverRogueGotoNextRoomCommitRequest {
    pub const MSG_ID: u16 = 21542;
}

impl PbMessage for RoverRogueGotoNextRoomCommitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            8 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueGotoNextRoomCommitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueGotoNextRoomCommitResponse {
    pub const MSG_ID: u16 = 29236;
}

impl PbMessage for RoverRogueGotoNextRoomCommitResponse {
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
pub struct RoverRogueInfoNotify {
    pub ActivityId: i32,
    pub Bless: Vec<RoverRogueGainEntry>,
    pub RoverRogueRoadType: i32,
    pub LootDict: Vec<RoverRogueLootEntry>,
    pub RoverMaxHp: i32,
    pub TalentPoint: i32,
    pub RoverType: i32,
    pub RoverEnergy: i32,
    pub ResurrectionRest: i32,
    pub Gold: i32,
    pub RoverHp: i32,
    pub ResurrectionMax: i32,
    pub RoleEnhance: Vec<RoverRogueGainEntry>,
    pub Item: Vec<RoverRogueGainEntry>,
    pub RoverRogueCoinId: i32,
    pub InstState: Option<RoverRogueInstState>,
    pub ItemData: Vec<RoverRogueItemCurrencyData>,
    pub LootItem: Vec<RoverRogueGainEntry>,
    pub InstId: i32,
}

impl RoverRogueInfoNotify {
    pub const MSG_ID: u16 = 16398;
}

impl PbMessage for RoverRogueInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.Bless {
            crate::wire::write_submessage(out, 10, item);
        }
        if self.RoverRogueRoadType != 0 {
            crate::wire::write_tag(out, 29, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverRogueRoadType);
        }
        for item in &self.LootDict {
            crate::wire::write_submessage(out, 51, item);
        }
        if self.RoverMaxHp != 0 {
            crate::wire::write_tag(out, 56, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverMaxHp);
        }
        if self.TalentPoint != 0 {
            crate::wire::write_tag(out, 99, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentPoint);
        }
        if self.RoverType != 0 {
            crate::wire::write_tag(out, 106, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverType);
        }
        if self.RoverEnergy != 0 {
            crate::wire::write_tag(out, 109, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverEnergy);
        }
        if self.ResurrectionRest != 0 {
            crate::wire::write_tag(out, 122, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResurrectionRest);
        }
        if self.Gold != 0 {
            crate::wire::write_tag(out, 136, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Gold);
        }
        if self.RoverHp != 0 {
            crate::wire::write_tag(out, 140, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverHp);
        }
        if self.ResurrectionMax != 0 {
            crate::wire::write_tag(out, 160, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResurrectionMax);
        }
        for item in &self.RoleEnhance {
            crate::wire::write_submessage(out, 163, item);
        }
        for item in &self.Item {
            crate::wire::write_submessage(out, 183, item);
        }
        if self.RoverRogueCoinId != 0 {
            crate::wire::write_tag(out, 185, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverRogueCoinId);
        }
        if let Some(v) = &self.InstState {
            crate::wire::write_tag(out, 189, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ItemData {
            crate::wire::write_submessage(out, 213, item);
        }
        for item in &self.LootItem {
            crate::wire::write_submessage(out, 229, item);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 233, crate::wire::WireType::Varint);
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Bless
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            29 => self.RoverRogueRoadType = r.read_varint_i32()?,
            51 => {
                let bytes = r.read_len_delimited()?;
                self.LootDict
                    .push(<RoverRogueLootEntry as PbMessage>::decode(bytes)?);
            }
            56 => self.RoverMaxHp = r.read_varint_i32()?,
            99 => self.TalentPoint = r.read_varint_i32()?,
            106 => self.RoverType = r.read_varint_i32()?,
            109 => self.RoverEnergy = r.read_varint_i32()?,
            122 => self.ResurrectionRest = r.read_varint_i32()?,
            136 => self.Gold = r.read_varint_i32()?,
            140 => self.RoverHp = r.read_varint_i32()?,
            160 => self.ResurrectionMax = r.read_varint_i32()?,
            163 => {
                let bytes = r.read_len_delimited()?;
                self.RoleEnhance
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            183 => {
                let bytes = r.read_len_delimited()?;
                self.Item
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            185 => self.RoverRogueCoinId = r.read_varint_i32()?,
            189 => {
                self.InstState = Some(
                    <RoverRogueInstState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            213 => {
                let bytes = r.read_len_delimited()?;
                self.ItemData
                    .push(<RoverRogueItemCurrencyData as PbMessage>::decode(bytes)?);
            }
            229 => {
                let bytes = r.read_len_delimited()?;
                self.LootItem
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            233 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueInsCtx {
    pub RoverId: i32,
    pub ContinueLastProgress: bool,
    pub EquippedLootIds: Vec<i32>,
    pub RoverRoleType: i32,
}

impl PbMessage for RoverRogueInsCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoverId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverId);
        }
        if self.ContinueLastProgress {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.EquippedLootIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RoverRoleType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverRoleType);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.RoverId = r.read_varint_i32()?,
            2 => self.ContinueLastProgress = r.read_bool()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.EquippedLootIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => self.RoverRoleType = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueInsListRequest {
    pub ActivityId: i32,
}

impl RoverRogueInsListRequest {
    pub const MSG_ID: u16 = 29791;
}

impl PbMessage for RoverRogueInsListRequest {
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
pub struct RoverRogueInsListResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub List: Vec<RoverRogueInsEntry>,
}

impl RoverRogueInsListResponse {
    pub const MSG_ID: u16 = 27005;
}

impl PbMessage for RoverRogueInsListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.List {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.List
                    .push(<RoverRogueInsEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueItemCurrencyData {
    pub ItemId: i32,
    pub Count: i32,
}

impl PbMessage for RoverRogueItemCurrencyData {
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
pub struct RoverRogueLootChangeNotify {
    pub EnableUse: bool,
    pub LootData: Vec<RoverRogueGainEntry>,
}

impl RoverRogueLootChangeNotify {
    pub const MSG_ID: u16 = 22228;
}

impl PbMessage for RoverRogueLootChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EnableUse {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.LootData {
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
            2 => self.EnableUse = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.LootData
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueLootChangeRequest {
    pub ActivityId: i32,
    pub LootId: i32,
    pub IncId: i32,
}

impl RoverRogueLootChangeRequest {
    pub const MSG_ID: u16 = 22152;
}

impl PbMessage for RoverRogueLootChangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.LootId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LootId);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            13 => self.LootId = r.read_varint_i32()?,
            15 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueLootChangeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueLootChangeResponse {
    pub const MSG_ID: u16 = 28188;
}

impl PbMessage for RoverRogueLootChangeResponse {
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
pub struct RoverRogueLootEntry {
    pub LootId: i32,
    pub Level: i32,
    pub RoomPassedCount: i32,
}

impl PbMessage for RoverRogueLootEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LootId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LootId);
        }
        if self.Level != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Level);
        }
        if self.RoomPassedCount != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomPassedCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.LootId = r.read_varint_i32()?,
            2 => self.Level = r.read_varint_i32()?,
            3 => self.RoomPassedCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueLootInfo {
    pub LootRoomPassedCounter: i32,
    pub LootLv: i32,
    pub Unlock: bool,
    pub Progress: i32,
    pub Target: i32,
}

impl PbMessage for RoverRogueLootInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LootRoomPassedCounter != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LootRoomPassedCounter);
        }
        if self.LootLv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LootLv);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Progress != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Progress);
        }
        if self.Target != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.LootRoomPassedCounter = r.read_varint_i32()?,
            2 => self.LootLv = r.read_varint_i32()?,
            3 => self.Unlock = r.read_bool()?,
            4 => self.Progress = r.read_varint_i32()?,
            5 => self.Target = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueLootInfoRequest {
    pub ActivityId: i32,
}

impl RoverRogueLootInfoRequest {
    pub const MSG_ID: u16 = 19483;
}

impl PbMessage for RoverRogueLootInfoRequest {
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
pub struct RoverRogueLootInfoResponse {
    pub Loots: Vec<RoverRogueGainEntry>,
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueLootInfoResponse {
    pub const MSG_ID: u16 = 17028;
}

impl PbMessage for RoverRogueLootInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Loots {
            crate::wire::write_submessage(out, 2, item);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Loots
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
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
pub struct RoverRogueNextRoomEntry {
    pub RoomTypeId: i32,
    pub RoomId: i32,
    pub EntityIncId: i64,
}

impl PbMessage for RoverRogueNextRoomEntry {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoomTypeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomTypeId);
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
        if self.EntityIncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.RoomTypeId = r.read_varint_i32()?,
            2 => self.RoomId = r.read_varint_i32()?,
            3 => self.EntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueQuitRequest {
    pub ActivityId: i32,
}

impl RoverRogueQuitRequest {
    pub const MSG_ID: u16 = 27391;
}

impl PbMessage for RoverRogueQuitRequest {
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
pub struct RoverRogueQuitResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueQuitResponse {
    pub const MSG_ID: u16 = 16168;
}

impl PbMessage for RoverRogueQuitResponse {
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
pub struct RoverRogueRefreshGainRequest {
    pub BindId: i32,
}

impl RoverRogueRefreshGainRequest {
    pub const MSG_ID: u16 = 29906;
}

impl PbMessage for RoverRogueRefreshGainRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BindId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BindId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.BindId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueRefreshGainResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ChooseData: Option<RoverRogueChooseData>,
}

impl RoverRogueRefreshGainResponse {
    pub const MSG_ID: u16 = 21110;
}

impl PbMessage for RoverRogueRefreshGainResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ChooseData {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.ChooseData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueResourcesItem {
    pub ConfId: i32,
    pub RoundNum: i32,
}

impl PbMessage for RoverRogueResourcesItem {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfId);
        }
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ConfId = r.read_varint_i32()?,
            2 => self.RoundNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueResourcesUpdateNotify {
    pub Coin: i32,
    pub EffectId: i32,
    pub HpMax: i32,
    pub AttChange: Vec<RoverRogueAttChange>,
    pub GoldEfficiency: i32,
    pub Item: Vec<RoverRogueResourcesItem>,
    pub TalentPoint: i32,
}

impl RoverRogueResourcesUpdateNotify {
    pub const MSG_ID: u16 = 27960;
}

impl PbMessage for RoverRogueResourcesUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Coin != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Coin);
        }
        if self.EffectId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectId);
        }
        if self.HpMax != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HpMax);
        }
        for item in &self.AttChange {
            crate::wire::write_submessage(out, 8, item);
        }
        if self.GoldEfficiency != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoldEfficiency);
        }
        for item in &self.Item {
            crate::wire::write_submessage(out, 13, item);
        }
        if self.TalentPoint != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentPoint);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Coin = r.read_varint_i32()?,
            5 => self.EffectId = r.read_varint_i32()?,
            7 => self.HpMax = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.AttChange
                    .push(<RoverRogueAttChange as PbMessage>::decode(bytes)?);
            }
            9 => self.GoldEfficiency = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Item
                    .push(<RoverRogueResourcesItem as PbMessage>::decode(bytes)?);
            }
            15 => self.TalentPoint = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueResultInfo {
    pub InstId: i32,
    pub Grade: Option<RoverRogueGrade>,
    pub TotalCostTime: i64,
    pub CurLayer: i32,
    pub MaxLayer: i32,
    pub RoverId: i32,
    pub IsDeadTrigger: bool,
    pub GainCount: i32,
    pub RoomPassedCount: i32,
    pub Rewards: Vec<RoverRogueSettleReward>,
    pub BossRoom: i32,
    pub EliteRoom: i32,
    pub TotalGold: i32,
    pub GainBlessConfId: Vec<i32>,
    pub GainRoleConfId: Vec<i32>,
    pub GainItemConfId: Vec<i32>,
    pub GainLootConfId: Vec<i32>,
    pub RoleType: i32,
    pub GainLootInfo: Vec<RoverRogueGainEntry>,
    pub SettleType: Option<RoverRogueInstState>,
}

impl PbMessage for RoverRogueResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        if let Some(v) = &self.Grade {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.TotalCostTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TotalCostTime);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.RoverId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoverId);
        }
        if self.IsDeadTrigger {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.GainCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GainCount);
        }
        if self.RoomPassedCount != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomPassedCount);
        }
        for item in &self.Rewards {
            crate::wire::write_submessage(out, 11, item);
        }
        if self.BossRoom != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossRoom);
        }
        if self.EliteRoom != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EliteRoom);
        }
        if self.TotalGold != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalGold);
        }
        for item in &self.GainBlessConfId {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.GainRoleConfId {
            crate::wire::write_tag(out, 16, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.GainItemConfId {
            crate::wire::write_tag(out, 17, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.GainLootConfId {
            crate::wire::write_tag(out, 18, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RoleType != 0 {
            crate::wire::write_tag(out, 19, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleType);
        }
        for item in &self.GainLootInfo {
            crate::wire::write_submessage(out, 20, item);
        }
        if let Some(v) = &self.SettleType {
            crate::wire::write_tag(out, 21, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            3 => {
                self.Grade = Some(
                    <RoverRogueGrade as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            4 => self.TotalCostTime = r.read_varint_i64()?,
            5 => self.CurLayer = r.read_varint_i32()?,
            6 => self.MaxLayer = r.read_varint_i32()?,
            7 => self.RoverId = r.read_varint_i32()?,
            8 => self.IsDeadTrigger = r.read_bool()?,
            9 => self.GainCount = r.read_varint_i32()?,
            10 => self.RoomPassedCount = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.Rewards
                    .push(<RoverRogueSettleReward as PbMessage>::decode(bytes)?);
            }
            12 => self.BossRoom = r.read_varint_i32()?,
            13 => self.EliteRoom = r.read_varint_i32()?,
            14 => self.TotalGold = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.GainBlessConfId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            16 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GainRoleConfId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            17 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GainItemConfId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            18 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.GainLootConfId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            19 => self.RoleType = r.read_varint_i32()?,
            20 => {
                let bytes = r.read_len_delimited()?;
                self.GainLootInfo
                    .push(<RoverRogueGainEntry as PbMessage>::decode(bytes)?);
            }
            21 => {
                self.SettleType = Some(
                    <RoverRogueInstState as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct RoverRogueResultNotify {
    pub ResultInfo: Option<RoverRogueResultInfo>,
}

impl RoverRogueResultNotify {
    pub const MSG_ID: u16 = 19486;
}

impl PbMessage for RoverRogueResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ResultInfo {
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
                self.ResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueResultRequest {
    pub InstId: i32,
    pub ActivityId: i32,
}

impl RoverRogueResultRequest {
    pub const MSG_ID: u16 = 28320;
}

impl PbMessage for RoverRogueResultRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
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
            2 => self.InstId = r.read_varint_i32()?,
            3 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueResultResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ResultInfo: Option<RoverRogueResultInfo>,
}

impl RoverRogueResultResponse {
    pub const MSG_ID: u16 = 20710;
}

impl PbMessage for RoverRogueResultResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.ResultInfo {
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
                self.ResultInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueRoomInfoNotify {
    pub ShopEntityIncId: i32,
    pub RoomId: i32,
    pub NextRoomEntries: Vec<RoverRogueNextRoomEntry>,
    pub MaxLayer: i32,
    pub RoomTypeId: i32,
    pub CurLayer: i32,
}

impl RoverRogueRoomInfoNotify {
    pub const MSG_ID: u16 = 25772;
}

impl PbMessage for RoverRogueRoomInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopEntityIncId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopEntityIncId);
        }
        if self.RoomId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomId);
        }
        for item in &self.NextRoomEntries {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.MaxLayer != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxLayer);
        }
        if self.RoomTypeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoomTypeId);
        }
        if self.CurLayer != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurLayer);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.ShopEntityIncId = r.read_varint_i32()?,
            6 => self.RoomId = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.NextRoomEntries
                    .push(<RoverRogueNextRoomEntry as PbMessage>::decode(bytes)?);
            }
            8 => self.MaxLayer = r.read_varint_i32()?,
            13 => self.RoomTypeId = r.read_varint_i32()?,
            15 => self.CurLayer = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueSettleReward {
    pub ItemId: i32,
    pub Num: i32,
}

impl PbMessage for RoverRogueSettleReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.Num = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopBuyRequest {
    pub GoodIncId: i32,
    pub ActivityId: i32,
}

impl RoverRogueShopBuyRequest {
    pub const MSG_ID: u16 = 15194;
}

impl PbMessage for RoverRogueShopBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GoodIncId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoodIncId);
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
            2 => self.GoodIncId = r.read_varint_i32()?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopBuyResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueShopBuyResponse {
    pub const MSG_ID: u16 = 29518;
}

impl PbMessage for RoverRogueShopBuyResponse {
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
pub struct RoverRogueShopGood {
    pub IncId: i32,
    pub ShopItemId: i32,
    pub RuntimeParam: i32,
    pub FinalPrice: i32,
    pub Price: i32,
    pub IsBought: bool,
    pub ShowItemId: i32,
}

impl PbMessage for RoverRogueShopGood {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.ShopItemId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopItemId);
        }
        if self.RuntimeParam != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RuntimeParam);
        }
        if self.FinalPrice != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinalPrice);
        }
        if self.Price != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Price);
        }
        if self.IsBought {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.ShowItemId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShowItemId);
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
            2 => self.ShopItemId = r.read_varint_i32()?,
            3 => self.RuntimeParam = r.read_varint_i32()?,
            4 => self.FinalPrice = r.read_varint_i32()?,
            5 => self.Price = r.read_varint_i32()?,
            6 => self.IsBought = r.read_bool()?,
            7 => self.ShowItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopInfoUpdateNotify {
    pub Snapshot: Option<RoverRogueShopSnapshot>,
}

impl RoverRogueShopInfoUpdateNotify {
    pub const MSG_ID: u16 = 23170;
}

impl PbMessage for RoverRogueShopInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Snapshot {
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
                self.Snapshot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopRefreshRequest {
    pub ActivityId: i32,
}

impl RoverRogueShopRefreshRequest {
    pub const MSG_ID: u16 = 21321;
}

impl PbMessage for RoverRogueShopRefreshRequest {
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
pub struct RoverRogueShopRefreshResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Snapshot: Option<RoverRogueShopSnapshot>,
}

impl RoverRogueShopRefreshResponse {
    pub const MSG_ID: u16 = 21963;
}

impl PbMessage for RoverRogueShopRefreshResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Snapshot {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.Snapshot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopSnapshot {
    pub ShopConfigId: i32,
    pub Goods: Vec<RoverRogueShopGood>,
    pub BoughtIncIds: Vec<i32>,
    pub RefreshCount: i32,
    pub NextRefreshCost: i32,
    pub MaxRefreshCount: i32,
}

impl PbMessage for RoverRogueShopSnapshot {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopConfigId);
        }
        for item in &self.Goods {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.BoughtIncIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.RefreshCount != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RefreshCount);
        }
        if self.NextRefreshCost != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NextRefreshCost);
        }
        if self.MaxRefreshCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxRefreshCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.ShopConfigId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.Goods
                    .push(<RoverRogueShopGood as PbMessage>::decode(bytes)?);
            }
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BoughtIncIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.RefreshCount = r.read_varint_i32()?,
            5 => self.NextRefreshCost = r.read_varint_i32()?,
            6 => self.MaxRefreshCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueShopUpdateNotify {
    pub Snapshot: Option<RoverRogueShopSnapshot>,
}

impl RoverRogueShopUpdateNotify {
    pub const MSG_ID: u16 = 18904;
}

impl PbMessage for RoverRogueShopUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Snapshot {
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
                self.Snapshot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueSubLevelNotify {
    pub X: f32,
    pub Y: f32,
    pub Layer: i32,
    pub A: f32,
    pub Z: f32,
    pub UnLoadSubLevels: Vec<Vec<u8>>,
    pub LoadSubLevels: Vec<Vec<u8>>,
}

impl RoverRogueSubLevelNotify {
    pub const MSG_ID: u16 = 15783;
}

impl PbMessage for RoverRogueSubLevelNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.X != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.X).to_bits().to_le_bytes());
        }
        if self.Y != 0.0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Y).to_bits().to_le_bytes());
        }
        if self.Layer != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Layer);
        }
        if self.A != 0.0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.A).to_bits().to_le_bytes());
        }
        if self.Z != 0.0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Z).to_bits().to_le_bytes());
        }
        for item in &self.UnLoadSubLevels {
            crate::wire::write_tag(out, 11, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
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
            3 => self.Y = r.read_f32()?,
            4 => self.Layer = r.read_varint_i32()?,
            5 => self.A = r.read_f32()?,
            7 => self.Z = r.read_f32()?,
            11 => self.UnLoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            14 => self.LoadSubLevels.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTalentInfoRequest {
    pub ActivityId: i32,
}

impl RoverRogueTalentInfoRequest {
    pub const MSG_ID: u16 = 18812;
}

impl PbMessage for RoverRogueTalentInfoRequest {
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
pub struct RoverRogueTalentInfoResponse {
    pub Point: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub TalentId: Vec<i32>,
}

impl RoverRogueTalentInfoResponse {
    pub const MSG_ID: u16 = 15617;
}

impl PbMessage for RoverRogueTalentInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Point != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Point);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.TalentId {
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
            6 => self.Point = r.read_varint_i32()?,
            12 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TalentId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTalentUnlockNotify {
    pub TalentIds: Vec<i32>,
}

impl RoverRogueTalentUnlockNotify {
    pub const MSG_ID: u16 = 24427;
}

impl PbMessage for RoverRogueTalentUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TalentIds {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TalentIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTalentUnlockRequest {
    pub TalentId: i32,
    pub ActivityId: i32,
}

impl RoverRogueTalentUnlockRequest {
    pub const MSG_ID: u16 = 16548;
}

impl PbMessage for RoverRogueTalentUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TalentId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TalentId);
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
            8 => self.TalentId = r.read_varint_i32()?,
            11 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTalentUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl RoverRogueTalentUnlockResponse {
    pub const MSG_ID: u16 = 15025;
}

impl PbMessage for RoverRogueTalentUnlockResponse {
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
pub struct RoverRogueTaskRewardTakeRequest {
    pub ActivityId: i32,
    pub RewardIds: Vec<i32>,
}

impl RoverRogueTaskRewardTakeRequest {
    pub const MSG_ID: u16 = 29942;
}

impl PbMessage for RoverRogueTaskRewardTakeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.RewardIds {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RewardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTaskRewardTakeResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub SuccessRewardIds: Vec<i32>,
}

impl RoverRogueTaskRewardTakeResponse {
    pub const MSG_ID: u16 = 15438;
}

impl PbMessage for RoverRogueTaskRewardTakeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.SuccessRewardIds {
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
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SuccessRewardIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueTaskUpdateNotify {
    pub Tasks: Vec<ConditionTask>,
}

impl RoverRogueTaskUpdateNotify {
    pub const MSG_ID: u16 = 23286;
}

impl PbMessage for RoverRogueTaskUpdateNotify {
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
                    .push(<ConditionTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RoverRogueUpdateReviveTimesNotify {
    pub ReviveTimes: i32,
    pub ReviveTimesMax: i32,
}

impl RoverRogueUpdateReviveTimesNotify {
    pub const MSG_ID: u16 = 15569;
}

impl PbMessage for RoverRogueUpdateReviveTimesNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveTimes != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveTimes);
        }
        if self.ReviveTimesMax != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveTimesMax);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            12 => self.ReviveTimes = r.read_varint_i32()?,
            14 => self.ReviveTimesMax = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunTestRequest {
    pub Type: Vec<u8>,
    pub Params: Vec<u8>,
}

impl RunTestRequest {
    pub const MSG_ID: u16 = 29128;
}

impl PbMessage for RunTestRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Type.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Type);
        }
        if !self.Params.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
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
            2 => self.Type = r.read_len_delimited()?.to_vec(),
            8 => self.Params = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RunTestResponse {
    pub Code: Option<ErrorCode>,
}

impl RunTestResponse {
    pub const MSG_ID: u16 = 22000;
}

impl PbMessage for RunTestResponse {
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
pub struct RunTurntableActivityRequest {
    pub ActivityId: i32,
}

impl RunTurntableActivityRequest {
    pub const MSG_ID: u16 = 15007;
}

impl PbMessage for RunTurntableActivityRequest {
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
pub struct RunTurntableActivityResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IsFinishCurRound: bool,
    pub RewardId: i32,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl RunTurntableActivityResponse {
    pub const MSG_ID: u16 = 15518;
}

impl PbMessage for RunTurntableActivityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsFinishCurRound {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.RewardId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RewardId);
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 14, &self.ItemMap);
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
            9 => self.IsFinishCurRound = r.read_bool()?,
            13 => self.RewardId = r.read_varint_i32()?,
            14 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitHumanChangleNotify {
    pub DownloadBlockId: Vec<i32>,
    pub DeleteBlockId: Vec<i32>,
    pub PlayerId: i32,
}

impl SceneBlockSplitHumanChangleNotify {
    pub const MSG_ID: u16 = 28372;
}

impl PbMessage for SceneBlockSplitHumanChangleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DownloadBlockId {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.DeleteBlockId {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DownloadBlockId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DeleteBlockId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            8 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitHumanFullNotify {
    pub DownloadBlockId: Vec<i32>,
    pub PlayerId: i32,
}

impl SceneBlockSplitHumanFullNotify {
    pub const MSG_ID: u16 = 18425;
}

impl PbMessage for SceneBlockSplitHumanFullNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DownloadBlockId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.DownloadBlockId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitHumanSwitchStateNotify {
    pub SwitchState: Option<ESceneBlockSwithState>,
    pub PlayerId: i32,
}

impl SceneBlockSplitHumanSwitchStateNotify {
    pub const MSG_ID: u16 = 18158;
}

impl PbMessage for SceneBlockSplitHumanSwitchStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SwitchState {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            8 => {
                self.SwitchState = Some(
                    <ESceneBlockSwithState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneBlockSplitPopWindowNotify {
    pub BlockId: i32,
    pub WindowType: Option<ESceneBlockSplitPopWindowType>,
    pub Param1: i32,
}

impl SceneBlockSplitPopWindowNotify {
    pub const MSG_ID: u16 = 24204;
}

impl PbMessage for SceneBlockSplitPopWindowNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BlockId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BlockId);
        }
        if let Some(v) = &self.WindowType {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Param1 != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Param1);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.BlockId = r.read_varint_i32()?,
            8 => {
                self.WindowType = Some(
                    <ESceneBlockSplitPopWindowType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            14 => self.Param1 = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneDateNotify {
    pub CurrDate: u32,
}

impl SceneDateNotify {
    pub const MSG_ID: u16 = 25540;
}

impl PbMessage for SceneDateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrDate != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            3 => self.CurrDate = r.read_varint_u32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneFishPointInfoUpdateNotify {
    pub FishPoint: Option<OneFishPointInfo>,
    pub IsRemove: bool,
    pub SceneConfigId: i32,
}

impl SceneFishPointInfoUpdateNotify {
    pub const MSG_ID: u16 = 17940;
}

impl PbMessage for SceneFishPointInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FishPoint {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.IsRemove {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.SceneConfigId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SceneConfigId);
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
                self.FishPoint
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => self.IsRemove = r.read_bool()?,
            11 => self.SceneConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayFirstNotify {
    pub GamePlayId: i32,
}

impl SceneGamePlayFirstNotify {
    pub const MSG_ID: u16 = 21553;
}

impl PbMessage for SceneGamePlayFirstNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            10 => self.GamePlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayInfo {
    pub GamePlayId: i32,
    pub IsFirst: bool,
    pub Status: i32,
    pub OpenTime: u32,
}

impl PbMessage for SceneGamePlayInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayId);
        }
        if self.IsFirst {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Status != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.OpenTime));
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.GamePlayId = r.read_varint_i32()?,
            2 => self.IsFirst = r.read_bool()?,
            3 => self.Status = r.read_varint_i32()?,
            4 => self.OpenTime = r.read_varint_u32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayInfoNotify {
    pub GamePlayInfo: Vec<SceneGamePlayInfo>,
}

impl SceneGamePlayInfoNotify {
    pub const MSG_ID: u16 = 21532;
}

impl PbMessage for SceneGamePlayInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.GamePlayInfo {
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
                self.GamePlayInfo
                    .push(<SceneGamePlayInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayOpenTimeNotify {
    pub OpenTime: u32,
    pub GamePlayId: i32,
}

impl SceneGamePlayOpenTimeNotify {
    pub const MSG_ID: u16 = 17378;
}

impl PbMessage for SceneGamePlayOpenTimeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_u64(out, u64::from(self.OpenTime));
        }
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            2 => self.OpenTime = r.read_varint_u32()?,
            9 => self.GamePlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayScopeNotify {
    pub GamePlayId: i32,
    pub IsEnter: bool,
}

impl SceneGamePlayScopeNotify {
    pub const MSG_ID: u16 = 18816;
}

impl PbMessage for SceneGamePlayScopeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamePlayId);
        }
        if self.IsEnter {
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
            10 => self.GamePlayId = r.read_varint_i32()?,
            12 => self.IsEnter = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGamePlayStatusNotify {
    pub Status: i32,
    pub GamePlayId: i32,
}

impl SceneGamePlayStatusNotify {
    pub const MSG_ID: u16 = 24317;
}

impl PbMessage for SceneGamePlayStatusNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Status != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Status);
        }
        if self.GamePlayId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
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
            5 => self.Status = r.read_varint_i32()?,
            6 => self.GamePlayId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneGroupDataAndCurFormationEntityRequest {}

impl SceneGroupDataAndCurFormationEntityRequest {
    pub const MSG_ID: u16 = 26969;
}

impl PbMessage for SceneGroupDataAndCurFormationEntityRequest {
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
pub struct SceneGroupDataAndCurFormationEntityResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PlayerSceneGroupDataList: Vec<PlayerSceneGroupDataList>,
    pub CurFormationRoleEntitys: Vec<EntityPb>,
}

impl SceneGroupDataAndCurFormationEntityResponse {
    pub const MSG_ID: u16 = 28552;
}

impl PbMessage for SceneGroupDataAndCurFormationEntityResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PlayerSceneGroupDataList {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.CurFormationRoleEntitys {
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerSceneGroupDataList
                    .push(<PlayerSceneGroupDataList as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.CurFormationRoleEntitys
                    .push(<EntityPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemAiComponentPb {
    pub Enable: bool,
    pub AiInfo: Option<SceneItemAiInfoPb>,
}

impl PbMessage for SceneItemAiComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Enable {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.AiInfo {
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
            1 => self.Enable = r.read_bool()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.AiInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemAiDisableNotify {
    pub EntityId: i64,
}

impl SceneItemAiDisableNotify {
    pub const MSG_ID: u16 = 21693;
}

impl PbMessage for SceneItemAiDisableNotify {
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
pub struct SceneItemAiEnableNotify {
    pub AiInfo: Option<SceneItemAiInfoPb>,
    pub EntityId: i64,
}

impl SceneItemAiEnableNotify {
    pub const MSG_ID: u16 = 26148;
}

impl PbMessage for SceneItemAiEnableNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AiInfo {
            crate::wire::write_submessage(out, 1, v);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.AiInfo
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
pub struct SceneItemAiInfoPb {
    pub SceneItemPatrolInfoPb: Option<SceneItemPatrolInfoPb>,
}

impl PbMessage for SceneItemAiInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SceneItemPatrolInfoPb {
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
                self.SceneItemPatrolInfoPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemAiPatrolPointNotify {
    pub PatrolPointIndex: i32,
    pub EntityId: i64,
}

impl SceneItemAiPatrolPointNotify {
    pub const MSG_ID: u16 = 28782;
}

impl PbMessage for SceneItemAiPatrolPointNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PatrolPointIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PatrolPointIndex);
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
            5 => self.PatrolPointIndex = r.read_varint_i32()?,
            8 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemAttributeChangeNotify {
    pub EntityId: i64,
    pub IsAdd: bool,
    pub AttributeId: i32,
}

impl SceneItemAttributeChangeNotify {
    pub const MSG_ID: u16 = 17296;
}

impl PbMessage for SceneItemAttributeChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        if self.IsAdd {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.AttributeId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttributeId);
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
            6 => self.IsAdd = r.read_bool()?,
            9 => self.AttributeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemAttributeComponentPb {
    pub AttributeIds: Vec<i32>,
}

impl PbMessage for SceneItemAttributeComponentPb {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AttributeIds {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AttributeIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemBlackboardNotify {
    pub EntityId: i64,
    pub Params: Vec<SceneItemBlackboardParam>,
    pub FullUpdate: bool,
}

impl SceneItemBlackboardNotify {
    pub const MSG_ID: u16 = 20724;
}

impl PbMessage for SceneItemBlackboardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EntityId);
        }
        for item in &self.Params {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.FullUpdate {
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
            6 => self.EntityId = r.read_varint_i64()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.Params
                    .push(<SceneItemBlackboardParam as PbMessage>::decode(bytes)?);
            }
            14 => self.FullUpdate = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneItemLifeCycleComponentPb {
    pub ContextId: i64,
}

impl PbMessage for SceneItemLifeCycleComponentPb {
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
