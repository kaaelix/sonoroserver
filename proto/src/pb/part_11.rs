use crate::enums::*;
use crate::pb::*;
use crate::wire::{DecodeError, PbMessage, Reader, WireType};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopConditionFinishNotify {
    pub Items: Vec<PayShopItem>,
}

impl PayShopConditionFinishNotify {
    pub const MSG_ID: u16 = 17751;
}

impl PbMessage for PayShopConditionFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
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
                self.Items.push(<PayShopItem as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopDirectBuyRequest {
    pub Version: Vec<u8>,
    pub Id: i32,
}

impl PayShopDirectBuyRequest {
    pub const MSG_ID: u16 = 28237;
}

impl PbMessage for PayShopDirectBuyRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Version.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Version);
        }
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
            6 => self.Version = r.read_len_delimited()?.to_vec(),
            12 => self.Id = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopDirectBuyResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ReceiptId: Vec<u8>,
    pub CallbackUrl: Vec<u8>,
}

impl PayShopDirectBuyResponse {
    pub const MSG_ID: u16 = 29388;
}

impl PbMessage for PayShopDirectBuyResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ReceiptId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.ReceiptId);
        }
        if !self.CallbackUrl.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.CallbackUrl);
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
            8 => self.ReceiptId = r.read_len_delimited()?.to_vec(),
            9 => self.CallbackUrl = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopRecommendConfigInfo {
    pub Id: i32,
    pub RecommendType: i32,
    pub RecommendId: i32,
    pub TabName: Vec<u8>,
    pub PrefabPath: Vec<u8>,
    pub Sort: i32,
    pub Show: bool,
    pub TabImage: Vec<u8>,
}

impl PbMessage for PayShopRecommendConfigInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.RecommendType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommendType);
        }
        if self.RecommendId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RecommendId);
        }
        if !self.TabName.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TabName);
        }
        if !self.PrefabPath.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PrefabPath);
        }
        if self.Sort != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sort);
        }
        if self.Show {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if !self.TabImage.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TabImage);
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
            2 => self.RecommendType = r.read_varint_i32()?,
            3 => self.RecommendId = r.read_varint_i32()?,
            4 => self.TabName = r.read_len_delimited()?.to_vec(),
            5 => self.PrefabPath = r.read_len_delimited()?.to_vec(),
            6 => self.Sort = r.read_varint_i32()?,
            7 => self.Show = r.read_bool()?,
            8 => self.TabImage = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopTabConfigInfo {
    pub ShopId: i32,
    pub TabId: i32,
    pub Sort: i32,
    pub Name: Vec<u8>,
    pub Logic: i32,
    pub Enable: bool,
    pub BeginTime: i64,
    pub EndTime: i64,
    pub TabSelectSpritePath: Vec<u8>,
    pub TabContentPath: Vec<u8>,
    pub Money: Vec<i32>,
}

impl PbMessage for PayShopTabConfigInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ShopId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ShopId);
        }
        if self.TabId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TabId);
        }
        if self.Sort != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Sort);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.Logic != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Logic);
        }
        if self.Enable {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.BeginTime != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.BeginTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        if !self.TabSelectSpritePath.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TabSelectSpritePath);
        }
        if !self.TabContentPath.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.TabContentPath);
        }
        for item in &self.Money {
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
            1 => self.ShopId = r.read_varint_i32()?,
            2 => self.TabId = r.read_varint_i32()?,
            3 => self.Sort = r.read_varint_i32()?,
            4 => self.Name = r.read_len_delimited()?.to_vec(),
            5 => self.Logic = r.read_varint_i32()?,
            6 => self.Enable = r.read_bool()?,
            7 => self.BeginTime = r.read_varint_i64()?,
            8 => self.EndTime = r.read_varint_i64()?,
            9 => self.TabSelectSpritePath = r.read_len_delimited()?.to_vec(),
            10 => self.TabContentPath = r.read_len_delimited()?.to_vec(),
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Money, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PayShopUnlockNotify {
    pub UnlockList: Vec<i32>,
}

impl PayShopUnlockNotify {
    pub const MSG_ID: u16 = 16644;
}

impl PbMessage for PayShopUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockList {
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
            6 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.UnlockList, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbBattlePassTask {
    pub Id: i32,
    pub Current: i32,
    pub Target: i32,
    pub IsFinished: bool,
    pub IsTaken: bool,
}

impl PbMessage for PbBattlePassTask {
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbFormationAutoAddRoleNotify {
    pub Roles: Vec<i32>,
    pub FormationId: i32,
}

impl PbFormationAutoAddRoleNotify {
    pub const MSG_ID: u16 = 21027;
}

impl PbMessage for PbFormationAutoAddRoleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Roles {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Roles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.FormationId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbGetRoleListRequest {}

impl PbGetRoleListRequest {
    pub const MSG_ID: u16 = 28529;
}

impl PbMessage for PbGetRoleListRequest {
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
pub struct PbGetRoleListResponse {
    pub RoleList: Vec<roleInfo>,
    pub Code: Option<ErrorCode>,
}

impl PbGetRoleListResponse {
    pub const MSG_ID: u16 = 28176;
}

impl PbMessage for PbGetRoleListResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RoleList {
            crate::wire::write_submessage(out, 7, item);
        }
        if let Some(v) = &self.Code {
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
                self.RoleList.push(<roleInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
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
pub struct PbHandInItem {
    pub HandItemInfo: Vec<PbHandInItemInfo>,
    pub Count: i32,
    pub PbHandInItemType: Option<PbHandInItemType>,
}

impl PbMessage for PbHandInItem {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.HandItemInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if let Some(v) = &self.PbHandInItemType {
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
                self.HandItemInfo
                    .push(<PbHandInItemInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.Count = r.read_varint_i32()?,
            3 => {
                self.PbHandInItemType = Some(
                    <PbHandInItemType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PbHandInItemInfo {
    pub IncId: i32,
    pub Num: i32,
    pub ItemId: i32,
}

impl PbMessage for PbHandInItemInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IncId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.IncId);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.IncId = r.read_varint_i32()?,
            2 => self.Num = r.read_varint_i32()?,
            3 => self.ItemId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbRoleActiveNotify {
    pub Role: Option<roleInfo>,
}

impl PbRoleActiveNotify {
    pub const MSG_ID: u16 = 16014;
}

impl PbMessage for PbRoleActiveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Role {
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
                self.Role
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbRoleExpNotify {
    pub RoleId: i32,
    pub Exp: i32,
    pub Level: i32,
}

impl PbRoleExpNotify {
    pub const MSG_ID: u16 = 26249;
}

impl PbMessage for PbRoleExpNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
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
            9 => self.RoleId = r.read_varint_i32()?,
            14 => self.Exp = r.read_varint_i32()?,
            15 => self.Level = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbRoleResonLockFinishNotify {
    pub ResonId: i32,
    pub RoleId: i32,
}

impl PbRoleResonLockFinishNotify {
    pub const MSG_ID: u16 = 26928;
}

impl PbMessage for PbRoleResonLockFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ResonId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ResonId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            5 => self.ResonId = r.read_varint_i32()?,
            8 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PbRoleScenePropsNotify {
    pub BaseProp: Vec<ArrayIntInt>,
    pub AddProp: Vec<ArrayIntInt>,
    pub RoleId: i32,
}

impl PbRoleScenePropsNotify {
    pub const MSG_ID: u16 = 18564;
}

impl PbMessage for PbRoleScenePropsNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.BaseProp {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.AddProp {
            crate::wire::write_submessage(out, 4, item);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.BaseProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.AddProp
                    .push(<ArrayIntInt as PbMessage>::decode(bytes)?);
            }
            7 => self.RoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomAffixInfo {
    pub ConfigId: i32,
    pub Isunlock: bool,
    pub ElementUnits: Vec<ElementUnit>,
}

impl PbMessage for PhantomAffixInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConfigId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConfigId);
        }
        if self.Isunlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ElementUnits {
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
            1 => self.ConfigId = r.read_varint_i32()?,
            2 => self.Isunlock = r.read_bool()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.ElementUnits
                    .push(<ElementUnit as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomAttrRecommendInfo {
    pub AttrType: i32,
    pub AddType: i32,
    pub Usage: i32,
}

impl PbMessage for PhantomAttrRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AttrType != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttrType);
        }
        if self.AddType != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddType);
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
            1 => self.AttrType = r.read_varint_i32()?,
            2 => self.AddType = r.read_varint_i32()?,
            3 => self.Usage = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomAttrRecommendRequest {
    pub RoleId: i32,
}

impl PhantomAttrRecommendRequest {
    pub const MSG_ID: u16 = 18178;
}

impl PbMessage for PhantomAttrRecommendRequest {
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
pub struct PhantomAttrRecommendResponse {
    pub CostPhantomAttrRecommendInfos: Vec<CostPhantomAttrRecommendInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomAttrRecommendResponse {
    pub const MSG_ID: u16 = 29590;
}

impl PbMessage for PhantomAttrRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CostPhantomAttrRecommendInfos {
            crate::wire::write_submessage(out, 5, item);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.CostPhantomAttrRecommendInfos
                    .push(<CostPhantomAttrRecommendInfo as PbMessage>::decode(bytes)?);
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
pub struct PhantomBatchOperRequest {
    pub Oper: Option<PhantomBatchOper>,
    pub IncrId: Vec<i32>,
}

impl PhantomBatchOperRequest {
    pub const MSG_ID: u16 = 15779;
}

impl PbMessage for PhantomBatchOperRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Oper {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.IncrId {
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
            1 => {
                self.Oper = Some(
                    <PhantomBatchOper as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncrId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBatchOperResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl PhantomBatchOperResponse {
    pub const MSG_ID: u16 = 25271;
}

impl PbMessage for PhantomBatchOperResponse {
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
pub struct PhantomBatchRefiningRequest {
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomBatchRefiningRequest {
    pub const MSG_ID: u16 = 27203;
}

impl PbMessage for PhantomBatchRefiningRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StackEntries {
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
                self.StackEntries
                    .push(<PhantomStackConsumeEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBatchRefiningResponse {
    pub CostPhantoms: Vec<PhantomItem>,
    pub ErrorCode: Option<ErrorCode>,
    pub ExtraItems: Vec<AddCountItemInfo>,
    pub Items: Vec<AddCountItemInfo>,
}

impl PhantomBatchRefiningResponse {
    pub const MSG_ID: u16 = 18643;
}

impl PbMessage for PhantomBatchRefiningResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CostPhantoms {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.ExtraItems {
            crate::wire::write_submessage(out, 9, item);
        }
        for item in &self.Items {
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
                self.CostPhantoms
                    .push(<PhantomItem as PbMessage>::decode(bytes)?);
            }
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.ExtraItems
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Items
                    .push(<AddCountItemInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleActivityInfo {
    pub PhantomBattleChallengeInfos: Vec<PhantomBattleChallengeInfo>,
    pub ActivityTasks: Vec<ActivityTask>,
    pub PhantomBattleMasterInfo: Option<PhantomBattleMasterInfo>,
    pub PhantomBattleBadgeInfos: Vec<PhantomBattleBadgeInfo>,
    pub PhantomBattleBadgeRewardInfos: Vec<PhantomBattleBadgeRewardInfo>,
    pub PhantomBattleCardInfos: Vec<PhantomBattleCardInfo>,
    pub PhantomBattleCardRewardInfos: Vec<PhantomBattleCardRewardInfo>,
    pub PhantomBattleRoleInfos: Vec<PhantomBattleRoleInfo>,
    pub PhantomBattleCardGroupInfos: Vec<PhantomBattleCardGroupInfo>,
    pub TimeLimitShopEndTime: i64,
}

impl PbMessage for PhantomBattleActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleChallengeInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.ActivityTasks {
            crate::wire::write_submessage(out, 2, item);
        }
        if let Some(v) = &self.PhantomBattleMasterInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        for item in &self.PhantomBattleBadgeInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.PhantomBattleBadgeRewardInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.PhantomBattleCardInfos {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PhantomBattleCardRewardInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        for item in &self.PhantomBattleRoleInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.PhantomBattleCardGroupInfos {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.TimeLimitShopEndTime != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TimeLimitShopEndTime);
        }
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
                self.PhantomBattleChallengeInfos
                    .push(<PhantomBattleChallengeInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleMasterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleBadgeInfos
                    .push(<PhantomBattleBadgeInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleBadgeRewardInfos
                    .push(<PhantomBattleBadgeRewardInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardInfos
                    .push(<PhantomBattleCardInfo as PbMessage>::decode(bytes)?);
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardRewardInfos
                    .push(<PhantomBattleCardRewardInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleRoleInfos
                    .push(<PhantomBattleRoleInfo as PbMessage>::decode(bytes)?);
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardGroupInfos
                    .push(<PhantomBattleCardGroupInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.TimeLimitShopEndTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleAddBuffEffectCtx {
    pub PhantomBattleBuffId: Vec<i32>,
}

impl PbMessage for PhantomBattleAddBuffEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleBuffId {
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
                &mut self.PhantomBattleBuffId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleAddBuffInfoNotify {
    pub AddBuffs: BTreeMap<i32, PhantomBattleAddBuffEffectCtx>,
}

impl PhantomBattleAddBuffInfoNotify {
    pub const MSG_ID: u16 = 22993;
}

impl PbMessage for PhantomBattleAddBuffInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.AddBuffs.is_empty() {
            crate::wire::write_map(out, 10, &self.AddBuffs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => crate::wire::decode_map_entry(r, &mut self.AddBuffs)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleAreaSkillCdNotify {
    pub Cd: i32,
}

impl PhantomBattleAreaSkillCdNotify {
    pub const MSG_ID: u16 = 24988;
}

impl PbMessage for PhantomBattleAreaSkillCdNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Cd != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cd);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Cd = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleAreaSkillNotify {
    pub PhantomBattleSkillTriggerInfos: Option<PhantomBattleSkillTriggerInfo>,
}

impl PhantomBattleAreaSkillNotify {
    pub const MSG_ID: u16 = 16501;
}

impl PbMessage for PhantomBattleAreaSkillNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleSkillTriggerInfos {
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
                self.PhantomBattleSkillTriggerInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleAreaSkillRequest {}

impl PhantomBattleAreaSkillRequest {
    pub const MSG_ID: u16 = 25124;
}

impl PbMessage for PhantomBattleAreaSkillRequest {
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
pub struct PhantomBattleAreaSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleSkillTriggerInfos: Option<PhantomBattleSkillTriggerInfo>,
    pub Cd: i32,
}

impl PhantomBattleAreaSkillResponse {
    pub const MSG_ID: u16 = 29761;
}

impl PbMessage for PhantomBattleAreaSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleSkillTriggerInfos {
            crate::wire::write_submessage(out, 12, v);
        }
        if self.Cd != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Cd);
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
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSkillTriggerInfos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.Cd = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBackLibraryRequest {
    pub CardUid: i32,
}

impl PhantomBattleBackLibraryRequest {
    pub const MSG_ID: u16 = 22264;
}

impl PbMessage for PhantomBattleBackLibraryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.CardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBackLibraryResponse {
    pub CardUid: i32,
    pub CurCardLibraryNum: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleBackLibraryResponse {
    pub const MSG_ID: u16 = 15743;
}

impl PbMessage for PhantomBattleBackLibraryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
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
            9 => self.CardUid = r.read_varint_i32()?,
            12 => self.CurCardLibraryNum = r.read_varint_i32()?,
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
pub struct PhantomBattleBackSlotCardLibraryRequest {
    pub CardUid: i32,
}

impl PhantomBattleBackSlotCardLibraryRequest {
    pub const MSG_ID: u16 = 18589;
}

impl PbMessage for PhantomBattleBackSlotCardLibraryRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.CardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBackSlotCardLibraryResponse {
    pub CurCardLibraryNum: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub CardUid: i32,
}

impl PhantomBattleBackSlotCardLibraryResponse {
    pub const MSG_ID: u16 = 24521;
}

impl PbMessage for PhantomBattleBackSlotCardLibraryResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.CurCardLibraryNum = r.read_varint_i32()?,
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => self.CardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeInfo {
    pub PhantomBattleBadgeId: i32,
    pub IsUnlock: bool,
}

impl PbMessage for PhantomBattleBadgeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleBadgeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleBadgeId);
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
            1 => self.PhantomBattleBadgeId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeInfoUpdateNotify {
    pub PhantomBattleBadgeInfos: Vec<PhantomBattleBadgeInfo>,
}

impl PhantomBattleBadgeInfoUpdateNotify {
    pub const MSG_ID: u16 = 20719;
}

impl PbMessage for PhantomBattleBadgeInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleBadgeInfos {
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
                self.PhantomBattleBadgeInfos
                    .push(<PhantomBattleBadgeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeRewardInfo {
    pub PhantomBattleBadgeRewardId: i32,
    pub UnlockNum: i32,
    pub IsRewarded: bool,
}

impl PbMessage for PhantomBattleBadgeRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleBadgeRewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleBadgeRewardId);
        }
        if self.UnlockNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockNum);
        }
        if self.IsRewarded {
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
            1 => self.PhantomBattleBadgeRewardId = r.read_varint_i32()?,
            2 => self.UnlockNum = r.read_varint_i32()?,
            3 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeRewardInfoUpdateNotify {
    pub PhantomBattleBadgeRewardInfos: Vec<PhantomBattleBadgeRewardInfo>,
}

impl PhantomBattleBadgeRewardInfoUpdateNotify {
    pub const MSG_ID: u16 = 20290;
}

impl PbMessage for PhantomBattleBadgeRewardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleBadgeRewardInfos {
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
                self.PhantomBattleBadgeRewardInfos
                    .push(<PhantomBattleBadgeRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeRewardRequest {
    pub ActivityId: i32,
    pub BadgeRewardId: Vec<i32>,
}

impl PhantomBattleBadgeRewardRequest {
    pub const MSG_ID: u16 = 22984;
}

impl PbMessage for PhantomBattleBadgeRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.BadgeRewardId {
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
            8 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.BadgeRewardId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBadgeRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleBadgeRewardResponse {
    pub const MSG_ID: u16 = 24270;
}

impl PbMessage for PhantomBattleBadgeRewardResponse {
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
pub struct PhantomBattleBoardFighterInfoCtx {}

impl PbMessage for PhantomBattleBoardFighterInfoCtx {
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
pub struct PhantomBattleBoardInfoNotify {
    pub PhantomBattleGamerFourCTaskInfo: Vec<PhantomBattleGamerFourCTaskInfo>,
    pub PhantomBattleFighterInfo: Vec<PhantomBattleFighterInfo>,
    pub ChallengeId: i32,
    pub PlayerAreaCardSkillUnlock: bool,
    pub NpcAreaCardSkillUnlock: bool,
}

impl PhantomBattleBoardInfoNotify {
    pub const MSG_ID: u16 = 18147;
}

impl PbMessage for PhantomBattleBoardInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleGamerFourCTaskInfo {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 9, item);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.PlayerAreaCardSkillUnlock {
            crate::wire::write_tag(out, 12, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.NpcAreaCardSkillUnlock {
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleGamerFourCTaskInfo.push(
                    <PhantomBattleGamerFourCTaskInfo as PbMessage>::decode(bytes)?,
                );
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFighterInfo
                    .push(<PhantomBattleFighterInfo as PbMessage>::decode(bytes)?);
            }
            10 => self.ChallengeId = r.read_varint_i32()?,
            12 => self.PlayerAreaCardSkillUnlock = r.read_bool()?,
            13 => self.NpcAreaCardSkillUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBoardSettleNotify {
    pub SettleType: Option<PhantomBattleBoardSettleType>,
    pub IsWin: bool,
    pub Reward: Option<PhantomBattleSettleReward>,
    pub ActId: i32,
}

impl PhantomBattleBoardSettleNotify {
    pub const MSG_ID: u16 = 19126;
}

impl PbMessage for PhantomBattleBoardSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SettleType {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsWin {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Reward {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.ActId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActId);
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
                self.SettleType = Some(
                    <PhantomBattleBoardSettleType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            7 => self.IsWin = r.read_bool()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => self.ActId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBuffTriggerInfo {
    pub SourceFighterUId: i32,
    pub PhantomBattleBuffId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub PhantomBattleEffectResultInfo: Option<PhantomBattleEffectResultInfo>,
}

impl PbMessage for PhantomBattleBuffTriggerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceFighterUId);
        }
        if self.PhantomBattleBuffId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleBuffId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.PhantomBattleEffectResultInfo {
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
            1 => self.SourceFighterUId = r.read_varint_i32()?,
            2 => self.PhantomBattleBuffId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
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
pub struct PhantomBattleBuffTriggerInfoNotify {
    pub PhantomBattleBuffTriggerInfo: Vec<PhantomBattleBuffTriggerInfo>,
}

impl PhantomBattleBuffTriggerInfoNotify {
    pub const MSG_ID: u16 = 21641;
}

impl PbMessage for PhantomBattleBuffTriggerInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleBuffTriggerInfo {
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
                self.PhantomBattleBuffTriggerInfo
                    .push(<PhantomBattleBuffTriggerInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBurnBuffNotify {}

impl PhantomBattleBurnBuffNotify {
    pub const MSG_ID: u16 = 18581;
}

impl PbMessage for PhantomBattleBurnBuffNotify {
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
pub struct PhantomBattleBvBFightNotify {}

impl PhantomBattleBvBFightNotify {
    pub const MSG_ID: u16 = 19494;
}

impl PbMessage for PhantomBattleBvBFightNotify {
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
pub struct PhantomBattleBvBLoadingFinishRequest {}

impl PhantomBattleBvBLoadingFinishRequest {
    pub const MSG_ID: u16 = 20661;
}

impl PbMessage for PhantomBattleBvBLoadingFinishRequest {
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
pub struct PhantomBattleBvBLoadingFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleBvBLoadingFinishResponse {
    pub const MSG_ID: u16 = 29345;
}

impl PbMessage for PhantomBattleBvBLoadingFinishResponse {
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
pub struct PhantomBattleBvBLoadingNotify {
    pub PlayerMonsterEntityIncId: BTreeMap<i32, i64>,
    pub NpcMonsterEntityIncId: BTreeMap<i32, i64>,
}

impl PhantomBattleBvBLoadingNotify {
    pub const MSG_ID: u16 = 21837;
}

impl PbMessage for PhantomBattleBvBLoadingNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PlayerMonsterEntityIncId.is_empty() {
            crate::wire::write_map(out, 8, &self.PlayerMonsterEntityIncId);
        }
        if !self.NpcMonsterEntityIncId.is_empty() {
            crate::wire::write_map(out, 15, &self.NpcMonsterEntityIncId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => crate::wire::decode_map_entry(r, &mut self.PlayerMonsterEntityIncId)?,
            15 => crate::wire::decode_map_entry(r, &mut self.NpcMonsterEntityIncId)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleBvBSettleNotify {
    pub NpcOriginalDamage: i32,
    pub NpcCurLife: i32,
    pub IsFinalSettle: bool,
    pub IsWin: bool,
    pub PlayerDeductLife: i32,
    pub PlayerOriginalDamage: i32,
    pub NpcDeductLife: i32,
    pub PlayerCurLife: i32,
}

impl PhantomBattleBvBSettleNotify {
    pub const MSG_ID: u16 = 21619;
}

impl PbMessage for PhantomBattleBvBSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NpcOriginalDamage != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NpcOriginalDamage);
        }
        if self.NpcCurLife != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NpcCurLife);
        }
        if self.IsFinalSettle {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsWin {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerDeductLife != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerDeductLife);
        }
        if self.PlayerOriginalDamage != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerOriginalDamage);
        }
        if self.NpcDeductLife != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NpcDeductLife);
        }
        if self.PlayerCurLife != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerCurLife);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.NpcOriginalDamage = r.read_varint_i32()?,
            2 => self.NpcCurLife = r.read_varint_i32()?,
            3 => self.IsFinalSettle = r.read_bool()?,
            6 => self.IsWin = r.read_bool()?,
            11 => self.PlayerDeductLife = r.read_varint_i32()?,
            12 => self.PlayerOriginalDamage = r.read_varint_i32()?,
            13 => self.NpcDeductLife = r.read_varint_i32()?,
            15 => self.PlayerCurLife = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCallCardEffectCtx {
    pub PhantomBattleFighterInfo: Vec<PhantomBattleFighterInfo>,
    pub FromType: Option<PhantomBattleSelectedCardFromType>,
    pub HeapNum: i32,
    pub HeadNum: i32,
    pub Camp: Option<PhantomBattleEffectCardCamp>,
}

impl PbMessage for PhantomBattleCallCardEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.FromType {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HeapNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeapNum);
        }
        if self.HeadNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadNum);
        }
        if let Some(v) = &self.Camp {
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
                self.PhantomBattleFighterInfo
                    .push(<PhantomBattleFighterInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                self.FromType = Some(
                    <PhantomBattleSelectedCardFromType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            3 => self.HeapNum = r.read_varint_i32()?,
            4 => self.HeadNum = r.read_varint_i32()?,
            5 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomBattleCallCardNotify {
    pub CardFrom: Option<PhantomBattleSelectedCardFromType>,
    pub CanSelectNum: i32,
    pub OwnerCardUid: i32,
    pub EffectUid: i32,
    pub SelectedCardList: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleCallCardNotify {
    pub const MSG_ID: u16 = 24644;
}

impl PbMessage for PhantomBattleCallCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CardFrom {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CanSelectNum != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanSelectNum);
        }
        if self.OwnerCardUid != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerCardUid);
        }
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
        }
        for item in &self.SelectedCardList {
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
            2 => {
                self.CardFrom = Some(
                    <PhantomBattleSelectedCardFromType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            9 => self.CanSelectNum = r.read_varint_i32()?,
            10 => self.OwnerCardUid = r.read_varint_i32()?,
            13 => self.EffectUid = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.SelectedCardList
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCallCardRequest {
    pub EffectUid: i32,
    pub SelectedCardUid: Vec<i32>,
}

impl PhantomBattleCallCardRequest {
    pub const MSG_ID: u16 = 25399;
}

impl PbMessage for PhantomBattleCallCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
        }
        for item in &self.SelectedCardUid {
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
            4 => self.EffectUid = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectedCardUid,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCallCardResponse {
    pub PhantomBattleFighterInfo: Vec<PhantomBattleFighterInfo>,
    pub EffectUid: i32,
    pub HeapNum: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCallCardResponse {
    pub const MSG_ID: u16 = 26502;
}

impl PbMessage for PhantomBattleCallCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
        }
        if self.HeapNum != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeapNum);
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
                self.PhantomBattleFighterInfo
                    .push(<PhantomBattleFighterInfo as PbMessage>::decode(bytes)?);
            }
            9 => self.EffectUid = r.read_varint_i32()?,
            11 => self.HeapNum = r.read_varint_i32()?,
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
pub struct PhantomBattleCardAttrInfoNotify {
    pub CardFighterUId: i32,
    pub BattleStatus: BTreeMap<i32, i32>,
}

impl PhantomBattleCardAttrInfoNotify {
    pub const MSG_ID: u16 = 27398;
}

impl PbMessage for PhantomBattleCardAttrInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardFighterUId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardFighterUId);
        }
        if !self.BattleStatus.is_empty() {
            crate::wire::write_map(out, 14, &self.BattleStatus);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.CardFighterUId = r.read_varint_i32()?,
            14 => crate::wire::decode_map_entry(r, &mut self.BattleStatus)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardDurableEmptyNotify {
    pub CardHeapSize: i32,
    pub DealType: Option<PhantomBattleCardDurableDealType>,
    pub CardUid: i32,
}

impl PhantomBattleCardDurableEmptyNotify {
    pub const MSG_ID: u16 = 24020;
}

impl PbMessage for PhantomBattleCardDurableEmptyNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardHeapSize != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardHeapSize);
        }
        if let Some(v) = &self.DealType {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.CardHeapSize = r.read_varint_i32()?,
            14 => {
                self.DealType = Some(
                    <PhantomBattleCardDurableDealType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.CardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardFighterInfoCtx {
    pub CardUid: i32,
    pub CardConfId: i32,
    pub GamerIndex: i32,
    pub SlotIndex: i32,
    pub Attrs: BTreeMap<i32, i32>,
    pub EvolveCount: i32,
    pub Factors: Vec<i32>,
    pub CanUnlimitedEvolve: bool,
    pub IsCopy: bool,
    pub SkillUnlockInfo: Option<PhantomBattleCardSkillUnlockInfo>,
    pub LockFactors: Vec<i32>,
}

impl PbMessage for PhantomBattleCardFighterInfoCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
        if self.CardConfId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfId);
        }
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
        if !self.Attrs.is_empty() {
            crate::wire::write_map(out, 5, &self.Attrs);
        }
        if self.EvolveCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EvolveCount);
        }
        for item in &self.Factors {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CanUnlimitedEvolve {
            crate::wire::write_tag(out, 8, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsCopy {
            crate::wire::write_tag(out, 9, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.SkillUnlockInfo {
            crate::wire::write_submessage(out, 10, v);
        }
        for item in &self.LockFactors {
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
            1 => self.CardUid = r.read_varint_i32()?,
            2 => self.CardConfId = r.read_varint_i32()?,
            3 => self.GamerIndex = r.read_varint_i32()?,
            4 => self.SlotIndex = r.read_varint_i32()?,
            5 => crate::wire::decode_map_entry(r, &mut self.Attrs)?,
            6 => self.EvolveCount = r.read_varint_i32()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Factors, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            8 => self.CanUnlimitedEvolve = r.read_bool()?,
            9 => self.IsCopy = r.read_bool()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.SkillUnlockInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockFactors, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupAddRequest {
    pub EquipCardIds: Vec<i32>,
    pub ActivityId: i32,
    pub Name: Vec<u8>,
}

impl PhantomBattleCardGroupAddRequest {
    pub const MSG_ID: u16 = 16307;
}

impl PbMessage for PhantomBattleCardGroupAddRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipCardIds {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipCardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.ActivityId = r.read_varint_i32()?,
            13 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupAddResponse {
    pub PhantomBattleCardGroupInfo: Option<PhantomBattleCardGroupInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardGroupAddResponse {
    pub const MSG_ID: u16 = 16063;
}

impl PbMessage for PhantomBattleCardGroupAddResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleCardGroupInfo {
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
                self.PhantomBattleCardGroupInfo
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
pub struct PhantomBattleCardGroupDeleteRequest {
    pub Index: i32,
    pub ActivityId: i32,
}

impl PhantomBattleCardGroupDeleteRequest {
    pub const MSG_ID: u16 = 16154;
}

impl PbMessage for PhantomBattleCardGroupDeleteRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Index != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
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
            12 => self.Index = r.read_varint_i32()?,
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupDeleteResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LastUseCardGroupIndex: i32,
    pub PhantomBattleCardGroupInfo: Vec<PhantomBattleCardGroupInfo>,
}

impl PhantomBattleCardGroupDeleteResponse {
    pub const MSG_ID: u16 = 25229;
}

impl PbMessage for PhantomBattleCardGroupDeleteResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LastUseCardGroupIndex != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastUseCardGroupIndex);
        }
        for item in &self.PhantomBattleCardGroupInfo {
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
            3 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.LastUseCardGroupIndex = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardGroupInfo
                    .push(<PhantomBattleCardGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupInfo {
    pub Name: Vec<u8>,
    pub EquipCardIds: Vec<i32>,
    pub CanUse: bool,
    pub LastUseChallengeId: Vec<i32>,
    pub Index: i32,
    pub SkillUnlockInfos: Vec<PhantomBattleCardSkillUnlockInfo>,
}

impl PbMessage for PhantomBattleCardGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        for item in &self.EquipCardIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CanUse {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.LastUseChallengeId {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
        }
        for item in &self.SkillUnlockInfos {
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
            1 => self.Name = r.read_len_delimited()?.to_vec(),
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipCardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.CanUse = r.read_bool()?,
            4 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.LastUseChallengeId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.Index = r.read_varint_i32()?,
            6 => {
                let bytes = r.read_len_delimited()?;
                self.SkillUnlockInfos.push(
                    <PhantomBattleCardSkillUnlockInfo as PbMessage>::decode(bytes)?,
                );
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupInfoNotify {
    pub PhantomBattleCardGroupInfos: Vec<PhantomBattleCardGroupInfo>,
}

impl PhantomBattleCardGroupInfoNotify {
    pub const MSG_ID: u16 = 17347;
}

impl PbMessage for PhantomBattleCardGroupInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleCardGroupInfos {
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
                self.PhantomBattleCardGroupInfos
                    .push(<PhantomBattleCardGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupNameRequest {
    pub ActivityId: i32,
    pub Index: i32,
    pub Name: Vec<u8>,
}

impl PhantomBattleCardGroupNameRequest {
    pub const MSG_ID: u16 = 18469;
}

impl PbMessage for PhantomBattleCardGroupNameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        if self.Index != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Index);
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
            3 => self.ActivityId = r.read_varint_i32()?,
            7 => self.Index = r.read_varint_i32()?,
            11 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupNameResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleCardGroupInfo: Option<PhantomBattleCardGroupInfo>,
}

impl PhantomBattleCardGroupNameResponse {
    pub const MSG_ID: u16 = 26141;
}

impl PbMessage for PhantomBattleCardGroupNameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleCardGroupInfo {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardGroupInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupUpdateRequest {
    pub EquipCardIds: Vec<i32>,
    pub ActivityId: i32,
    pub Index: i32,
}

impl PhantomBattleCardGroupUpdateRequest {
    pub const MSG_ID: u16 = 21338;
}

impl PbMessage for PhantomBattleCardGroupUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipCardIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.EquipCardIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.ActivityId = r.read_varint_i32()?,
            14 => self.Index = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardGroupUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleCardGroupInfo: Option<PhantomBattleCardGroupInfo>,
}

impl PhantomBattleCardGroupUpdateResponse {
    pub const MSG_ID: u16 = 19388;
}

impl PbMessage for PhantomBattleCardGroupUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleCardGroupInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardGroupInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardInfo {
    pub PhantomBattleCardId: i32,
    pub IsUnlock: bool,
    pub IsUnlockOutLook: bool,
}

impl PbMessage for PhantomBattleCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleCardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleCardId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsUnlockOutLook {
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
            1 => self.PhantomBattleCardId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsUnlockOutLook = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardInfoUpdateNotify {
    pub PhantomBattleCardInfos: Vec<PhantomBattleCardInfo>,
}

impl PhantomBattleCardInfoUpdateNotify {
    pub const MSG_ID: u16 = 26022;
}

impl PbMessage for PhantomBattleCardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleCardInfos {
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
                self.PhantomBattleCardInfos
                    .push(<PhantomBattleCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardOutLookUpRequest {
    pub CardId: i32,
}

impl PhantomBattleCardOutLookUpRequest {
    pub const MSG_ID: u16 = 21176;
}

impl PbMessage for PhantomBattleCardOutLookUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardOutLookUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardOutLookUpResponse {
    pub const MSG_ID: u16 = 28304;
}

impl PbMessage for PhantomBattleCardOutLookUpResponse {
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
pub struct PhantomBattleCardPosChangeInfo {
    pub FighterUid: i32,
    pub AfterPos: i32,
}

impl PbMessage for PhantomBattleCardPosChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.FighterUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FighterUid);
        }
        if self.AfterPos != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AfterPos);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.FighterUid = r.read_varint_i32()?,
            3 => self.AfterPos = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardPosChangeNotify {
    pub ChangeInfo: Vec<PhantomBattleCardPosChangeInfo>,
    pub GamerIndex: i32,
}

impl PhantomBattleCardPosChangeNotify {
    pub const MSG_ID: u16 = 23839;
}

impl PbMessage for PhantomBattleCardPosChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChangeInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
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
                self.ChangeInfo
                    .push(<PhantomBattleCardPosChangeInfo as PbMessage>::decode(
                        bytes,
                    )?);
            }
            8 => self.GamerIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRewardInfo {
    pub PhantomBattleCardRewardId: i32,
    pub UnlockNum: i32,
    pub IsRewarded: bool,
}

impl PbMessage for PhantomBattleCardRewardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleCardRewardId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleCardRewardId);
        }
        if self.UnlockNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UnlockNum);
        }
        if self.IsRewarded {
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
            1 => self.PhantomBattleCardRewardId = r.read_varint_i32()?,
            2 => self.UnlockNum = r.read_varint_i32()?,
            3 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRewardInfoUpdateNotify {
    pub PhantomBattleCardRewardInfos: Vec<PhantomBattleCardRewardInfo>,
}

impl PhantomBattleCardRewardInfoUpdateNotify {
    pub const MSG_ID: u16 = 18519;
}

impl PbMessage for PhantomBattleCardRewardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleCardRewardInfos {
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
                self.PhantomBattleCardRewardInfos
                    .push(<PhantomBattleCardRewardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRewardRequest {
    pub ActivityId: i32,
    pub CardRewardId: Vec<i32>,
}

impl PhantomBattleCardRewardRequest {
    pub const MSG_ID: u16 = 24305;
}

impl PbMessage for PhantomBattleCardRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.CardRewardId {
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
            7 => self.ActivityId = r.read_varint_i32()?,
            13 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardRewardId, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardRewardResponse {
    pub const MSG_ID: u16 = 25254;
}

impl PbMessage for PhantomBattleCardRewardResponse {
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
pub struct PhantomBattleCardRoleSkillRequest {
    pub SelectFighterUId: Vec<i32>,
    pub SkillId: i32,
}

impl PhantomBattleCardRoleSkillRequest {
    pub const MSG_ID: u16 = 16984;
}

impl PbMessage for PhantomBattleCardRoleSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            13 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRoleSkillResponse {
    pub PhantomBattleSkillTriggerInfo: Option<PhantomBattleSkillTriggerInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardRoleSkillResponse {
    pub const MSG_ID: u16 = 18955;
}

impl PbMessage for PhantomBattleCardRoleSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleSkillTriggerInfo {
            crate::wire::write_submessage(out, 10, v);
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
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSkillTriggerInfo
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
pub struct PhantomBattleCardRoleTargetInfoRequest {
    pub SkillId: i32,
}

impl PhantomBattleCardRoleTargetInfoRequest {
    pub const MSG_ID: u16 = 29346;
}

impl PbMessage for PhantomBattleCardRoleTargetInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            3 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardRoleTargetInfoResponse {
    pub FighterUId: Vec<i32>,
    pub EffectType: i32,
    pub Num: i32,
    pub SkillId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardRoleTargetInfoResponse {
    pub const MSG_ID: u16 = 20116;
}

impl PbMessage for PhantomBattleCardRoleTargetInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FighterUId {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EffectType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectType);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FighterUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            7 => self.EffectType = r.read_varint_i32()?,
            9 => self.Num = r.read_varint_i32()?,
            11 => self.SkillId = r.read_varint_i32()?,
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
pub struct PhantomBattleCardSelectEffectCtx {
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub SelectCount: i32,
}

impl PbMessage for PhantomBattleCardSelectEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleHandCardInfos {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.SelectCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SelectCount);
        }
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
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.SelectCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardSelectRequest {
    pub CardUid: Vec<i32>,
}

impl PhantomBattleCardSelectRequest {
    pub const MSG_ID: u16 = 16151;
}

impl PbMessage for PhantomBattleCardSelectRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CardUid {
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
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardUid, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardSelectResponse {
    pub Error: Option<ErrorCode>,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub CurCardLibraryNum: i32,
}

impl PhantomBattleCardSelectResponse {
    pub const MSG_ID: u16 = 19276;
}

impl PbMessage for PhantomBattleCardSelectResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomBattleHandCardInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            1 => {
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            11 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardSkillRequest {
    pub CardUId: i32,
    pub SelectFighterUId: Vec<i32>,
}

impl PhantomBattleCardSkillRequest {
    pub const MSG_ID: u16 = 19909;
}

impl PbMessage for PhantomBattleCardSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        for item in &self.SelectFighterUId {
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
            10 => self.CardUId = r.read_varint_i32()?,
            12 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardSkillResponse {
    pub CardUId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub LibraryCount: i32,
    pub PhantomBattleSkillTriggerInfo: Option<PhantomBattleSkillTriggerInfo>,
}

impl PhantomBattleCardSkillResponse {
    pub const MSG_ID: u16 = 26492;
}

impl PbMessage for PhantomBattleCardSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LibraryCount != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LibraryCount);
        }
        if let Some(v) = &self.PhantomBattleSkillTriggerInfo {
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
            1 => self.CardUId = r.read_varint_i32()?,
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.LibraryCount = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSkillTriggerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardTargetInfoRequest {
    pub CardUId: i32,
    pub SkillId: i32,
}

impl PhantomBattleCardTargetInfoRequest {
    pub const MSG_ID: u16 = 22368;
}

impl PbMessage for PhantomBattleCardTargetInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            4 => self.CardUId = r.read_varint_i32()?,
            14 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardTargetInfoResponse {
    pub FighterUId: Vec<i32>,
    pub EffectType: i32,
    pub CardUId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub Num: i32,
    pub SkillId: i32,
}

impl PhantomBattleCardTargetInfoResponse {
    pub const MSG_ID: u16 = 17636;
}

impl PbMessage for PhantomBattleCardTargetInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FighterUId {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EffectType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectType);
        }
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FighterUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => self.EffectType = r.read_varint_i32()?,
            6 => self.CardUId = r.read_varint_i32()?,
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            14 => self.Num = r.read_varint_i32()?,
            15 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardUnlockRequest {
    pub CardId: i32,
}

impl PhantomBattleCardUnlockRequest {
    pub const MSG_ID: u16 = 19404;
}

impl PbMessage for PhantomBattleCardUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            14 => self.CardId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCardUnlockResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleCardUnlockResponse {
    pub const MSG_ID: u16 = 23550;
}

impl PbMessage for PhantomBattleCardUnlockResponse {
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
pub struct PhantomBattleCardUpdateNotify {
    pub CanUnlimitedEvolve: bool,
    pub CardUId: i32,
    pub LockFactors: Vec<i32>,
    pub Factors: BTreeMap<i32, i32>,
}

impl PhantomBattleCardUpdateNotify {
    pub const MSG_ID: u16 = 15912;
}

impl PbMessage for PhantomBattleCardUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CanUnlimitedEvolve {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        for item in &self.LockFactors {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.Factors.is_empty() {
            crate::wire::write_map(out, 13, &self.Factors);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.CanUnlimitedEvolve = r.read_bool()?,
            7 => self.CardUId = r.read_varint_i32()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockFactors, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => crate::wire::decode_map_entry(r, &mut self.Factors)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleChallengeInfo {
    pub PhantomBattleChallengeId: i32,
    pub IsUnlock: bool,
    pub IsPassRewarded: bool,
    pub LastCardRoleId: i32,
    pub LastCardGroupIndex: i32,
    pub FinishConditions: Vec<i32>,
    pub IsUncover: bool,
    pub IsShow: bool,
}

impl PbMessage for PhantomBattleChallengeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleChallengeId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsPassRewarded {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LastCardRoleId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastCardRoleId);
        }
        if self.LastCardGroupIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastCardGroupIndex);
        }
        for item in &self.FinishConditions {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.IsUncover {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsShow {
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
            1 => self.PhantomBattleChallengeId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsPassRewarded = r.read_bool()?,
            4 => self.LastCardRoleId = r.read_varint_i32()?,
            5 => self.LastCardGroupIndex = r.read_varint_i32()?,
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.FinishConditions,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.IsUncover = r.read_bool()?,
            8 => self.IsShow = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleChallengeInfoUpdateNotify {
    pub PhantomBattleChallengeInfos: Vec<PhantomBattleChallengeInfo>,
}

impl PhantomBattleChallengeInfoUpdateNotify {
    pub const MSG_ID: u16 = 17529;
}

impl PbMessage for PhantomBattleChallengeInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleChallengeInfos {
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
                self.PhantomBattleChallengeInfos
                    .push(<PhantomBattleChallengeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleChallengeUpdateNotify {
    pub IsShow: bool,
    pub FirstWin: bool,
    pub ChallengeInfo: Vec<PhantomBattleChallengeInfo>,
    pub ErrorCode: Option<ErrorCode>,
    pub ChallengeId: i32,
    pub IsUncover: bool,
    pub Unlock: bool,
    pub IsPassRewarded: bool,
}

impl PhantomBattleChallengeUpdateNotify {
    pub const MSG_ID: u16 = 27137;
}

impl PbMessage for PhantomBattleChallengeUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsShow {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.FirstWin {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ChallengeInfo {
            crate::wire::write_submessage(out, 4, item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.IsUncover {
            crate::wire::write_tag(out, 10, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Unlock {
            crate::wire::write_tag(out, 11, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsPassRewarded {
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
            2 => self.IsShow = r.read_bool()?,
            3 => self.FirstWin = r.read_bool()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.ChallengeInfo
                    .push(<PhantomBattleChallengeInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => self.ChallengeId = r.read_varint_i32()?,
            10 => self.IsUncover = r.read_bool()?,
            11 => self.Unlock = r.read_bool()?,
            14 => self.IsPassRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCheckCardSkillUnlockRequest {
    pub CardConfig: Vec<i32>,
}

impl PhantomBattleCheckCardSkillUnlockRequest {
    pub const MSG_ID: u16 = 28636;
}

impl PbMessage for PhantomBattleCheckCardSkillUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CardConfig {
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
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardConfig, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCheckCardSkillUnlockResponse {
    pub TargetNum: i32,
    pub IsUnlock: bool,
    pub CardConfig: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
    pub CurNum: i32,
}

impl PhantomBattleCheckCardSkillUnlockResponse {
    pub const MSG_ID: u16 = 22143;
}

impl PbMessage for PhantomBattleCheckCardSkillUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetNum);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.CardConfig {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.TargetNum = r.read_varint_i32()?,
            6 => self.IsUnlock = r.read_bool()?,
            8 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardConfig, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            13 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.CurNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleCopyCardEffectCtx {
    pub PhantomBattleFighterInfo: Vec<PhantomBattleFighterInfo>,
    pub Camp: Option<PhantomBattleEffectCardCamp>,
}

impl PbMessage for PhantomBattleCopyCardEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        if let Some(v) = &self.Camp {
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
                self.PhantomBattleFighterInfo
                    .push(<PhantomBattleFighterInfo as PbMessage>::decode(bytes)?);
            }
            2 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomBattleDealCardNotify {
    pub CurCardLibraryNum: i32,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleDealCardNotify {
    pub const MSG_ID: u16 = 15397;
}

impl PbMessage for PhantomBattleDealCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        for item in &self.PhantomBattleHandCardInfos {
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
            10 => self.CurCardLibraryNum = r.read_varint_i32()?,
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDealCardReplaceNotify {
    pub ReplaceNum: i32,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleDealCardReplaceNotify {
    pub const MSG_ID: u16 = 23835;
}

impl PbMessage for PhantomBattleDealCardReplaceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReplaceNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReplaceNum);
        }
        for item in &self.PhantomBattleHandCardInfos {
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
            12 => self.ReplaceNum = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDealCardReplaceRequest {
    pub ReplaceCardUid: Vec<i32>,
}

impl PhantomBattleDealCardReplaceRequest {
    pub const MSG_ID: u16 = 19834;
}

impl PbMessage for PhantomBattleDealCardReplaceRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReplaceCardUid {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.ReplaceCardUid, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDealCardReplaceResponse {
    pub CurCardLibraryNum: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleDealCardReplaceResponse {
    pub const MSG_ID: u16 = 29228;
}

impl PbMessage for PhantomBattleDealCardReplaceResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomBattleHandCardInfos {
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
            4 => self.CurCardLibraryNum = r.read_varint_i32()?,
            9 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDealEffectCtx {
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub CardLibraryNum: i32,
}

impl PbMessage for PhantomBattleDealEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleHandCardInfos {
            crate::wire::write_submessage(out, 1, item);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            2 => self.CardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDebugBuffInfo {
    pub BuffUId: i32,
    pub BuffConfId: i32,
    pub RemainRound: i32,
}

impl PbMessage for PhantomBattleDebugBuffInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BuffUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffUId);
        }
        if self.BuffConfId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BuffConfId);
        }
        if self.RemainRound != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RemainRound);
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
            2 => self.BuffConfId = r.read_varint_i32()?,
            3 => self.RemainRound = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDebugCardInfo {
    pub PhantomBattleHandCardInfo: Option<PhantomBattleHandCardInfo>,
    pub FightPower: i32,
}

impl PbMessage for PhantomBattleDebugCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleHandCardInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.FightPower != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightPower);
        }
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
                self.PhantomBattleHandCardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.FightPower = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDebugFighterInfo {
    pub PhantomBattleCardFighterInfoCtx: Option<PhantomBattleCardFighterInfoCtx>,
    pub PhantomBattleBuffInfos: Vec<PhantomBattleDebugBuffInfo>,
    pub FightPower: i32,
}

impl PbMessage for PhantomBattleDebugFighterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleCardFighterInfoCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.PhantomBattleBuffInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.FightPower != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FightPower);
        }
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
                self.PhantomBattleCardFighterInfoCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleBuffInfos
                    .push(<PhantomBattleDebugBuffInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.FightPower = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDebugGamerInfo {
    pub GamerIndex: i32,
    pub PhantomBattleDebugFighterInfos: Vec<PhantomBattleDebugFighterInfo>,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub LibraryCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub PhantomBattleHandDebugCardInfos: Vec<PhantomBattleDebugCardInfo>,
    pub LibraryDebugCardInfos: Vec<PhantomBattleDebugCardInfo>,
}

impl PbMessage for PhantomBattleDebugGamerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        for item in &self.PhantomBattleDebugFighterInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.PhantomBattleHandCardInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.LibraryCardInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.PhantomBattleHandDebugCardInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.LibraryDebugCardInfos {
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
            1 => self.GamerIndex = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleDebugFighterInfos
                    .push(<PhantomBattleDebugFighterInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.LibraryCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandDebugCardInfos
                    .push(<PhantomBattleDebugCardInfo as PbMessage>::decode(bytes)?);
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.LibraryDebugCardInfos
                    .push(<PhantomBattleDebugCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDebugInfoRequest {}

impl PhantomBattleDebugInfoRequest {
    pub const MSG_ID: u16 = 23362;
}

impl PbMessage for PhantomBattleDebugInfoRequest {
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
pub struct PhantomBattleDebugInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleDebugGamerInfos: Vec<PhantomBattleDebugGamerInfo>,
}

impl PhantomBattleDebugInfoResponse {
    pub const MSG_ID: u16 = 17509;
}

impl PbMessage for PhantomBattleDebugInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomBattleDebugGamerInfos {
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
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleDebugGamerInfos
                    .push(<PhantomBattleDebugGamerInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDeployNotify {}

impl PhantomBattleDeployNotify {
    pub const MSG_ID: u16 = 25628;
}

impl PbMessage for PhantomBattleDeployNotify {
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
pub struct PhantomBattleDestroyCardEffectCtx {
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
    pub Camp: Option<PhantomBattleEffectCardCamp>,
}

impl PbMessage for PhantomBattleDestroyCardEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Camp {
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
                self.PhantomBattleFighterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomBattleDirectDamage {
    pub TargetIsPlayer: bool,
    pub Damage: i32,
    pub CardFighterUid: i32,
    pub FinalHp: i32,
}

impl PbMessage for PhantomBattleDirectDamage {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetIsPlayer {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Damage);
        }
        if self.CardFighterUid != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardFighterUid);
        }
        if self.FinalHp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FinalHp);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetIsPlayer = r.read_bool()?,
            2 => self.Damage = r.read_varint_i32()?,
            3 => self.CardFighterUid = r.read_varint_i32()?,
            4 => self.FinalHp = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDiscardCardNotify {
    pub DiscardCardNum: i32,
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleDiscardCardNotify {
    pub const MSG_ID: u16 = 27588;
}

impl PbMessage for PhantomBattleDiscardCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DiscardCardNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DiscardCardNum);
        }
        for item in &self.PhantomBattleHandCardInfos {
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
            6 => self.DiscardCardNum = r.read_varint_i32()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDiscardCardRequest {
    pub DiscardCardIds: Vec<i32>,
}

impl PhantomBattleDiscardCardRequest {
    pub const MSG_ID: u16 = 16286;
}

impl PbMessage for PhantomBattleDiscardCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DiscardCardIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DiscardCardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleDiscardCardResponse {
    pub DiscardCardIds: Vec<i32>,
    pub HandCardNum: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub CurCardLibraryNum: i32,
}

impl PhantomBattleDiscardCardResponse {
    pub const MSG_ID: u16 = 25508;
}

impl PbMessage for PhantomBattleDiscardCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.DiscardCardIds {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            4 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.DiscardCardIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            5 => self.HandCardNum = r.read_varint_i32()?,
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => self.CurCardLibraryNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleEffectResultInfo {
    pub PhantomBattleDealEffectCtx: Option<PhantomBattleDealEffectCtx>,
    pub NpcPhantomBattleDealEffectCtx: Option<NpcPhantomBattleDealEffectCtx>,
    pub PhantomBattleAddBuffEffectCtx: Option<PhantomBattleAddBuffEffectCtx>,
    pub Result: Option<ErrorCode>,
    pub PhantomBattleCardSelectEffectCtx: Option<PhantomBattleCardSelectEffectCtx>,
    pub NpcPhantomBattleCardSelectEffectCtx: Option<NpcPhantomBattleCardSelectEffectCtx>,
    pub NpcPhantomBattleAreaLockEffectCtx: Option<NpcPhantomBattleAreaLockEffectCtx>,
    pub PhantomBattleCopyCardEffectCtx: Option<PhantomBattleCopyCardEffectCtx>,
    pub PhantomBattleDestroyCardEffectCtx: Option<PhantomBattleDestroyCardEffectCtx>,
    pub PhantomBattleTakeTargetAttrEffectCtx: Option<PhantomBattleTakeTargetAttrEffectCtx>,
    pub PhantomBattleFactorsEffectCtx: Option<PhantomBattleFactorsEffectCtx>,
    pub PhantomBattleCallCardEffectCtx: Option<PhantomBattleCallCardEffectCtx>,
    pub PhantomBattleDirectDamage: Option<PhantomBattleDirectDamage>,
    pub PhantomBattleReconstruct: Option<PhantomBattleReconstruct>,
}

impl PbMessage for PhantomBattleEffectResultInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleDealEffectCtx {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.NpcPhantomBattleDealEffectCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.PhantomBattleAddBuffEffectCtx {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.Result {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleCardSelectEffectCtx {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.NpcPhantomBattleCardSelectEffectCtx {
            crate::wire::write_submessage(out, 6, v);
        }
        if let Some(v) = &self.NpcPhantomBattleAreaLockEffectCtx {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.PhantomBattleCopyCardEffectCtx {
            crate::wire::write_submessage(out, 9, v);
        }
        if let Some(v) = &self.PhantomBattleDestroyCardEffectCtx {
            crate::wire::write_submessage(out, 10, v);
        }
        if let Some(v) = &self.PhantomBattleTakeTargetAttrEffectCtx {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.PhantomBattleFactorsEffectCtx {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.PhantomBattleCallCardEffectCtx {
            crate::wire::write_submessage(out, 13, v);
        }
        if let Some(v) = &self.PhantomBattleDirectDamage {
            crate::wire::write_submessage(out, 14, v);
        }
        if let Some(v) = &self.PhantomBattleReconstruct {
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
                self.PhantomBattleDealEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleDealEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleAddBuffEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                self.Result = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardSelectEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleCardSelectEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.NpcPhantomBattleAreaLockEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCopyCardEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleDestroyCardEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleTakeTargetAttrEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFactorsEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCallCardEffectCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleDirectDamage
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleReconstruct
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleEnterCtx {
    pub ChallengeId: i32,
    pub CardRoleId: i32,
    pub CardGroupIndex: i32,
}

impl PbMessage for PhantomBattleEnterCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CardRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardRoleId);
        }
        if self.CardGroupIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardGroupIndex);
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
            2 => self.CardRoleId = r.read_varint_i32()?,
            3 => self.CardGroupIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleEnterSlotRequest {
    pub CardUid: i32,
    pub SlotIndex: i32,
}

impl PhantomBattleEnterSlotRequest {
    pub const MSG_ID: u16 = 21374;
}

impl PbMessage for PhantomBattleEnterSlotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
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
            8 => self.CardUid = r.read_varint_i32()?,
            11 => self.SlotIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleEnterSlotResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
}

impl PhantomBattleEnterSlotResponse {
    pub const MSG_ID: u16 = 16311;
}

impl PbMessage for PhantomBattleEnterSlotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleFighterInfo {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            9 => {
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
pub struct PhantomBattleEvolveRequest {
    pub SlotIndex: i32,
    pub CardUid: i32,
}

impl PhantomBattleEvolveRequest {
    pub const MSG_ID: u16 = 20873;
}

impl PbMessage for PhantomBattleEvolveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SlotIndex);
        }
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.SlotIndex = r.read_varint_i32()?,
            15 => self.CardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleEvolveResponse {
    pub PhantomBattleFighterInfo: Option<PhantomBattleFighterInfo>,
    pub CanEvolveNum: i32,
    pub CardLibraryNum: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleEvolveResponse {
    pub const MSG_ID: u16 = 20416;
}

impl PbMessage for PhantomBattleEvolveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleFighterInfo {
            crate::wire::write_submessage(out, 6, v);
        }
        if self.CanEvolveNum != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanEvolveNum);
        }
        if self.CardLibraryNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardLibraryNum);
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
            6 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFighterInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => self.CanEvolveNum = r.read_varint_i32()?,
            12 => self.CardLibraryNum = r.read_varint_i32()?,
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
pub struct PhantomBattleFactorsEffectCtx {
    pub Factors: Vec<i32>,
}

impl PbMessage for PhantomBattleFactorsEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Factors {
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Factors, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleFighterInfo {
    pub UId: i32,
    pub PhantomBattleBoardFighterInfoCtx: Option<PhantomBattleBoardFighterInfoCtx>,
    pub PhantomBattleGamerFighterInfoCtx: Option<PhantomBattleGamerFighterInfoCtx>,
    pub PhantomBattleCardFighterInfoCtx: Option<PhantomBattleCardFighterInfoCtx>,
}

impl PbMessage for PhantomBattleFighterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.UId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.UId);
        }
        if let Some(v) = &self.PhantomBattleBoardFighterInfoCtx {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.PhantomBattleGamerFighterInfoCtx {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.PhantomBattleCardFighterInfoCtx {
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
            1 => self.UId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleBoardFighterInfoCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleGamerFighterInfoCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleCardFighterInfoCtx
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleFourCTaskDealCardNotify {
    pub PhantomBattleHandCardInfo: Option<PhantomBattleHandCardInfo>,
}

impl PhantomBattleFourCTaskDealCardNotify {
    pub const MSG_ID: u16 = 20251;
}

impl PbMessage for PhantomBattleFourCTaskDealCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleHandCardInfo {
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
                self.PhantomBattleHandCardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleFourCTaskInfoUpdateNotify {
    pub PhantomBattleGamerFourCTaskInfo: Vec<PhantomBattleGamerFourCTaskInfo>,
}

impl PhantomBattleFourCTaskInfoUpdateNotify {
    pub const MSG_ID: u16 = 26498;
}

impl PbMessage for PhantomBattleFourCTaskInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleGamerFourCTaskInfo {
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
                self.PhantomBattleGamerFourCTaskInfo.push(
                    <PhantomBattleGamerFourCTaskInfo as PbMessage>::decode(bytes)?,
                );
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleFourTaskInfo {
    pub ConditionId: i32,
    pub CurProgress: i32,
    pub TargetProgress: i32,
    pub IsFinish: bool,
}

impl PbMessage for PhantomBattleFourTaskInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ConditionId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConditionId);
        }
        if self.CurProgress != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurProgress);
        }
        if self.TargetProgress != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetProgress);
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
            1 => self.ConditionId = r.read_varint_i32()?,
            2 => self.CurProgress = r.read_varint_i32()?,
            3 => self.TargetProgress = r.read_varint_i32()?,
            4 => self.IsFinish = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleGamerFighterInfoCtx {
    pub GamerIndex: i32,
    pub CardRoleId: i32,
    pub BattleStatus: BTreeMap<i32, i32>,
    pub CardLibraryNum: i32,
    pub HandCardNum: i32,
    pub CanEvolveNum: i32,
    pub BattleAttr: BTreeMap<i32, i32>,
}

impl PbMessage for PhantomBattleGamerFighterInfoCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        if self.CardRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardRoleId);
        }
        if !self.BattleStatus.is_empty() {
            crate::wire::write_map(out, 3, &self.BattleStatus);
        }
        if self.CardLibraryNum != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardLibraryNum);
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandCardNum);
        }
        if self.CanEvolveNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanEvolveNum);
        }
        if !self.BattleAttr.is_empty() {
            crate::wire::write_map(out, 7, &self.BattleAttr);
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
            2 => self.CardRoleId = r.read_varint_i32()?,
            3 => crate::wire::decode_map_entry(r, &mut self.BattleStatus)?,
            4 => self.CardLibraryNum = r.read_varint_i32()?,
            5 => self.HandCardNum = r.read_varint_i32()?,
            6 => self.CanEvolveNum = r.read_varint_i32()?,
            7 => crate::wire::decode_map_entry(r, &mut self.BattleAttr)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleGamerFourCTaskInfo {
    pub GamerIndex: i32,
    pub PhantomBattleFourTaskInfos: Vec<PhantomBattleFourTaskInfo>,
    pub IsFinish: bool,
    pub CardConfId: i32,
}

impl PbMessage for PhantomBattleGamerFourCTaskInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        for item in &self.PhantomBattleFourTaskInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.IsFinish {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.CardConfId != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfId);
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
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFourTaskInfos
                    .push(<PhantomBattleFourTaskInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.IsFinish = r.read_bool()?,
            4 => self.CardConfId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleGamerStatusInfoNotify {
    pub GamerIndex: i32,
    pub BattleStatus: BTreeMap<i32, i32>,
}

impl PhantomBattleGamerStatusInfoNotify {
    pub const MSG_ID: u16 = 22067;
}

impl PbMessage for PhantomBattleGamerStatusInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GamerIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GamerIndex);
        }
        if !self.BattleStatus.is_empty() {
            crate::wire::write_map(out, 15, &self.BattleStatus);
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
            15 => crate::wire::decode_map_entry(r, &mut self.BattleStatus)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleGuideRewardRequest {}

impl PhantomBattleGuideRewardRequest {
    pub const MSG_ID: u16 = 26718;
}

impl PbMessage for PhantomBattleGuideRewardRequest {
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
pub struct PhantomBattleGuideRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleGuideRewardResponse {
    pub const MSG_ID: u16 = 21038;
}

impl PbMessage for PhantomBattleGuideRewardResponse {
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
pub struct PhantomBattleHandCardInfo {
    pub CardUid: i32,
    pub CardConfId: i32,
    pub CanUse: bool,
}

impl PbMessage for PhantomBattleHandCardInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
        if self.CardConfId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfId);
        }
        if self.CanUse {
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
            1 => self.CardUid = r.read_varint_i32()?,
            2 => self.CardConfId = r.read_varint_i32()?,
            3 => self.CanUse = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleHandCardInfoUpdateNotify {
    pub PhantomBattleHandCardInfo: Vec<PhantomBattleHandCardInfo>,
}

impl PhantomBattleHandCardInfoUpdateNotify {
    pub const MSG_ID: u16 = 28224;
}

impl PbMessage for PhantomBattleHandCardInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleHandCardInfo {
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
                self.PhantomBattleHandCardInfo
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleLeaveSlotRequest {
    pub SlotIndex: i32,
}

impl PhantomBattleLeaveSlotRequest {
    pub const MSG_ID: u16 = 19738;
}

impl PbMessage for PhantomBattleLeaveSlotRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SlotIndex != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
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
            4 => self.SlotIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleLeaveSlotResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomBattleHandCardInfo: Option<PhantomBattleHandCardInfo>,
}

impl PhantomBattleLeaveSlotResponse {
    pub const MSG_ID: u16 = 25415;
}

impl PbMessage for PhantomBattleLeaveSlotResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PhantomBattleHandCardInfo {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleHandCardInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleMasterInfo {
    pub PhantomBattleMasterLevelId: i32,
    pub Exp: i32,
    pub RewardedPhantomBattleMasterLevelIds: Vec<i32>,
    pub WeekRewardExp: i32,
    pub LastUseCardGroupIndex: i32,
    pub LastUseCardRoleId: i32,
}

impl PbMessage for PhantomBattleMasterInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleMasterLevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleMasterLevelId);
        }
        if self.Exp != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Exp);
        }
        for item in &self.RewardedPhantomBattleMasterLevelIds {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.WeekRewardExp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeekRewardExp);
        }
        if self.LastUseCardGroupIndex != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastUseCardGroupIndex);
        }
        if self.LastUseCardRoleId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LastUseCardRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PhantomBattleMasterLevelId = r.read_varint_i32()?,
            2 => self.Exp = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RewardedPhantomBattleMasterLevelIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => self.WeekRewardExp = r.read_varint_i32()?,
            5 => self.LastUseCardGroupIndex = r.read_varint_i32()?,
            6 => self.LastUseCardRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleMasterInfoNotify {
    pub PhantomBattleMasterInfo: Option<PhantomBattleMasterInfo>,
    pub ActivityId: i32,
}

impl PhantomBattleMasterInfoNotify {
    pub const MSG_ID: u16 = 21654;
}

impl PbMessage for PhantomBattleMasterInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomBattleMasterInfo {
            crate::wire::write_submessage(out, 8, v);
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
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleMasterInfo
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
pub struct PhantomBattleMasterLevelRewardRequest {
    pub PhantomBattleMasterLevelId: i32,
}

impl PhantomBattleMasterLevelRewardRequest {
    pub const MSG_ID: u16 = 25661;
}

impl PbMessage for PhantomBattleMasterLevelRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleMasterLevelId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleMasterLevelId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.PhantomBattleMasterLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleMasterLevelRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleMasterLevelRewardResponse {
    pub const MSG_ID: u16 = 22561;
}

impl PbMessage for PhantomBattleMasterLevelRewardResponse {
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
pub struct PhantomBattleNpcCardUpdateInfo {
    pub CardUId: i32,
    pub Factors: BTreeMap<i32, i32>,
    pub CanUnlimitedEvolve: bool,
    pub LockFactor: Vec<i32>,
}

impl PbMessage for PhantomBattleNpcCardUpdateInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if !self.Factors.is_empty() {
            crate::wire::write_map(out, 2, &self.Factors);
        }
        if self.CanUnlimitedEvolve {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.LockFactor {
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
            1 => self.CardUId = r.read_varint_i32()?,
            2 => crate::wire::decode_map_entry(r, &mut self.Factors)?,
            3 => self.CanUnlimitedEvolve = r.read_bool()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.LockFactor, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleNpcShowOverRequest {}

impl PhantomBattleNpcShowOverRequest {
    pub const MSG_ID: u16 = 29519;
}

impl PbMessage for PhantomBattleNpcShowOverRequest {
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
pub struct PhantomBattleNpcShowOverResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleNpcShowOverResponse {
    pub const MSG_ID: u16 = 15620;
}

impl PbMessage for PhantomBattleNpcShowOverResponse {
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
pub struct PhantomBattlePhantomCardSkillRequest {
    pub CardUid: i32,
}

impl PhantomBattlePhantomCardSkillRequest {
    pub const MSG_ID: u16 = 28104;
}

impl PbMessage for PhantomBattlePhantomCardSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
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
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattlePhantomCardSkillResponse {
    pub EffectType: i32,
    pub CardUId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub FighterUId: Vec<i32>,
    pub Num: i32,
}

impl PhantomBattlePhantomCardSkillResponse {
    pub const MSG_ID: u16 = 22092;
}

impl PbMessage for PhantomBattlePhantomCardSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.EffectType != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectType);
        }
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.FighterUId {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Num != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            4 => self.EffectType = r.read_varint_i32()?,
            8 => self.CardUId = r.read_varint_i32()?,
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FighterUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            12 => self.Num = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleReChallengeRequest {
    pub ChallengeId: i32,
}

impl PhantomBattleReChallengeRequest {
    pub const MSG_ID: u16 = 27585;
}

impl PbMessage for PhantomBattleReChallengeRequest {
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
pub struct PhantomBattleReChallengeResponse {
    pub PhantomBattleNPCId: i32,
    pub ChallengeId: i32,
    pub CardGroupId: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleReChallengeResponse {
    pub const MSG_ID: u16 = 25662;
}

impl PbMessage for PhantomBattleReChallengeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleNPCId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleNPCId);
        }
        if self.ChallengeId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ChallengeId);
        }
        if self.CardGroupId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardGroupId);
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
            2 => self.PhantomBattleNPCId = r.read_varint_i32()?,
            8 => self.ChallengeId = r.read_varint_i32()?,
            11 => self.CardGroupId = r.read_varint_i32()?,
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
pub struct PhantomBattleReconstruct {
    pub Camp: Option<PhantomBattleEffectCardCamp>,
    pub CardUid: Vec<i32>,
    pub HeapNum: i32,
    pub Type: Option<ReconstructFromType>,
    pub HandNum: i32,
}

impl PbMessage for PhantomBattleReconstruct {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Camp {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.CardUid {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.HeapNum != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeapNum);
        }
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HandNum != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HandNum);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CardUid, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.HeapNum = r.read_varint_i32()?,
            4 => {
                self.Type = Some(
                    <ReconstructFromType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => self.HandNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleReconstructCardNotify {
    pub SelectedCardList: Vec<PhantomBattleHandCardInfo>,
    pub EffectUid: i32,
    pub CardFrom: Option<PhantomBattleSelectedCardFromType>,
    pub CanSelectNum: i32,
    pub OwnerCardUid: i32,
}

impl PhantomBattleReconstructCardNotify {
    pub const MSG_ID: u16 = 15963;
}

impl PbMessage for PhantomBattleReconstructCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectedCardList {
            crate::wire::write_submessage(out, 3, item);
        }
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
        }
        if let Some(v) = &self.CardFrom {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CanSelectNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CanSelectNum);
        }
        if self.OwnerCardUid != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerCardUid);
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
                self.SelectedCardList
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            4 => self.EffectUid = r.read_varint_i32()?,
            9 => {
                self.CardFrom = Some(
                    <PhantomBattleSelectedCardFromType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.CanSelectNum = r.read_varint_i32()?,
            15 => self.OwnerCardUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleReconstructCardRequest {
    pub SelectedCardUid: Vec<i32>,
    pub EffectUid: i32,
}

impl PhantomBattleReconstructCardRequest {
    pub const MSG_ID: u16 = 26025;
}

impl PbMessage for PhantomBattleReconstructCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SelectedCardUid {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
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
                &mut self.SelectedCardUid,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.EffectUid = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleReconstructCardResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub HeapNum: i32,
    pub EffectUid: i32,
    pub SelectedCardUid: Vec<i32>,
}

impl PhantomBattleReconstructCardResponse {
    pub const MSG_ID: u16 = 16661;
}

impl PbMessage for PhantomBattleReconstructCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HeapNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeapNum);
        }
        if self.EffectUid != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectUid);
        }
        for item in &self.SelectedCardUid {
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.HeapNum = r.read_varint_i32()?,
            13 => self.EffectUid = r.read_varint_i32()?,
            15 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectedCardUid,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRepeatedTaskRewardRequest {
    pub TaskId: Vec<i32>,
}

impl PhantomBattleRepeatedTaskRewardRequest {
    pub const MSG_ID: u16 = 22983;
}

impl PbMessage for PhantomBattleRepeatedTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TaskId {
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
            10 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TaskId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRepeatedTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleRepeatedTaskRewardResponse {
    pub const MSG_ID: u16 = 16815;
}

impl PbMessage for PhantomBattleRepeatedTaskRewardResponse {
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
pub struct PhantomBattleReward {
    pub ItemId: i32,
    pub Count: i32,
    pub IncId: i32,
}

impl PbMessage for PhantomBattleReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ItemId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ItemId);
        }
        if self.Count != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Count);
        }
        if self.IncId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
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
            1 => self.ItemId = r.read_varint_i32()?,
            2 => self.Count = r.read_varint_i32()?,
            3 => self.IncId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRoleInfo {
    pub PhantomBattleRoleId: i32,
    pub IsUnlock: bool,
    pub IsRewarded: bool,
}

impl PbMessage for PhantomBattleRoleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleRoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleRoleId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRewarded {
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
            1 => self.PhantomBattleRoleId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.IsRewarded = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRoleInfoUpdateNotify {
    pub PhantomBattleRoleInfos: Vec<PhantomBattleRoleInfo>,
}

impl PhantomBattleRoleInfoUpdateNotify {
    pub const MSG_ID: u16 = 20429;
}

impl PbMessage for PhantomBattleRoleInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleRoleInfos {
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
                self.PhantomBattleRoleInfos
                    .push(<PhantomBattleRoleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRoleRewardRequest {
    pub PhantomBattleRoleId: i32,
}

impl PhantomBattleRoleRewardRequest {
    pub const MSG_ID: u16 = 29748;
}

impl PbMessage for PhantomBattleRoleRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomBattleRoleId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomBattleRoleId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.PhantomBattleRoleId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRoleRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleRoleRewardResponse {
    pub const MSG_ID: u16 = 26978;
}

impl PbMessage for PhantomBattleRoleRewardResponse {
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
pub struct PhantomBattleRoundChangeNotify {
    pub RoundNum: i32,
    pub SealInfos: Vec<AreaSealInfo>,
    pub PhantomBattleFighterInfo: Vec<PhantomBattleFighterInfo>,
}

impl PhantomBattleRoundChangeNotify {
    pub const MSG_ID: u16 = 19265;
}

impl PbMessage for PhantomBattleRoundChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoundNum != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoundNum);
        }
        for item in &self.SealInfos {
            crate::wire::write_submessage(out, 8, item);
        }
        for item in &self.PhantomBattleFighterInfo {
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
            6 => self.RoundNum = r.read_varint_i32()?,
            8 => {
                let bytes = r.read_len_delimited()?;
                self.SealInfos
                    .push(<AreaSealInfo as PbMessage>::decode(bytes)?);
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleFighterInfo
                    .push(<PhantomBattleFighterInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleRoundOverRequest {}

impl PhantomBattleRoundOverRequest {
    pub const MSG_ID: u16 = 22166;
}

impl PbMessage for PhantomBattleRoundOverRequest {
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
pub struct PhantomBattleRoundOverResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleRoundOverResponse {
    pub const MSG_ID: u16 = 18522;
}

impl PbMessage for PhantomBattleRoundOverResponse {
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
pub struct PhantomBattleSelectReserveCardNotify {
    pub PhantomBattleHandCardInfos: Vec<PhantomBattleHandCardInfo>,
    pub ReserveCardNum: i32,
}

impl PhantomBattleSelectReserveCardNotify {
    pub const MSG_ID: u16 = 19284;
}

impl PbMessage for PhantomBattleSelectReserveCardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleHandCardInfos {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.ReserveCardNum != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReserveCardNum);
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
                self.PhantomBattleHandCardInfos
                    .push(<PhantomBattleHandCardInfo as PbMessage>::decode(bytes)?);
            }
            15 => self.ReserveCardNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSelectReserveCardRequest {
    pub ReserveCardUIds: Vec<i32>,
}

impl PhantomBattleSelectReserveCardRequest {
    pub const MSG_ID: u16 = 22146;
}

impl PbMessage for PhantomBattleSelectReserveCardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReserveCardUIds {
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
                &mut self.ReserveCardUIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSelectReserveCardResponse {
    pub ReserveCardUIds: Vec<i32>,
    pub CurCardLibraryNum: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub HandCardNum: i32,
}

impl PhantomBattleSelectReserveCardResponse {
    pub const MSG_ID: u16 = 27073;
}

impl PbMessage for PhantomBattleSelectReserveCardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ReserveCardUIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CurCardLibraryNum != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurCardLibraryNum);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.HandCardNum != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
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
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.ReserveCardUIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => self.CurCardLibraryNum = r.read_varint_i32()?,
            11 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.HandCardNum = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSelectTargetEffectNotify {
    pub Num: i32,
    pub EffectType: i32,
    pub OwnerUid: i32,
    pub FighterUId: Vec<i32>,
}

impl PhantomBattleSelectTargetEffectNotify {
    pub const MSG_ID: u16 = 21545;
}

impl PbMessage for PhantomBattleSelectTargetEffectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Num != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Num);
        }
        if self.EffectType != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectType);
        }
        if self.OwnerUid != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.OwnerUid);
        }
        for item in &self.FighterUId {
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
            3 => self.Num = r.read_varint_i32()?,
            5 => self.EffectType = r.read_varint_i32()?,
            7 => self.OwnerUid = r.read_varint_i32()?,
            13 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FighterUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSelectTargetRequest {
    pub FighterUId: Vec<i32>,
}

impl PhantomBattleSelectTargetRequest {
    pub const MSG_ID: u16 = 20698;
}

impl PbMessage for PhantomBattleSelectTargetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FighterUId {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FighterUId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSelectTargetResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleSelectTargetResponse {
    pub const MSG_ID: u16 = 24690;
}

impl PbMessage for PhantomBattleSelectTargetResponse {
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
pub struct PhantomBattleSettleReward {
    pub BeforeLv: i32,
    pub AfterLv: i32,
    pub AddExp: i32,
    pub TotalExp: i32,
    pub Reward: Vec<PhantomBattleReward>,
}

impl PbMessage for PhantomBattleSettleReward {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.BeforeLv != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BeforeLv);
        }
        if self.AfterLv != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AfterLv);
        }
        if self.AddExp != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddExp);
        }
        if self.TotalExp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalExp);
        }
        for item in &self.Reward {
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
            1 => self.BeforeLv = r.read_varint_i32()?,
            2 => self.AfterLv = r.read_varint_i32()?,
            3 => self.AddExp = r.read_varint_i32()?,
            4 => self.TotalExp = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Reward
                    .push(<PhantomBattleReward as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSkillEffectFailNotify {
    pub CardUid: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub EffectType: i32,
    pub CardConfigId: i32,
}

impl PhantomBattleSkillEffectFailNotify {
    pub const MSG_ID: u16 = 29240;
}

impl PbMessage for PhantomBattleSkillEffectFailNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CardUid != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUid);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.EffectType != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.EffectType);
        }
        if self.CardConfigId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardConfigId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.CardUid = r.read_varint_i32()?,
            6 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            7 => self.EffectType = r.read_varint_i32()?,
            9 => self.CardConfigId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSkillTriggerInfo {
    pub SourceFighterUId: i32,
    pub SkillId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub PhantomBattleEffectResultInfo: Option<PhantomBattleEffectResultInfo>,
}

impl PbMessage for PhantomBattleSkillTriggerInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SourceFighterUId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SourceFighterUId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if let Some(v) = &self.PhantomBattleEffectResultInfo {
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
            1 => self.SourceFighterUId = r.read_varint_i32()?,
            2 => self.SkillId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            4 => {
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
pub struct PhantomBattleSkillTriggerInfoNotify {
    pub PhantomBattleSkillTriggerInfos: Vec<PhantomBattleSkillTriggerInfo>,
}

impl PhantomBattleSkillTriggerInfoNotify {
    pub const MSG_ID: u16 = 29886;
}

impl PbMessage for PhantomBattleSkillTriggerInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomBattleSkillTriggerInfos {
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
                self.PhantomBattleSkillTriggerInfos
                    .push(<PhantomBattleSkillTriggerInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSlotCardSkillRequest {
    pub SkillId: i32,
    pub SelectFighterUId: Vec<i32>,
    pub CardUId: i32,
}

impl PhantomBattleSlotCardSkillRequest {
    pub const MSG_ID: u16 = 26717;
}

impl PbMessage for PhantomBattleSlotCardSkillRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillId);
        }
        for item in &self.SelectFighterUId {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
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
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.SelectFighterUId,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            5 => self.CardUId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSlotCardSkillResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub LibraryCount: i32,
    pub PhantomBattleSkillTriggerInfo: Option<PhantomBattleSkillTriggerInfo>,
    pub CardUId: i32,
    pub SkillId: i32,
}

impl PhantomBattleSlotCardSkillResponse {
    pub const MSG_ID: u16 = 26700;
}

impl PbMessage for PhantomBattleSlotCardSkillResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.LibraryCount != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LibraryCount);
        }
        if let Some(v) = &self.PhantomBattleSkillTriggerInfo {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.CardUId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CardUId);
        }
        if self.SkillId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.LibraryCount = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomBattleSkillTriggerInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.CardUId = r.read_varint_i32()?,
            11 => self.SkillId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSlotInsteadRequest {
    pub StartSlotIndex: i32,
    pub TargetSlotIndex: i32,
}

impl PhantomBattleSlotInsteadRequest {
    pub const MSG_ID: u16 = 28150;
}

impl PbMessage for PhantomBattleSlotInsteadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StartSlotIndex != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartSlotIndex);
        }
        if self.TargetSlotIndex != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetSlotIndex);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.StartSlotIndex = r.read_varint_i32()?,
            15 => self.TargetSlotIndex = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleSlotInsteadResponse {
    pub TargetSlotIndex: i32,
    pub StartSlotIndex: i32,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleSlotInsteadResponse {
    pub const MSG_ID: u16 = 17240;
}

impl PbMessage for PhantomBattleSlotInsteadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetSlotIndex != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetSlotIndex);
        }
        if self.StartSlotIndex != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.StartSlotIndex);
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
            3 => self.TargetSlotIndex = r.read_varint_i32()?,
            10 => self.StartSlotIndex = r.read_varint_i32()?,
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
pub struct PhantomBattleStartRequest {}

impl PhantomBattleStartRequest {
    pub const MSG_ID: u16 = 18864;
}

impl PbMessage for PhantomBattleStartRequest {
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
pub struct PhantomBattleStartResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleStartResponse {
    pub const MSG_ID: u16 = 26552;
}

impl PbMessage for PhantomBattleStartResponse {
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
pub struct PhantomBattleTakeTargetAttrEffectCtx {
    pub TargetUid: i32,
    pub TakeAttrInfo: Option<NpcPhantomBattleCardAttrInfo>,
    pub Camp: Option<PhantomBattleEffectCardCamp>,
}

impl PbMessage for PhantomBattleTakeTargetAttrEffectCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetUid != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TargetUid);
        }
        if let Some(v) = &self.TakeAttrInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.Camp {
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
            1 => self.TargetUid = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.TakeAttrInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                self.Camp = Some(
                    <PhantomBattleEffectCardCamp as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PhantomBattleTaskInfoNotify {
    pub ActivityTasks: Vec<ActivityTask>,
}

impl PhantomBattleTaskInfoNotify {
    pub const MSG_ID: u16 = 29022;
}

impl PbMessage for PhantomBattleTaskInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ActivityTasks {
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
                self.ActivityTasks
                    .push(<ActivityTask as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleTaskRewardRequest {
    pub TaskId: i32,
}

impl PhantomBattleTaskRewardRequest {
    pub const MSG_ID: u16 = 22908;
}

impl PbMessage for PhantomBattleTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TaskId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            5 => self.TaskId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomBattleTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomBattleTaskRewardResponse {
    pub const MSG_ID: u16 = 27700;
}

impl PbMessage for PhantomBattleTaskRewardResponse {
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
pub struct PhantomCollectReceiveRequest {
    pub Type: Option<PhantomCollectRewardType>,
    pub ActivityId: i32,
}

impl PhantomCollectReceiveRequest {
    pub const MSG_ID: u16 = 24841;
}

impl PbMessage for PhantomCollectReceiveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Type {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            4 => {
                self.Type = Some(
                    <PhantomCollectRewardType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            15 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomCollectReceiveResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomCollectReceiveResponse {
    pub const MSG_ID: u16 = 21188;
}

impl PbMessage for PhantomCollectReceiveResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 6, &self.ItemMap);
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
            6 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct PhantomCollectUpdateNotify {
    pub PhantomCollectReward: Option<PhantomCollectReward>,
    pub ActivityId: i32,
}

impl PhantomCollectUpdateNotify {
    pub const MSG_ID: u16 = 23250;
}

impl PbMessage for PhantomCollectUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PhantomCollectReward {
            crate::wire::write_submessage(out, 9, v);
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
            9 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomCollectReward
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomDirectRefiningWeeklyResetNotify {
    pub DirectRefineWeekTimes: i32,
}

impl PhantomDirectRefiningWeeklyResetNotify {
    pub const MSG_ID: u16 = 28896;
}

impl PbMessage for PhantomDirectRefiningWeeklyResetNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DirectRefineWeekTimes != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DirectRefineWeekTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.DirectRefineWeekTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomEquipGroupInfo {
    pub IncId: Vec<i32>,
    pub Name: Vec<u8>,
}

impl PbMessage for PhantomEquipGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.IncId {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.IncId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            2 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomEquipGroupInfoNotify {
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
}

impl PhantomEquipGroupInfoNotify {
    pub const MSG_ID: u16 = 16384;
}

impl PbMessage for PhantomEquipGroupInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhantomEquipGroupInfos {
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
                self.PhantomEquipGroupInfos
                    .push(<PhantomEquipGroupInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomEquipGroupInfosRequest {
    pub RoleId: i32,
}

impl PhantomEquipGroupInfosRequest {
    pub const MSG_ID: u16 = 25068;
}

impl PbMessage for PhantomEquipGroupInfosRequest {
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
pub struct PhantomEquipGroupInfosResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub IsOldData: bool,
    pub PhantomEquipGroupInfos: Vec<PhantomEquipGroupInfo>,
}

impl PhantomEquipGroupInfosResponse {
    pub const MSG_ID: u16 = 17071;
}

impl PbMessage for PhantomEquipGroupInfosResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.IsOldData {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.PhantomEquipGroupInfos {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            6 => self.IsOldData = r.read_bool()?,
            7 => {
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
pub struct PhantomEquipInfoNotify {
    pub EquipInfo: Vec<RolePhantomEquipInfo>,
}

impl PhantomEquipInfoNotify {
    pub const MSG_ID: u16 = 29430;
}

impl PbMessage for PhantomEquipInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.EquipInfo {
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
                self.EquipInfo
                    .push(<RolePhantomEquipInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomFetterGroupInfo {
    pub PhantomFetterGroupId: i32,
    pub CountNeed: i32,
}

impl PbMessage for PhantomFetterGroupInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PhantomFetterGroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PhantomFetterGroupId);
        }
        if self.CountNeed != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CountNeed);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.PhantomFetterGroupId = r.read_varint_i32()?,
            2 => self.CountNeed = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomFetterRecommendInfo {
    pub Usage: i32,
    pub PhantomFetterGroupInfos: Vec<PhantomFetterGroupInfo>,
    pub CostCombinationInfos: Vec<CostCombinationInfo>,
    pub MainPhantomRecommendInfos: Vec<MainPhantomRecommendInfo>,
}

impl PbMessage for PhantomFetterRecommendInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Usage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Usage);
        }
        for item in &self.PhantomFetterGroupInfos {
            crate::wire::write_submessage(out, 3, item);
        }
        for item in &self.CostCombinationInfos {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.MainPhantomRecommendInfos {
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
            2 => self.Usage = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomFetterGroupInfos
                    .push(<PhantomFetterGroupInfo as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.CostCombinationInfos
                    .push(<CostCombinationInfo as PbMessage>::decode(bytes)?);
            }
            5 => {
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
pub struct PhantomFetterRecommendRequest {
    pub RoleId: i32,
}

impl PhantomFetterRecommendRequest {
    pub const MSG_ID: u16 = 29643;
}

impl PbMessage for PhantomFetterRecommendRequest {
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
pub struct PhantomFetterRecommendResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub PhantomFetterRecommendInfos: Vec<PhantomFetterRecommendInfo>,
}

impl PhantomFetterRecommendResponse {
    pub const MSG_ID: u16 = 25672;
}

impl PbMessage for PhantomFetterRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.PhantomFetterRecommendInfos {
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
            8 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PhantomFetterRecommendInfos
                    .push(<PhantomFetterRecommendInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomFormationChangeNotify {
    pub CurrRole: i32,
    pub IsExit: bool,
    pub RoleIds: Vec<i32>,
    pub FormationId: i32,
}

impl PhantomFormationChangeNotify {
    pub const MSG_ID: u16 = 16773;
}

impl PbMessage for PhantomFormationChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurrRole != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurrRole);
        }
        if self.IsExit {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.RoleIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.FormationId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            4 => self.CurrRole = r.read_varint_i32()?,
            6 => self.IsExit = r.read_bool()?,
            7 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RoleIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            11 => self.FormationId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicCollectUpdateRequest {
    pub CollectMusicIds: Vec<i32>,
}

impl PhantomMusicCollectUpdateRequest {
    pub const MSG_ID: u16 = 18730;
}

impl PbMessage for PhantomMusicCollectUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CollectMusicIds {
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
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CollectMusicIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicCollectUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomMusicCollectUpdateResponse {
    pub const MSG_ID: u16 = 18049;
}

impl PbMessage for PhantomMusicCollectUpdateResponse {
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
pub struct PhantomMusicInfoRequest {}

impl PhantomMusicInfoRequest {
    pub const MSG_ID: u16 = 16180;
}

impl PbMessage for PhantomMusicInfoRequest {
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
pub struct PhantomMusicInfoResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CollectMusicIds: Vec<i32>,
    pub AlbumTimeInfos: Vec<PhonographAlbumTimeInfo>,
    pub CurMusicId: i32,
}

impl PhantomMusicInfoResponse {
    pub const MSG_ID: u16 = 16334;
}

impl PbMessage for PhantomMusicInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.CollectMusicIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AlbumTimeInfos {
            crate::wire::write_submessage(out, 7, item);
        }
        if self.CurMusicId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurMusicId);
        }
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
            2 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.CollectMusicIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.AlbumTimeInfos
                    .push(<PhonographAlbumTimeInfo as PbMessage>::decode(bytes)?);
            }
            13 => self.CurMusicId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicNotify {
    pub Music: i32,
}

impl PhantomMusicNotify {
    pub const MSG_ID: u16 = 19275;
}

impl PbMessage for PhantomMusicNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Music != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Music);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            4 => self.Music = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicSaveRequest {
    pub Music: i32,
}

impl PhantomMusicSaveRequest {
    pub const MSG_ID: u16 = 17072;
}

impl PbMessage for PhantomMusicSaveRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Music != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Music);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.Music = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicSaveResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl PhantomMusicSaveResponse {
    pub const MSG_ID: u16 = 25499;
}

impl PbMessage for PhantomMusicSaveResponse {
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
pub struct PhantomMusicSwitchRequest {
    pub MusicId: i32,
}

impl PhantomMusicSwitchRequest {
    pub const MSG_ID: u16 = 23417;
}

impl PbMessage for PhantomMusicSwitchRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MusicId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MusicId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            5 => self.MusicId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicSwitchResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomMusicSwitchResponse {
    pub const MSG_ID: u16 = 15618;
}

impl PbMessage for PhantomMusicSwitchResponse {
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
pub struct PhantomMusicUnlockRequest {
    pub Items: Vec<i32>,
}

impl PhantomMusicUnlockRequest {
    pub const MSG_ID: u16 = 25144;
}

impl PbMessage for PhantomMusicUnlockRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Items {
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
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Items, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomMusicUnlockResponse {
    pub AddMusicIds: Vec<i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomMusicUnlockResponse {
    pub const MSG_ID: u16 = 23502;
}

impl PbMessage for PhantomMusicUnlockResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AddMusicIds {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
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
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AddMusicIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
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
pub struct PhantomRecommendRequest {
    pub RoleId: i32,
}

impl PhantomRecommendRequest {
    pub const MSG_ID: u16 = 21254;
}

impl PbMessage for PhantomRecommendRequest {
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
pub struct PhantomRecommendResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub FetterGroupId: i32,
    pub RoleId: i32,
    pub MonsterIdList: Vec<i32>,
    pub MainPropId: i32,
}

impl PhantomRecommendResponse {
    pub const MSG_ID: u16 = 26730;
}

impl PbMessage for PhantomRecommendResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.FetterGroupId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FetterGroupId);
        }
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        for item in &self.MonsterIdList {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.MainPropId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MainPropId);
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
            6 => self.FetterGroupId = r.read_varint_i32()?,
            8 => self.RoleId = r.read_varint_i32()?,
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.MonsterIdList, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            15 => self.MainPropId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomReturnPreviewRequest {
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomReturnPreviewRequest {
    pub const MSG_ID: u16 = 18276;
}

impl PbMessage for PhantomReturnPreviewRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StackEntries {
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
                self.StackEntries
                    .push(<PhantomStackConsumeEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomReturnPreviewResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub ItemMap: BTreeMap<i32, i32>,
}

impl PhantomReturnPreviewResponse {
    pub const MSG_ID: u16 = 21268;
}

impl PbMessage for PhantomReturnPreviewResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 11, &self.ItemMap);
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
            11 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomReturnRequest {
    pub StackEntries: Vec<PhantomStackConsumeEntry>,
}

impl PhantomReturnRequest {
    pub const MSG_ID: u16 = 16594;
}

impl PbMessage for PhantomReturnRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.StackEntries {
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
                self.StackEntries
                    .push(<PhantomStackConsumeEntry as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomReturnResponse {
    pub ItemMap: BTreeMap<i32, i32>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhantomReturnResponse {
    pub const MSG_ID: u16 = 20325;
}

impl PbMessage for PhantomReturnResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.ItemMap.is_empty() {
            crate::wire::write_map(out, 6, &self.ItemMap);
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
            6 => crate::wire::decode_map_entry(r, &mut self.ItemMap)?,
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
pub struct PhantomSettingRequest {}

impl PhantomSettingRequest {
    pub const MSG_ID: u16 = 24025;
}

impl PbMessage for PhantomSettingRequest {
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
pub struct PhantomSettingResponse {
    pub AutoDisuseSettings: Vec<OnePhantomSetting>,
    pub AutoLockSettings: Vec<OnePhantomSetting>,
}

impl PhantomSettingResponse {
    pub const MSG_ID: u16 = 29760;
}

impl PbMessage for PhantomSettingResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AutoDisuseSettings {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.AutoLockSettings {
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
                self.AutoDisuseSettings
                    .push(<OnePhantomSetting as PbMessage>::decode(bytes)?);
            }
            13 => {
                let bytes = r.read_len_delimited()?;
                self.AutoLockSettings
                    .push(<OnePhantomSetting as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSettingUpdateRequest {
    pub SettingType: Option<PhantomSettingType>,
    pub Setting: Option<OnePhantomSetting>,
}

impl PhantomSettingUpdateRequest {
    pub const MSG_ID: u16 = 27251;
}

impl PbMessage for PhantomSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.SettingType {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Setting {
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
            1 => {
                self.SettingType = Some(
                    <PhantomSettingType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.Setting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Setting: Option<OnePhantomSetting>,
}

impl PhantomSettingUpdateResponse {
    pub const MSG_ID: u16 = 25467;
}

impl PbMessage for PhantomSettingUpdateResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Setting {
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
                self.Setting
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomSkinAddNotify {
    pub SkinIds: Vec<i32>,
}

impl PhantomSkinAddNotify {
    pub const MSG_ID: u16 = 20095;
}

impl PbMessage for PhantomSkinAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.SkinIds {
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
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.SkinIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomUnlockNotify {
    pub UnlockMonsterItemList: Vec<i32>,
    pub UnlockQualityItemList: Vec<i32>,
}

impl PhantomUnlockNotify {
    pub const MSG_ID: u16 = 27009;
}

impl PbMessage for PhantomUnlockNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.UnlockMonsterItemList {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.UnlockQualityItemList {
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
            1 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockMonsterItemList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            3 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.UnlockQualityItemList,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhantomUpdateNotify {
    pub TotalCost: i32,
}

impl PhantomUpdateNotify {
    pub const MSG_ID: u16 = 17025;
}

impl PbMessage for PhantomUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TotalCost != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TotalCost);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            14 => self.TotalCost = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightActivityPb {
    pub ActivityId: i32,
    pub PhotoFightLevelGroupInfo: Vec<PhotoFightLevelGroupPb>,
    pub Targets: Vec<PhotoFightTargetPb>,
}

impl PbMessage for PhotoFightActivityPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
        }
        for item in &self.PhotoFightLevelGroupInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        for item in &self.Targets {
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.PhotoFightLevelGroupInfo
                    .push(<PhotoFightLevelGroupPb as PbMessage>::decode(bytes)?);
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Targets
                    .push(<PhotoFightTargetPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightCtx {
    pub ActivityId: i32,
    pub LevelId: i32,
}

impl PbMessage for PhotoFightCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ActivityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ActivityId);
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
            1 => self.ActivityId = r.read_varint_i32()?,
            2 => self.LevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightLevelGroupPb {
    pub GroupId: i32,
    pub OpenTime: i64,
    pub EndTime: i64,
    pub PhotoFightLevelInfo: Vec<PhotoFightLevelPb>,
}

impl PbMessage for PhotoFightLevelGroupPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if self.OpenTime != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpenTime);
        }
        if self.EndTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.EndTime);
        }
        for item in &self.PhotoFightLevelInfo {
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
            1 => self.GroupId = r.read_varint_i32()?,
            2 => self.OpenTime = r.read_varint_i64()?,
            3 => self.EndTime = r.read_varint_i64()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PhotoFightLevelInfo
                    .push(<PhotoFightLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightLevelInfoNotify {
    pub PhotoFightLevelInfos: Vec<PhotoFightLevelPb>,
}

impl PhotoFightLevelInfoNotify {
    pub const MSG_ID: u16 = 18787;
}

impl PbMessage for PhotoFightLevelInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PhotoFightLevelInfos {
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
                self.PhotoFightLevelInfos
                    .push(<PhotoFightLevelPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightLevelPb {
    pub LevelId: i32,
    pub InstId: i32,
    pub Roles: Vec<i32>,
    pub ClearTimes: i32,
    pub IsOpen: bool,
}

impl PbMessage for PhotoFightLevelPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.Roles {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.ClearTimes != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ClearTimes);
        }
        if self.IsOpen {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.InstId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Roles, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            4 => self.ClearTimes = r.read_varint_i32()?,
            5 => self.IsOpen = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightResultNotify {
    pub LevelId: i32,
    pub GroupId: i32,
    pub ErrorCode: Option<ErrorCode>,
    pub Succ: bool,
    pub Targets: Vec<PhotoFightTargetPb>,
    pub ActivityId: i32,
}

impl PhotoFightResultNotify {
    pub const MSG_ID: u16 = 17633;
}

impl PbMessage for PhotoFightResultNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.GroupId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GroupId);
        }
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.Succ {
            crate::wire::write_tag(out, 5, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.Targets {
            crate::wire::write_submessage(out, 9, item);
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.GroupId = r.read_varint_i32()?,
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => self.Succ = r.read_bool()?,
            9 => {
                let bytes = r.read_len_delimited()?;
                self.Targets
                    .push(<PhotoFightTargetPb as PbMessage>::decode(bytes)?);
            }
            13 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightSettleRequest {}

impl PhotoFightSettleRequest {
    pub const MSG_ID: u16 = 15995;
}

impl PbMessage for PhotoFightSettleRequest {
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
pub struct PhotoFightSettleResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PhotoFightSettleResponse {
    pub const MSG_ID: u16 = 26567;
}

impl PbMessage for PhotoFightSettleResponse {
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
pub struct PhotoFightTargetPb {
    pub Id: i32,
    pub Status: Option<ActivityTaskState>,
    pub PreItemMap: BTreeMap<i32, i32>,
}

impl PbMessage for PhotoFightTargetPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if let Some(v) = &self.Status {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            4 => {
                self.Status = Some(
                    <ActivityTaskState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            5 => crate::wire::decode_map_entry(r, &mut self.PreItemMap)?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightTaskRewardRequest {
    pub TargetIds: Vec<i32>,
    pub ActivityId: i32,
}

impl PhotoFightTaskRewardRequest {
    pub const MSG_ID: u16 = 24443;
}

impl PbMessage for PhotoFightTaskRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.TargetIds {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
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
            1 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.TargetIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            10 => self.ActivityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoFightTaskRewardResponse {
    pub Targets: Vec<PhotoFightTargetPb>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PhotoFightTaskRewardResponse {
    pub const MSG_ID: u16 = 20913;
}

impl PbMessage for PhotoFightTaskRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Targets {
            crate::wire::write_submessage(out, 5, item);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Targets
                    .push(<PhotoFightTargetPb as PbMessage>::decode(bytes)?);
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
pub struct PhotoMemoryCollectInfo {
    pub Id: i32,
    pub Flag: i32,
    pub CollectTime: i64,
}

impl PbMessage for PhotoMemoryCollectInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Flag != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Flag);
        }
        if self.CollectTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.CollectTime);
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
            2 => self.Flag = r.read_varint_i32()?,
            3 => self.CollectTime = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoMemoryCollectNotify {
    pub CollectInfo: Option<PhotoMemoryCollectInfo>,
}

impl PhotoMemoryCollectNotify {
    pub const MSG_ID: u16 = 21614;
}

impl PbMessage for PhotoMemoryCollectNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CollectInfo {
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
                self.CollectInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoMemoryRewardRequest {
    pub CollectIds: Vec<i32>,
}

impl PhotoMemoryRewardRequest {
    pub const MSG_ID: u16 = 27468;
}

impl PbMessage for PhotoMemoryRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.CollectIds {
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
            15 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.CollectIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoMemoryRewardResponse {
    pub Error: Option<ErrorCode>,
}

impl PhotoMemoryRewardResponse {
    pub const MSG_ID: u16 = 15832;
}

impl PbMessage for PhotoMemoryRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
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
pub struct PhotoMemoryTopicInfo {
    pub Id: i32,
    pub CollectInfos: Vec<PhotoMemoryCollectInfo>,
    pub IsUnlock: bool,
}

impl PbMessage for PhotoMemoryTopicInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        for item in &self.CollectInfos {
            crate::wire::write_submessage(out, 2, item);
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
            1 => self.Id = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.CollectInfos
                    .push(<PhotoMemoryCollectInfo as PbMessage>::decode(bytes)?);
            }
            3 => self.IsUnlock = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotoMemoryUpdateNotify {
    pub Topics: Vec<PhotoMemoryTopicInfo>,
}

impl PhotoMemoryUpdateNotify {
    pub const MSG_ID: u16 = 20573;
}

impl PbMessage for PhotoMemoryUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.Topics {
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
                self.Topics
                    .push(<PhotoMemoryTopicInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PhotographPush {}

impl PhotographPush {
    pub const MSG_ID: u16 = 27304;
}

impl PbMessage for PhotographPush {
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
pub struct PickUpDropItemRequest {
    pub DropEntityId: i64,
    pub ItemGetFlow: Option<SecItemGetFlowData>,
}

impl PickUpDropItemRequest {
    pub const MSG_ID: u16 = 29110;
}

impl PbMessage for PickUpDropItemRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DropEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DropEntityId);
        }
        if let Some(v) = &self.ItemGetFlow {
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
            2 => self.DropEntityId = r.read_varint_i64()?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.ItemGetFlow
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PickUpDropItemResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PickUpDropItemResponse {
    pub const MSG_ID: u16 = 24726;
}

impl PbMessage for PickUpDropItemResponse {
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
pub struct PilotThrowOperationRequest {
    pub OpType: Option<PilotThrowOperationType>,
}

impl PilotThrowOperationRequest {
    pub const MSG_ID: u16 = 20827;
}

impl PbMessage for PilotThrowOperationRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.OpType {
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
                self.OpType = Some(
                    <PilotThrowOperationType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PilotThrowOperationResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PilotThrowOperationResponse {
    pub const MSG_ID: u16 = 28660;
}

impl PbMessage for PilotThrowOperationResponse {
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
pub struct PinballBonusFullWinRequest {
    pub Payload: Option<PinballSettlePayload>,
}

impl PinballBonusFullWinRequest {
    pub const MSG_ID: u16 = 19347;
}

impl PbMessage for PinballBonusFullWinRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Payload {
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
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballBonusFullWinResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl PinballBonusFullWinResponse {
    pub const MSG_ID: u16 = 16828;
}

impl PbMessage for PinballBonusFullWinResponse {
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
pub struct PinballCtx {
    pub LevelId: i32,
    pub FormationRoles: Vec<i32>,
}

impl PbMessage for PinballCtx {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.FormationRoles {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.FormationRoles, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballEntityKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for PinballEntityKilledCtxPb {
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
pub struct PinballFixSpawnFinishNotify {
    pub SpawnId: i32,
    pub WaveId: i32,
}

impl PinballFixSpawnFinishNotify {
    pub const MSG_ID: u16 = 22699;
}

impl PbMessage for PinballFixSpawnFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SpawnId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpawnId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
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
            12 => self.SpawnId = r.read_varint_i32()?,
            14 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballFormationRolePb {
    pub RoleId: i32,
    pub Trial: bool,
    pub LevelId: i32,
    pub HasWeapon: bool,
    pub WeaponId: i32,
    pub RandomAffixIds: Vec<i32>,
}

impl PbMessage for PinballFormationRolePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Trial {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.HasWeapon {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.WeaponId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponId);
        }
        for item in &self.RandomAffixIds {
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
            1 => self.RoleId = r.read_varint_i32()?,
            2 => self.Trial = r.read_bool()?,
            3 => self.LevelId = r.read_varint_i32()?,
            4 => self.HasWeapon = r.read_bool()?,
            6 => self.WeaponId = r.read_varint_i32()?,
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.RandomAffixIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballFriendRankRequest {}

impl PinballFriendRankRequest {
    pub const MSG_ID: u16 = 27697;
}

impl PbMessage for PinballFriendRankRequest {
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
pub struct PinballFriendRankResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub RankData: Vec<PinballRankData>,
    pub PlayerSelf: Option<PinballRankData>,
}

impl PinballFriendRankResponse {
    pub const MSG_ID: u16 = 20019;
}

impl PbMessage for PinballFriendRankResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        for item in &self.RankData {
            crate::wire::write_submessage(out, 5, item);
        }
        if let Some(v) = &self.PlayerSelf {
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
            4 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.RankData
                    .push(<PinballRankData as PbMessage>::decode(bytes)?);
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerSelf
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballInGameKickBallRequest {}

impl PinballInGameKickBallRequest {
    pub const MSG_ID: u16 = 22423;
}

impl PbMessage for PinballInGameKickBallRequest {
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
pub struct PinballInGameKickBallResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballInGameKickBallResponse {
    pub const MSG_ID: u16 = 23854;
}

impl PbMessage for PinballInGameKickBallResponse {
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
pub struct PinballInGameStarChangeNotify {
    pub LevelId: i32,
    pub StarInfo: Vec<i32>,
}

impl PinballInGameStarChangeNotify {
    pub const MSG_ID: u16 = 22124;
}

impl PbMessage for PinballInGameStarChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.StarInfo {
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
            2 => self.LevelId = r.read_varint_i32()?,
            3 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StarInfo, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballInGameStateChangeNotify {
    pub InGameState: Option<PinballInGameStatePb>,
    pub SettlePreSubmitPb: Option<PinballSettlePreSubmitPb>,
    pub InProgressPb: Option<PinballInProgressPb>,
    pub ChangeReason: Option<PinballStateChangeReasonPb>,
}

impl PinballInGameStateChangeNotify {
    pub const MSG_ID: u16 = 20926;
}

impl PbMessage for PinballInGameStateChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InGameState {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.SettlePreSubmitPb {
            crate::wire::write_submessage(out, 11, v);
        }
        if let Some(v) = &self.InProgressPb {
            crate::wire::write_submessage(out, 12, v);
        }
        if let Some(v) = &self.ChangeReason {
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
                self.InGameState = Some(
                    <PinballInGameStatePb as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.SettlePreSubmitPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => {
                let bytes = r.read_len_delimited()?;
                self.InProgressPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                self.ChangeReason = Some(
                    <PinballStateChangeReasonPb as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PinballInGameSyncPayloadRequest {
    pub Payload: Option<PinballSettlePayload>,
}

impl PinballInGameSyncPayloadRequest {
    pub const MSG_ID: u16 = 18030;
}

impl PbMessage for PinballInGameSyncPayloadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Payload {
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
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballInGameSyncPayloadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballInGameSyncPayloadResponse {
    pub const MSG_ID: u16 = 24557;
}

impl PbMessage for PinballInGameSyncPayloadResponse {
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
pub struct PinballInProgressPb {
    pub LevelId: i32,
    pub ReviveTimesCost: i32,
    pub MaxReviveTimes: i32,
    pub PinballFormation: Vec<PinballFormationRolePb>,
    pub StarInfo: Vec<i32>,
    pub MonsterAttrRate: Vec<i32>,
    pub AllyKscBuffIds: Vec<i32>,
}

impl PbMessage for PinballInProgressPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        if self.ReviveTimesCost != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveTimesCost);
        }
        if self.MaxReviveTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReviveTimes);
        }
        for item in &self.PinballFormation {
            crate::wire::write_submessage(out, 4, item);
        }
        for item in &self.StarInfo {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.MonsterAttrRate {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.AllyKscBuffIds {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => self.ReviveTimesCost = r.read_varint_i32()?,
            3 => self.MaxReviveTimes = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.PinballFormation
                    .push(<PinballFormationRolePb as PbMessage>::decode(bytes)?);
            }
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StarInfo, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            6 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.MonsterAttrRate,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            7 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.AllyKscBuffIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballMonsterKilledCtxPb {
    pub EntityId: i64,
}

impl PbMessage for PinballMonsterKilledCtxPb {
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
pub struct PinballQuitRequest {
    pub NeedNotify: bool,
    pub Payload: Option<PinballSettlePayload>,
}

impl PinballQuitRequest {
    pub const MSG_ID: u16 = 23709;
}

impl PbMessage for PinballQuitRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NeedNotify {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Payload {
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
            3 => self.NeedNotify = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballQuitResponse {
    pub ErrCode: Option<ErrorCode>,
}

impl PinballQuitResponse {
    pub const MSG_ID: u16 = 21377;
}

impl PbMessage for PinballQuitResponse {
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
pub struct PinballRankData {
    pub Name: Vec<u8>,
    pub TowerLevel: i32,
    pub CostTime: i32,
    pub Formation: Vec<PinballRoleData>,
}

impl PbMessage for PinballRankData {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 1, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.TowerLevel != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TowerLevel);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostTime);
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
            1 => self.Name = r.read_len_delimited()?.to_vec(),
            2 => self.TowerLevel = r.read_varint_i32()?,
            3 => self.CostTime = r.read_varint_i32()?,
            4 => {
                let bytes = r.read_len_delimited()?;
                self.Formation
                    .push(<PinballRoleData as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballResult {
    pub LevelId: i32,
    pub RolePbs: Vec<PinballResultRolePb>,
    pub MvpRoleId: i32,
    pub StarInfo: Vec<i32>,
    pub Score: i32,
    pub CostTime: i32,
    pub FirstPass: bool,
}

impl PbMessage for PinballResult {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelId);
        }
        for item in &self.RolePbs {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.MvpRoleId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MvpRoleId);
        }
        for item in &self.StarInfo {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.CostTime != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CostTime);
        }
        if self.FirstPass {
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
            1 => self.LevelId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RolePbs
                    .push(<PinballResultRolePb as PbMessage>::decode(bytes)?);
            }
            3 => self.MvpRoleId = r.read_varint_i32()?,
            4 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.StarInfo, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            5 => self.Score = r.read_varint_i32()?,
            6 => self.CostTime = r.read_varint_i32()?,
            7 => self.FirstPass = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballResultRolePb {
    pub RoleId: i32,
    pub Damage: i64,
    pub SkillTimes: i32,
    pub DropCostHp: i64,
    pub HitCostHp: i64,
    pub TempDieTimes: i32,
    pub TempReviveTimes: i32,
}

impl PbMessage for PinballResultRolePb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.Damage != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Damage);
        }
        if self.SkillTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SkillTimes);
        }
        if self.DropCostHp != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.DropCostHp);
        }
        if self.HitCostHp != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.HitCostHp);
        }
        if self.TempDieTimes != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TempDieTimes);
        }
        if self.TempReviveTimes != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TempReviveTimes);
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
            2 => self.Damage = r.read_varint_i64()?,
            3 => self.SkillTimes = r.read_varint_i32()?,
            4 => self.DropCostHp = r.read_varint_i64()?,
            5 => self.HitCostHp = r.read_varint_i64()?,
            6 => self.TempDieTimes = r.read_varint_i32()?,
            7 => self.TempReviveTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballRoleLevelUpRequest {
    pub RoleId: i32,
    pub AddLv: i32,
}

impl PinballRoleLevelUpRequest {
    pub const MSG_ID: u16 = 20427;
}

impl PbMessage for PinballRoleLevelUpRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.AddLv != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AddLv);
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
            15 => self.AddLv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballRoleLevelUpResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballRoleLevelUpResponse {
    pub const MSG_ID: u16 = 29568;
}

impl PbMessage for PinballRoleLevelUpResponse {
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
pub struct PinballSetPayloadRequest {
    pub Payload: Option<PinballSettlePayload>,
}

impl PinballSetPayloadRequest {
    pub const MSG_ID: u16 = 23171;
}

impl PbMessage for PinballSetPayloadRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Payload {
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
                self.Payload
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballSetPayloadResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballSetPayloadResponse {
    pub const MSG_ID: u16 = 26267;
}

impl PbMessage for PinballSetPayloadResponse {
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
pub struct PinballSettleNotify {
    pub Win: bool,
    pub Result: Option<PinballResult>,
}

impl PinballSettleNotify {
    pub const MSG_ID: u16 = 24613;
}

impl PbMessage for PinballSettleNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Win {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if let Some(v) = &self.Result {
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
            4 => self.Win = r.read_bool()?,
            12 => {
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
pub struct PinballSettlePayload {
    pub RolePbs: Vec<PinballResultRolePb>,
    pub Score: i32,
    pub DashTimes: i32,
    pub BossSkillTimes: i32,
    pub FullEnergyTimes: i32,
    pub AttackTimes: i32,
}

impl PbMessage for PinballSettlePayload {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RolePbs {
            crate::wire::write_submessage(out, 1, item);
        }
        if self.Score != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Score);
        }
        if self.DashTimes != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DashTimes);
        }
        if self.BossSkillTimes != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.BossSkillTimes);
        }
        if self.FullEnergyTimes != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FullEnergyTimes);
        }
        if self.AttackTimes != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AttackTimes);
        }
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
                self.RolePbs
                    .push(<PinballResultRolePb as PbMessage>::decode(bytes)?);
            }
            2 => self.Score = r.read_varint_i32()?,
            3 => self.DashTimes = r.read_varint_i32()?,
            4 => self.BossSkillTimes = r.read_varint_i32()?,
            5 => self.FullEnergyTimes = r.read_varint_i32()?,
            6 => self.AttackTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballSettlePreSubmitPb {
    pub ReviveTimesCost: i32,
    pub MaxReviveTimes: i32,
}

impl PbMessage for PinballSettlePreSubmitPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ReviveTimesCost != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveTimesCost);
        }
        if self.MaxReviveTimes != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxReviveTimes);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            3 => self.ReviveTimesCost = r.read_varint_i32()?,
            4 => self.MaxReviveTimes = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballSpawnFirstFinishNotify {
    pub SpawnId: i32,
    pub WaveId: i32,
}

impl PinballSpawnFirstFinishNotify {
    pub const MSG_ID: u16 = 26869;
}

impl PbMessage for PinballSpawnFirstFinishNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SpawnId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.SpawnId);
        }
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
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
            3 => self.SpawnId = r.read_varint_i32()?,
            9 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballStartNotify {
    pub InProgressPb: Option<PinballInProgressPb>,
    pub RealLevelId: i32,
}

impl PinballStartNotify {
    pub const MSG_ID: u16 = 29654;
}

impl PbMessage for PinballStartNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.InProgressPb {
            crate::wire::write_submessage(out, 7, v);
        }
        if self.RealLevelId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RealLevelId);
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
                self.InProgressPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            13 => self.RealLevelId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballTaskRewardRequest {
    pub TaskIds: Vec<i32>,
}

impl PinballTaskRewardRequest {
    pub const MSG_ID: u16 = 17084;
}

impl PbMessage for PinballTaskRewardRequest {
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
pub struct PinballTaskRewardResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballTaskRewardResponse {
    pub const MSG_ID: u16 = 16536;
}

impl PbMessage for PinballTaskRewardResponse {
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
pub struct PinballTeamDieRequest {}

impl PinballTeamDieRequest {
    pub const MSG_ID: u16 = 15247;
}

impl PbMessage for PinballTeamDieRequest {
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
pub struct PinballTeamDieResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballTeamDieResponse {
    pub const MSG_ID: u16 = 15684;
}

impl PbMessage for PinballTeamDieResponse {
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
pub struct PinballTeamReviveRequest {}

impl PinballTeamReviveRequest {
    pub const MSG_ID: u16 = 26619;
}

impl PbMessage for PinballTeamReviveRequest {
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
pub struct PinballTeamReviveResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballTeamReviveResponse {
    pub const MSG_ID: u16 = 15477;
}

impl PbMessage for PinballTeamReviveResponse {
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
pub struct PinballUpdateNotify {
    pub NewRole: Option<PinballRoles>,
    pub GroupFormation: Option<GroupFormation>,
    pub NewWeapon: Option<PinballWeapons>,
    pub ConditionTask: Option<ConditionTask>,
    pub LevelData: Option<PinballLevelData>,
}

impl PinballUpdateNotify {
    pub const MSG_ID: u16 = 19395;
}

impl PbMessage for PinballUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.NewRole {
            crate::wire::write_submessage(out, 2, v);
        }
        if let Some(v) = &self.GroupFormation {
            crate::wire::write_submessage(out, 4, v);
        }
        if let Some(v) = &self.NewWeapon {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.ConditionTask {
            crate::wire::write_submessage(out, 7, v);
        }
        if let Some(v) = &self.LevelData {
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
                self.NewRole
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GroupFormation
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => {
                let bytes = r.read_len_delimited()?;
                self.NewWeapon
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            14 => {
                let bytes = r.read_len_delimited()?;
                self.LevelData
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballUpdateWaveNotify {
    pub WaveId: i32,
}

impl PinballUpdateWaveNotify {
    pub const MSG_ID: u16 = 23526;
}

impl PbMessage for PinballUpdateWaveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.WaveId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
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
            8 => self.WaveId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballWeaponDegradeRequest {
    pub WeaponIncIds: Vec<i32>,
}

impl PinballWeaponDegradeRequest {
    pub const MSG_ID: u16 = 15550;
}

impl PbMessage for PinballWeaponDegradeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.WeaponIncIds {
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
            5 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.WeaponIncIds, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballWeaponDegradeResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballWeaponDegradeResponse {
    pub const MSG_ID: u16 = 24456;
}

impl PbMessage for PinballWeaponDegradeResponse {
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
pub struct PinballWeaponWearRequest {
    pub RoleId: i32,
    pub WeaponInc: i32,
}

impl PinballWeaponWearRequest {
    pub const MSG_ID: u16 = 22578;
}

impl PbMessage for PinballWeaponWearRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RoleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RoleId);
        }
        if self.WeaponInc != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.WeaponInc);
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
            8 => self.WeaponInc = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PinballWeaponWearResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PinballWeaponWearResponse {
    pub const MSG_ID: u16 = 17279;
}

impl PbMessage for PinballWeaponWearResponse {
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
pub struct PlaceItemOnBoardNotify {
    pub PosOnBoard: Option<BoardGridPositionInfo>,
    pub TargetBoardEntityId: i64,
    pub PlacementEntityId: i64,
    pub ToPlace: i32,
}

impl PlaceItemOnBoardNotify {
    pub const MSG_ID: u16 = 15283;
}

impl PbMessage for PlaceItemOnBoardNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PosOnBoard {
            crate::wire::write_submessage(out, 5, v);
        }
        if self.TargetBoardEntityId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetBoardEntityId);
        }
        if self.PlacementEntityId != 0 {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlacementEntityId);
        }
        if self.ToPlace != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToPlace);
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
                self.PosOnBoard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            6 => self.TargetBoardEntityId = r.read_varint_i64()?,
            14 => self.PlacementEntityId = r.read_varint_i64()?,
            15 => self.ToPlace = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlaceItemOnBoardRequest {
    pub TargetBoardEntityId: i64,
    pub PosRot: Option<PositionAndRotate>,
    pub ToPlace: i32,
    pub PosOnBoard: Option<BoardGridPositionInfo>,
    pub PlacementEntityId: i64,
}

impl PlaceItemOnBoardRequest {
    pub const MSG_ID: u16 = 23598;
}

impl PbMessage for PlaceItemOnBoardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.TargetBoardEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.TargetBoardEntityId);
        }
        if let Some(v) = &self.PosRot {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.ToPlace != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToPlace);
        }
        if let Some(v) = &self.PosOnBoard {
            crate::wire::write_submessage(out, 13, v);
        }
        if self.PlacementEntityId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlacementEntityId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.TargetBoardEntityId = r.read_varint_i64()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.PosRot
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            12 => self.ToPlace = r.read_varint_i32()?,
            13 => {
                let bytes = r.read_len_delimited()?;
                self.PosOnBoard
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.PlacementEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlaceItemOnBoardResponse {
    pub ToPlace: i32,
    pub ErrCode: Option<ErrorCode>,
}

impl PlaceItemOnBoardResponse {
    pub const MSG_ID: u16 = 22601;
}

impl PbMessage for PlaceItemOnBoardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ToPlace != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ToPlace);
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
            2 => self.ToPlace = r.read_varint_i32()?,
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
pub struct PlacementInfoChangeInfo {
    pub PlacementItemPb: Option<PlacementItemPb>,
    pub PlacementEntityId: i64,
}

impl PbMessage for PlacementInfoChangeInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlacementItemPb {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.PlacementEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.PlacementEntityId);
        }
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
                self.PlacementItemPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.PlacementEntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlacementInfoChangeNotify {
    pub ChangedList: Vec<PlacementInfoChangeInfo>,
}

impl PlacementInfoChangeNotify {
    pub const MSG_ID: u16 = 19545;
}

impl PbMessage for PlacementInfoChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.ChangedList {
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
                self.ChangedList
                    .push(<PlacementInfoChangeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayEntityMontagePush {
    pub StartTimeSeconds: f32,
    pub MontageName: Vec<u8>,
    pub StartSection: Vec<u8>,
    pub MontageHashCode: i32,
    pub SpeedRatio: f32,
}

impl PlayEntityMontagePush {
    pub const MSG_ID: u16 = 24591;
}

impl PbMessage for PlayEntityMontagePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.StartTimeSeconds != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.StartTimeSeconds).to_bits().to_le_bytes());
        }
        if !self.MontageName.is_empty() {
            crate::wire::write_tag(out, 4, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MontageName);
        }
        if !self.StartSection.is_empty() {
            crate::wire::write_tag(out, 5, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.StartSection);
        }
        if self.MontageHashCode != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MontageHashCode);
        }
        if self.SpeedRatio != 0.0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.SpeedRatio).to_bits().to_le_bytes());
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.StartTimeSeconds = r.read_f32()?,
            4 => self.MontageName = r.read_len_delimited()?.to_vec(),
            5 => self.StartSection = r.read_len_delimited()?.to_vec(),
            10 => self.MontageHashCode = r.read_varint_i32()?,
            13 => self.SpeedRatio = r.read_f32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayInstShareFirstInfo {
    pub LevelPlayId: i32,
    pub InstId: i32,
}

impl PbMessage for PlayInstShareFirstInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.LevelPlayId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.LevelPlayId);
        }
        if self.InstId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
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
            1 => self.LevelPlayId = r.read_varint_i32()?,
            2 => self.InstId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayInstShareFirstInfoNotify {
    pub FirstInfo: Vec<PlayInstShareFirstInfo>,
}

impl PlayInstShareFirstInfoNotify {
    pub const MSG_ID: u16 = 22822;
}

impl PbMessage for PlayInstShareFirstInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.FirstInfo {
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
                self.FirstInfo
                    .push(<PlayInstShareFirstInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessCasketRequest {
    pub IsEnter: bool,
    pub AccessEntityIncIds: Vec<i64>,
    pub CasketEntityIncId: i64,
}

impl PlayerAccessCasketRequest {
    pub const MSG_ID: u16 = 19061;
}

impl PbMessage for PlayerAccessCasketRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.IsEnter {
            crate::wire::write_tag(out, 1, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.AccessEntityIncIds {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, *item);
        }
        if self.CasketEntityIncId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
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
            1 => self.IsEnter = r.read_bool()?,
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AccessEntityIncIds,
                |sr| Ok(sr.read_varint_i64()?),
            )?,
            15 => self.CasketEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessCasketResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub CasketEntityIncId: i64,
}

impl PlayerAccessCasketResponse {
    pub const MSG_ID: u16 = 18900;
}

impl PbMessage for PlayerAccessCasketResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.CasketEntityIncId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            1 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => self.CasketEntityIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessRangeNotify {
    pub RangeEntiyId: i64,
    pub AccessRangeType: Option<AccessRangeType>,
    pub AccessPlayerId: i32,
}

impl PlayerAccessRangeNotify {
    pub const MSG_ID: u16 = 24586;
}

impl PbMessage for PlayerAccessRangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RangeEntiyId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RangeEntiyId);
        }
        if let Some(v) = &self.AccessRangeType {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.AccessPlayerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.AccessPlayerId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.RangeEntiyId = r.read_varint_i64()?,
            11 => {
                self.AccessRangeType = Some(
                    <AccessRangeType as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            12 => self.AccessPlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerAccessRangeRequest {
    pub RangeEntityId: i64,
    pub AccessRangeType: Option<AccessRangeType>,
}

impl PlayerAccessRangeRequest {
    pub const MSG_ID: u16 = 20505;
}

impl PbMessage for PlayerAccessRangeRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RangeEntityId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RangeEntityId);
        }
        if let Some(v) = &self.AccessRangeType {
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
            2 => self.RangeEntityId = r.read_varint_i64()?,
            9 => {
                self.AccessRangeType = Some(
                    <AccessRangeType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PlayerAccessRangeResponse {
    pub AccessRangeResult: Option<AccessRangeResultMessage>,
    pub RangeEntityId: i64,
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerAccessRangeResponse {
    pub const MSG_ID: u16 = 29671;
}

impl PbMessage for PlayerAccessRangeResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.AccessRangeResult {
            crate::wire::write_submessage(out, 1, v);
        }
        if self.RangeEntityId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.RangeEntityId);
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
                self.AccessRangeResult
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.RangeEntityId = r.read_varint_i64()?,
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
pub struct PlayerAoiRangeNotify {
    pub MaxY: i32,
    pub MinY: i32,
    pub MinX: i32,
    pub MaxX: i32,
}

impl PlayerAoiRangeNotify {
    pub const MSG_ID: u16 = 17622;
}

impl PbMessage for PlayerAoiRangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MaxY != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxY);
        }
        if self.MinY != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinY);
        }
        if self.MinX != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MinX);
        }
        if self.MaxX != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MaxX);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            1 => self.MaxY = r.read_varint_i32()?,
            3 => self.MinY = r.read_varint_i32()?,
            9 => self.MinX = r.read_varint_i32()?,
            10 => self.MaxX = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerCreatePsnSessionRequest {
    pub PsnSessionId: Vec<u8>,
}

impl PlayerCreatePsnSessionRequest {
    pub const MSG_ID: u16 = 21404;
}

impl PbMessage for PlayerCreatePsnSessionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PsnSessionId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnSessionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.PsnSessionId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerCreatePsnSessionResponse {}

impl PlayerCreatePsnSessionResponse {
    pub const MSG_ID: u16 = 16742;
}

impl PbMessage for PlayerCreatePsnSessionResponse {
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
pub struct PlayerCreateXboxSessionRequest {
    pub XboxSessionId: Vec<u8>,
}

impl PlayerCreateXboxSessionRequest {
    pub const MSG_ID: u16 = 19073;
}

impl PbMessage for PlayerCreateXboxSessionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.XboxSessionId.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxSessionId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.XboxSessionId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerCreateXboxSessionResponse {}

impl PlayerCreateXboxSessionResponse {
    pub const MSG_ID: u16 = 17939;
}

impl PbMessage for PlayerCreateXboxSessionResponse {
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
pub struct PlayerEnterPollutionRequest {
    pub ManagerEntityId: i64,
    pub RangeType: Option<PlayEnterOrExitPollutionRangeType>,
}

impl PlayerEnterPollutionRequest {
    pub const MSG_ID: u16 = 27660;
}

impl PbMessage for PlayerEnterPollutionRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.ManagerEntityId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.ManagerEntityId);
        }
        if let Some(v) = &self.RangeType {
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
            1 => self.ManagerEntityId = r.read_varint_i64()?,
            13 => {
                self.RangeType = Some(
                    <PlayEnterOrExitPollutionRangeType as crate::wire::PbEnum>::from_i32_or_default(
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
pub struct PlayerEnterPollutionResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerEnterPollutionResponse {
    pub const MSG_ID: u16 = 20980;
}

impl PbMessage for PlayerEnterPollutionResponse {
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
pub struct PlayerEnterWorldTeamNotify {
    pub PlayerInfo: Option<WorldTeamPlayerInfo>,
}

impl PlayerEnterWorldTeamNotify {
    pub const MSG_ID: u16 = 21617;
}

impl PbMessage for PlayerEnterWorldTeamNotify {
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
pub struct PlayerFightRolesInfo {
    pub PlayerId: i32,
    pub FightRolesInfo: Vec<FightRoleInformation>,
    pub IsPhantom: bool,
    pub IsRetain: bool,
}

impl PbMessage for PlayerFightRolesInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.FightRolesInfo {
            crate::wire::write_submessage(out, 2, item);
        }
        if self.IsPhantom {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsRetain {
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
            1 => self.PlayerId = r.read_varint_i32()?,
            2 => {
                let bytes = r.read_len_delimited()?;
                self.FightRolesInfo
                    .push(<FightRoleInformation as PbMessage>::decode(bytes)?);
            }
            3 => self.IsPhantom = r.read_bool()?,
            4 => self.IsRetain = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerGameTokenRequest {}

impl PlayerGameTokenRequest {
    pub const MSG_ID: u16 = 18839;
}

impl PbMessage for PlayerGameTokenRequest {
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
pub struct PlayerGameTokenResponse {
    pub GameToken: Vec<u8>,
}

impl PlayerGameTokenResponse {
    pub const MSG_ID: u16 = 26225;
}

impl PbMessage for PlayerGameTokenResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.GameToken.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.GameToken);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            9 => self.GameToken = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerGravityUpdateNotify {
    pub PreGravityDirection: Option<Vector>,
    pub CurGravityDirection: Option<Vector>,
}

impl PlayerGravityUpdateNotify {
    pub const MSG_ID: u16 = 23394;
}

impl PbMessage for PlayerGravityUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PreGravityDirection {
            crate::wire::write_submessage(out, 14, v);
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
            14 => {
                let bytes = r.read_len_delimited()?;
                self.PreGravityDirection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
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
pub struct PlayerHandFoundationCoreChangeNotify {
    pub FoundationCoreInfo: Option<FoundationCoreInfo>,
    pub FromFoundationEntityId: i32,
}

impl PlayerHandFoundationCoreChangeNotify {
    pub const MSG_ID: u16 = 29717;
}

impl PbMessage for PlayerHandFoundationCoreChangeNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.FoundationCoreInfo {
            crate::wire::write_submessage(out, 3, v);
        }
        if self.FromFoundationEntityId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.FromFoundationEntityId);
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
            13 => self.FromFoundationEntityId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerHeadAddNotify {
    pub AddPlayerHeadIds: Vec<i32>,
}

impl PlayerHeadAddNotify {
    pub const MSG_ID: u16 = 29216;
}

impl PbMessage for PlayerHeadAddNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.AddPlayerHeadIds {
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
            13 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.AddPlayerHeadIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerHeadRequest {}

impl PlayerHeadRequest {
    pub const MSG_ID: u16 = 18199;
}

impl PbMessage for PlayerHeadRequest {
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
pub struct PlayerHeadResponse {
    pub PlayerHeadIds: Vec<i32>,
}

impl PlayerHeadResponse {
    pub const MSG_ID: u16 = 22413;
}

impl PbMessage for PlayerHeadResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerHeadIds {
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
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PlayerHeadIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerHeadUpdateNotify {
    pub PlayerHeadIds: Vec<i32>,
}

impl PlayerHeadUpdateNotify {
    pub const MSG_ID: u16 = 26848;
}

impl PbMessage for PlayerHeadUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerHeadIds {
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
            9 => {
                crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PlayerHeadIds, |sr| {
                    Ok(sr.read_varint_i32()?)
                })?
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerLeaveWorldTeamNotify {
    pub PlayerId: i32,
    pub Reason: Option<WorldTeamLeaveReason>,
}

impl PlayerLeaveWorldTeamNotify {
    pub const MSG_ID: u16 = 21585;
}

impl PbMessage for PlayerLeaveWorldTeamNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 12, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
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
            12 => self.PlayerId = r.read_varint_i32()?,
            15 => {
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
pub struct PlayerLivingStatesNotify {
    pub PlayerId: i32,
    pub LivingList: Vec<LivingState>,
}

impl PlayerLivingStatesNotify {
    pub const MSG_ID: u16 = 26412;
}

impl PbMessage for PlayerLivingStatesNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        for item in &self.LivingList {
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
            3 => self.PlayerId = r.read_varint_i32()?,
            7 => {
                let bytes = r.read_len_delimited()?;
                self.LivingList
                    .push(<LivingState as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerLocation {
    pub MapId: i32,
    pub PlayerId: i32,
    pub Location: Option<Vector>,
    pub GravityDirection: Option<Vector>,
    pub AreaId: i32,
}

impl PbMessage for PlayerLocation {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MapId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.MapId);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 3, v);
        }
        if let Some(v) = &self.GravityDirection {
            crate::wire::write_submessage(out, 4, v);
        }
        if self.AreaId != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
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
            1 => self.MapId = r.read_varint_i32()?,
            2 => self.PlayerId = r.read_varint_i32()?,
            3 => {
                let bytes = r.read_len_delimited()?;
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            4 => {
                let bytes = r.read_len_delimited()?;
                self.GravityDirection
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            5 => self.AreaId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerNetStateBackServerPush {
    pub Uid: i64,
}

impl PlayerNetStateBackServerPush {
    pub const MSG_ID: u16 = 19999;
}

impl PbMessage for PlayerNetStateBackServerPush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Uid != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Uid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Uid = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerNetStateNotify {
    pub PingState: Option<ENetPingState>,
    pub PlayerId: i32,
}

impl PlayerNetStateNotify {
    pub const MSG_ID: u16 = 25067;
}

impl PbMessage for PlayerNetStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PingState {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
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
            5 => {
                self.PingState = Some(<ENetPingState as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            12 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerNetStatePush {
    pub RttMs: i32,
}

impl PlayerNetStatePush {
    pub const MSG_ID: u16 = 24558;
}

impl PbMessage for PlayerNetStatePush {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.RttMs != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RttMs);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.RttMs = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerNetStateS2cNotify {
    pub Uid: i64,
}

impl PlayerNetStateS2cNotify {
    pub const MSG_ID: u16 = 18103;
}

impl PbMessage for PlayerNetStateS2cNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Uid != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.Uid);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.Uid = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerPsnBlockListRequest {
    pub PsnAccountIds: Vec<Vec<u8>>,
}

impl PlayerPsnBlockListRequest {
    pub const MSG_ID: u16 = 16257;
}

impl PbMessage for PlayerPsnBlockListRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PsnAccountIds {
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
            15 => self.PsnAccountIds.push(r.read_len_delimited()?.to_vec()),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerPsnBlockListResponse {}

impl PlayerPsnBlockListResponse {
    pub const MSG_ID: u16 = 17327;
}

impl PbMessage for PlayerPsnBlockListResponse {
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
pub struct PlayerPsnInfoGetRequest {
    pub PsnOnlineId: Vec<u8>,
}

impl PlayerPsnInfoGetRequest {
    pub const MSG_ID: u16 = 29956;
}

impl PbMessage for PlayerPsnInfoGetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.PsnOnlineId.is_empty() {
            crate::wire::write_tag(out, 13, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnOnlineId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            13 => self.PsnOnlineId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerPsnInfoGetResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub Info: Option<PlayerDetails>,
}

impl PlayerPsnInfoGetResponse {
    pub const MSG_ID: u16 = 21086;
}

impl PbMessage for PlayerPsnInfoGetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.Info {
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
            10 => {
                self.ErrorCode = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            13 => {
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
pub struct PlayerPsnSessionNotify {
    pub PlayerId: i32,
    pub PsnSessionId: Vec<u8>,
}

impl PlayerPsnSessionNotify {
    pub const MSG_ID: u16 = 15107;
}

impl PbMessage for PlayerPsnSessionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.PsnSessionId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.PsnSessionId);
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
            10 => self.PsnSessionId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerRecallInfoNotify {
    pub CurRecallId: i32,
}

impl PlayerRecallInfoNotify {
    pub const MSG_ID: u16 = 20622;
}

impl PbMessage for PlayerRecallInfoNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.CurRecallId != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.CurRecallId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.CurRecallId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerRenameNotify {
    pub Name: Vec<u8>,
}

impl PlayerRenameNotify {
    pub const MSG_ID: u16 = 16634;
}

impl PbMessage for PlayerRenameNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 3, WireType::LengthDelimited);
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
            3 => self.Name = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerReviveNotify {
    pub Location: Option<Vector>,
    pub RollbackSubLevel: Vec<Vec<u8>>,
    pub NeedLoading: bool,
    pub ReviveRoleInfos: Vec<ReviveRoleInformation>,
    pub RollbackHideSubLevel: Vec<Vec<u8>>,
    pub NeedRollbackSubLevel: bool,
    pub PlayerId: i32,
    pub Rotator: Option<Rotator>,
    pub ReviveType: i32,
    pub Gravity: Option<Vector>,
    pub TeleportId: i32,
}

impl PlayerReviveNotify {
    pub const MSG_ID: u16 = 22493;
}

impl PbMessage for PlayerReviveNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Location {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.RollbackSubLevel {
            crate::wire::write_tag(out, 2, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.NeedLoading {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        for item in &self.ReviveRoleInfos {
            crate::wire::write_submessage(out, 5, item);
        }
        for item in &self.RollbackHideSubLevel {
            crate::wire::write_tag(out, 6, crate::wire::WireType::LengthDelimited);
            crate::wire::write_bytes(out, item);
        }
        if self.NeedRollbackSubLevel {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if let Some(v) = &self.Rotator {
            crate::wire::write_submessage(out, 10, v);
        }
        if self.ReviveType != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ReviveType);
        }
        if let Some(v) = &self.Gravity {
            crate::wire::write_submessage(out, 14, v);
        }
        if self.TeleportId != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TeleportId);
        }
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
                self.Location
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => self.RollbackSubLevel.push(r.read_len_delimited()?.to_vec()),
            3 => self.NeedLoading = r.read_bool()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ReviveRoleInfos
                    .push(<ReviveRoleInformation as PbMessage>::decode(bytes)?);
            }
            6 => self
                .RollbackHideSubLevel
                .push(r.read_len_delimited()?.to_vec()),
            7 => self.NeedRollbackSubLevel = r.read_bool()?,
            8 => self.PlayerId = r.read_varint_i32()?,
            10 => {
                let bytes = r.read_len_delimited()?;
                self.Rotator
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            11 => self.ReviveType = r.read_varint_i32()?,
            14 => {
                let bytes = r.read_len_delimited()?;
                self.Gravity
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            15 => self.TeleportId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSettleInfo {
    pub PlayerId: i32,
    pub Name: Vec<u8>,
    pub HeadId: i32,
    pub Ranking: i32,
    pub ConsumeTime: i32,
    pub Finish: bool,
    pub GoldPoints: i32,
    pub RankPoints: i32,
    pub DistancePoints: i32,
}

impl PbMessage for PlayerSettleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.Name.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Name);
        }
        if self.HeadId != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.HeadId);
        }
        if self.Ranking != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Ranking);
        }
        if self.ConsumeTime != 0 {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ConsumeTime);
        }
        if self.Finish {
            crate::wire::write_tag(out, 6, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.GoldPoints != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.GoldPoints);
        }
        if self.RankPoints != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.RankPoints);
        }
        if self.DistancePoints != 0 {
            crate::wire::write_tag(out, 9, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.DistancePoints);
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
            3 => self.HeadId = r.read_varint_i32()?,
            4 => self.Ranking = r.read_varint_i32()?,
            5 => self.ConsumeTime = r.read_varint_i32()?,
            6 => self.Finish = r.read_bool()?,
            7 => self.GoldPoints = r.read_varint_i32()?,
            8 => self.RankPoints = r.read_varint_i32()?,
            9 => self.DistancePoints = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSkillInfoPb {
    pub RoleCommonSkillInfoPb: Option<SkillInfoPb>,
    pub RoleSkillInfoPbList: Vec<RoleSkillInfoPb>,
}

impl PbMessage for PlayerSkillInfoPb {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.RoleCommonSkillInfoPb {
            crate::wire::write_submessage(out, 1, v);
        }
        for item in &self.RoleSkillInfoPbList {
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
                self.RoleCommonSkillInfoPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
                let bytes = r.read_len_delimited()?;
                self.RoleSkillInfoPbList
                    .push(<RoleSkillInfoPb as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerSkillInfoPbNotify {
    pub PlayerSkillInfoPb: Option<PlayerSkillInfoPb>,
}

impl PlayerSkillInfoPbNotify {
    pub const MSG_ID: u16 = 29311;
}

impl PbMessage for PlayerSkillInfoPbNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.PlayerSkillInfoPb {
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
                self.PlayerSkillInfoPb
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTeleportStateNotify {
    pub CurRolePosInfo: Option<EntitySimplyMoveInfo>,
    pub TeleportState: Option<EPlayerTeleportState>,
    pub PlayerId: i32,
}

impl PlayerTeleportStateNotify {
    pub const MSG_ID: u16 = 27864;
}

impl PbMessage for PlayerTeleportStateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.CurRolePosInfo {
            crate::wire::write_submessage(out, 5, v);
        }
        if let Some(v) = &self.TeleportState {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 11, crate::wire::WireType::Varint);
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
            5 => {
                let bytes = r.read_len_delimited()?;
                self.CurRolePosInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            8 => {
                self.TeleportState = Some(
                    <EPlayerTeleportState as crate::wire::PbEnum>::from_i32_or_default(
                        r.read_varint_i32()?,
                    ),
                )
            }
            11 => self.PlayerId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTitleInfo {
    pub PlayerTitleId: i32,
    pub IsUnlock: bool,
    pub UnlockTime: i64,
    pub ExtraInfo: i32,
    pub ConditionTask: Option<ConditionTask>,
}

impl PbMessage for PlayerTitleInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerTitleId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerTitleId);
        }
        if self.IsUnlock {
            crate::wire::write_tag(out, 2, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.UnlockTime != 0 {
            crate::wire::write_tag(out, 3, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.UnlockTime);
        }
        if self.ExtraInfo != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.ExtraInfo);
        }
        if let Some(v) = &self.ConditionTask {
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
            1 => self.PlayerTitleId = r.read_varint_i32()?,
            2 => self.IsUnlock = r.read_bool()?,
            3 => self.UnlockTime = r.read_varint_i64()?,
            4 => self.ExtraInfo = r.read_varint_i32()?,
            5 => {
                let bytes = r.read_len_delimited()?;
                self.ConditionTask
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTitleInfoUpdateNotify {
    pub PlayerTitleInfo: Vec<PlayerTitleInfo>,
    pub PlayerTitleLimitInfos: Vec<PlayerTitleLimitInfo>,
}

impl PlayerTitleInfoUpdateNotify {
    pub const MSG_ID: u16 = 20253;
}

impl PbMessage for PlayerTitleInfoUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerTitleInfo {
            crate::wire::write_submessage(out, 6, item);
        }
        for item in &self.PlayerTitleLimitInfos {
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
                let bytes = r.read_len_delimited()?;
                self.PlayerTitleInfo
                    .push(<PlayerTitleInfo as PbMessage>::decode(bytes)?);
            }
            8 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTitleLimitInfos
                    .push(<PlayerTitleLimitInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerTitleRequest {}

impl PlayerTitleRequest {
    pub const MSG_ID: u16 = 25420;
}

impl PbMessage for PlayerTitleRequest {
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
pub struct PlayerTitleResponse {
    pub PlayerTitleInfo: Vec<PlayerTitleInfo>,
    pub PlayerTitleLimitInfos: Vec<PlayerTitleLimitInfo>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerTitleResponse {
    pub const MSG_ID: u16 = 23404;
}

impl PbMessage for PlayerTitleResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerTitleInfo {
            crate::wire::write_submessage(out, 1, item);
        }
        for item in &self.PlayerTitleLimitInfos {
            crate::wire::write_submessage(out, 3, item);
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
            1 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTitleInfo
                    .push(<PlayerTitleInfo as PbMessage>::decode(bytes)?);
            }
            3 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTitleLimitInfos
                    .push(<PlayerTitleLimitInfo as PbMessage>::decode(bytes)?);
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
pub struct PlayerTitleUpdateNotify {
    pub RemovedPlayerTitleIds: Vec<i32>,
    pub PlayerTitleInfos: Vec<PlayerTitleInfo>,
}

impl PlayerTitleUpdateNotify {
    pub const MSG_ID: u16 = 24894;
}

impl PbMessage for PlayerTitleUpdateNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.RemovedPlayerTitleIds {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
        for item in &self.PlayerTitleInfos {
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
            7 => crate::wire::decode_repeated_scalar(
                wire_type,
                r,
                &mut self.RemovedPlayerTitleIds,
                |sr| Ok(sr.read_varint_i32()?),
            )?,
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PlayerTitleInfos
                    .push(<PlayerTitleInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerVoxelEnvRequest {
    pub VoxelEnv: i32,
}

impl PlayerVoxelEnvRequest {
    pub const MSG_ID: u16 = 27177;
}

impl PbMessage for PlayerVoxelEnvRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.VoxelEnv != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VoxelEnv);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            10 => self.VoxelEnv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerVoxelEnvResponse {
    pub ErrorCode: Option<ErrorCode>,
    pub VoxelEnv: i32,
}

impl PlayerVoxelEnvResponse {
    pub const MSG_ID: u16 = 15604;
}

impl PbMessage for PlayerVoxelEnvResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.ErrorCode {
            crate::wire::write_tag(out, 5, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if self.VoxelEnv != 0 {
            crate::wire::write_tag(out, 6, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.VoxelEnv);
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
            6 => self.VoxelEnv = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerXboxInfoGetRequest {
    pub XboxOnlineId: Vec<u8>,
}

impl PlayerXboxInfoGetRequest {
    pub const MSG_ID: u16 = 17106;
}

impl PbMessage for PlayerXboxInfoGetRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.XboxOnlineId.is_empty() {
            crate::wire::write_tag(out, 8, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxOnlineId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.XboxOnlineId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerXboxInfoGetResponse {
    pub Info: Option<PlayerDetails>,
    pub ErrorCode: Option<ErrorCode>,
}

impl PlayerXboxInfoGetResponse {
    pub const MSG_ID: u16 = 29123;
}

impl PbMessage for PlayerXboxInfoGetResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Info {
            crate::wire::write_submessage(out, 10, v);
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
                self.Info
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
pub struct PlayerXboxSessionNotify {
    pub PlayerId: i32,
    pub XboxSessionId: Vec<u8>,
}

impl PlayerXboxSessionNotify {
    pub const MSG_ID: u16 = 28993;
}

impl PbMessage for PlayerXboxSessionNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PlayerId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PlayerId);
        }
        if !self.XboxSessionId.is_empty() {
            crate::wire::write_tag(out, 10, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.XboxSessionId);
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
            10 => self.XboxSessionId = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PolluteGridCellRequest {
    pub TrapGridPlacementPbInfo: Option<GridPlacementPbInfo>,
    pub IsPolluted: bool,
}

impl PolluteGridCellRequest {
    pub const MSG_ID: u16 = 21401;
}

impl PbMessage for PolluteGridCellRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.TrapGridPlacementPbInfo {
            crate::wire::write_submessage(out, 2, v);
        }
        if self.IsPolluted {
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
                let bytes = r.read_len_delimited()?;
                self.TrapGridPlacementPbInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            7 => self.IsPolluted = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PolluteGridCellResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PolluteGridCellResponse {
    pub const MSG_ID: u16 = 19787;
}

impl PbMessage for PolluteGridCellResponse {
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
pub struct PollutedGridPbData {}

impl PbMessage for PollutedGridPbData {
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
pub struct PopularEntrie {
    pub InstId: i32,
    pub Entries: Vec<i32>,
    pub SeasonId: i32,
}

impl PbMessage for PopularEntrie {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.InstId != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.InstId);
        }
        for item in &self.Entries {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, *item);
        }
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
            1 => self.InstId = r.read_varint_i32()?,
            2 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.Entries, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            3 => self.SeasonId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PortalReplaceNotify {
    pub NewCreatorId: i64,
    pub EntityId: i64,
}

impl PortalReplaceNotify {
    pub const MSG_ID: u16 = 28606;
}

impl PbMessage for PortalReplaceNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NewCreatorId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.NewCreatorId);
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
            10 => self.NewCreatorId = r.read_varint_i64()?,
            12 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PositionAndRotate {
    pub Pos: Option<Vector>,
    pub Rot: Option<Rotator>,
}

impl PbMessage for PositionAndRotate {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Pos {
            crate::wire::write_submessage(out, 1, v);
        }
        if let Some(v) = &self.Rot {
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
                self.Pos
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            2 => {
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
pub struct PostAkEventNotify {
    pub DelayTime: f32,
    pub MusicEventType: Vec<u8>,
    pub PersistWhenExitDungeon: bool,
    pub IsBanVehicleRadio: bool,
    pub TreeOwnerId: i32,
    pub FadeOutTimeWhenFlowEnd: f32,
    pub AkEvent: Vec<u8>,
    pub TreeIncId: i64,
}

impl PostAkEventNotify {
    pub const MSG_ID: u16 = 19508;
}

impl PbMessage for PostAkEventNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.DelayTime != 0.0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.DelayTime).to_bits().to_le_bytes());
        }
        if !self.MusicEventType.is_empty() {
            crate::wire::write_tag(out, 2, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.MusicEventType);
        }
        if self.PersistWhenExitDungeon {
            crate::wire::write_tag(out, 3, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.IsBanVehicleRadio {
            crate::wire::write_tag(out, 4, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.TreeOwnerId != 0 {
            crate::wire::write_tag(out, 7, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.TreeOwnerId);
        }
        if self.FadeOutTimeWhenFlowEnd != 0.0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.FadeOutTimeWhenFlowEnd).to_bits().to_le_bytes());
        }
        if !self.AkEvent.is_empty() {
            crate::wire::write_tag(out, 9, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.AkEvent);
        }
        if self.TreeIncId != 0 {
            crate::wire::write_tag(out, 10, crate::wire::WireType::Varint);
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
            1 => self.DelayTime = r.read_f32()?,
            2 => self.MusicEventType = r.read_len_delimited()?.to_vec(),
            3 => self.PersistWhenExitDungeon = r.read_bool()?,
            4 => self.IsBanVehicleRadio = r.read_bool()?,
            7 => self.TreeOwnerId = r.read_varint_i32()?,
            8 => self.FadeOutTimeWhenFlowEnd = r.read_f32()?,
            9 => self.AkEvent = r.read_len_delimited()?.to_vec(),
            10 => self.TreeIncId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreOpenDetectionCtx {
    pub ConfigId: i32,
}

impl PbMessage for PreOpenDetectionCtx {
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
pub struct PreheatSignActivityInfo {
    pub PreheatSignNodeInfos: Vec<PreheatSignNodeInfo>,
}

impl PbMessage for PreheatSignActivityInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PreheatSignNodeInfos {
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
                self.PreheatSignNodeInfos
                    .push(<PreheatSignNodeInfo as PbMessage>::decode(bytes)?);
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignRewardRequest {
    pub AnswerStatus: bool,
    pub PreheatNodeId: i32,
}

impl PreheatSignRewardRequest {
    pub const MSG_ID: u16 = 20458;
}

impl PbMessage for PreheatSignRewardRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.AnswerStatus {
            crate::wire::write_tag(out, 7, WireType::Varint);
            crate::wire::write_varint_u64(out, 1);
        }
        if self.PreheatNodeId != 0 {
            crate::wire::write_tag(out, 13, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreheatNodeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            7 => self.AnswerStatus = r.read_bool()?,
            13 => self.PreheatNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignRewardResponse {
    pub Error: Option<ErrorCode>,
    pub PreheatSignSurveyInfo: Option<PreheatSignSurveyInfo>,
}

impl PreheatSignRewardResponse {
    pub const MSG_ID: u16 = 27792;
}

impl PbMessage for PreheatSignRewardResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 14, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PreheatSignSurveyInfo {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            15 => {
                let bytes = r.read_len_delimited()?;
                self.PreheatSignSurveyInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignSurveyInfo {
    pub SupportNum: i64,
    pub OpposeNum: i64,
    pub AnswerStatus: bool,
}

impl PbMessage for PreheatSignSurveyInfo {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.SupportNum != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.SupportNum);
        }
        if self.OpposeNum != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i64(out, self.OpposeNum);
        }
        if self.AnswerStatus {
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
            1 => self.SupportNum = r.read_varint_i64()?,
            2 => self.OpposeNum = r.read_varint_i64()?,
            3 => self.AnswerStatus = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignSurveyInfoRequest {
    pub PreheatNodeId: i32,
}

impl PreheatSignSurveyInfoRequest {
    pub const MSG_ID: u16 = 15013;
}

impl PbMessage for PreheatSignSurveyInfoRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.PreheatNodeId != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PreheatNodeId);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            8 => self.PreheatNodeId = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PreheatSignSurveyInfoResponse {
    pub Error: Option<ErrorCode>,
    pub PreheatSignSurveyInfo: Option<PreheatSignSurveyInfo>,
}

impl PreheatSignSurveyInfoResponse {
    pub const MSG_ID: u16 = 20369;
}

impl PbMessage for PreheatSignSurveyInfoResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if let Some(v) = &self.Error {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, crate::wire::PbEnum::to_i32(*v));
        }
        if let Some(v) = &self.PreheatSignSurveyInfo {
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
                self.Error = Some(<ErrorCode as crate::wire::PbEnum>::from_i32_or_default(
                    r.read_varint_i32()?,
                ))
            }
            11 => {
                let bytes = r.read_len_delimited()?;
                self.PreheatSignSurveyInfo
                    .get_or_insert_with(Default::default)
                    .merge_from(bytes)?;
            }
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressGameNotify {
    pub Payload: Vec<u8>,
}

impl PressGameNotify {
    pub const MSG_ID: u16 = 22674;
}

impl PbMessage for PressGameNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Payload = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressGameRequest {
    pub Payload: Vec<u8>,
    pub PayloadSize: i32,
    pub NotifyCount: i32,
}

impl PressGameRequest {
    pub const MSG_ID: u16 = 19675;
}

impl PbMessage for PressGameRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
        if self.PayloadSize != 0 {
            crate::wire::write_tag(out, 8, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PayloadSize);
        }
        if self.NotifyCount != 0 {
            crate::wire::write_tag(out, 15, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NotifyCount);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Payload = r.read_len_delimited()?.to_vec(),
            8 => self.PayloadSize = r.read_varint_i32()?,
            15 => self.NotifyCount = r.read_varint_i32()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressGameResponse {
    pub Payload: Vec<u8>,
}

impl PressGameResponse {
    pub const MSG_ID: u16 = 23963;
}

impl PbMessage for PressGameResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 11, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            11 => self.Payload = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressNotify {
    pub Payload: Vec<u8>,
}

impl PressNotify {
    pub const MSG_ID: u16 = 15929;
}

impl PbMessage for PressNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 15, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            15 => self.Payload = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressRequest {
    pub NotifyCount: i32,
    pub PayloadSize: i32,
    pub Payload: Vec<u8>,
}

impl PressRequest {
    pub const MSG_ID: u16 = 15361;
}

impl PbMessage for PressRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.NotifyCount != 0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.NotifyCount);
        }
        if self.PayloadSize != 0 {
            crate::wire::write_tag(out, 4, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.PayloadSize);
        }
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 14, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            2 => self.NotifyCount = r.read_varint_i32()?,
            4 => self.PayloadSize = r.read_varint_i32()?,
            14 => self.Payload = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PressResponse {
    pub Payload: Vec<u8>,
}

impl PressResponse {
    pub const MSG_ID: u16 = 25825;
}

impl PbMessage for PressResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if !self.Payload.is_empty() {
            crate::wire::write_tag(out, 6, WireType::LengthDelimited);
            crate::wire::write_bytes(out, &self.Payload);
        }
    }

    fn decode_field(
        &mut self,
        field_number: u32,
        wire_type: WireType,
        r: &mut Reader<'_>,
    ) -> Result<(), DecodeError> {
        match field_number {
            6 => self.Payload = r.read_len_delimited()?.to_vec(),
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PrivateChatClearNotify {
    pub PlayerId: Vec<i32>,
}

impl PrivateChatClearNotify {
    pub const MSG_ID: u16 = 19574;
}

impl PbMessage for PrivateChatClearNotify {
    fn encode(&self, out: &mut Vec<u8>) {
        for item in &self.PlayerId {
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
            14 => crate::wire::decode_repeated_scalar(wire_type, r, &mut self.PlayerId, |sr| {
                Ok(sr.read_varint_i32()?)
            })?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProgressBarFinishRequest {
    pub EntityId: i64,
}

impl ProgressBarFinishRequest {
    pub const MSG_ID: u16 = 24347;
}

impl PbMessage for ProgressBarFinishRequest {
    fn encode(&self, out: &mut Vec<u8>) {
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
            2 => self.EntityId = r.read_varint_i64()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProgressBarFinishResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl ProgressBarFinishResponse {
    pub const MSG_ID: u16 = 25134;
}

impl PbMessage for ProgressBarFinishResponse {
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
pub struct PropConfig {
    pub Id: i32,
    pub Value: f32,
    pub IsRatio: bool,
}

impl PbMessage for PropConfig {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.Id != 0 {
            crate::wire::write_tag(out, 1, crate::wire::WireType::Varint);
            crate::wire::write_varint_i32(out, self.Id);
        }
        if self.Value != 0.0 {
            crate::wire::write_tag(out, 2, crate::wire::WireType::Fixed32);
            out.extend_from_slice(&(self.Value).to_bits().to_le_bytes());
        }
        if self.IsRatio {
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
            1 => self.Id = r.read_varint_i32()?,
            2 => self.Value = r.read_f32()?,
            3 => self.IsRatio = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PsnSettingRequest {}

impl PsnSettingRequest {
    pub const MSG_ID: u16 = 29119;
}

impl PbMessage for PsnSettingRequest {
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
pub struct PsnSettingResponse {
    pub MatchPsnUser: bool,
}

impl PsnSettingResponse {
    pub const MSG_ID: u16 = 23928;
}

impl PbMessage for PsnSettingResponse {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MatchPsnUser {
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
            5 => self.MatchPsnUser = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PsnSettingUpdateRequest {
    pub MatchPsnUser: bool,
}

impl PsnSettingUpdateRequest {
    pub const MSG_ID: u16 = 24513;
}

impl PbMessage for PsnSettingUpdateRequest {
    fn encode(&self, out: &mut Vec<u8>) {
        if self.MatchPsnUser {
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
            12 => self.MatchPsnUser = r.read_bool()?,
            _ => r.skip_field(wire_type)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PsnSettingUpdateResponse {
    pub ErrorCode: Option<ErrorCode>,
}

impl PsnSettingUpdateResponse {
    pub const MSG_ID: u16 = 19977;
}

impl PbMessage for PsnSettingUpdateResponse {
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
